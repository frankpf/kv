extern crate bloom_filter;
extern crate bytes;

use hex::ToHex;
use bloom_filter::BloomFilter;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use openssl::sha::sha1;
use byteorder::{ByteOrder, LittleEndian};

struct KVItem {
	key: Bytes,
	val: Bytes,
}

const NUM_BUCKETS: usize = 16;
// NUM_BUCKETS = 8192;
// HTABLE_SIZE = 32MB;
// BUCKET_SIZE = 4KB;

struct Bucket {
	items: Vec<KVItem>,
	current: usize,
}

pub struct HTable {
	data: Vec<Bucket>,
	bf: BloomFilter,
}

impl Bucket {
	fn new() -> Bucket {
		Bucket {
			items: vec![],
			current: 0,
		}
	}

	fn put(&mut self, item: KVItem) {
		self.items.push(item);
		self.current += 1;
	}

	fn get(&self, key: Bytes) -> Option<&Bytes> {
		for i in 0..self.current {
			let it = &self.items[i];
			if it.key == key {
				return Some(&it.val)
			}
		}

		None
		/*
		self.items
			.iter()
			.find(|it| it.key == key)
			.map(|it| &it.val)
			*/
	}
}

impl HTable {
	fn new() -> HTable {
		let mut buckets = vec![];
		for i in 0..NUM_BUCKETS {
			buckets.push(Bucket::new());
		}

		HTable {
			data: buckets,
			bf: BloomFilter::new(1000),
		}
	}


	fn put(&mut self, key: Bytes, val: Bytes) {
		println!("Inserting...");

		// Hash the key
		let key_hash = sha1(key.as_ref());


		// Pick a bucket using the SHA-1 suffix (bits 96-159)
		// 8 bits * 12 u8's = bit 96
		let hash_suffix = &key_hash[12..];
		let num = LittleEndian::read_u64(hash_suffix);
		let bucket_idx = num as usize % NUM_BUCKETS;
		debug_assert!(bucket_idx <= NUM_BUCKETS);
		let bucket = &mut self.data[bucket_idx];

		let item = KVItem { key, val };
		bucket.put(item);
	}

	fn get(&self, key: Bytes) -> Option<&Bytes> {
		println!("Looking up...");
		let key_hash = sha1(key.as_ref());

		// Pick a bucket using the SHA-1 suffix (bits 96-159)
		// 8 bits * 12 u8's = bit 96
		let hash_suffix = &key_hash[12..];
		let num = LittleEndian::read_u64(hash_suffix);
		let bucket_idx = num as usize % NUM_BUCKETS;
		debug_assert!(bucket_idx <= NUM_BUCKETS);
		let bucket = &self.data[bucket_idx];

		bucket.get(key)
	}
}


#[test]
fn htable_test() {
	use super::*;

	let mut ht = HTable::new();

	let key = Bytes::from(&b"mykey"[..]);
	let val = Bytes::from(&b"AAA"[..]);


	ht.put(key, val);

	let key2 = Bytes::from(&b"mykey"[..]);

	match ht.get(key2) {
		Some(v) => println!("Found: {:?}", v),
		None => println!("Oops :("),
	}

	let key3 = Bytes::from(&b"mykey"[..]);
	let val3 = Bytes::from(&b"ZZZ"[..]);
	match ht.get(key3) {
		Some(v) => println!("Found: {:?}", v),
		None => println!("Oops :("),
	}
}
