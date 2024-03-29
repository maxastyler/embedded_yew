use cyw43::NetDriver;
use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_time::Duration;
use picoserve as ps;
use ps::{
    response::{self, Content},
    routing::{get, parse_path_segment},
};
use static_cell::make_static;

use crate::{INDEX_HTML, YEW_JS, YEW_WASM};

struct EmbassyTimer;

pub const WEB_TASK_POOL_SIZE: usize = 5;

impl ps::Timer for EmbassyTimer {
    type Duration = embassy_time::Duration;
    type TimeoutError = embassy_time::TimeoutError;

    async fn run_with_timeout<F: core::future::Future>(
        &mut self,
        duration: Self::Duration,
        future: F,
    ) -> Result<F::Output, Self::TimeoutError> {
        embassy_time::with_timeout(duration, future).await
    }
}

type AppRouter = impl picoserve::routing::PathRouter;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(
    id: usize,
    stack: &'static embassy_net::Stack<cyw43::NetDriver<'static>>,
    app: &'static picoserve::Router<AppRouter>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];

    loop {
        let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

        log::info!("{id}: Listening on TCP:80...");
        if let Err(e) = socket.accept(80).await {
            log::warn!("{id}: accept error: {:?}", e);
            continue;
        }

        log::info!(
            "{id}: Received connection from {:?}",
            socket.remote_endpoint()
        );

        let (socket_rx, socket_tx) = socket.split();
        match picoserve::serve(
            app,
            EmbassyTimer,
            config,
            &mut [0; 2048],
            socket_rx,
            socket_tx,
        )
        .await
        {
            Ok(handled_requests_count) => {
                log::info!(
                    "{handled_requests_count} requests handled from {:?}",
                    socket.remote_endpoint()
                );
            }
            Err(err) => log::error!("{err:?}"),
        }
    }
}

fn make_app() -> ps::Router<AppRouter> {
    ps::Router::new()
        .route(
            "/",
            get(move || async move { response::File::html(INDEX_HTML) }),
        )
        .route(
            "/yew_part.js",
            get(move || async move { response::File::javascript(YEW_JS) }),
        )
        .route(
            "/yew_part_bg.wasm",
            get(move || async move { response::File::with_content_type("application/wasm", YEW_WASM) }),
        )
}

pub async fn start_server(spawner: &Spawner, stack: &'static Stack<NetDriver<'static>>) {
    let app = make_static!(make_app());

    let config = make_static!(picoserve::Config::new(picoserve::Timeouts {
        start_read_request: Some(Duration::from_secs(5)),
        read_request: Some(Duration::from_secs(1)),
        write: Some(Duration::from_secs(1)),
    })
    .keep_connection_alive());

    for id in 0..WEB_TASK_POOL_SIZE {
        spawner.must_spawn(web_task(id, stack, app, config));
    }
}
