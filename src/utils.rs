fn mod_bytes(bytes: &[u8], num: usize) -> usize {
	if num == 0 {
		return 0;
	}

	if (num & (num-1)) != 0 {
		panic!("{} is not a power of 2!", num);
	}

	let pow = (num as f64).log(2.0) as usize;
	let mut rem: usize = 0;
	let base: usize = 2;
	for i in 0..pow {
		let offset = i % 8;
		let bit = 1 << offset;
		let byte_group = i / 8;
		let curr = 255 * byte_group + (bytes[bytes.len() - byte_group - 1] & bit) as usize;
		rem += curr;
	}
	rem
}

#[test]
fn mod_bytes_test() {
	let x: [u8; 11] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
	let n406: [u8; 2] = [1_u8, 0b10010110_u8];
	let n45530: [u8; 2] = [0b10110001_u8, 0b11011010_u8];
	let n45530: [u8; 2] = [0b10110001_u8, 0b11011010_u8];

	let r1 = mod_bytes(&n406, 64);
	assert!(r1 == 22, "406 % 64 = {}, should be {}", r1, 22);
	let r2 = mod_bytes(&n45530, 4);
	assert!(r2 == 2, "45530 % 4 = {}, should be {}", r2, 2);
	let r3 = mod_bytes(&n45530, 16);
	assert!(r3 == 10, "45530 % 16 = {}, should be {}", r3, 10);
}
