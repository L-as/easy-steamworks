macro_rules! steam_extern {
	($($t:tt)*) => {
		#[cfg_attr(unix,    link(name = "steam_api"))]
		#[cfg_attr(windows, link(name = "steam_api64"))]
		#[no_mangle]
		extern "C" {
			$($t)*
		}
	};
}
