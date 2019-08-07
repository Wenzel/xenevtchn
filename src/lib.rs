use std::ptr::null_mut;
use std::os::raw::c_int;

pub struct Xce {
    handle: *mut xenevtchn_sys::xenevtchn_handle,
    fd: c_int,
}

impl Xce {

    pub fn new(domid: u32, evtchn_port: u32) -> Self {
        let handle = unsafe {
            xenevtchn_sys::xenevtchn_open(null_mut(), 0)
        };

        let fd = unsafe {
            xenevtchn_sys::xenevtchn_fd(handle)
        };
        unsafe {
            xenevtchn_sys::xenevtchn_bind_interdomain(handle, domid, evtchn_port);
        };
        Xce {
            handle: handle,
            fd: fd
        }
    }

    pub fn close(&mut self) {
        unsafe {
            xenevtchn_sys::xenevtchn_close(self.handle);
        };
    }
}
