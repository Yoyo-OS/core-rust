// SPDX-License-Identifier: GPL-3.0-only

use tokio::sync::oneshot;
use tracing::{warn};
use zbus::dbus_interface;

pub struct SessionService {
	pub exit_tx: Option<oneshot::Sender<()>>,
}

#[dbus_interface(name = "com.yoyo.Session")]
impl SessionService {
	fn logout(&mut self) {
		match self.exit_tx.take() {
			Some(tx) => {
				tx.send(()).ok();
			}
			None => {
				warn!("previously failed to properly exit session");
			}
		}
	}

	fn restart(&self) {
		warn!("restarting session");
	}
}