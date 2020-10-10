// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_28", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_28", feature = "dox"))]
use glib::GString;
use std::fmt;
use webkit2_sys;

glib_wrapper! {
    pub struct UserMessage(Object<webkit2_sys::WebKitUserMessage, webkit2_sys::WebKitUserMessageClass, UserMessageClass>);

    match fn {
        get_type => || webkit2_sys::webkit_user_message_get_type(),
    }
}

impl UserMessage {
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    pub fn new(name: &str, parameters: Option<&glib::Variant>) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_new(
                name.to_glib_none().0,
                parameters.to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //pub fn with_fd_list(name: &str, parameters: Option<&glib::Variant>, fd_list: /*Ignored*/Option<&gio::UnixFDList>) -> UserMessage {
    //    unsafe { TODO: call webkit2_sys:webkit_user_message_new_with_fd_list() }
    //}
}

pub const NONE_USER_MESSAGE: Option<&UserMessage> = None;

pub trait UserMessageExt: 'static {
    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //fn get_fd_list(&self) -> /*Ignored*/Option<gio::UnixFDList>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_name(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_parameters(&self) -> Option<glib::Variant>;

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn send_reply<P: IsA<UserMessage>>(&self, reply: &P);
}

impl<O: IsA<UserMessage>> UserMessageExt for O {
    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //fn get_fd_list(&self) -> /*Ignored*/Option<gio::UnixFDList> {
    //    unsafe { TODO: call webkit2_sys:webkit_user_message_get_fd_list() }
    //}

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn get_parameters(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_user_message_get_parameters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    fn send_reply<P: IsA<UserMessage>>(&self, reply: &P) {
        unsafe {
            webkit2_sys::webkit_user_message_send_reply(
                self.as_ref().to_glib_none().0,
                reply.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for UserMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserMessage")
    }
}
