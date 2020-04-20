use core::task::Context;
use core::task::Poll;
use core::future::Future;
use std::pin::Pin;
// use tokio::prelude::*;

struct ManagementDaemon {
	name: String,
}
impl ManagementDaemon {
	fn new() -> ManagementDaemon {
		ManagementDaemon {
			name: "Mr. Management Daemon".to_string()
		}
	}
}
impl Future for ManagementDaemon
{
	type Output = ();

	fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
		println!("{}", self.name);
		Poll::Ready(())
	}
}

#[tokio::main]
async fn main() {
	tokio::task::spawn(
		ManagementDaemon::new()
	);
}
