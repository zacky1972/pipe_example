extern crate libc;

use libc::{c_void, size_t, pipe, fork, close, write, read};
use std::ffi::CString;


fn main() {
	let mut pp = [0; 2];
	let mut qq = [0; 2];
	let message = "Hello";

	unsafe {
		pipe(pp.as_mut_ptr());
		pipe(qq.as_mut_ptr());

		match fork() {
			0 => {
				// 子プロセス
				close(pp[1]);
				close(qq[0]);

				let mut data: [u8; 256] = [0; 256];
				let mut buf = data.as_mut_ptr() as *mut c_void;
				read(pp[0], buf, 256);
				for i in data.iter_mut() {
					*i = *i + 1;
				}
				write(qq[1], buf, (data.len()) as size_t);
			},
			_ => {
				// 親プロセス
				close(pp[0]);
				close(qq[1]);

				let src_data =  CString::new(message).unwrap().as_ptr();
				let src_buf = src_data as *const c_void;
				write(pp[1], src_buf, (message.len() + 1) as size_t);

				let mut dst_data: [u8; 256] = [0; 256];
				let mut dst_buf = dst_data.as_mut_ptr() as *mut c_void;
				read(qq[0], dst_buf, 256);
				for x in dst_data.iter() {
					print!("{} ", x);
				}
			},
		}
	}
}
