use std::mem::{Discriminant,discriminant};

use types::backend_types::backend_libusb_types::{
	LibUsbDeviceDetails,
	BackendLibUsb
};

pub enum AccessorFallbackPolicy {
	AllowFallback(),
	TerminateOnFailure()
}

pub struct AccessorWatchdogPolicy {
	pub retry_attempts: u8,
	pub fallback_policy: AccessorFallbackPolicy
}

pub enum BackendAccessorDeviceDetails {
	BackendLibUsbDeviceDetails(LibUsbDeviceDetails),
	BackendDummyDeviceDetails()
}

pub struct AccessorDetails {
	pub watchdog_policy: AccessorWatchdogPolicy,
	pub backend_device_details: BackendAccessorDeviceDetails
}

pub struct ForeignInstrumentDetails {
	pub name: String,
	pub accessor_details_list: Vec<AccessorDetails>
}

pub enum BackendAccessor {
	AccessorLibUsb(BackendLibUsb),
	AccessorDummy()
}

impl BackendAccessor {
	pub fn new(device_details: &BackendAccessorDeviceDetails) -> BackendAccessor {
		match device_details {
			BackendAccessorDeviceDetails::BackendLibUsbDeviceDetails(d) => {
				BackendAccessor::AccessorLibUsb(BackendLibUsb::new())
			},
			BackendAccessorDeviceDetails::BackendDummyDeviceDetails() => {
				BackendAccessor::AccessorDummy()
			}
		}
	}
	pub fn get_discriminant(&self) -> Discriminant<BackendAccessor> {
		discriminant(&self)
	}
	//pub initialize(&self) -> Result<()>;
}
