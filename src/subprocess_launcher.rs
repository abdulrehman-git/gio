use SubprocessLauncher;
#[cfg(any(unix, feature = "dox"))]
use ffi;
#[cfg(any(unix, feature = "dox"))]
use glib::translate::*;
#[cfg(any(unix, all(feature = "dox", unix)))]
use std::os::unix::io::IntoRawFd;
#[cfg(all(feature = "dox", not(unix)))]
pub trait IntoRawFd: Sized {
    fn into_raw_fd(self) -> i32 { 0 }
}

impl SubprocessLauncher {
    #[cfg(any(unix, feature = "dox"))]
    pub fn take_fd<F: IntoRawFd, G: IntoRawFd>(&self, source_fd: F, target_fd: G) {
        unsafe {
            ffi::g_subprocess_launcher_take_fd(self.to_glib_none().0,
                                               source_fd.into_raw_fd(),
                                               target_fd.into_raw_fd());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn take_stderr_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.to_glib_none().0,
                                                      fd.into_raw_fd());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn take_stdin_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.to_glib_none().0,
                                                     fd.into_raw_fd());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn take_stdout_fd<F: IntoRawFd>(&self, fd: F) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.to_glib_none().0,
                                                      fd.into_raw_fd());
        }
    }
}
