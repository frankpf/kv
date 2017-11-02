// TODO: Remove these
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate openssl;
extern crate byteorder;
extern crate bloom_filter;
extern crate bytes;
extern crate hex;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};


mod lsm;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROW_SIZE: usize = 100; // CHECK THIS
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE; // CHECK THIS
// const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

struct Table {
	pager: Pager,
	num_rows: usize
}

type Page = Vec<u8>;

struct Pager {
	file: File,
	file_len: usize,
	pages: Vec<Option<Page>>,
}

impl Pager {
	fn new(filename: &str) -> Pager {
		let mut file = OpenOptions::new()
					.read(true)
					.write(true)
					.create(true)
					.open(filename)
					.expect("Could not open file");

		let file_len = file.seek(SeekFrom::End(0)).unwrap();

		Pager {
			file,
			file_len: file_len as usize,
			pages: vec![None; TABLE_MAX_PAGES],
		}
	}

	fn get_page(&mut self, page_num: usize) -> Option<&Page> {
		if page_num > TABLE_MAX_PAGES {
			panic!("Page out of bounds ({} > {})", page_num, TABLE_MAX_PAGES);
		}

		if self.pages[page_num] == None {
			let page = vec![0; PAGE_SIZE];
			// Cache miss. Allocate memory and load from file
			let mut num_pages = self.file_len / PAGE_SIZE;

			if self.file_len % PAGE_SIZE == 0 {
				num_pages += 1;
			}

			if page_num < num_pages {
				let offset = page_num * PAGE_SIZE;
				self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
				let mut buf = vec![0u8; PAGE_SIZE];
				self.file.read_exact(&mut buf).unwrap();
			}

			self.pages[page_num] = Some(page);
		}

		self.pages[page_num].as_ref()
	}

	fn sync(&mut self, page_num: usize, size: usize) {
		let page = self.pages[page_num].as_ref();

		match page {
			Some(ref content) => {
				let offset = (page_num * PAGE_SIZE) as u64;
				self.file.seek(SeekFrom::Start(offset)).unwrap();

				self.file.write(&content).unwrap();
			},
			None => panic!("tried to sync a null page"),
		}

	}
}

impl Table {
	fn start(&self) -> Cursor {
		Cursor {
			table: self,
			row_num: 0,
			end_of_table: false,
		}
	}

	fn end(&self) -> Cursor {
		Cursor {
			table: self,
			row_num: self.num_rows,
			end_of_table: true,
		}
	}
}

/*
impl<'a> Cursor<'a> {
	fn value(&self) -> Option<&Page> {
		let row_num = self.row_num;
		let page_num = row_num / ROWS_PER_PAGE;
		let page = self.table.pager.get_page(page_num);
//		let row_offset = row_num % ROWS_PER_PAGE;
		let byte_offset = row_offset * ROW_SIZE;
	}
}
*/

struct Cursor<'a> {
	/* This lifetime specifier indicates that `table` lives at least as long as the cursor */
	table: &'a Table,
	row_num: usize,
	/* Indicates if the cursor is one position past the last row */
	end_of_table: bool, // Indicates that the cursor is in the position (last element + 1)
}

fn db_open(filename: &str) -> Table {
	let pager = Pager::new(filename);
	let num_rows = pager.file_len / ROW_SIZE;

	Table {
		pager,
		num_rows,
	}
}

fn db_close(table: Table) {
	let mut pager = table.pager;
	let num_full_pages = table.num_rows / PAGE_SIZE;

	for i in 0..num_full_pages {
		if pager.pages[i] == None {
			continue;
		}

		pager.sync(i, PAGE_SIZE);
		pager.pages[i] = None;
	}

	let num_additional_rows = table.num_rows % ROWS_PER_PAGE;
	if num_additional_rows > 0 {
		let page_num = num_full_pages;
		if let Some(_) = pager.pages[page_num] {
			pager.sync(page_num, num_additional_rows * ROW_SIZE);
			pager.pages[page_num] = None;
		}
	}

	for i in 0..TABLE_MAX_PAGES {
		pager.pages[i] = None;
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
