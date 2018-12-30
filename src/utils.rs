use std::{
	marker::PhantomData,
	mem::{size_of, zeroed},
};

use futures::{
	Future,
	Poll,
};

use crate::{
	Client,
	Pipe,
	Raw,
	RawRef,
	Error,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct APICall<'a>(u64, PhantomData<&'a ()>);

pub unsafe trait APICallResult {
	const ID: u32;
}

pub trait SteamFuture {
	type Item;
	fn poll(&mut self, utils: &Utils) -> Poll<Self::Item, Error>;
}

interface!(Utils);

impl<'a> Utils<'a> {
	pub fn new(client: &'a Client<'a>) -> Option<Self> {
		let utils = unsafe {
			SteamAPI_ISteamClient_GetISteamUtils(
				client.into(),
				client.pipe(),
				b"SteamUtils009\0" as *const _ as *const _,
			)
		};

		utils.as_ref()
	}

	pub fn is_apicall_completed(&self, call: APICall) -> bool {
		let mut b = false;
		unsafe { SteamAPI_ISteamUtils_IsAPICallCompleted((&*self).into(), call, &mut b as *mut bool) }
	}

	pub unsafe fn get_apicall_result<T: APICallResult>(&self, call: APICall) -> Result<T, ()> {
		let mut result: T = zeroed();

		let mut _b = false; // ignore Steam saying we have errors, because we don't. Steam just has trouble accepting that fact.
		if !SteamAPI_ISteamUtils_GetAPICallResult(
			(&*self).into(),
			call,
			&mut result as *mut _ as *mut u8,
			size_of::<T>() as u32,
			T::ID,
			&mut _b as *mut bool,
		) {
			return Err(());
		}

		Ok(result)
	}

	pub fn poll<T: SteamFuture + 'a>(&'a self, future: T) -> impl Future<Item = T::Item, Error = Error> + 'a {
		struct Wrapper<'a, T: SteamFuture> {
			utils: &'a Utils<'a>,
			future: T,
		}

		impl<'a, T: SteamFuture> Future for Wrapper<'a, T> {
			type Item = T::Item;
			type Error = Error;
			fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
				self.future.poll(self.utils)
			}
		}

		Wrapper {utils: self, future}
	}
}

steam_extern! {
	fn SteamAPI_ISteamClient_GetISteamUtils<'a>(a: RawRef<'a, Client<'a>>, b: Pipe<'a>, c: *const i8) -> Raw<'a, Utils<'a>>;
	fn SteamAPI_ISteamUtils_IsAPICallCompleted<'a>(a: RawRef<'a, Utils<'a>>, b: APICall<'a>, c: *mut bool) -> bool;
	fn SteamAPI_ISteamUtils_GetAPICallResult<'a>(a: RawRef<'a, Utils<'a>>, b: APICall<'a>, c: *mut u8, d: u32, e: u32, f: *mut bool) -> bool;
}
