extern mod std;

use std::time::Tm;

use mod c;
use libc::types::os::arch::c95::{c_char,c_int,c_uint};
use libc::types::common::c95::{c_void};

// TODO:
// - git_signature_dup
// - git_signature_now

pub struct Signature {
    c_sig: *c::git_signature,

    drop {
        if self.c_sig != ptr::null() {
            c::git_signature_free(self.c_sig);
        }
    }
}
pub fn Signature(c_sig: *c::git_signature) -> Signature {
    return Signature{ c_sig: c_sig };
}

impl Signature {
    // TODO accessors
}


pub fn Signature_create(name: &str, email: &str, time: std::time::Tm) -> Signature {
    let mut retval: c_int = 0;
    let c_sig: *c::git_signature = ptr::null();
    let spec = time.to_timespec();

    do str::as_c_str(name) |name_bytes| {
        do str::as_c_str(email) |email_bytes| {
            retval = c::git_signature_new(ptr::addr_of(&c_sig), name_bytes, email_bytes, spec.sec as c::git_time_t, time.tm_gmtoff);
        }
    }

    if retval != 0 {
        fail;
    }

    return Signature(c_sig);
}
