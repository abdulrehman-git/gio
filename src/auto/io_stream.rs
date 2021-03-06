// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use InputStream;
use OutputStream;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct IOStream(Object<ffi::GIOStream, ffi::GIOStreamClass, IOStreamClass>);

    match fn {
        get_type => || ffi::g_io_stream_get_type(),
    }
}

impl IOStream {}

pub const NONE_IO_STREAM: Option<&IOStream> = None;

pub trait IOStreamExt: 'static {
    fn clear_pending(&self);

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn close_async_future(&self, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    fn get_input_stream(&self) -> Option<InputStream>;

    fn get_output_stream(&self) -> Option<OutputStream>;

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn set_pending(&self) -> Result<(), Error>;

    fn get_property_closed(&self) -> bool;

    fn connect_property_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IOStream>> IOStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_io_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn close_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_io_stream_close_async(self.as_ref().to_glib_none().0, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn close_async_future(&self, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.close_async(
                io_priority,
                Some(&cancellable),
                move |res| {
                    let obj = obj_clone.into_inner();
                    let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn get_input_stream(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_input_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn get_output_stream(&self) -> Option<OutputStream> {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_output_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_io_stream_has_pending(self.as_ref().to_glib_none().0))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_io_stream_is_closed(self.as_ref().to_glib_none().0))
        }
    }

    fn set_pending(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_closed(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"closed\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::closed\0".as_ptr() as *const _,
                Some(transmute(notify_closed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_closed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GIOStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IOStream> {
    let f: &F = &*(f as *const F);
    f(&IOStream::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for IOStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IOStream")
    }
}
