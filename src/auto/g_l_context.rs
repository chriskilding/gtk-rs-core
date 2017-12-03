// This file was generated by gir (38add47) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_16", feature = "dox"))]
use Display;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Error;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Window;
use ffi;
use glib;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GLContext(Object<ffi::GdkGLContext>);

    match fn {
        get_type => || ffi::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_gl_context_clear_current();
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_current())
        }
    }
}

pub trait GLContextExt {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_debug_enabled(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_display(&self) -> Option<Display>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_forward_compatible(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_required_version(&self) -> (i32, i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_shared_context(&self) -> Option<GLContext>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_use_es(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_version(&self) -> (i32, i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_window(&self) -> Option<Window>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn is_legacy(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn make_current(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn realize(&self) -> Result<(), Error>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_debug_enabled(&self, enabled: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_forward_compatible(&self, compatible: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_required_version(&self, major: i32, minor: i32);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_use_es(&self, use_es: i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_shared_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GLContext> + IsA<glib::object::Object>> GLContextExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_debug_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_debug_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_display(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_forward_compatible(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_forward_compatible(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gdk_gl_context_get_required_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_shared_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_shared_context(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_use_es(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_use_es(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gdk_gl_context_get_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn is_legacy(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_is_legacy(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn make_current(&self) {
        unsafe {
            ffi::gdk_gl_context_make_current(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn realize(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_gl_context_realize(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_debug_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gdk_gl_context_set_debug_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_forward_compatible(&self, compatible: bool) {
        unsafe {
            ffi::gdk_gl_context_set_forward_compatible(self.to_glib_none().0, compatible.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gdk_gl_context_set_required_version(self.to_glib_none().0, major, minor);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_use_es(&self, use_es: i32) {
        unsafe {
            ffi::gdk_gl_context_set_use_es(self.to_glib_none().0, use_es);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_shared_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shared-context",
                transmute(notify_shared_context_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::window",
                transmute(notify_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkGLContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_shared_context_trampoline<P>(this: *mut ffi::GdkGLContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_window_trampoline<P>(this: *mut ffi::GdkGLContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GLContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GLContext::from_glib_borrow(this).downcast_unchecked())
}
