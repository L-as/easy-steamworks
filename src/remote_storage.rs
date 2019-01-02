use const_cstr::const_cstr;
use std::{ffi::CStr, marker::PhantomData, os::raw::c_char};

use futures::{Async, Future, Poll};

use crate::{
	APICall,
	Client,
	Error,
	MaybeAPICall,
	MaybeRaw,
	Pipe,
	Raw,
	RawResult,
	Strings,
	StringsContainer,
	User,
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
	Private,
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
			)
			.check()?
		};
		let utils = client.utils.clone();

		Some(RemoteStorage { raw, utils })
	}

	pub fn file_write(
		&'a self,
		name: &CStr,
		data: impl AsRef<[u8]>,
	) -> Option<impl Future<Item = (), Error = Error> + 'a> {
		declare_future! {
			Data (1331) {
				result: RawResult,
			} -> ();

			map(|Data {result}| Result::from(result));
		}

		let data = data.as_ref();
		let api_call = unsafe {
			SteamAPI_ISteamRemoteStorage_FileWriteAsync(
				self.raw.clone(),
				name.as_ptr(),
				data.as_ptr(),
				data.len() as u32,
			)
		};

		Some(Handle {
			api_call: unsafe { APICall::new(api_call)? },
			utils:    self.utils.clone(),
		})
	}

	pub fn file_delete(&self, name: &CStr) -> Result<(), ()> {
		if unsafe { SteamAPI_ISteamRemoteStorage_FileDelete(self.raw.clone(), name.as_ptr()) } {
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
	) -> Option<impl Future<Item = Item, Error = Error> + 'a> {
		declare_future! {
			Data (1309) {
				result:           RawResult,
				item:             Item,
				accept_agreement: bool,
			} -> Item;

			map(
				|Data {
					result,
					item,
					accept_agreement,
				}| {
					assert!(!accept_agreement);
					Result::from(result).map(|_| item)
				}
			);
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

		Some(Handle {
			api_call: unsafe { APICall::new(api_call)? },
			utils:    self.utils.clone(),
		})
	}

	pub fn update(&'a self, item: Item) -> ItemUpdater<'a> {
		let update_handle = unsafe {
			SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest(self.raw.clone(), item)
		};
		ItemUpdater {
			remote_storage: self,
			update_handle,
		}
	}
}

pub struct ItemUpdater<'a> {
	remote_storage: &'a RemoteStorage<'a>,
	update_handle:  UpdateHandle<'a>,
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
	item_updater_methods!(
		file SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile;
		preview SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile;
		description SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription;
		change_description SteamAPI_ISteamRemoteStorage_UpdatePublishedFileSetChangeDescription;
		title SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle;
	);

	pub fn finish(self) -> Option<impl Future<Item = Item, Error = Error> + 'a> {
		declare_future! {
			Data (1316) {
				result:           RawResult,
				item:             Item,
				accept_agreement: bool,
			} -> Item;

			map(
				|Data {
					result,
					item,
					accept_agreement,
				}| {
					assert!(!accept_agreement);
					Result::from(result).map(|_| item)
				}
			);
		}

		let api_call = unsafe {
			SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate(
				self.remote_storage.raw.clone(),
				self.update_handle,
			)
		};

		Some(Handle {
			api_call: unsafe { APICall::new(api_call)? },
			utils:    self.remote_storage.utils.clone(),
		})
	}

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
	) -> MaybeAPICall;

	fn SteamAPI_ISteamRemoteStorage_FileWriteAsync<'a>(a: Raw<RemoteStorage<'a>>, b: *const c_char, c: *const u8, d: u32) -> MaybeAPICall;
	fn SteamAPI_ISteamRemoteStorage_FileDelete<'a>(a: Raw<RemoteStorage<'a>>, b: *const c_char) -> bool;

	fn SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest<'a>(a: Raw<RemoteStorage<'a>>, b: Item)      -> UpdateHandle<'a>;
	fn SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>) -> MaybeAPICall;

	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)                 -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)          -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)          -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileSetChangeDescription<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char) -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTags<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const Strings)                -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle<'a>(a: Raw<RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)                -> bool;
}
