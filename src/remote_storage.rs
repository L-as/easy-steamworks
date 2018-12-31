use std::{
	os::raw::c_char,
	marker::PhantomData,
	ffi::CStr,
};
use const_cstr::const_cstr;

use futures::{Future, Poll, Async};

use crate::{
	APICall,
	Client,
	Error,
	User,
	Pipe,
	Raw,
	MaybeRaw,
	StringsContainer,
	Strings,
	Utils,
	RawResult,
	APICallResult,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Item(pub u64);

#[repr(transparent)]
#[derive(Debug)]
struct UpdateHandle<'a>(u64, PhantomData<&'a ()>);

#[repr(u32)]
pub enum Visibility {
	Public,
	FriendsOnly,
	Private
}

#[repr(u32)]
pub enum FileType {
	Community,
	Microtransaction,
	Collection,
}

interface!(RemoteStorage);
impl<'a> RemoteStorage<'a> {
	pub fn new(client: &Client<'a>) -> Option<Self> {
		let raw = unsafe {
			SteamAPI_ISteamClient_GetISteamRemoteStorage(
				client.raw.clone(),
				client.user(),
				client.pipe(),
				const_cstr!("STEAMREMOTESTORAGE_INTERFACE_VERSION014\0").as_ptr(),
			).check()?
		};
		let utils = client.utils.clone();

		Some(RemoteStorage {raw, utils})
	}

	pub fn file_write(&self, name: &CStr, data: impl AsRef<[u8]>) -> Result<(), ()> {
		let data = data.as_ref();
		if unsafe {
			SteamAPI_ISteamRemoteStorage_FileWrite(
				self.raw.clone(),
				name.as_ptr(),
				data.as_ptr(),
				data.len() as u32,
			)
		} {
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn file_delete(&self, name: &CStr) -> Result<(), ()> {
		if unsafe {
			SteamAPI_ISteamRemoteStorage_FileDelete(
				self.raw.clone(),
				name.as_ptr(),
			)
		} {
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn publish(
		&'a self,
		appid: u32,
		contents_path: &CStr,
		preview_path: &CStr,
		title: &CStr,
		description: &CStr,
		tags: &[impl AsRef<CStr>],
	) -> impl Future<Item = Item, Error = Error> + 'a {
		#[repr(packed)]
		struct Data {
			pub result:       RawResult,
			pub item:         Item,
			accept_agreement: bool,
		}

		unsafe impl APICallResult for Data {
			const ID: u32 = 1309;
		}

		struct Handle<'a> {
			api_call: APICall<'a>,
			utils: Utils<'a>,
		}

		impl Future for Handle<'_> {
			type Item = Item;
			type Error = Error;
			fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
				if self.utils.is_apicall_completed(self.api_call) {
					let data: Result<Data, _> = unsafe {self.utils.get_apicall_result(self.api_call)};
					data
						.map_err(|_| Error::Fail)
						.and_then(|Data {result, item, accept_agreement}| {
							assert!(!accept_agreement);
							Result::from(result).map(|_| item)
						})
						.map(Async::Ready)
				} else {
					Ok(Async::NotReady)
				}
			}
		}

		let tags = StringsContainer::from(tags.iter().map(|t| t.as_ref()));

		let api_call = unsafe {
			SteamAPI_ISteamRemoteStorage_PublishWorkshopFile(
				self.raw.clone(),
				contents_path.as_ptr(),
				preview_path.as_ptr(),
				appid,
				title.as_ptr(),
				description.as_ptr(),
				Visibility::Public,
				&tags.strings as *const Strings,
				FileType::Community,
			)
		};

		Handle {api_call, utils: self.utils.clone()}
	}

	pub fn update(
		&'a self,
		item: Item,
	) -> ItemUpdater<'a> {
		let update_handle = unsafe {
			SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest(
				self.raw.clone(),
				item,
			)
		};
		ItemUpdater {remote_storage: self, update_handle}
	}
}

pub struct ItemUpdater<'a> {
	remote_storage: &'a RemoteStorage<'a>,
	update_handle: UpdateHandle<'a>,
}

macro_rules! item_updater_methods {
	($($method:ident $ffi:ident);*;) => {
		$(
			pub fn $method(self, cstr: &CStr) -> Result<Self, ()> {
				if unsafe {
					$ffi(
						self.remote_storage.raw.clone(),
						UpdateHandle(self.update_handle.0, PhantomData),
						cstr.as_ptr(),
					)
				} {
					Ok(self)
				} else {
					Err(())
				}
			}
		)*
	};
}

impl<'a> ItemUpdater<'a> {
	pub fn finish(self) -> impl Future<Item = Item, Error = Error> + 'a {
		#[repr(packed)]
		struct Data {
			pub result:       RawResult,
			pub item:         Item,
			accept_agreement: bool,
		}

		unsafe impl APICallResult for Data {
			const ID: u32 = 1309;
		}

		struct Handle<'a> {
			api_call: APICall<'a>,
			utils: Utils<'a>,
		}

		impl Future for Handle<'_> {
			type Item = Item;
			type Error = Error;
			fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
				if self.utils.is_apicall_completed(self.api_call) {
					let data: Result<Data, _> = unsafe {self.utils.get_apicall_result(self.api_call)};
					data
						.map_err(|_| Error::Fail)
						.and_then(|Data {result, item, accept_agreement}| {
							assert!(!accept_agreement);
							Result::from(result).map(|_| item)
						})
						.map(Async::Ready)
				} else {
					Ok(Async::NotReady)
				}
			}
		}


		let api_call = unsafe {
			SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate(
				self.remote_storage.raw.clone(),
				self.update_handle,
			)
		};

		Handle {api_call, utils: self.remote_storage.utils.clone()}
	}
	item_updater_methods!(
		file SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile;
		preview SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile;
		description SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription;
		change_description SteamAPI_ISteamRemoteStorage_UpdatePublishedFileSetChangeDescription;
		title SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle;
	);
	pub fn tags(self, tags: &[impl AsRef<CStr>]) -> Result<Self, ()> {
		let tags = StringsContainer::from(tags.iter().map(|t| t.as_ref()));
		if unsafe {
			SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTags(
				self.remote_storage.raw.clone(),
				UpdateHandle(self.update_handle.0, PhantomData),
				&tags.strings as *const Strings,
			)
		} {
			Ok(self)
		} else {
			Err(())
		}
	}
}

steam_extern! {
	fn SteamAPI_ISteamClient_GetISteamRemoteStorage<'a>(a: Raw<Client<'a>>, b: User<'a>, c: Pipe<'a>, d: *const c_char) -> MaybeRaw<RemoteStorage<'a>>;

	fn SteamAPI_ISteamRemoteStorage_PublishWorkshopFile<'a>(
		a: Raw<RemoteStorage<'a>>,
		b: *const c_char,
		c: *const c_char,
		d: u32,
		e: *const c_char,
		f: *const c_char,
		g: Visibility,
		h: *const Strings,
		i: FileType
	) -> APICall<'a>;

	fn SteamAPI_ISteamRemoteStorage_FileWrite<'a>(a: Raw<RemoteStorage<'a>>, b: *const c_char, c: *const u8, d: u32) -> bool;
	fn SteamAPI_ISteamRemoteStorage_FileDelete<'a>(a: Raw<RemoteStorage<'a>>, b: *const c_char) -> bool;

	fn SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest<'a>(a: Raw<RemoteStorage<'a>>, b: Item)      -> UpdateHandle<'a>;
	fn SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>) -> APICall<'a>;

	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)                 -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)          -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)          -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileSetChangeDescription<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char) -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTags<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const Strings)                -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)                -> bool;
}
