use core::task::Context;
use core::task::Poll;
use core::future::Future;
use std::pin::Pin;
// use tokio::prelude::*;

trait Detector: Future
{
	fn get_name(&self) -> String;
}

pub struct DummyDetector {
	name: String
}
impl DummyDetector {
	fn new() -> DummyDetector {
		DummyDetector {
			name: "Mrs. Dummy Detector".to_string()
		}
	}
}
impl Future for DummyDetector {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
		println!("{}", self.name);
		Poll::Ready(())
	}
}
impl Detector for DummyDetector {
	fn get_name(&self) -> String {
		self.name.to_string()
	}
}

type DetectorBoxed = Box<dyn Detector<Output = ()> + Send>;
type DetectorCreatorPair = (&'static str, fn() -> DetectorBoxed);

fn dummy_detector_boxed_creator() -> DetectorBoxed {
	Box::new(DummyDetector::new())
}

fn get_detector_creator_pairs() -> Vec<DetectorCreatorPair> {
	vec![
		("Dummy Detector", dummy_detector_boxed_creator)
	]
}

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
		for (detector_name, detector_creator) in get_detector_creator_pairs().iter() {
			println!("{} : {}", detector_name, detector_creator().get_name());
		};
		Poll::Ready(())
	}
}

#[tokio::main]
async fn main() {
	tokio::task::spawn(
		ManagementDaemon::new()
	);
}
