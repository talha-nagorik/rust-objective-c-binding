use objc::{class, msg_send, sel, sel_impl};

type Id = *mut objc::runtime::Object;

/// Returned by `[NSProcessInfo operatingSystemVersion]`. Contains the current
#[repr(C)]
#[derive(Debug)]
pub struct NSOperatingSystemVersion {
    major_version: libc::c_longlong,
    minor_version: libc::c_longlong,
    patch_version: libc::c_longlong,
}

/// Implement Objective-C type encoding for the struct. Allows the `objc` crate
/// to perform function signature matching before performing calls into the Objective-C
/// runtime. This is needed to be able to enable the `verify_message` feature on `objc`.
unsafe impl objc::Encode for NSOperatingSystemVersion {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{?=qqq}") }
    }
}

pub fn get_os_version() -> NSOperatingSystemVersion {
    // the object is lazily instantiated, so we don't release it
    let proc_info: Id = unsafe { msg_send![class!(NSProcessInfo), processInfo] };
    unsafe { msg_send![proc_info, operatingSystemVersion] }
}
