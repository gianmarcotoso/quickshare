pub mod state;

use std::{net::SocketAddr, time::Duration};

use tokio::sync::watch;

use crate::{
    app::state::AppState,
    core::{app_config::AppConfig, session::Session, version::VERSION},
    features::uploads::actions::close_all_batches::close_all_batches,
    libs::{log_timestamp::log_timestamp, network_urls::network_urls, terminal_qr::terminal_qr},
    router::router,
};

const SHUTDOWN_GRACE: Duration = Duration::from_secs(5);

pub fn init(config: AppConfig, executable_sha256: String) -> AppState {
    AppState::new(config, Session::new(executable_sha256))
}

pub async fn start(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let config = state.config();
    let session = state.session();
    let address = format!("{}:{}", config.bind, config.port);

    tracing::info!(version = VERSION, executable_sha256 = %session.executable_sha256(), "quickshare");

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|error| {
            let hint = match error.kind() {
                std::io::ErrorKind::AddrInUse => {
                    " (port already in use: is another quickshare running?)"
                }
                std::io::ErrorKind::AddrNotAvailable => {
                    " (no interface has this address: is the hotspot up?)"
                }
                _ => "",
            };

            format!("cannot listen on {address}: {error}{hint}")
        })?;

    tracing::info!(
        started = %log_timestamp(session.started_at()),
        token = %session.token(),
        folder = %config.data_path.display(),
        "session started"
    );
    tracing::info!(%address, "listening");

    let urls = network_urls(&config.bind, config.port);

    if urls.is_empty() {
        tracing::warn!("no network interface found: the page is reachable only from this computer");
    }

    for network_url in &urls {
        let url = format!("{}?t={}", network_url.url, session.token());

        if network_url.interface.is_empty() {
            tracing::info!(%url, "open");
        } else {
            tracing::info!(%url, interface = %network_url.interface, "open");
        }

        if let Some(qr) = terminal_qr(&url) {
            eprintln!("{qr}");
        }
    }

    let (signal_sender, signal) = watch::channel(false);
    tokio::spawn(async move {
        wait_for_signal().await;
        let _ = signal_sender.send(true);
    });

    let graceful = {
        let mut signal = signal.clone();
        async move {
            let _ = signal.wait_for(|received| *received).await;
            tracing::info!("stopping: waiting for running uploads");
        }
    };

    let server = axum::serve(
        listener,
        router(state.clone()).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(graceful);

    let mut timeout = signal;
    tokio::select! {
        result = server => result?,
        _ = async {
            let _ = timeout.wait_for(|received| *received).await;
            tokio::time::sleep(SHUTDOWN_GRACE).await;
        } => tracing::warn!("uploads still running: stopping anyway"),
    }

    close_all_batches(&state);

    tracing::info!("session closed");

    Ok(())
}

async fn wait_for_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("cannot listen for SIGTERM");

        tokio::select! {
            _ = ctrl_c => {}
            _ = terminate.recv() => {}
        }
    }

    #[cfg(not(unix))]
    let _ = ctrl_c.await;
}
