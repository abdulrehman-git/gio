// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use NetworkConnectivity;
use SocketConnectable;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
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
    pub struct NetworkMonitor(Interface<ffi::GNetworkMonitor>);

    match fn {
        get_type => || ffi::g_network_monitor_get_type(),
    }
}

impl NetworkMonitor {
    pub fn get_default() -> Option<NetworkMonitor> {
        unsafe {
            from_glib_none(ffi::g_network_monitor_get_default())
        }
    }
}

pub const NONE_NETWORK_MONITOR: Option<&NetworkMonitor> = None;

pub trait NetworkMonitorExt: 'static {
    fn can_reach<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(&self, connectable: &P, cancellable: Option<&Q>) -> Result<(), Error>;

    fn can_reach_async<P: IsA<SocketConnectable>, Q: IsA<Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, connectable: &P, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn can_reach_async_future<P: IsA<SocketConnectable> + Clone + 'static>(&self, connectable: &P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn get_connectivity(&self) -> NetworkConnectivity;

    fn get_network_available(&self) -> bool;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_network_metered(&self) -> bool;

    fn connect_network_changed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_connectivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_network_available_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_property_network_metered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NetworkMonitor>> NetworkMonitorExt for O {
    fn can_reach<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(&self, connectable: &P, cancellable: Option<&Q>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_network_monitor_can_reach(self.as_ref().to_glib_none().0, connectable.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn can_reach_async<P: IsA<SocketConnectable>, Q: IsA<Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, connectable: &P, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn can_reach_async_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_network_monitor_can_reach_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = can_reach_async_trampoline::<R>;
        unsafe {
            ffi::g_network_monitor_can_reach_async(self.as_ref().to_glib_none().0, connectable.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn can_reach_async_future<P: IsA<SocketConnectable> + Clone + 'static>(&self, connectable: &P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let connectable = connectable.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.can_reach_async(
                &connectable,
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

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn get_connectivity(&self) -> NetworkConnectivity {
        unsafe {
            from_glib(ffi::g_network_monitor_get_connectivity(self.as_ref().to_glib_none().0))
        }
    }

    fn get_network_available(&self) -> bool {
        unsafe {
            from_glib(ffi::g_network_monitor_get_network_available(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_network_metered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_network_monitor_get_network_metered(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_network_changed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"network-changed\0".as_ptr() as *const _,
                Some(transmute(network_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_connectivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::connectivity\0".as_ptr() as *const _,
                Some(transmute(notify_connectivity_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_network_available_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::network-available\0".as_ptr() as *const _,
                Some(transmute(notify_network_available_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_property_network_metered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::network-metered\0".as_ptr() as *const _,
                Some(transmute(notify_network_metered_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn network_changed_trampoline<P, F: Fn(&P, bool) + 'static>(this: *mut ffi::GNetworkMonitor, network_available: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<NetworkMonitor> {
    let f: &F = &*(f as *const F);
    f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast(), from_glib(network_available))
}

#[cfg(any(feature = "v2_44", feature = "dox"))]
unsafe extern "C" fn notify_connectivity_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GNetworkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkMonitor> {
    let f: &F = &*(f as *const F);
    f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_network_available_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GNetworkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkMonitor> {
    let f: &F = &*(f as *const F);
    f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
unsafe extern "C" fn notify_network_metered_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GNetworkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkMonitor> {
    let f: &F = &*(f as *const F);
    f(&NetworkMonitor::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for NetworkMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NetworkMonitor")
    }
}
