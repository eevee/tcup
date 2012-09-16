use std;

import std::time::tm;

import c;
import libc::types::os::arch::c95::{c_char,c_int,c_uint};
import libc::types::common::c95::{c_void};

// TODO:
// - git_signature_dup
// - git_signature_now

class Signature {
    let c_sig: *c::git_signature;

    new(c_sig: *c::git_signature) {
        self.c_sig = c_sig;
    }

    // TODO accessors

    drop {
        c::bindgen::git_signature_free(self.c_sig);
    }
}


fn Signature_create(name: str, email: str, time: std::time::tm) -> Signature {
    let mut retval: c_int = 0;
    let c_sig: *c::git_signature = ptr::null();
    let spec = time.to_timespec();

    do str::as_c_str(name) |name_bytes| {
        do str::as_c_str(email) |email_bytes| {
            retval = c::bindgen::git_signature_new(ptr::addr_of(c_sig), name_bytes, email_bytes, spec.sec as c::git_time_t, time.tm_gmtoff);
        }
    }

    if retval != 0 {
        fail;
    }

    ret Signature(c_sig);
}
