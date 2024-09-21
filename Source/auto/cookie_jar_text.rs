// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::CookieJar;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
use glib::object::Cast;
use glib::{object::IsA, translate::*, StaticType};
use std::fmt;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupCookieJarText")]
	pub struct CookieJarText(Object<ffi::SoupCookieJarText, ffi::SoupCookieJarTextClass>) @extends CookieJar, @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_cookie_jar_text_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupCookieJarText")]
	pub struct CookieJarText(Object<ffi::SoupCookieJarText, ffi::SoupCookieJarTextClass>) @extends CookieJar;

	match fn {
		type_ => || ffi::soup_cookie_jar_text_get_type(),
	}
}

impl CookieJarText {
	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	#[doc(alias = "soup_cookie_jar_text_new")]
	pub fn new(filename: &str, read_only: bool) -> CookieJarText {
		crate::assert_initialized_main_thread!();
		unsafe {
			CookieJar::from_glib_full(ffi::soup_cookie_jar_text_new(
				filename.to_glib_none().0,
				read_only.into_glib(),
			))
			.unsafe_cast()
		}
	}
}

pub const NONE_COOKIE_JAR_TEXT: Option<&CookieJarText> = None;

pub trait CookieJarTextExt: 'static {
	fn filename(&self) -> Option<glib::GString>;
}

impl<O: IsA<CookieJarText>> CookieJarTextExt for O {
	fn filename(&self) -> Option<glib::GString> {
		unsafe {
			let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"filename\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `filename` getter")
		}
	}
}

impl fmt::Display for CookieJarText {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("CookieJarText")
	}
}
