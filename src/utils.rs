use std::{
	marker::PhantomData,
	mem::{size_of, zeroed},
	num::NonZeroU64,
};

use crate::{Client, Interface, Raw};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct APICall<'a>(NonZeroU64, PhantomData<&'a ()>);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaybeAPICall(u64);

impl APICall<'_> {
	pub unsafe fn new(api_call: MaybeAPICall) -> Option<Self> {
		NonZeroU64::new(api_call.0).map(|n| APICall(n, PhantomData))
	}
}

pub unsafe trait APICallResult {
	const ID: u32;
}

#[derive(Clone)]
pub struct Utils<'a> {
	pub(crate) raw:     Raw<Utils<'a>>,
	pub(crate) _marker: PhantomData<&'a ()>,
}

impl Interface for Utils<'_> {}

impl<'a> Utils<'a> {
	pub fn new(client: &Client<'a>) -> Option<Self> {
		Some(client.utils.clone())
	}

	pub fn is_apicall_completed(&self, call: APICall<'_>) -> bool {
		let mut b = false;
		unsafe {
			SteamAPI_ISteamUtils_IsAPICallCompleted(self.raw.clone(), call, &mut b as *mut bool)
		}
	}

	pub unsafe fn get_apicall_result<T: APICallResult>(&self, call: APICall<'_>) -> Result<T, ()> {
		let mut result: T = zeroed();

		let mut _b = false; // ignore Steam saying we have errors, because we don't. Steam just has trouble accepting that fact.
		if !SteamAPI_ISteamUtils_GetAPICallResult(
			self.raw.clone(),
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
}

steam_extern! {
	fn SteamAPI_ISteamUtils_IsAPICallCompleted(a: Raw<Utils<'_>>, b: APICall<'_>, c: *mut bool) -> bool;
	fn SteamAPI_ISteamUtils_GetAPICallResult(a: Raw<Utils<'_>>, b: APICall<'_>, c: *mut u8, d: u32, e: u32, f: *mut bool) -> bool;
}
