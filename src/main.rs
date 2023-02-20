extern crate tracing;

mod systemd;
mod process_manager;
mod service;

use std::env;
use async_signals::Signals;
use futures_util::StreamExt;
use color_eyre::{eyre::WrapErr, Result};
use tokio::{
	sync::{oneshot}
};
use tracing::{metadata::LevelFilter, info, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use zbus::ConnectionBuilder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install().wrap_err("failed to install color_eyre error handler")?;
	// Initializes the logger
    // 初始化日志记录器
    tracing_subscriber::registry()
		.with(tracing_journald::layer().wrap_err("failed to connect to journald")?)
		.with(fmt::layer())
		.with(
			EnvFilter::builder()
				.with_default_directive(LevelFilter::INFO.into())
				.from_env_lossy(),
		)
		.try_init()
		.wrap_err("failed to initialize logger")?;
    log_panics::init();

    info!("Starting yoyo-session");

    // Start yoyo-session.target
    // 启动 yoyo-session.target
	systemd::start_systemd_target()
		.await
		.wrap_err("failed to start systemd target")?;
	// Always stop the target when the process exits or panics.
    // 当进程退出或出现恐慌时，始终停止target
	scopeguard::defer! {
		if let Err(error) = systemd::stop_systemd_target() {
			error!("failed to stop systemd target: {:?}", error);
		}
	}

    // Start window manager
    // 启动窗口管理器
    process_manager::start_window_manager()
        .await
        .wrap_err("failed to start window manager")?;

    // Start daemon process
    // 启动守护进程
    process_manager::start_daemon_process()
        .await
        .wrap_err("failed to start daemon process")?;

    // Start desktop process
    // 启动桌面进程
    process_manager::start_desktop_process()
        .await
        .wrap_err("failed to start desktop process")?;
    
    let (exit_tx, exit_rx) = oneshot::channel();
	let _ = ConnectionBuilder::session()?
		.name("com.yoyo.Session")?
		.serve_at(
			"/Session",
			service::SessionService {
				exit_tx: Some(exit_tx),
			},
		)?
		.build()
		.await?;

    let mut signals = Signals::new(vec![libc::SIGTERM, libc::SIGINT]).unwrap();
	loop {
		tokio::select! {
			_ = exit_rx => {
				info!("session exited by request");
				break;
			},
			signal = signals.next() => match signal {
				Some(libc::SIGTERM | libc::SIGINT) => {
					info!("received request to terminate");
					break;
				}
				Some(signal) => unreachable!("received unhandled signal {}", signal),
				None => break,
			}
		}
	}
	tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let key = "QT_QPA_PLATFORM";
    env::set_var(key, "wayland");
    assert_eq!(env::var(key), Ok("wayland".to_string()));
    println!("{}",env::var("QT_QPA_PLATFORM").unwrap());
    Ok(())
}
