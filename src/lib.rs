use std::collections::{HashMap,HashSet};
use std::mem::Discriminant;

#[macro_use]
extern crate lazy_static;

mod types;
use types::foreign_instruments_types::{
	ForeignInstrumentDetails,
	BackendAccessor
};

mod devices;
use devices::*;

lazy_static! {
	pub static ref FOREIGN_INSTRUMENT_DETAILS: Vec<ForeignInstrumentDetails> = {
		vec![
			native_instruments::komplete_kontrol_s25::details(), // Komplete Kontrol S25
			native_instruments::maschine_jam::details(), // Maschine Jam
			//vid_pid_device_data_hashmap.insert((0x17CC, 0x1700), "baz"); // Maschine Mikro MK3
		]
	};
}

pub fn get_distinct_backends() -> HashMap<Discriminant<BackendAccessor>,BackendAccessor> {
	let mut backends = HashMap::new();
	for instrument_details in FOREIGN_INSTRUMENT_DETAILS.iter() {
		eprintln!("Finding backends for: {}", instrument_details.name);
		for accessor_details in instrument_details.accessor_details_list.iter() {
			let backend = BackendAccessor::new(&accessor_details.backend_device_details);
			let backend_discriminant = backend.get_discriminant();
			if ! backends.contains_key(&backend_discriminant) {
				eprintln!("Adding new backend: {:#?}", backend_discriminant);
				backends.insert(backend_discriminant, backend);
			}
		}
	}
	//eprintln!("All backends: {:#?}", backends);
	backends
}


pub trait Detector {
	fn detect_device(&self) -> Box<dyn Accessor>;
}

pub struct DummyDetector;

impl Detector for DummyDetector {
	fn detect_device(&self) -> Box<dyn Accessor> {
		Box::new(
			DummyAccessor{ }
		)
	}
}


pub trait Accessor {
	fn initialize(&self) -> Result();
}

pub struct DummyAccessor;
impl Accessor for DummyAccessor {
	fn get(&self) -> u8 {
		9
	}
}


//pub fn is_supported_vid_pid_pair(v: u16, p: u16) -> bool {
	////println!("Is valid? ID {:04x}:{:04x}", v, p);
	////FOREIGN_INSTRUMENTS.contains_key(&(v,p))
	//false
//}
