static VALID_VID_PID_PAIRS: [(u16,u16); 1] = [
	//(0x17CC, 0x1340), // Komplete Kontrol S25
	(0x17CC, 0x1500), // Maschine Jam
	//(0x17CC, 0x1700), // Maschine Mikro MK3
	
];

pub fn is_valid_vid_pid_pair(v: u16, p: u16) -> bool {
	//println!("Is valid? ID {:04x}:{:04x}", v, p);
	VALID_VID_PID_PAIRS.iter().any(|&pair| pair == (v,p))
}
