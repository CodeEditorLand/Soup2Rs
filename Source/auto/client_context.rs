// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

use crate::{Address, AuthDomain, Socket};

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct ClientContext(Boxed<ffi::SoupClientContext>);

	match fn {
		copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::soup_client_context_get_type(), ptr as *mut _) as *mut ffi::SoupClientContext,
		free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::soup_client_context_get_type(), ptr as *mut _),
		type_ => || ffi::soup_client_context_get_type(),
	}
}

impl ClientContext {
	#[doc(alias = "soup_client_context_get_address")]
	#[doc(alias = "get_address")]
	pub fn address(&mut self) -> Option<Address> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_address(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[doc(alias = "soup_client_context_get_auth_domain")]
	#[doc(alias = "get_auth_domain")]
	pub fn auth_domain(&mut self) -> Option<AuthDomain> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_auth_domain(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[doc(alias = "soup_client_context_get_auth_user")]
	#[doc(alias = "get_auth_user")]
	pub fn auth_user(&mut self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_auth_user(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_client_context_get_gsocket")]
	#[doc(alias = "get_gsocket")]
	pub fn gsocket(&mut self) -> Option<gio::Socket> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_gsocket(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[doc(alias = "soup_client_context_get_host")]
	#[doc(alias = "get_host")]
	pub fn host(&mut self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_host(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_client_context_get_local_address")]
	#[doc(alias = "get_local_address")]
	pub fn local_address(&mut self) -> Option<gio::SocketAddress> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_local_address(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_client_context_get_remote_address")]
	#[doc(alias = "get_remote_address")]
	pub fn remote_address(&mut self) -> Option<gio::SocketAddress> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_remote_address(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[doc(alias = "soup_client_context_get_socket")]
	#[doc(alias = "get_socket")]
	pub fn socket(&mut self) -> Option<Socket> {
		unsafe {
			from_glib_none(ffi::soup_client_context_get_socket(
				self.to_glib_none_mut().0,
			))
		}
	}

	#[cfg(any(feature = "v2_50", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	#[doc(alias = "soup_client_context_steal_connection")]
	pub fn steal_connection(&mut self) -> Option<gio::IOStream> {
		unsafe {
			from_glib_full(ffi::soup_client_context_steal_connection(
				self.to_glib_none_mut().0,
			))
		}
	}
}
