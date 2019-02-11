// std crates
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use std::collections::HashSet;
use std::mem::discriminant;

#[macro_use]
extern crate futures;
use futures::{stream, Future, Stream, Poll, Async};
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

struct InstrumentDaemon {
	detectors: DetectorList,
}
impl InstrumentDaemon {
	fn new() -> InstrumentDaemon {
		InstrumentDaemon {
			detectors: get_detectors()
		}
	}
}

impl Future for InstrumentDaemon
{
	type Item = ();
	type Error = ();

	fn poll(&mut self) -> Poll<(), ()> {
		println!("Hello Tokio!");
		for detector in self.detectors.iter() {
			println!("Found detector: {}.", detector.get_name());
		}
		Ok(Async::Ready(()))
	}
}

fn main() {
	tokio::run(
		InstrumentDaemon::new()
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
