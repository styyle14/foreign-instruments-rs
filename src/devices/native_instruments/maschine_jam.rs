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
		name: "Maschine Jam".to_string(),
		accessor_details_list: vec![
			AccessorDetails {
				watchdog_policy: AccessorWatchdogPolicy {
					retry_attempts: 1,
					fallback_policy: TerminateOnFailure()
				},
				backend_device_details: BackendAccessorDeviceDetails::BackendLibUsbDeviceDetails(
					LibUsbDeviceDetails {
						vendor_id: 0x17CC,
						product_id: 0x1500,
						interfaces: vec![
							LibUsbInterfaceDetails { // Human Interaction Interface
								number: 0,
								device_class: HID,
								endpoints: vec![
									LibUsbEndpointDetails { // Button, Knob, Smartstrip Inputs
										address: 0x81,
										direction: IN
									},
									LibUsbEndpointDetails { // LED Outputs
										address: 0x01,
										direction: OUT
									},
								]
							},
							LibUsbInterfaceDetails { // Firmware Interface
								number: 1,
								device_class: UNSUPPORTED,
								endpoints: vec![ ]
							},
						]
					}
				)
			},
		]
	}
}
