use core::libc::types::os::arch::c95::{c_char,c_int,c_uchar,c_uint};
use core::libc::types::common::c95::{c_void};

use mod c;

//static OBJECTID_RAW_SIZE: uint = c::GIT_OID_RAWSZ;
//static OBJECTID_HEX_SIZE: uint = c::GIT_OID_HEXSZ;
// TODO: minprefixlen

// TODO what the fuck
//pub fn BLANK_OID() -> c::git_oid {
    //return { id: (0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar), };
//}
pub static BLANK_OID: c::git_oid = c::Struct__git_oid{ id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };


struct ObjectID {
    c_oid: *c::git_oid,

    //drop {
        //c::git_object_free(self.c_commit as *c::git_object);
    //}

    // TODO not done...
}
fn ObjectID(c_oid: *c::git_oid) -> ObjectID {
    return ObjectID{ c_oid: c_oid };
}

impl ObjectID {
    // ...
}
