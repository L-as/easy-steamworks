use std::{
	os::raw::c_char,
	marker::PhantomData,
	ffi::CStr,
};

use futures::{Poll, Async};

use crate::{
	APICall,
	Client,
	Error,
	SteamFuture,
	User,
	Pipe,
	Raw,
	RawRef,
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
	pub fn new<'b: 'a>(client: &'b Client<'a>) -> Option<Self> {
		let storage = unsafe {
			SteamAPI_ISteamClient_GetISteamRemoteStorage(
				client.into(),
				client.user(),
				client.pipe(),
				b"STEAMREMOTESTORAGE_INTERFACE_VERSION014\0" as *const _ as *const _,
			)
		};

		storage.as_ref()
	}

	pub fn file_write(&self, name: &CStr, data: impl AsRef<[u8]>) -> Result<(), ()> {
		let data = data.as_ref();
		if unsafe {
			SteamAPI_ISteamRemoteStorage_FileWrite(
				(&*self).into(),
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
				(&*self).into(),
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
	) -> impl SteamFuture<Item = Item> + 'a {
		#[repr(packed)]
		struct Data {
			pub result:       RawResult,
			pub item:         Item,
			accept_agreement: bool,
		}

		unsafe impl APICallResult for Data {
			const ID: u32 = 1309;
		}

		struct Handle<'a>(APICall<'a>);

		impl<'a> SteamFuture for Handle<'a> {
			type Item = Item;
			fn poll(&mut self, utils: &Utils) -> Poll<Self::Item, Error> {
				if utils.is_apicall_completed(self.0) {
					let data: Result<Data, _> = unsafe {utils.get_apicall_result(self.0)};
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
				(&*self).into(),
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

		Handle(api_call)
	}

	pub fn update(
		&'a self,
		item: Item,
	) -> ItemUpdater<'a> {
		let update_handle = unsafe {
			SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest(
				(&*self).into(),
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
						self.remote_storage.into(),
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
	pub fn finish(self) -> impl SteamFuture<Item = Item> + 'a {
		#[repr(packed)]
		struct Data {
			pub result:       RawResult,
			pub item:         Item,
			accept_agreement: bool,
		}

		unsafe impl APICallResult for Data {
			const ID: u32 = 1316;
		}

		struct Handle<'a>(APICall<'a>);

		impl<'a> SteamFuture for Handle<'a> {
			type Item = Item;
			fn poll(&mut self, utils: &Utils) -> Poll<Self::Item, Error> {
				if utils.is_apicall_completed(self.0) {
					let data: Result<Data, ()> = unsafe {utils.get_apicall_result(self.0)};
					data
						.map_err(|_| Error::Fail)
						.and_then(|Data {result, item, accept_agreement}| {
							// FIXME
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
				self.remote_storage.into(),
				self.update_handle,
			)
		};

		Handle(api_call)
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
				self.remote_storage.into(),
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
	fn SteamAPI_ISteamClient_GetISteamRemoteStorage<'a>(a: RawRef<'a, Client<'a>>, b: User<'a>, c: Pipe<'a>, d: *const c_char) -> Raw<'a, RemoteStorage<'a>>;

	fn SteamAPI_ISteamRemoteStorage_PublishWorkshopFile<'a>(
		a: RawRef<'a, RemoteStorage<'a>>,
		b: *const c_char,
		c: *const c_char,
		d: u32,
		e: *const c_char,
		f: *const c_char,
		g: Visibility,
		h: *const Strings,
		i: FileType
	) -> APICall<'a>;

	fn SteamAPI_ISteamRemoteStorage_FileWrite<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: *const c_char, c: *const u8, d: u32) -> bool;
	fn SteamAPI_ISteamRemoteStorage_FileDelete<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: *const c_char) -> bool;

	fn SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: Item)      -> UpdateHandle<'a>;
	fn SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>) -> APICall<'a>;

	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)                 -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)          -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)          -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileSetChangeDescription<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char) -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTags<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const Strings)                -> bool;
	fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle<'a>(a: RawRef<'a, RemoteStorage<'a>>, b: UpdateHandle<'a>, c: *const c_char)                -> bool;
}
