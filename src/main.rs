// std crates
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::collections::HashSet;
use std::mem::discriminant;

#[macro_use]
extern crate futures;
use futures::{stream, sync::mpsc, Future, Stream, Poll, Async, Sink, future::lazy};
extern crate tokio;


// crates.io crates
extern crate libusb;
extern crate crossbeam;

// local crates
extern crate foreigninstruments;
use foreigninstruments::*;

// ----------------------

//fn filter_on_supported_devices(device_iter: &libusb::Device<'_>) -> bool {
	//let device_desc = device_iter.device_descriptor().unwrap();
	//is_supported_vid_pid_pair(device_desc.vendor_id(), device_desc.product_id())
//}

//fn find_specific_device(device_iter: &libusb::Device<'_>, bus: u8, address: u8, vid: u16, pid: u16) -> bool {
	//let device_desc = device_iter.device_descriptor().unwrap();
	//device_iter.bus_number() == bus 
		//&& device_iter.address() == address
		//&& device_desc.vendor_id() == vid
		//&& device_desc.product_id() == pid
//}

//fn spawn_device_thread(bus: u8, address: u8, vid: u16, pid: u16) -> std::thread::JoinHandle<()> {
	//thread::spawn(move || {
		//match libusb::Context::new() {
			//Ok(context) => {
				//let device = context.devices().unwrap().iter()
					//.find(|di| find_specific_device(&&di, bus, address, vid, pid)).unwrap();
				//let device_desc = device.device_descriptor().unwrap();
				//println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
					//device.bus_number(),
					//device.address(),
					//device_desc.vendor_id(),
					//device_desc.product_id()
				//);
				//let config_descriptor = device.active_config_descriptor().expect("Error getting configuration descriptor.");
				//println!("Number of Interfaces: {:#?}", config_descriptor.num_interfaces());
				//for interface in config_descriptor.interfaces() {
					//for interface_descriptor in interface.descriptors() {
						//println!("Interface Number: {:#?}, Number of Endpoints: {:#?}", interface_descriptor.interface_number(), interface_descriptor.num_endpoints());
						//for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
							//println!("Endpoint Address: {:#?}, Direction: {:#?}",
								//endpoint_descriptor.address(),
								//endpoint_descriptor.direction(),
							//);
						//}
					//}
				//}
				//let mut handle = device.open().expect("LIBUSB: Failed to open specific device.");
				//if handle.kernel_driver_active(0).unwrap() {
					//println!("Kernel driver active. Unloading now.");
					//handle.detach_kernel_driver(0).expect("Unable to detach kernel driver.");
				//} else {
					//println!("Kernel driver is not active.");
				//}
				//handle.reset().expect("Failed to reset USB device.");
				//handle.claim_interface(0).expect("Could not claim interface.");
				//let handle_ref = Arc::new(handle);
				//crossbeam::thread::scope(|scope| {
					//let handle_ref_read_thread = Arc::clone(&handle_ref);
					//scope.spawn(move |_| {
						//let mut buffer: [u8; 200] = [0; 200];
						//eprintln!("Preparing to read.");
						//match handle_ref_read_thread.read_interrupt(0x81, &mut buffer, Duration::from_secs(std::u64::MAX/1024)) {
							//Ok(num) => {
								//eprintln!("Number bytes read: {:#?}, Byte[0] = {:02x}", num, buffer[0]);
							//},
							//Err(e) => {
								//eprintln!("Error reading device: {}", e);
							//},
						//}
					//});
					//let handle_ref_write_thread = Arc::clone(&handle_ref);
					//scope.spawn(move |_| {
						//let mut buffer: [u8; 60] = [0x0f; 60];
						//buffer[0] = 0x80;
						//eprintln!("Preparing to write.");
						//match handle_ref_write_thread.write_interrupt(0x01, &mut buffer, Duration::from_secs(std::u64::MAX/1024)) {
							//Ok(num) => {
								//eprintln!("Number bytes written: {:#?}, Byte[0] = {:02x}", num, buffer[0]);
							//},
							//Err(e) => {
								//eprintln!("Error writing device: {}", e);
							//},
						//};
					//});
				//}).unwrap();
			//},
			//Err(e) => {
				//eprintln!("Error opening device: {}", e);
			//},
		//}
	//})
//}

struct DetectorDescriptor {
	detector_name: String,
	detector_creation_function: fn() -> DetectorBoxed,
	instrument_manager_sender: mpsc::Sender<u8>
}
impl DetectorDescriptor {
	fn new(detector_name: String, detector_creation_function: fn() -> DetectorBoxed, instrument_manager_sender: mpsc::Sender<u8>) -> DetectorDescriptor {
		DetectorDescriptor {
			detector_name,
			detector_creation_function,
			instrument_manager_sender
		}
	}
}

struct InstrumentState {
	instrument_name: String,
	active: bool,
}
struct InstrumentManager {
	detector_instrument_receiver: mpsc::Receiver<u8>,
	instrument_states: Vec<InstrumentState>,
}
impl InstrumentManager {
	fn new(detector_instrument_receiver: mpsc::Receiver<u8>) -> InstrumentManager {
		InstrumentManager {
			detector_instrument_receiver,
			instrument_states: Vec::new(),
		}
	}
}

struct ManagementDaemon {
	detector_descriptors: Vec<Arc<DetectorDescriptor>>,
	instrument_manager: Arc<InstrumentManager>,
	//connector_manager: ConenectorManager,
	//renderer_manager: RendererManager,
}
impl ManagementDaemon {
	fn new() -> ManagementDaemon {
		let (tx, rx) = mpsc::channel(1024);
		ManagementDaemon {
			detector_descriptors: get_detector_creator_pairs().iter()
				.map(|(detector_name, detector_creator)| {
					Arc::new(
						DetectorDescriptor::new(detector_name.to_string(), *detector_creator, tx.clone())
					)
				})
				.collect(),
			instrument_manager: Arc::new(InstrumentManager::new(rx)),
			//connector_manager: ConenectorManager::new(),
			//renderer_manager: RendererManager::new(),
		}
	}
}
impl Future for ManagementDaemon
{
	type Item = ();
	type Error = ();

	fn poll(&mut self) -> Poll<(), ()> {
		println!("Starting Foreign Instruments daemon...");
		//let instrument_manager_ptr = Arc::clone(&self.instrument_manager);
		//tokio::spawn(lazy(move || {
			//instrument_manager_ptr.detector_instrument_receiver.for_each(|msg| {
				//println!("Got `{}`", msg);
				//Ok(())
			//})
		//}));
		self.detector_descriptors.iter().for_each(|detector_descriptor| {
			println!("Detector Manager Name: {}", detector_descriptor.detector_name);
			let detector_descriptor_ptr = Arc::clone(detector_descriptor);
			tokio::spawn(lazy(move || {
				let sender = &detector_descriptor_ptr.instrument_manager_sender;
				(detector_descriptor_ptr.detector_creation_function)().for_each(|num| {
					let num2 = num;
					sender.send(num2);
					Ok(())
				})
			}));
		});
			//let mut detector = detector_creator();
			//tokio::spawn(lazy( || {
				//println!("Found detector");
				//Ok(())
			//}));
			//match try_ready!(detector.poll()) {
				//Some(o) => {
					//println!("Ready: {}", o);
				//},
				//None => {
					//println!("NoNE");
				//}
				////Async::Ready() => {
					////println!("Async was ready");
				////},
				////Async::NotReady() => {
					////println!("Async was NOT ready");
				////}
			//};
		Ok(Async::Ready(()))
	}
}

fn main() {
	tokio::run(
		ManagementDaemon::new()
	)
}


		//match (try_ready!(detector.poll())) {
			//Some(i) => {
				//eprintln!("Found Instrument.",);
			//},
			//None => {
				//println!("No intrument found.");
			//}
		//};
	//}
		//match detector.detect_instrument() {
			//Ok(instrument) => {
				//eprintln!("Found Instrument: {}.", instrument.get_name());
				//match instrument.get_accessor() {
					//Some(accessor) => {
						//eprintln!("Accessor Initialization result: {}", accessor.initialize());
					//},
					//None => {
						//eprintln!("No accessor found.");
					//}
				//}
			//},
			//None => {
				//eprintln!("No intrument found.");
			//}
		//}
	//}
	// initialize each backend
	//		generate valid devices list
	//add each backend to refresh list
	//on refresh, scan for all valid devices
	//if device newly present, launch thread
	//if device was present, but now is gone, cleanup thread
	//if device still present and thread still active, do nothing
	//if device still present, but thread no longer active, constult accessor policy for device
	//let mut jhandles = Vec::new();
	//match libusb::Context::new() {
		//Ok(main_context) => {
			//assert!(main_context.has_hotplug());
			//assert!(main_context.has_hid_access());
			//assert!(main_context.supports_detach_kernel_driver());
			//for device in main_context.devices().unwrap().iter()
				//.filter(|di| filter_on_supported_devices(di))
				//.collect::<Vec<_>>()
			//{
				//let device_desc = device.device_descriptor().unwrap();
				//jhandles.push(
					//spawn_device_thread(
						//device.bus_number(),
						//device.address(),
						//device_desc.vendor_id(),
						//device_desc.product_id()
					//)
				//);
			//}
		//},
		//Err(e) => {
			//eprintln!("Error opening device: {}", e);
		//},
	//}
	//for jhandle in jhandles {
		//jhandle.join().expect("Thread did not join().");
	//}
