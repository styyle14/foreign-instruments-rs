mod types;
use types::foreign_instruments_types::{
	ForeignInstrumentDetails,
	BackendAccessor
};

mod devices;
use devices::*;

//pub fn get_distinct_backends() -> HashMap<Discriminant<BackendAccessor>,BackendAccessor> {
	//let mut backends = HashMap::new();
	//for instrument_details in FOREIGN_INSTRUMENT_DETAILS.iter() {
		//eprintln!("Finding backends for: {}", instrument_details.name);
		//for accessor_details in instrument_details.accessor_details_list.iter() {
			//let backend = BackendAccessor::new(&accessor_details.backend_device_details);
			//let backend_discriminant = backend.get_discriminant();
			//if ! backends.contains_key(&backend_discriminant) {
				//eprintln!("Adding new backend: {:#?}", backend_discriminant);
				//backends.insert(backend_discriminant, backend);
			//}
		//}
	//}
	////eprintln!("All backends: {:#?}", backends);
	//backends
//}

pub trait Detector {
	fn detect_instrument(&self) -> Option<Box<dyn Instrument>> ;
}
pub trait Instrument {
	fn get_name(&self) -> String;
	fn get_accessor(&self) -> Option<Box<dyn Accessor>>;
}
pub trait Accessor {
	fn initialize(&self) -> bool;
}


pub struct DummyDetector;
impl Detector for DummyDetector {
	fn detect_instrument(&self) -> Option<Box<dyn Instrument>> {
		Some(
			Box::new(
				DummyInstrument::new()
			)
		)
	}
}
pub struct DummyInstrument {
	name: String
}
impl DummyInstrument {
	fn new() -> DummyInstrument {
		DummyInstrument {
			name: "Dummy Instrument".to_string()
		}
	}
}
impl Instrument for DummyInstrument {
	fn get_name(&self) -> String {
		self.name.to_string()
	}
	fn get_accessor(&self) -> Option<Box<dyn Accessor>> {
		Some(Box::new(DummyAccessor{ }))
	}
}

pub struct DummyAccessor;
impl Accessor for DummyAccessor {
	fn initialize(&self) -> bool {
		true
	}
}

pub fn get_detectors() -> Vec<Box<Detector>> {
	vec![
		Box::new(DummyDetector{ })
	]
}


//pub fn is_supported_vid_pid_pair(v: u16, p: u16) -> bool {
	////println!("Is valid? ID {:04x}:{:04x}", v, p);
	////FOREIGN_INSTRUMENTS.contains_key(&(v,p))
	//false
//}
