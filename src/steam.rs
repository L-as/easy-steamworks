use std::{
	marker::PhantomData,
	os::raw::c_char,
};

use crate::Raw;

pub struct Steam;
impl Steam {
	pub fn new_client<'a>(&'a mut self) -> Result<Client<'a>, ()> {
		if unsafe { !SteamAPI_Init() } {
			return Err(());
		}

		let client: Raw<Client<'a>> = unsafe {
			Raw::from_raw(SteamInternal_CreateInterface(
				b"SteamClient017\0" as *const _ as *const _,
			))
		};

		client.as_ref()
	}
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct User<'a>(i32, PhantomData<&'a ()>);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pipe<'a>(i32, PhantomData<&'a ()>);

interface!(Client);

impl Client<'_> {
	pub fn user(&self) -> User<'_> {
		unsafe {SteamAPI_GetHSteamUser()}
	}

	pub fn pipe(&self) -> Pipe<'_> {
		unsafe {SteamAPI_GetHSteamPipe()}
	}
}

impl Drop for Client<'_> {
	fn drop(&mut self) {
		unsafe { SteamAPI_Shutdown() }
	}
}

steam_extern! {
	fn SteamAPI_Init() -> bool;
	fn SteamAPI_Shutdown();

	fn SteamAPI_GetHSteamUser<'a>() -> User<'a>;
	fn SteamAPI_GetHSteamPipe<'a>() -> Pipe<'a>;

	fn SteamInternal_CreateInterface(a: *const c_char) -> *mut crate::InterfaceData;
}
