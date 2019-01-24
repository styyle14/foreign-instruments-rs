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

pub trait Instrument {
	fn get_name(&self) -> String;
	fn get_detector(&self) -> Result<Box<dyn Detector>, &str>;
}
pub trait Detector {
	fn detect_device(&self) -> Result<Box<dyn Accessor>, &str> ;
}
pub trait Accessor {
	fn initialize(&self) -> bool;
}

pub struct ForeignInstrument{
	name: String,
	detector: Option<Box<dyn Detector>>
}
impl ForeignInstrument {
	pub fn new() -> ForeignInstrument {
		ForeignInstrument {
			name: "Unimplemented ForeignInstrument".to_string(),
			detector: None
		}
	}
}
impl Instrument for ForeignInstrument {
	fn get_name(&self) -> String {
		self.name.to_string()
	}
	fn get_detector(&self) -> Result<Box<dyn Detector>, &str> {
		Err("No detector implemented for this instrument.")
	}
}

pub struct DummyInstrument(ForeignInstrument);
impl DummyInstrument {
	fn new() -> ForeignInstrument {
		ForeignInstrument {
			name: "Dummy Instrument Dummy".to_string(),
			detector: Some(Box::new(DummyDetector {}))
		}
	}
}
pub struct DummyDetector;
impl Detector for DummyDetector {
	fn detect_device(&self) -> Result<Box<dyn Accessor>, &str> {
		Ok(
			Box::new(
				DummyAccessor{ }
			)
		)
	}
}
pub struct DummyAccessor;
impl Accessor for DummyAccessor {
	fn initialize(&self) -> bool {
		true
	}
}

pub fn get_foreign_instruments() -> Vec<Box<Instrument>> {
	vec![
		Box::new(DummyInstrument::new())
	]
}


//pub fn is_supported_vid_pid_pair(v: u16, p: u16) -> bool {
	////println!("Is valid? ID {:04x}:{:04x}", v, p);
	////FOREIGN_INSTRUMENTS.contains_key(&(v,p))
	//false
//}
