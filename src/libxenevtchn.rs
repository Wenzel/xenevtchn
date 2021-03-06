use libloading::{os::unix::Symbol as RawSymbol, Library, Symbol};
use log::info;
use std::os::raw::c_int;
use xenevtchn_sys::{
    evtchn_port_t, xenevtchn_handle, xenevtchn_port_or_error_t, xentoollog_logger,
};

const LIBXENEVTCHN_FILENAME: &str = "libxenevtchn.so";

//xenevtchn_pending
type FnXenevtchnPending = fn(xce: *mut xenevtchn_handle) -> xenevtchn_port_or_error_t;

//xenevtchn_unmask
type FnXenevtchnUnmask = fn(xce: *mut xenevtchn_handle, port: evtchn_port_t) -> c_int;

//xenevtchn_notify
type FnXenevtchnNotify = fn(xce: *mut xenevtchn_handle, port: evtchn_port_t) -> c_int;

//xenevtchn_fd
type FnXenevtchnFd = fn(xce: *mut xenevtchn_handle) -> c_int;

//xenevtchn_open
type FnXenevtchnOpen =
    fn(logger: *mut xentoollog_logger, open_flags: ::std::os::raw::c_uint) -> *mut xenevtchn_handle;

// xenevtchn_close
type FnXenevtchnClose = fn(xce: *mut xenevtchn_handle) -> c_int;

//xenevtchn_bind_interdomain
type FnXenevtchnBindInterdomain = fn(
    xce: *mut xenevtchn_handle,
    domid: u32,
    remote_port: evtchn_port_t,
) -> xenevtchn_port_or_error_t;

// xenevtchn_unbind
type FnXenevtchnUnbind = fn(xce: *mut xenevtchn_handle, port: evtchn_port_t) -> c_int;

#[derive(Debug)]
pub struct LibXenEvtchn {
    lib: Library,
    pub xenevtchn_pending: RawSymbol<FnXenevtchnPending>,
    pub xenevtchn_unmask: RawSymbol<FnXenevtchnUnmask>,
    pub xenevtchn_notify: RawSymbol<FnXenevtchnNotify>,
    pub xenevtchn_fd: RawSymbol<FnXenevtchnFd>,
    pub xenevtchn_open: RawSymbol<FnXenevtchnOpen>,
    pub xenevtchn_close: RawSymbol<FnXenevtchnClose>,
    pub xenevtchn_bind_interdomain: RawSymbol<FnXenevtchnBindInterdomain>,
    pub xenevtchn_unbind: RawSymbol<FnXenevtchnUnbind>,
}

impl LibXenEvtchn {
    pub unsafe fn new() -> Self {
        info!("Loading {}", LIBXENEVTCHN_FILENAME);
        let lib = Library::new(LIBXENEVTCHN_FILENAME).unwrap();

        let xenevtchn_pending_sym: Symbol<FnXenevtchnPending> =
            lib.get(b"xenevtchn_pending\0").unwrap();
        let xenevtchn_pending = xenevtchn_pending_sym.into_raw();

        let xenevtchn_unmask_sym: Symbol<FnXenevtchnUnmask> =
            lib.get(b"xenevtchn_unmask\0").unwrap();
        let xenevtchn_unmask = xenevtchn_unmask_sym.into_raw();

        let xenevtchn_notify_sym: Symbol<FnXenevtchnNotify> =
            lib.get(b"xenevtchn_notify\0").unwrap();
        let xenevtchn_notify = xenevtchn_notify_sym.into_raw();

        let xenevtchn_fd_sym: Symbol<FnXenevtchnFd> = lib.get(b"xenevtchn_fd\0").unwrap();
        let xenevtchn_fd = xenevtchn_fd_sym.into_raw();

        let xenevtchn_open_sym: Symbol<FnXenevtchnOpen> = lib.get(b"xenevtchn_open\0").unwrap();
        let xenevtchn_open = xenevtchn_open_sym.into_raw();

        let xenevtchn_close_sym: Symbol<FnXenevtchnClose> = lib.get(b"xenevtchn_close\0").unwrap();
        let xenevtchn_close = xenevtchn_close_sym.into_raw();

        let xenevtchn_bind_interdomain_sym: Symbol<FnXenevtchnBindInterdomain> =
            lib.get(b"xenevtchn_bind_interdomain\0").unwrap();
        let xenevtchn_bind_interdomain = xenevtchn_bind_interdomain_sym.into_raw();

        let xenevtchn_unbind_sym: Symbol<FnXenevtchnUnbind> =
            lib.get(b"xenevtchn_unbind\0").unwrap();
        let xenevtchn_unbind = xenevtchn_unbind_sym.into_raw();

        LibXenEvtchn {
            lib,
            xenevtchn_pending,
            xenevtchn_unmask,
            xenevtchn_notify,
            xenevtchn_fd,
            xenevtchn_open,
            xenevtchn_close,
            xenevtchn_bind_interdomain,
            xenevtchn_unbind,
        }
    }
}
