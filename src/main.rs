extern crate tracing;

mod systemd;
mod process_manager;

use std::env;
use color_eyre::{eyre::WrapErr, Result};
use tracing::{metadata::LevelFilter, info, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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

    let key = "QT_QPA_PLATFORM";
    env::set_var(key, "wayland");
    assert_eq!(env::var(key), Ok("wayland".to_string()));
    println!("{}",env::var("QT_QPA_PLATFORM").unwrap());
    Ok(())
}
