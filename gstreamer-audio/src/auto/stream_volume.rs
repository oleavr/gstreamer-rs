// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use StreamVolumeFormat;
use ffi;
use glib;
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
    pub struct StreamVolume(Object<ffi::GstStreamVolume, ffi::GstStreamVolumeInterface>);

    match fn {
        get_type => || ffi::gst_stream_volume_get_type(),
    }
}

impl StreamVolume {
    pub fn convert_volume(from: StreamVolumeFormat, to: StreamVolumeFormat, val: f64) -> f64 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_stream_volume_convert_volume(from.to_glib(), to.to_glib(), val)
        }
    }
}

unsafe impl Send for StreamVolume {}
unsafe impl Sync for StreamVolume {}

pub trait StreamVolumeExt {
    fn get_mute(&self) -> bool;

    fn get_volume(&self, format: StreamVolumeFormat) -> f64;

    fn set_mute(&self, mute: bool);

    fn set_volume(&self, format: StreamVolumeFormat, val: f64);

    fn connect_property_mute_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_volume_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StreamVolume> + IsA<glib::object::Object>> StreamVolumeExt for O {
    fn get_mute(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_stream_volume_get_mute(self.to_glib_none().0))
        }
    }

    fn get_volume(&self, format: StreamVolumeFormat) -> f64 {
        unsafe {
            ffi::gst_stream_volume_get_volume(self.to_glib_none().0, format.to_glib())
        }
    }

    fn set_mute(&self, mute: bool) {
        unsafe {
            ffi::gst_stream_volume_set_mute(self.to_glib_none().0, mute.to_glib());
        }
    }

    fn set_volume(&self, format: StreamVolumeFormat, val: f64) {
        unsafe {
            ffi::gst_stream_volume_set_volume(self.to_glib_none().0, format.to_glib(), val);
        }
    }

    fn connect_property_mute_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mute",
                transmute(notify_mute_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_volume_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::volume",
                transmute(notify_volume_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_mute_trampoline<P>(this: *mut ffi::GstStreamVolume, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StreamVolume> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&StreamVolume::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_volume_trampoline<P>(this: *mut ffi::GstStreamVolume, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StreamVolume> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&StreamVolume::from_glib_borrow(this).downcast_unchecked())
}
