use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

pub(crate) trait Interface: Sized {}

#[repr(transparent)]
pub(crate) struct Raw<T: Interface>(NonNull<c_void>, PhantomData<T>);
impl<T: Interface> Clone for Raw<T> {
	fn clone(&self) -> Self {
		Raw(self.0, PhantomData)
	}
}

#[repr(transparent)]
pub(crate) struct MaybeRaw<T: Interface>(*mut c_void, PhantomData<T>);
impl<T: Interface> MaybeRaw<T> {
	pub(crate) fn check(self) -> Option<Raw<T>> {
		if self.0.is_null() {
			None
		} else {
			unsafe { Some(Raw(NonNull::new_unchecked(self.0), PhantomData)) }
		}
	}
}

impl<T: Interface> From<*mut c_void> for MaybeRaw<T> {
	fn from(p: *mut c_void) -> Self {
		MaybeRaw(p, PhantomData)
	}
}

macro_rules! interface {
	($name:ident) => {
		#[derive(Clone)]
		pub struct $name<'a> {
			pub(crate) raw:   crate::Raw<$name<'a>>,
			pub(crate) utils: crate::Utils<'a>,
		}
		impl crate::Interface for $name<'_> {}
	};
}
