use std::{
	marker::PhantomData,
	os::raw::c_char,
	ffi::c_void,
	sync::Mutex,
};
use lazy_static::lazy_static;
use const_cstr::const_cstr;
use crate::{
	Raw,
	MaybeRaw,
	Utils,
};

pub struct Steam {}
lazy_static! {
	pub static ref STEAM: Mutex<Steam> = Mutex::new(Steam {});
}

impl Steam {
	pub fn new_client(&mut self) -> Option<Client<'_>> {
		if unsafe { !SteamAPI_Init() } {
			return None;
		}

		let raw: MaybeRaw<_> = unsafe {
			SteamInternal_CreateInterface(
				const_cstr!("SteamClient017").as_ptr(),
			)
		}.into();
		let raw = raw.check()?;

		let utils = unsafe {
			SteamAPI_ISteamClient_GetISteamUtils(
				raw.clone(),
				SteamAPI_GetHSteamPipe(),
				const_cstr!("SteamUtils009").as_ptr(),
			).check()?
		};

		let utils = Utils {raw: utils, _marker: PhantomData};

		Some(Client {raw, utils})
	}
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct User<'a>(i32, PhantomData<&'a ()>);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pipe<'a>(i32, PhantomData<&'a ()>);

pub struct Client<'a> {
	pub(crate) raw: Raw<Client<'a>>,
	pub(crate) utils: Utils<'a>,
}
impl crate::Interface for Client<'_> {}

impl<'a> Client<'a> {
	pub fn user(&self) -> User<'a> {
		unsafe {SteamAPI_GetHSteamUser()}
	}

	pub fn pipe(&self) -> Pipe<'a> {
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

	fn SteamAPI_ISteamClient_GetISteamUtils<'a>(a: Raw<Client<'a>>, b: Pipe<'_>, c: *const c_char) -> MaybeRaw<Utils<'a>>;

	fn SteamInternal_CreateInterface(a: *const c_char) -> *mut c_void;
}
