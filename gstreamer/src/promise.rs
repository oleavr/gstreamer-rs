// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

use glib;
use glib::translate::{from_glib, from_glib_full};

#[repr(C)]
pub struct PromiseRef(ffi::GstPromise);

unsafe impl Send for PromiseRef {}
unsafe impl Sync for PromiseRef {}

pub type Promise = GstRc<PromiseRef>;

unsafe impl MiniObject for PromiseRef {
    type GstType = ffi::GstPromise;
}

impl GstRc<PromiseRef> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_promise_new()) }
    }
}

impl PromiseRef {
    pub fn set_change_callback(&mut self) {
        unsafe {
            ffi::gst_promise_set_change_callback(self.as_mut_ptr());
        }
    }

    fn set_change_callback<F>(&mut self, func: F)
    where
        F: Fn(&Promise) + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Promise) + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_promise_set_change_callback(
                self.as_mut_ptr(),
                Some(trampoline_promise_change_callback),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    pub fn wait(&self) -> PromiseResult {
        unsafe {
            ffi::gst_promise_set_change_callback(self.as_mut_ptr());
        }
    }

    pub fn reply(&mut self, s: &StructureRef) {
        unsafe {
            ffi::gst_promise_reply(self.as_mut_ptr(), s);
        }
    }

    pub fn interrupt(&mut self) {
        unsafe {
            ffi::gst_promise_interrupt(self.as_mut_ptr());
        }
    }

    pub fn expire(&mut self) {
        unsafe {
            ffi::gst_promise_expire(self.as_mut_ptr());
        }
    }
}

impl glib::types::StaticType for PromiseRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_promise_get_type()) }
    }
}

unsafe extern "C" fn trampoline_promise_change_callback(
    promise: *mut ffi::GstPromise,
    func: gpointer,
) {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Promise) + Send + Sync + 'static) =
        transmute(func);

    func(&from_glib_borrow(pad));
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<Box<Fn()>>::from_raw(ptr as *mut _);
}
