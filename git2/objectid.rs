use std;

import c;
import libc::types::os::arch::c95::{c_char,c_int,c_uchar,c_uint};
import libc::types::common::c95::{c_void};

//const OBJECTID_RAW_SIZE: uint = c::GIT_OID_RAWSZ;
//const OBJECTID_HEX_SIZE: uint = c::GIT_OID_HEXSZ;
// TODO: minprefixlen

// TODO what the fuck
fn BLANK_OID() -> c::git_oid {
    ret { id: (0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar), };
}


class ObjectID {
    let c_oid: *c::git_oid;

    new(c_oid: *c::git_oid) {
        self.c_oid = c_oid;
    }

    drop {
        //c::bindgen::git_object_free(self.c_commit as *c::git_object);
    }

    // TODO not done...
}
