// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Toplevel which can contain other widgets.

use glib::translate::*;
use glib::types;
use gdk;
use ffi;

use object::{Object, Downcast, Upcast};
use widgets;
use widgets::widget::Widget;
use {
    WindowPosition,
    WindowType,
};

/// Toplevel which can contain other widgets.
pub type Window = Object<ffi::GtkWindow>;

impl Window {
    pub fn new(window_type: WindowType) -> Window {
        unsafe { Widget::from_glib_none(ffi::gtk_window_new(window_type)).downcast_unchecked() }
    }
}

impl types::StaticType for Window {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_window_get_type()) }
    }
}

unsafe impl Upcast<widgets::widget::Widget> for Window { }
unsafe impl Upcast<widgets::container::Container> for Window { }
unsafe impl Upcast<widgets::bin::Bin> for Window { }
unsafe impl Upcast<::builder::Buildable> for Window { }

pub trait WindowExt {
    fn move_(&self, x: i32, y: i32);
    fn set_type_hint(&self, hint: gdk::WindowTypeHint);
    fn set_title(&self, title: &str);
    fn set_decorated(&self, setting: bool);
    fn get_title(&self) -> Option<String>;
    fn set_default_size(&self, width: i32, height: i32);
    fn set_window_position(&self, window_position: WindowPosition);
    #[cfg(feature = "gtk_3_10")]
    fn set_titlebar<T: Upcast<Widget>>(&self, titlebar: &T);
}

impl<O: Upcast<Window>> WindowExt for O {
    fn move_(&self, x: i32, y: i32) {
        unsafe {
            ffi::gtk_window_move(self.upcast().to_glib_none().0, x, y);
        }
    }

    fn set_type_hint(&self, hint: gdk::WindowTypeHint) {
        unsafe {
            ffi::gtk_window_set_type_hint(self.upcast().to_glib_none().0, hint);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_window_set_title(self.upcast().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_decorated(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_decorated(self.upcast().to_glib_none().0, setting.to_glib());
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_title(self.upcast().to_glib_none().0))
        }
    }

    fn set_default_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.upcast().to_glib_none().0, width, height)
        }
    }

    fn set_window_position(&self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.upcast().to_glib_none().0, window_position);
        }
    }

    #[cfg(feature = "gtk_3_10")]
    fn set_titlebar<T: Upcast<Widget>>(&self, titlebar: &T) {
        unsafe {
            ffi::gtk_window_set_titlebar(self.upcast().to_glib_none().0,
                titlebar.upcast().to_glib_none().0);
        }
    }
}
