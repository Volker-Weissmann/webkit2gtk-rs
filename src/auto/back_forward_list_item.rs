// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use webkit2_sys;

glib_wrapper! {
    pub struct BackForwardListItem(Object<webkit2_sys::WebKitBackForwardListItem, webkit2_sys::WebKitBackForwardListItemClass, BackForwardListItemClass>);

    match fn {
        get_type => || webkit2_sys::webkit_back_forward_list_item_get_type(),
    }
}

pub const NONE_BACK_FORWARD_LIST_ITEM: Option<&BackForwardListItem> = None;

pub trait BackForwardListItemExt: 'static {
    fn get_original_uri(&self) -> Option<GString>;

    fn get_title(&self) -> Option<GString>;

    fn get_uri(&self) -> Option<GString>;
}

impl<O: IsA<BackForwardListItem>> BackForwardListItemExt for O {
    fn get_original_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_back_forward_list_item_get_original_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_back_forward_list_item_get_title(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_back_forward_list_item_get_uri(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for BackForwardListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BackForwardListItem")
    }
}
