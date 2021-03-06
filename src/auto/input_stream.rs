// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct InputStream(Object<ffi::GInputStream, ffi::GInputStreamClass, InputStreamClass>);

    match fn {
        get_type => || ffi::g_input_stream_get_type(),
    }
}

pub const NONE_INPUT_STREAM: Option<&InputStream> = None;

pub trait InputStreamExt: 'static {
    fn clear_pending(&self);

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error>;

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn close_async_future(&self, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn read_bytes<P: IsA<Cancellable>>(&self, count: usize, cancellable: Option<&P>) -> Result<glib::Bytes, Error>;

    fn read_bytes_async<P: IsA<Cancellable>, Q: FnOnce(Result<glib::Bytes, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn read_bytes_async_future(&self, count: usize, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, glib::Bytes), Error = (Self, Error)>> where Self: Sized + Clone;

    fn set_pending(&self) -> Result<(), Error>;

    fn skip<P: IsA<Cancellable>>(&self, count: usize, cancellable: Option<&P>) -> Result<isize, Error>;

    fn skip_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn skip_async_future(&self, count: usize, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, isize), Error = (Self, Error)>> where Self: Sized + Clone;
}

impl<O: IsA<InputStream>> InputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_input_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_close(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn close_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_input_stream_close_async(self.as_ref().to_glib_none().0, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
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

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_input_stream_has_pending(self.as_ref().to_glib_none().0))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_input_stream_is_closed(self.as_ref().to_glib_none().0))
        }
    }

    fn read_bytes<P: IsA<Cancellable>>(&self, count: usize, cancellable: Option<&P>) -> Result<glib::Bytes, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read_bytes(self.as_ref().to_glib_none().0, count, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_bytes_async<P: IsA<Cancellable>, Q: FnOnce(Result<glib::Bytes, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_bytes_async_trampoline<Q: FnOnce(Result<glib::Bytes, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read_bytes_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_bytes_async_trampoline::<Q>;
        unsafe {
            ffi::g_input_stream_read_bytes_async(self.as_ref().to_glib_none().0, count, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn read_bytes_async_future(&self, count: usize, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, glib::Bytes), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.read_bytes_async(
                count,
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

    fn set_pending(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn skip<P: IsA<Cancellable>>(&self, count: usize, cancellable: Option<&P>) -> Result<isize, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_skip(self.as_ref().to_glib_none().0, count, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn skip_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn skip_async_trampoline<Q: FnOnce(Result<isize, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_skip_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = skip_async_trampoline::<Q>;
        unsafe {
            ffi::g_input_stream_skip_async(self.as_ref().to_glib_none().0, count, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn skip_async_future(&self, count: usize, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, isize), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.skip_async(
                count,
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
}

impl fmt::Display for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputStream")
    }
}
