// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::ffi::CStr;

#[doc(alias = "SOUP_ADDRESS_FAMILY")]
pub static ADDRESS_FAMILY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_ADDRESS_FAMILY).to_str().unwrap()
	});
#[doc(alias = "SOUP_ADDRESS_NAME")]
pub static ADDRESS_NAME:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_ADDRESS_NAME).to_str().unwrap()
	});
#[doc(alias = "SOUP_ADDRESS_PHYSICAL")]
pub static ADDRESS_PHYSICAL:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_ADDRESS_PHYSICAL).to_str().unwrap()
	});
#[doc(alias = "SOUP_ADDRESS_PORT")]
pub static ADDRESS_PORT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_ADDRESS_PORT).to_str().unwrap()
	});
#[doc(alias = "SOUP_ADDRESS_PROTOCOL")]
pub static ADDRESS_PROTOCOL:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_ADDRESS_PROTOCOL).to_str().unwrap()
	});
#[doc(alias = "SOUP_ADDRESS_SOCKADDR")]
pub static ADDRESS_SOCKADDR:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_ADDRESS_SOCKADDR).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_ADD_PATH")]
pub static AUTH_DOMAIN_ADD_PATH:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_ADD_PATH).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_BASIC_AUTH_CALLBACK")]
pub static AUTH_DOMAIN_BASIC_AUTH_CALLBACK:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_BASIC_AUTH_CALLBACK).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_BASIC_AUTH_DATA")]
pub static AUTH_DOMAIN_BASIC_AUTH_DATA:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_BASIC_AUTH_DATA).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_DIGEST_AUTH_CALLBACK")]
pub static AUTH_DOMAIN_DIGEST_AUTH_CALLBACK:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_DIGEST_AUTH_CALLBACK).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_DIGEST_AUTH_DATA")]
pub static AUTH_DOMAIN_DIGEST_AUTH_DATA:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_DIGEST_AUTH_DATA).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_FILTER")]
pub static AUTH_DOMAIN_FILTER:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_FILTER).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_FILTER_DATA")]
pub static AUTH_DOMAIN_FILTER_DATA:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_FILTER_DATA).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_GENERIC_AUTH_CALLBACK")]
pub static AUTH_DOMAIN_GENERIC_AUTH_CALLBACK:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_GENERIC_AUTH_CALLBACK).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_GENERIC_AUTH_DATA")]
pub static AUTH_DOMAIN_GENERIC_AUTH_DATA:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_GENERIC_AUTH_DATA).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_PROXY")]
pub static AUTH_DOMAIN_PROXY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_PROXY).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_REALM")]
pub static AUTH_DOMAIN_REALM:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_REALM).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_DOMAIN_REMOVE_PATH")]
pub static AUTH_DOMAIN_REMOVE_PATH:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_DOMAIN_REMOVE_PATH).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_HOST")]
pub static AUTH_HOST:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe { CStr::from_ptr(ffi::SOUP_AUTH_HOST).to_str().unwrap() });
#[doc(alias = "SOUP_AUTH_IS_AUTHENTICATED")]
pub static AUTH_IS_AUTHENTICATED:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_IS_AUTHENTICATED).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_IS_FOR_PROXY")]
pub static AUTH_IS_FOR_PROXY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_IS_FOR_PROXY).to_str().unwrap()
	});
#[doc(alias = "SOUP_AUTH_REALM")]
pub static AUTH_REALM:once_cell::sync::Lazy<&'static str> = once_cell::sync::Lazy::new(|| unsafe {
	CStr::from_ptr(ffi::SOUP_AUTH_REALM).to_str().unwrap()
});
#[doc(alias = "SOUP_AUTH_SCHEME_NAME")]
pub static AUTH_SCHEME_NAME:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_AUTH_SCHEME_NAME).to_str().unwrap()
	});
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
#[doc(alias = "SOUP_COOKIE_JAR_ACCEPT_POLICY")]
pub static COOKIE_JAR_ACCEPT_POLICY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_COOKIE_JAR_ACCEPT_POLICY).to_str().unwrap()
	});
#[doc(alias = "SOUP_COOKIE_JAR_DB_FILENAME")]
pub static COOKIE_JAR_DB_FILENAME:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_COOKIE_JAR_DB_FILENAME).to_str().unwrap()
	});
#[doc(alias = "SOUP_COOKIE_JAR_READ_ONLY")]
pub static COOKIE_JAR_READ_ONLY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_COOKIE_JAR_READ_ONLY).to_str().unwrap()
	});
#[doc(alias = "SOUP_COOKIE_JAR_TEXT_FILENAME")]
pub static COOKIE_JAR_TEXT_FILENAME:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_COOKIE_JAR_TEXT_FILENAME).to_str().unwrap()
	});
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
#[doc(alias = "SOUP_FORM_MIME_TYPE_MULTIPART")]
pub static FORM_MIME_TYPE_MULTIPART:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_FORM_MIME_TYPE_MULTIPART).to_str().unwrap()
	});
#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
#[doc(alias = "SOUP_FORM_MIME_TYPE_URLENCODED")]
pub static FORM_MIME_TYPE_URLENCODED:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_FORM_MIME_TYPE_URLENCODED).to_str().unwrap()
	});
#[doc(alias = "SOUP_HSTS_ENFORCER_DB_FILENAME")]
pub static HSTS_ENFORCER_DB_FILENAME:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_HSTS_ENFORCER_DB_FILENAME).to_str().unwrap()
	});
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
#[doc(alias = "SOUP_LOGGER_LEVEL")]
pub static LOGGER_LEVEL:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_LOGGER_LEVEL).to_str().unwrap()
	});
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
#[doc(alias = "SOUP_LOGGER_MAX_BODY_SIZE")]
pub static LOGGER_MAX_BODY_SIZE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_LOGGER_MAX_BODY_SIZE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
#[doc(alias = "SOUP_MESSAGE_FIRST_PARTY")]
pub static MESSAGE_FIRST_PARTY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_FIRST_PARTY).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_FLAGS")]
pub static MESSAGE_FLAGS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_FLAGS).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_HTTP_VERSION")]
pub static MESSAGE_HTTP_VERSION:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_HTTP_VERSION).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_IS_TOP_LEVEL_NAVIGATION")]
pub static MESSAGE_IS_TOP_LEVEL_NAVIGATION:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_IS_TOP_LEVEL_NAVIGATION).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_METHOD")]
pub static MESSAGE_METHOD:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_METHOD).to_str().unwrap()
	});
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
#[doc(alias = "SOUP_MESSAGE_PRIORITY")]
pub static MESSAGE_PRIORITY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_PRIORITY).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_REASON_PHRASE")]
pub static MESSAGE_REASON_PHRASE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_REASON_PHRASE).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_REQUEST_BODY")]
pub static MESSAGE_REQUEST_BODY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_REQUEST_BODY).to_str().unwrap()
	});
#[cfg(any(feature = "v2_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
#[doc(alias = "SOUP_MESSAGE_REQUEST_BODY_DATA")]
pub static MESSAGE_REQUEST_BODY_DATA:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_REQUEST_BODY_DATA).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_REQUEST_HEADERS")]
pub static MESSAGE_REQUEST_HEADERS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_REQUEST_HEADERS).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_RESPONSE_BODY")]
pub static MESSAGE_RESPONSE_BODY:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_RESPONSE_BODY).to_str().unwrap()
	});
#[cfg(any(feature = "v2_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_46")))]
#[doc(alias = "SOUP_MESSAGE_RESPONSE_BODY_DATA")]
pub static MESSAGE_RESPONSE_BODY_DATA:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_RESPONSE_BODY_DATA).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_RESPONSE_HEADERS")]
pub static MESSAGE_RESPONSE_HEADERS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_RESPONSE_HEADERS).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_SERVER_SIDE")]
pub static MESSAGE_SERVER_SIDE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_SERVER_SIDE).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_SITE_FOR_COOKIES")]
pub static MESSAGE_SITE_FOR_COOKIES:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_SITE_FOR_COOKIES).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_STATUS_CODE")]
pub static MESSAGE_STATUS_CODE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_STATUS_CODE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
#[doc(alias = "SOUP_MESSAGE_TLS_CERTIFICATE")]
pub static MESSAGE_TLS_CERTIFICATE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_TLS_CERTIFICATE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
#[doc(alias = "SOUP_MESSAGE_TLS_ERRORS")]
pub static MESSAGE_TLS_ERRORS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_TLS_ERRORS).to_str().unwrap()
	});
#[doc(alias = "SOUP_MESSAGE_URI")]
pub static MESSAGE_URI:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_MESSAGE_URI).to_str().unwrap()
	});
#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "SOUP_REQUEST_SESSION")]
pub static REQUEST_SESSION:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_REQUEST_SESSION).to_str().unwrap()
	});
#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "SOUP_REQUEST_URI")]
pub static REQUEST_URI:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_REQUEST_URI).to_str().unwrap()
	});
#[cfg(any(feature = "v2_68", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
#[doc(alias = "SOUP_SERVER_ADD_WEBSOCKET_EXTENSION")]
pub static SERVER_ADD_WEBSOCKET_EXTENSION:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_ADD_WEBSOCKET_EXTENSION).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_ASYNC_CONTEXT")]
pub static SERVER_ASYNC_CONTEXT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_ASYNC_CONTEXT).to_str().unwrap()
	});
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
#[doc(alias = "SOUP_SERVER_HTTPS_ALIASES")]
pub static SERVER_HTTPS_ALIASES:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_HTTPS_ALIASES).to_str().unwrap()
	});
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
#[doc(alias = "SOUP_SERVER_HTTP_ALIASES")]
pub static SERVER_HTTP_ALIASES:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_HTTP_ALIASES).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_INTERFACE")]
pub static SERVER_INTERFACE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_INTERFACE).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_PORT")]
pub static SERVER_PORT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_PORT).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_RAW_PATHS")]
pub static SERVER_RAW_PATHS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_RAW_PATHS).to_str().unwrap()
	});
#[cfg(any(feature = "v2_68", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
#[doc(alias = "SOUP_SERVER_REMOVE_WEBSOCKET_EXTENSION")]
pub static SERVER_REMOVE_WEBSOCKET_EXTENSION:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_REMOVE_WEBSOCKET_EXTENSION).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_SERVER_HEADER")]
pub static SERVER_SERVER_HEADER:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_SERVER_HEADER).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_SSL_CERT_FILE")]
pub static SERVER_SSL_CERT_FILE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_SSL_CERT_FILE).to_str().unwrap()
	});
#[doc(alias = "SOUP_SERVER_SSL_KEY_FILE")]
pub static SERVER_SSL_KEY_FILE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_SSL_KEY_FILE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SERVER_TLS_CERTIFICATE")]
pub static SERVER_TLS_CERTIFICATE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SERVER_TLS_CERTIFICATE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
#[doc(alias = "SOUP_SESSION_ACCEPT_LANGUAGE")]
pub static SESSION_ACCEPT_LANGUAGE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_ACCEPT_LANGUAGE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
#[doc(alias = "SOUP_SESSION_ACCEPT_LANGUAGE_AUTO")]
pub static SESSION_ACCEPT_LANGUAGE_AUTO:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_ACCEPT_LANGUAGE_AUTO).to_str().unwrap()
	});
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "SOUP_SESSION_ADD_FEATURE")]
pub static SESSION_ADD_FEATURE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_ADD_FEATURE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "SOUP_SESSION_ADD_FEATURE_BY_TYPE")]
pub static SESSION_ADD_FEATURE_BY_TYPE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_ADD_FEATURE_BY_TYPE).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_ASYNC_CONTEXT")]
pub static SESSION_ASYNC_CONTEXT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_ASYNC_CONTEXT).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SESSION_HTTPS_ALIASES")]
pub static SESSION_HTTPS_ALIASES:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_HTTPS_ALIASES).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SESSION_HTTP_ALIASES")]
pub static SESSION_HTTP_ALIASES:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_HTTP_ALIASES).to_str().unwrap()
	});
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "SOUP_SESSION_IDLE_TIMEOUT")]
pub static SESSION_IDLE_TIMEOUT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_IDLE_TIMEOUT).to_str().unwrap()
	});
#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
#[doc(alias = "SOUP_SESSION_LOCAL_ADDRESS")]
pub static SESSION_LOCAL_ADDRESS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_LOCAL_ADDRESS).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_MAX_CONNS")]
pub static SESSION_MAX_CONNS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_MAX_CONNS).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_MAX_CONNS_PER_HOST")]
pub static SESSION_MAX_CONNS_PER_HOST:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_MAX_CONNS_PER_HOST).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_PROXY_RESOLVER")]
pub static SESSION_PROXY_RESOLVER:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_PROXY_RESOLVER).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_PROXY_URI")]
pub static SESSION_PROXY_URI:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_PROXY_URI).to_str().unwrap()
	});
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
#[doc(alias = "SOUP_SESSION_REMOVE_FEATURE_BY_TYPE")]
pub static SESSION_REMOVE_FEATURE_BY_TYPE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_REMOVE_FEATURE_BY_TYPE).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_SSL_CA_FILE")]
pub static SESSION_SSL_CA_FILE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_SSL_CA_FILE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
#[doc(alias = "SOUP_SESSION_SSL_STRICT")]
pub static SESSION_SSL_STRICT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_SSL_STRICT).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SESSION_SSL_USE_SYSTEM_CA_FILE")]
pub static SESSION_SSL_USE_SYSTEM_CA_FILE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_SSL_USE_SYSTEM_CA_FILE).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_TIMEOUT")]
pub static SESSION_TIMEOUT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_TIMEOUT).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SESSION_TLS_DATABASE")]
pub static SESSION_TLS_DATABASE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_TLS_DATABASE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
#[doc(alias = "SOUP_SESSION_TLS_INTERACTION")]
pub static SESSION_TLS_INTERACTION:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_TLS_INTERACTION).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_USER_AGENT")]
pub static SESSION_USER_AGENT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_USER_AGENT).to_str().unwrap()
	});
#[doc(alias = "SOUP_SESSION_USE_NTLM")]
pub static SESSION_USE_NTLM:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_USE_NTLM).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SESSION_USE_THREAD_CONTEXT")]
pub static SESSION_USE_THREAD_CONTEXT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SESSION_USE_THREAD_CONTEXT).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_ASYNC_CONTEXT")]
pub static SOCKET_ASYNC_CONTEXT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_ASYNC_CONTEXT).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_FLAG_NONBLOCKING")]
pub static SOCKET_FLAG_NONBLOCKING:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_FLAG_NONBLOCKING).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_IS_SERVER")]
pub static SOCKET_IS_SERVER:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_IS_SERVER).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_LOCAL_ADDRESS")]
pub static SOCKET_LOCAL_ADDRESS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_LOCAL_ADDRESS).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_REMOTE_ADDRESS")]
pub static SOCKET_REMOTE_ADDRESS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_REMOTE_ADDRESS).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_SSL_CREDENTIALS")]
pub static SOCKET_SSL_CREDENTIALS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_SSL_CREDENTIALS).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_SSL_FALLBACK")]
pub static SOCKET_SSL_FALLBACK:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_SSL_FALLBACK).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_SSL_STRICT")]
pub static SOCKET_SSL_STRICT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_SSL_STRICT).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_TIMEOUT")]
pub static SOCKET_TIMEOUT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_TIMEOUT).to_str().unwrap()
	});
#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
#[doc(alias = "SOUP_SOCKET_TLS_CERTIFICATE")]
pub static SOCKET_TLS_CERTIFICATE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_TLS_CERTIFICATE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
#[doc(alias = "SOUP_SOCKET_TLS_ERRORS")]
pub static SOCKET_TLS_ERRORS:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_TLS_ERRORS).to_str().unwrap()
	});
#[doc(alias = "SOUP_SOCKET_TRUSTED_CERTIFICATE")]
pub static SOCKET_TRUSTED_CERTIFICATE:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_TRUSTED_CERTIFICATE).to_str().unwrap()
	});
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
#[doc(alias = "SOUP_SOCKET_USE_THREAD_CONTEXT")]
pub static SOCKET_USE_THREAD_CONTEXT:once_cell::sync::Lazy<&'static str> =
	once_cell::sync::Lazy::new(|| unsafe {
		CStr::from_ptr(ffi::SOUP_SOCKET_USE_THREAD_CONTEXT).to_str().unwrap()
	});
