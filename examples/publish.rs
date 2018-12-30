use easy_steamworks::{
	STEAM,
	RemoteStorage,
	Utils,
	Item,
};
use futures::Future;
use const_cstr::const_cstr;
use std::ffi::CStr;

fn main() {
	let mut steam = STEAM.lock().unwrap();
	let client = steam.new_client().unwrap();
	let storage = RemoteStorage::new(&client).unwrap();
	let utils = Utils::new(&client).unwrap();
	let future1 = storage.publish(0,
		const_cstr!("content.zip").as_cstr(),
		const_cstr!("preview.jpg").as_cstr(),
		const_cstr!("A Title").as_cstr(),
		const_cstr!("My description.").as_cstr(),
		&[] as &[&CStr],
	);
	let future2 = storage.publish(0,
		const_cstr!("content.zip").as_cstr(),
		const_cstr!("preview.jpg").as_cstr(),
		const_cstr!("A Title").as_cstr(),
		const_cstr!("My description.").as_cstr(),
		&[] as &[&CStr],
	);
	let _item1: Item = utils.poll(future1).wait().unwrap();
	let _item2: Item = utils.poll(future2).wait().unwrap();
}
