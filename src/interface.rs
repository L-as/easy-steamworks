use std::marker::PhantomData;

pub(crate) trait Interface<'a> : Sized {
	fn from_raw(p: &'a InterfaceData) -> Self;
	fn into_raw(&'a self) -> &'a crate::InterfaceData;
}

pub(crate) enum InterfaceData {}
#[repr(transparent)]
pub(crate) struct Raw<'a, T: Interface<'a>>(*mut InterfaceData, PhantomData<&'a mut T>);
#[repr(transparent)]
pub(crate) struct RawRef<'a, T: Interface<'a>>(*mut InterfaceData, PhantomData<&'a T>);

impl<'a, T: Interface<'a>> Raw<'a, T> {
	pub(crate) fn as_ref(&self) -> Option<T> {
		unsafe {self.0.as_ref().map(T::from_raw)}
	}
	pub(crate) unsafe fn from_raw(p: *mut InterfaceData) -> Self {
		Raw(p, PhantomData)
	}
}

impl<'a, T: Interface<'a>> From<&'a T> for RawRef<'a, T> {
	fn from(t: &'a T) -> Self {
		RawRef(t.into_raw() as *const _ as *mut _, PhantomData)
	}
}

macro_rules! interface {
	($name:ident) => {
		#[repr(transparent)]
		pub struct $name<'a>(&'a crate::InterfaceData);
		impl<'a> crate::Interface<'a> for $name<'a> {
			fn from_raw(p: &'a crate::InterfaceData) -> $name<'a> {
				$name(p)
			}
			fn into_raw(&'a self) -> &'a crate::InterfaceData {
				self.0
			}
		}
	};
}
