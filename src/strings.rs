use std::{ffi::CStr, marker::PhantomData, os::raw::c_char};

#[repr(C)]
#[repr(packed)]
pub(crate) struct Strings {
	elements: *const *const c_char,
	length:   i32,
}

pub(crate) struct StringsContainer<'a> {
	pub(crate) strings: Strings,
	_container:         Vec<*const c_char>,
	_marker:            PhantomData<[&'a CStr]>,
}

impl<'a, I: Iterator<Item = &'a CStr>> From<I> for StringsContainer<'a> {
	fn from(i: I) -> Self {
		let container: Vec<_> = i.map(|s| s.as_ptr()).collect();

		let strings = Strings {
			elements: container.as_slice().as_ptr() as *const *const c_char,
			length:   container.len() as i32,
		};

		StringsContainer {
			_container: container,
			strings,
			_marker: PhantomData,
		}
	}
}
