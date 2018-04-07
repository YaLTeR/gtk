// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Alignment(Object<ffi::GtkAlignment, ffi::GtkAlignmentClass>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_alignment_get_type(),
    }
}

impl Alignment {
    #[cfg_attr(feature = "v3_14", deprecated)]
    pub fn new(xalign: f32, yalign: f32, xscale: f32, yscale: f32) -> Alignment {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_alignment_new(xalign, yalign, xscale, yscale)).downcast_unchecked()
        }
    }
}

pub trait AlignmentExt {
    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_padding(&self) -> (u32, u32, u32, u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set(&self, xalign: f32, yalign: f32, xscale: f32, yscale: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_padding(&self, padding_top: u32, padding_bottom: u32, padding_left: u32, padding_right: u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_bottom_padding(&self) -> u32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_bottom_padding(&self, bottom_padding: u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_left_padding(&self) -> u32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_left_padding(&self, left_padding: u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_right_padding(&self) -> u32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_right_padding(&self, right_padding: u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_top_padding(&self) -> u32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_top_padding(&self, top_padding: u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_xalign(&self) -> f32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_xalign(&self, xalign: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_xscale(&self) -> f32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_xscale(&self, xscale: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_yalign(&self) -> f32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_yalign(&self, yalign: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_yscale(&self) -> f32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_yscale(&self, yscale: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_bottom_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_left_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_right_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_top_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_xscale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_yscale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Alignment> + IsA<glib::object::Object>> AlignmentExt for O {
    fn get_padding(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut padding_top = mem::uninitialized();
            let mut padding_bottom = mem::uninitialized();
            let mut padding_left = mem::uninitialized();
            let mut padding_right = mem::uninitialized();
            ffi::gtk_alignment_get_padding(self.to_glib_none().0, &mut padding_top, &mut padding_bottom, &mut padding_left, &mut padding_right);
            (padding_top, padding_bottom, padding_left, padding_right)
        }
    }

    fn set(&self, xalign: f32, yalign: f32, xscale: f32, yscale: f32) {
        unsafe {
            ffi::gtk_alignment_set(self.to_glib_none().0, xalign, yalign, xscale, yscale);
        }
    }

    fn set_padding(&self, padding_top: u32, padding_bottom: u32, padding_left: u32, padding_right: u32) {
        unsafe {
            ffi::gtk_alignment_set_padding(self.to_glib_none().0, padding_top, padding_bottom, padding_left, padding_right);
        }
    }

    fn get_property_bottom_padding(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bottom-padding".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_bottom_padding(&self, bottom_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "bottom-padding".to_glib_none().0, Value::from(&bottom_padding).to_glib_none().0);
        }
    }

    fn get_property_left_padding(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "left-padding".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_left_padding(&self, left_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "left-padding".to_glib_none().0, Value::from(&left_padding).to_glib_none().0);
        }
    }

    fn get_property_right_padding(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "right-padding".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_right_padding(&self, right_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "right-padding".to_glib_none().0, Value::from(&right_padding).to_glib_none().0);
        }
    }

    fn get_property_top_padding(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "top-padding".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_top_padding(&self, top_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "top-padding".to_glib_none().0, Value::from(&top_padding).to_glib_none().0);
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    fn get_property_xscale(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xscale".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_xscale(&self, xscale: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xscale".to_glib_none().0, Value::from(&xscale).to_glib_none().0);
        }
    }

    fn get_property_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }

    fn get_property_yscale(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yscale".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_yscale(&self, yscale: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yscale".to_glib_none().0, Value::from(&yscale).to_glib_none().0);
        }
    }

    fn connect_property_bottom_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bottom-padding",
                transmute(notify_bottom_padding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_left_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::left-padding",
                transmute(notify_left_padding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_right_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::right-padding",
                transmute(notify_right_padding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_top_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::top-padding",
                transmute(notify_top_padding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xalign",
                transmute(notify_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xscale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xscale",
                transmute(notify_xscale_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::yalign",
                transmute(notify_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_yscale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::yscale",
                transmute(notify_yscale_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_bottom_padding_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_left_padding_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_right_padding_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_top_padding_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xalign_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xscale_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_yalign_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_yscale_trampoline<P>(this: *mut ffi::GtkAlignment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Alignment> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Alignment::from_glib_borrow(this).downcast_unchecked())
}
