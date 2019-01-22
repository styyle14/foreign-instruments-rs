pub enum LibUsbInterfaceSupportedClass {
	HID,
	UNSUPPORTED
}

pub enum LibUsbEndpointDirection {
	IN,
	OUT
}

pub struct LibUsbEndpointDetails {
	pub address: u8,
	pub direction: LibUsbEndpointDirection
}

pub struct LibUsbInterfaceDetails {
	pub number: u8,
	pub device_class: LibUsbInterfaceSupportedClass,
	pub endpoints: Vec<LibUsbEndpointDetails>
}

pub struct LibUsbDeviceDetails {
	pub vendor_id: u16,
	pub product_id: u16,
	pub interfaces: Vec<LibUsbInterfaceDetails>
}

impl LibUsbDeviceDetails {
	pub fn new() -> LibUsbDeviceDetails {
		LibUsbDeviceDetails {
			vendor_id: 0,
			product_id: 0,
			interfaces: vec![ ]
		}
	}
}

pub struct BackendLibUsb {
	
}

impl BackendLibUsb {
	pub fn new() -> BackendLibUsb {
		BackendLibUsb {
			
		}
	}
}
