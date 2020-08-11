mod libxenevtchn;
extern crate xenevtchn_sys;
use libxenevtchn::LibXenEvtchn;
use log::debug;
use std::convert::TryInto;
use std::io::Error;
use std::os::raw::c_int;
use std::ptr::null_mut;
use xenevtchn_sys::xenevtchn_handle;
pub use xenevtchn_sys::{evtchn_port_t, xenevtchn_port_or_error_t};

#[derive(Debug)]
pub struct XenEventChannel {
    handle: *mut xenevtchn_handle,
    pub bind_port: c_int,
    libxenevtchn: LibXenEvtchn,
}

pub trait EventChannelSetup: std::fmt::Debug {
    fn get_bind_port(&self) -> i32;
    fn init(&mut self, domid: u32, evtchn_port: u32) -> Result<(), Error>;
    fn xenevtchn_pending(&self) -> Result<xenevtchn_port_or_error_t, Error>;
    fn xenevtchn_fd(&self) -> Result<i32, Error>;
    fn xenevtchn_unmask(&self, port: evtchn_port_t) -> Result<(), Error>;
    fn xenevtchn_notify(&self) -> Result<(), Error>;
}

pub fn create_xen_event_channel() -> XenEventChannel {
    XenEventChannel::new(unsafe { LibXenEvtchn::new() })
}

impl XenEventChannel {
    fn new(libxenevtchn: LibXenEvtchn) -> XenEventChannel {
        XenEventChannel {
            handle: null_mut(),
            bind_port: Default::default(),
            libxenevtchn,
        }
    }
}

impl EventChannelSetup for XenEventChannel {
    fn init(&mut self, domid: u32, evtchn_port: u32) -> Result<(), Error> {
        self.handle = unsafe { xenevtchn_sys::xenevtchn_open(null_mut(), 0) };
        if self.handle.is_null() {
            return Err(Error::last_os_error());
        }
        self.bind_port =
            unsafe { xenevtchn_sys::xenevtchn_bind_interdomain(self.handle, domid, evtchn_port) };
        debug!("bind_port = {x}", x = self.bind_port);
        if self.bind_port < 0 {
            return Err(Error::last_os_error());
        }

        self.libxenevtchn = unsafe { LibXenEvtchn::new() };
        Ok(())
    }

    fn get_bind_port(&self) -> i32 {
        self.bind_port
    }

    fn xenevtchn_pending(&self) -> Result<xenevtchn_port_or_error_t, Error> {
        let port = (self.libxenevtchn.xenevtchn_pending)(self.handle);
        if port < 0 {
            return Err(Error::last_os_error());
        }
        Ok(port)
    }

    fn xenevtchn_fd(&self) -> Result<i32, Error> {
        Ok((self.libxenevtchn.xenevtchn_fd)(self.handle))
    }

    fn xenevtchn_unmask(&self, port: evtchn_port_t) -> Result<(), Error> {
        let res = (self.libxenevtchn.xenevtchn_unmask)(self.handle, port);
        if res < 0 {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    fn xenevtchn_notify(&self) -> Result<(), Error> {
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
        unsafe {
            xenevtchn_sys::xenevtchn_unbind(self.handle, self.bind_port as u32);
        };
        unsafe {
            xenevtchn_sys::xenevtchn_close(self.handle);
        };
    }
}
