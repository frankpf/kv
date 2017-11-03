use std::collections::{BTreeMap, HashSet};
use std::collections::btree_map::Iter as BTreeIterator;
use std::str;

const MAX_MEMTABLE_SIZE: usize = 5 * 1024 * 1024;

use std::{u8,u32};

/* Max number of bytes the key and value may take */
const MAX_KEY_SIZE: usize = u8::MAX as usize;
const MAX_VAL_SIZE: usize = u32::MAX as usize;

#[derive(Debug)]
struct MemTable<'a> {
	size: usize,
	map: BTreeMap<&'a [u8], &'a [u8]>,
	deleted: HashSet<&'a [u8]>,
}

impl<'a> MemTable<'a> {
	fn new() -> MemTable<'a> {
		MemTable {
			size: 0,
			map: BTreeMap::new(),
			deleted: HashSet::new(),
		}
	}

	fn get(&self, key: &[u8]) -> Option<&[u8]> {
		if self.deleted.contains(key) {
			return None
		}

		match self.map.get(key) {
			Some(v) => Some(v),
			None => None,
		}
	}

	// TODO: Use real errors, not strings
	fn put(&mut self, key: &'a [u8], val: &'a [u8]) -> Result<(), &'static str> {
		if key.len() >= MAX_KEY_SIZE {
			return Err("Key is too big")
		}

		if val.len() >= MAX_VAL_SIZE {
			return Err("Value is too big")
		}

		if self.deleted.contains(key) {
			self.deleted.remove(key);
		}

		self.size += key.len();
		self.size += val.len();

		self.map.insert(key, val);

		Ok(())
	}

	fn delete(&mut self, key: &'a [u8]) -> Option<&[u8]> {
		if let Some(val) = self.map.remove(key) {
			self.deleted.insert(key);
			/* Return the deleted value */
			self.size -= key.len();
			self.size -= val.len();
			Some(val)
		} else {
			/* Nothing was deleted */
			None
		}
	}

	fn clear(&mut self) {
		self.size = 0;
		self.map.clear();
		self.deleted.clear();
	}

	fn iter(&self) -> BTreeIterator<&[u8], &[u8]> {
		self.map.iter()
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
