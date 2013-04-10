use core::libc::types::os::arch::c95::{c_char,c_int,c_uint};
use core::libc::types::common::c95::{c_void};

use std::time::Tm;

use mod c;

// TODO:
// - git_signature_dup
// - git_signature_now

pub struct Signature {
    c_sig: *c::git_signature,
}
pub fn Signature(c_sig: *c::git_signature) -> Signature {
    return Signature{ c_sig: c_sig };
}

impl Drop for Signature {
    fn finalize(&self) {
        if self.c_sig != ptr::null() {
            unsafe {
                c::git_signature_free(self.c_sig);
            }
        }
    }
}
impl Signature {
    // TODO accessors
}


pub fn Signature_create(name: &str, email: &str, time: Tm) -> Signature {
    let mut retval: c_int = 0;
    let c_sig: *c::git_signature = ptr::null();
    let spec = time.to_timespec();

    do str::as_c_str(name) |name_bytes| {
        do str::as_c_str(email) |email_bytes| {
            retval = unsafe {
                c::git_signature_new(ptr::addr_of(&c_sig), name_bytes, email_bytes, spec.sec as c::git_time_t, time.tm_gmtoff)
            };
        }
    }

    if retval != 0 {
        fail!(~"got an error from C code that i should handle better");
    }

    return Signature(c_sig);
}
