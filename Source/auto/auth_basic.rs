// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

use crate::Auth;

glib::wrapper! {
	#[doc(alias = "SoupAuthBasic")]
	pub struct AuthBasic(Object<ffi::SoupAuthBasic>) @extends Auth;

	match fn {
		type_ => || ffi::soup_auth_basic_get_type(),
	}
}

impl AuthBasic {}

impl fmt::Display for AuthBasic {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		f.write_str("AuthBasic")
	}
}
