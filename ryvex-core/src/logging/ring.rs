use core::mem::MaybeUninit;

use super::record::OwnedRecord;

pub struct Ring<const N: usize> {
	records:                [MaybeUninit<OwnedRecord>; N],
	pub(crate) write_index: u64,
}

impl<const N: usize> Default for Ring<N> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const N: usize> Ring<N> {
	pub const fn new() -> Self {
		Self {
			records:     unsafe {
				MaybeUninit::<[MaybeUninit<OwnedRecord>; N]>::uninit()
					.assume_init()
			},
			write_index: 0,
		}
	}

	pub fn push(&mut self, rec: OwnedRecord) -> u64 {
		let seq = self.write_index;

		self.records[(seq as usize) % N] = MaybeUninit::new(rec);
		self.write_index = seq + 1;

		seq
	}

	pub fn get(&self, seq: u64) -> Option<&OwnedRecord> {
		if seq >= self.write_index {
			return None;
		}

		if self.write_index - seq > N as u64 {
			return None;
		}

		Some(unsafe { self.records[(seq as usize) % N].assume_init_ref() })
	}

	pub fn iter_recent(&self) -> Iter<'_, N> {
		let start = self.write_index.saturating_sub(N as u64);
		Iter {
			ring: self,
			next: start,
			end:  self.write_index,
		}
	}
}

pub struct Iter<'a, const N: usize> {
	ring: &'a Ring<N>,
	next: u64,
	end:  u64,
}

impl<'a, const N: usize> Iterator for Iter<'a, N> {
	type Item = &'a OwnedRecord;

	fn next(&mut self) -> Option<Self::Item> {
		if self.next >= self.end {
			return None;
		}

		let ring = self.ring.get(self.next);
		self.next += 1;

		ring
	}
}

impl<'a, const N: usize> core::iter::DoubleEndedIterator for Iter<'a, N> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.next >= self.end {
			return None;
		}

		self.end -= 1;
		self.ring.get(self.end)
	}
}
