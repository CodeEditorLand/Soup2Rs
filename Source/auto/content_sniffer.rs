// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

use glib::object::IsA;
#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
use glib::translate::*;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupContentSniffer")]
	pub struct ContentSniffer(Object<ffi::SoupContentSniffer, ffi::SoupContentSnifferClass>) @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_content_sniffer_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupContentSniffer")]
	pub struct ContentSniffer(Object<ffi::SoupContentSniffer, ffi::SoupContentSnifferClass>);

	match fn {
		type_ => || ffi::soup_content_sniffer_get_type(),
	}
}

impl ContentSniffer {
	#[cfg(any(feature = "v2_28", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	#[doc(alias = "soup_content_sniffer_new")]
	pub fn new() -> ContentSniffer {
		crate::assert_initialized_main_thread!();

		unsafe { from_glib_full(ffi::soup_content_sniffer_new()) }
	}
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
impl Default for ContentSniffer {
	fn default() -> Self { Self::new() }
}

pub const NONE_CONTENT_SNIFFER:Option<&ContentSniffer> = None;

pub trait ContentSnifferExt: 'static {
	#[cfg(any(feature = "v2_28", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	#[doc(alias = "soup_content_sniffer_get_buffer_size")]
	#[doc(alias = "get_buffer_size")]
	fn buffer_size(&self) -> usize;

	//#[cfg(any(feature = "v2_28", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	//#[doc(alias = "soup_content_sniffer_sniff")]
	// fn sniff(&self, msg: &impl IsA<Message>, buffer: &mut Buffer, params:
	// /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28
	// }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString>;
}

impl<O:IsA<ContentSniffer>> ContentSnifferExt for O {
	#[cfg(any(feature = "v2_28", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	fn buffer_size(&self) -> usize {
		unsafe { ffi::soup_content_sniffer_get_buffer_size(self.as_ref().to_glib_none().0) }
	}

	//#[cfg(any(feature = "v2_28", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	// fn sniff(&self, msg: &impl IsA<Message>, buffer: &mut Buffer, params:
	// /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28
	// }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString> {    unsafe { TODO:
	// call ffi:soup_content_sniffer_sniff() }
	//}
}

impl fmt::Display for ContentSniffer {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("ContentSniffer") }
}
