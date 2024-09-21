// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;
use std::fmt;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
	#[doc(alias = "SoupContentDecoder")]
	pub struct ContentDecoder(Object<ffi::SoupContentDecoder, ffi::SoupContentDecoderClass>) @implements SessionFeature;

	match fn {
		type_ => || ffi::soup_content_decoder_get_type(),
	}
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupContentDecoder")]
	pub struct ContentDecoder(Object<ffi::SoupContentDecoder, ffi::SoupContentDecoderClass>);

	match fn {
		type_ => || ffi::soup_content_decoder_get_type(),
	}
}

impl ContentDecoder {}

pub const NONE_CONTENT_DECODER: Option<&ContentDecoder> = None;

impl fmt::Display for ContentDecoder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("ContentDecoder")
	}
}
