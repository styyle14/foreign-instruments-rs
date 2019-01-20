// std crates
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

// crates.io crates
extern crate libusb;

// local crates
extern crate foreigninstruments;
use foreigninstruments::is_valid_vid_pid_pair;


fn filter_valid_devices(device_iter: &libusb::Device<'_>) -> bool {
	let device_desc = device_iter.device_descriptor().unwrap();
	is_valid_vid_pid_pair(device_desc.vendor_id(), device_desc.product_id())
}

fn find_specific_device(device_iter: &libusb::Device<'_>, bus: u8, address: u8, vid: u16, pid: u16) -> bool {
	let device_desc = device_iter.device_descriptor().unwrap();
	device_iter.bus_number() == bus 
		&& device_iter.address() == address
		&& device_desc.vendor_id() == vid
		&& device_desc.product_id() == pid
}

fn spawn_device_thread(bus: u8, address: u8, vid: u16, pid: u16) -> std::thread::JoinHandle<()> {
	thread::spawn(move || {
		match libusb::Context::new() {
			Ok(new_context) => {
				let context_ref = Arc::new(Mutex::new(new_context));
				let context_thread = &*context_ref.lock().unwrap();
				let device = context_thread.devices().unwrap().iter()
					.find(|di| find_specific_device(&&di, bus, address, vid, pid)).unwrap();
				let device_desc = device.device_descriptor().unwrap();
				println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
					device.bus_number(),
					device.address(),
					device_desc.vendor_id(),
					device_desc.product_id()
				);
				let config_descriptor = device.active_config_descriptor().expect("Error getting configuration descriptor.");
				println!("Number of Interfaces: {:#?}", config_descriptor.num_interfaces());
				for interface in config_descriptor.interfaces() {
					for interface_descriptor in interface.descriptors() {
						println!("Interface Number: {:#?}, Number of Endpoints: {:#?}", interface_descriptor.interface_number(), interface_descriptor.num_endpoints());
						for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
							println!("Endpoint Address: {:#?}, Direction: {:#?}", endpoint_descriptor.address(), endpoint_descriptor.direction());
						}
					}
				}
				let mut handle = device.open().expect("LIBUSB: Failed to open specific device.");
				if handle.kernel_driver_active(0).unwrap() {
					println!("Kernel driver active. Unloading now.");
					handle.detach_kernel_driver(0).expect("Unable to detach kernel driver.");
				} else {
					println!("Kernel driver is not active.");
				}
				handle.claim_interface(0).expect("Could not claim interface.");
				let mut buffer: [u8; 200] = [0; 200];
				handle.read_interrupt(0x81, &mut buffer, Duration::new(50, 0));
				let ll = handle.read_languages(Duration::new(5, 0)).expect("test");
				for l in ll {
					println!("{:#?}", l);
				}
			},
			Err(e) => {
				eprintln!("Error opening device: {}", e);
			},
		}
	})
}

fn main() {
	let mut jhandles = Vec::new();
	match libusb::Context::new() {
		Ok(main_context) => {
			assert!(main_context.has_hotplug());
			assert!(main_context.has_hid_access());
			assert!(main_context.supports_detach_kernel_driver());
			for device in main_context.devices().unwrap().iter()
				.filter(|di| filter_valid_devices(di))
				.collect::<Vec<_>>()
			{
				let device_desc = device.device_descriptor().unwrap();
				jhandles.push(
					spawn_device_thread(
						device.bus_number(),
						device.address(),
						device_desc.vendor_id(),
						device_desc.product_id()
					)
				);
			}
		},
		Err(e) => {
			eprintln!("Error opening device: {}", e);
		},
	}
	for jhandle in jhandles {
		jhandle.join().expect("Thread did not join().");
	}
}
