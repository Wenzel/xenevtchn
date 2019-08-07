use std::ptr::null_mut;
use std::os::raw::c_int;
use std::io::Error;

pub struct XenEventChannel {
    handle: *mut xenevtchn_sys::xenevtchn_handle,
    fd: c_int,
    bind_port: c_int,
}

impl XenEventChannel {

    pub fn new(domid: u32, evtchn_port: u32) -> Result<Self,Error> {
        let handle = unsafe {
            xenevtchn_sys::xenevtchn_open(null_mut(), 0)
        };
        if handle == null_mut() { return Err(Error::last_os_error()); }

        let fd = unsafe {
            xenevtchn_sys::xenevtchn_fd(handle)
        };
        let bind_port = unsafe {
            xenevtchn_sys::xenevtchn_bind_interdomain(handle, domid, evtchn_port)
        };
        if bind_port < 0 { return Err(Error::last_os_error()); }

        Ok(XenEventChannel {
            handle,
            fd,
            bind_port,
        })
    }

    pub fn close(&mut self) {
        unsafe {
            xenevtchn_sys::xenevtchn_unbind(self.handle, self.bind_port as u32);
            xenevtchn_sys::xenevtchn_close(self.handle);
        };
    }
}
