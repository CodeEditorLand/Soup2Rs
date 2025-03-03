// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

#[cfg(any(feature = "v2_68", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
use glib::object::Cast;
use glib::{StaticType, object::IsA, translate::*};

use crate::HSTSEnforcer;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupHSTSEnforcerDB")]
	pub struct HSTSEnforcerDB(Object<ffi::SoupHSTSEnforcerDB, ffi::SoupHSTSEnforcerDBClass>) @extends HSTSEnforcer, @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_hsts_enforcer_db_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupHSTSEnforcerDB")]
	pub struct HSTSEnforcerDB(Object<ffi::SoupHSTSEnforcerDB, ffi::SoupHSTSEnforcerDBClass>) @extends HSTSEnforcer;

	match fn {
		type_ => || ffi::soup_hsts_enforcer_db_get_type(),
	}
}

impl HSTSEnforcerDB {
	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "soup_hsts_enforcer_db_new")]
	pub fn new(filename:&str) -> HSTSEnforcerDB {
		crate::assert_initialized_main_thread!();

		unsafe {
			HSTSEnforcer::from_glib_full(ffi::soup_hsts_enforcer_db_new(filename.to_glib_none().0))
				.unsafe_cast()
		}
	}
}

pub const NONE_HSTS_ENFORCER_DB:Option<&HSTSEnforcerDB> = None;

pub trait HSTSEnforcerDBExt: 'static {
	fn filename(&self) -> Option<glib::GString>;
}

impl<O:IsA<HSTSEnforcerDB>> HSTSEnforcerDBExt for O {
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

impl fmt::Display for HSTSEnforcerDB {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("HSTSEnforcerDB") }
}
