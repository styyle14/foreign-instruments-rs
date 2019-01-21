#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
	static ref VID_PID_DEVICE_HASHMAP: HashMap<(u16,u16), &'static str> = {
		let mut m = HashMap::new();
		//vid_pid_device_data_hashmap.insert((0x17CC, 0x1340), "foo"); // Komplete Kontrol S25
		m.insert((0x17CC, 0x1500), "bar"); // Maschine Jam
		//vid_pid_device_data_hashmap.insert((0x17CC, 0x1700), "baz"); // Maschine Mikro MK3
		m
	};
}

pub fn is_valid_vid_pid_pair(v: u16, p: u16) -> bool {
	//println!("Is valid? ID {:04x}:{:04x}", v, p);
	VID_PID_DEVICE_HASHMAP.contains_key(&(v,p))
}
