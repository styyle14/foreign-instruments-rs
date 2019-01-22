use types::foreign_instruments_types::{
	ForeignInstrumentDetails,
	AccessorDetails,
	AccessorWatchdogPolicy,
	AccessorFallbackPolicy::{TerminateOnFailure},
	BackendAccessorDeviceDetails
};
use types::backend_types::backend_libusb_types::{
	LibUsbDeviceDetails,
	LibUsbInterfaceDetails,
	LibUsbInterfaceSupportedClass::{HID, UNSUPPORTED},
	LibUsbEndpointDetails,
	LibUsbEndpointDirection::{IN, OUT}
};

pub fn details() -> ForeignInstrumentDetails {
	ForeignInstrumentDetails {
		name: "Komplete Kontrol S25".to_string(),
		accessor_details_list: vec![
			AccessorDetails {
				watchdog_policy: AccessorWatchdogPolicy {
					retry_attempts: 1,
					fallback_policy: TerminateOnFailure()
				},
				backend_device_details: BackendAccessorDeviceDetails::BackendDummyDeviceDetails()
			},
		]
	}
}
