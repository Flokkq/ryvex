use core::{
	cell::UnsafeCell,
	sync::atomic::{
		AtomicBool,
		Ordering,
	},
};

pub struct SpinLock<T> {
	flag: AtomicBool,
	data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
	pub const fn new(value: T) -> Self {
		Self {
			flag: AtomicBool::new(false),
			data: UnsafeCell::new(value),
		}
	}

	pub fn lock(&self) -> Guard<'_, T> {
		while self
			.flag
			.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
			.is_err()
		{
			core::hint::spin_loop();
		}

		Guard { lock: self }
	}
}

pub struct Guard<'a, T> {
	lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for Guard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.lock.data.get() }
	}
}

impl<'a, T> core::ops::DerefMut for Guard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.lock.data.get() }
	}
}

impl<'a, T> Drop for Guard<'a, T> {
	fn drop(&mut self) {
		self.lock.flag.store(false, Ordering::Release);
	}
}
