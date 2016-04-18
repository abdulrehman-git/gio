// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use Permission;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SimplePermission(Object<ffi::GSimplePermission>): Permission;

    match fn {
        get_type => || ffi::g_simple_permission_get_type(),
    }
}

impl SimplePermission {
    pub fn new(allowed: bool) -> SimplePermission {
        unsafe {
            Permission::from_glib_full(ffi::g_simple_permission_new(allowed.to_glib())).downcast_unchecked()
        }
    }
}
