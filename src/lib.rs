mod types;
use types::foreign_instruments_types::{
	ForeignInstrumentDetails,
	BackendAccessor
};

extern crate futures;
use futures::{Stream, Poll, Async, stream};

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

pub trait Detector : Stream {
	fn get_name(&self) -> String;
	//fn detect_instruments(&mut self) -> Box<dyn Stream<Item = Box<dyn Instrument>, Error = ()>>;
}
pub trait Instrument {
	fn get_name(&self) -> String;
	fn get_accessor(&self) -> Option<Box<dyn Accessor>>;
}
pub trait Accessor {
	fn initialize(&self) -> bool;
}


pub struct DummyDetector{
	name: String
}
impl DummyDetector {
	fn new() -> DummyDetector {
		DummyDetector {
			name: "Dummy Detector".to_string()
		}
	}
}
impl Detector for DummyDetector {
	fn get_name(&self) -> String {
		self.name.to_string()
	}
}
impl Stream for DummyDetector {
	type Item = u8;

	// The stream will never yield an error
	type Error = ();
	
	fn poll(&mut self) -> Poll<Option<u8>, ()> {
		Ok(Async::Ready(Some(9)))
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

pub type DetectorList = Vec<Box<Detector<Item = u8, Error = ()> + Send>>;

pub fn get_detectors() -> DetectorList {
	vec![
		Box::new(DummyDetector::new())
	]
}


//pub fn is_supported_vid_pid_pair(v: u16, p: u16) -> bool {
	////println!("Is valid? ID {:04x}:{:04x}", v, p);
	////FOREIGN_INSTRUMENTS.contains_key(&(v,p))
	//false
//}
