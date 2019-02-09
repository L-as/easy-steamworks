macro_rules! steam_extern {
	($($t:tt)*) => {
		#[no_mangle]
		extern "C" {
			$($t)*
		}
	};
}

macro_rules! declare_future {
	(
		Data ($id:tt) {
			$($ident:ident: $ty:ty),*$(,)*
		} -> $outty:ty;

		map($map:expr);
	) => {
		#[repr(packed)]
		struct Data {
			$($ident: $ty),*
		}

		unsafe impl crate::utils::APICallResult for Data {
			const ID: u32 = $id;
		}

		struct Handle<'a> {
			api_call: crate::utils::APICall<'a>,
			utils:    crate::utils::Utils<'a>,
		}

		impl futures::Future for Handle<'_> {
			type Error = crate::Error;
			type Item = $outty;

			fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
				use futures::task;

				if self.utils.is_apicall_completed(self.api_call) {
					let data: Result<Data, _> =
						unsafe { self.utils.get_apicall_result(self.api_call) };
					data.map_err(|_| Error::Fail)
						.and_then($map)
						.map(Async::Ready)
				} else {
					task::current().notify();
					Ok(Async::NotReady)
				}
			}
		}
	}
}
