mod libxenevtchn;
extern crate xenevtchn_sys;
use libxenevtchn::LibXenEvtchn;
use log::debug;
use std::convert::TryInto;
use std::io::Error;
use std::os::raw::c_int;
use std::ptr::null_mut;
use xenevtchn_sys::{evtchn_port_t, xenevtchn_handle, xenevtchn_port_or_error_t};

#[derive(Debug)]
pub struct XenEventChannel {
    handle: *mut xenevtchn_handle,
    bind_port: c_int,
    libxenevtchn: LibXenEvtchn,
}

impl XenEventChannel {
    pub fn new(domid: u32, evtchn_port: u32) -> Result<Self, Error> {
        let libxenevtchn = unsafe { LibXenEvtchn::new() };
        let handle = (libxenevtchn.xenevtchn_open)(null_mut(), 0);
        if handle.is_null() {
            return Err(Error::last_os_error());
        }
        let bind_port = (libxenevtchn.xenevtchn_bind_interdomain)(handle, domid, evtchn_port);
        debug!("bind_port = {x}", x = bind_port);
        if bind_port < 0 {
            return Err(Error::last_os_error());
        }

        Ok(XenEventChannel {
            handle,
            bind_port,
            libxenevtchn,
        })
    }

    pub fn xenevtchn_pending(&self) -> Result<xenevtchn_port_or_error_t, Error> {
        let port = (self.libxenevtchn.xenevtchn_pending)(self.handle);
        if port < 0 {
            return Err(Error::last_os_error());
        }
        Ok(port)
    }

    pub fn get_bind_port(&self) -> i32 {
        self.bind_port
    }

    pub fn xenevtchn_fd(&self) -> Result<i32, Error> {
        Ok((self.libxenevtchn.xenevtchn_fd)(self.handle))
    }

    pub fn xenevtchn_unmask(&self, port: evtchn_port_t) -> Result<(), Error> {
        let res = (self.libxenevtchn.xenevtchn_unmask)(self.handle, port);
        if res < 0 {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    pub fn xenevtchn_notify(&self) -> Result<(), Error> {
        let res =
            (self.libxenevtchn.xenevtchn_notify)(self.handle, self.bind_port.try_into().unwrap());
        if res < 0 {
            return Err(Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for XenEventChannel {
    fn drop(&mut self) {
        (self.libxenevtchn.xenevtchn_unbind)(self.handle, self.bind_port as u32);
        (self.libxenevtchn.xenevtchn_close)(self.handle);
    }
}
