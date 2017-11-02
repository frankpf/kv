use std::collections::BTreeMap;
use std::str;

const MAX_MEMTABLE_SIZE: usize = 5 * 1024 * 1024;

struct MemTable<'a> {
	data: BTreeMap<&'a [u8], &'a [u8]>,
	size: usize,
}

impl<'a> MemTable<'a> {
	fn new() -> MemTable<'a> {
		MemTable {
			data: BTreeMap::new(),
			size: 0
		}
	}

	fn get(&self, key: &[u8]) -> Option<&[u8]> {
		match self.data.get(key) {
			Some(v) => Some(*v),
			None => None,
		}
	}

	fn put(&mut self, key: &'a [u8], val: &'a [u8]) {
		self.size += key.len();
		self.size += val.len();

		self.data.insert(key, val);
	}

	fn delete(&mut self, key: &'a [u8]) -> Option<&[u8]> {
		if let Some(val) = self.data.remove(key) {
			self.size -= key.len();
			self.size -= val.len();
			Some(val)
		} else {
			None
		}
	}

	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
}

#[test]
fn memtable_test() {
	let key = b"email";
	println!("k: {:?}", key);
	let val = b"abcd@example.org";
	println!("v: {:?}", val);
	let mut mt = MemTable::new();
	println!("Current size: {}", mt.size);

	assert_eq!(mt.get(key), None);

	mt.put(key, val);
	println!("Current size: {}", mt.size);
	assert_eq!(mt.get(key).unwrap(), val);

	let new_val = b"efgh@example.org";
	mt.put(key, new_val);
	println!("Current size: {}", mt.size);
	assert_eq!(mt.get(key).unwrap(), new_val);

	assert_eq!(mt.delete(key).unwrap(), new_val);
	println!("Current size: {}", mt.size);
}

struct SSTable {}
struct LSMTree<'a> {
	buffer: MemTable<'a>,
//	tables: Vec<SSTable>,
}

impl<'a> LSMTree<'a> {
	fn new() -> LSMTree<'a> {
		unimplemented!();
	}

	fn compact_and_merge(&self, tables: Vec<&SSTable>) {
		unimplemented!();
	}
}

struct WriteLog {

}

struct SegmentFile {

}
