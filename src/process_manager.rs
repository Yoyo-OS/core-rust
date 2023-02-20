use color_eyre::{Result};
use launch_pad::{process::Process, ProcessManager};
use tracing::{info_span, info, Instrument, warn};

pub async fn start_window_manager() -> Result<()> {
	let process_manager = ProcessManager::new().await;

    let span = info_span!(parent: None, "kwin_wayland");
	let stdout_span = span.clone();
	let stderr_span = span.clone();
	process_manager
		.start(
			Process::new()
				.with_executable("kwin_wayland")
				.with_on_stdout(move |_, _, line| {
					let stdout_span = stdout_span.clone();
					async move {
						info!("{}", line);
					}
					.instrument(stdout_span)
				})
				.with_on_stderr(move |_, _, line| {
					let stderr_span = stderr_span.clone();
					async move {
						warn!("{}", line);
					}
					.instrument(stderr_span)
				}),
		)
		.await
		.expect("failed to start window manager");

	Ok(())
}

pub async fn start_desktop_process() -> Result<()> {
	let process_manager = ProcessManager::new().await;
    let list = [
        "yoyo-dock",
        "yoyo-desktop",
        "yoyo-launcher",
        "yoyo-powerman",
        "yoyo-wallpaper-color-pick"
    ];

    for process_name in list.iter() {
        let mut s = "failed to start ".to_string();
        s += &process_name.to_string();

        let span = info_span!(parent: None, "desktop_process");
        let stdout_span = span.clone();
        let stderr_span = span.clone();
        process_manager
		.start(
			Process::new()
				.with_executable(process_name)
                .with_on_stdout(move |_, _, line| {
					let stdout_span = stdout_span.clone();
					async move {
						info!("{}", line);
					}
					.instrument(stdout_span)
				})
				.with_on_stderr(move |_, _, line| {
					let stderr_span = stderr_span.clone();
					async move {
						warn!("{}", line);
					}
					.instrument(stderr_span)
				}),
		)
		.await
		.expect(&s);
    }
	Ok(())
}

pub async fn start_daemon_process() -> Result<()> {
	let process_manager = ProcessManager::new().await;
    let list = [
        "yoyo-settings-daemon",
        "yoyo-xembedsniproxy",
        "yoyo-gmenuproxy",
        "yoyo-permission-surveillance",
        "yoyo-clipboard",
        "yoyo-chotkeys",
        "yoyo-notificationd"
    ];

    for process_name in list.iter() {
        let mut s = "failed to start ".to_string();
        s += &process_name.to_string();

        let span = info_span!(parent: None, "daemon_process");
        let stdout_span = span.clone();
        let stderr_span = span.clone();
        process_manager
		.start(
			Process::new()
				.with_executable(process_name)
                .with_on_stdout(move |_, _, line| {
					let stdout_span = stdout_span.clone();
					async move {
						info!("{}", line);
					}
					.instrument(stdout_span)
				})
				.with_on_stderr(move |_, _, line| {
					let stderr_span = stderr_span.clone();
					async move {
						warn!("{}", line);
					}
					.instrument(stderr_span)
				}),
		)
		.await
		.expect(&s);
    }
	Ok(())
}