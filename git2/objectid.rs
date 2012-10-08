extern mod std;

use mod c;
use libc::types::os::arch::c95::{c_char,c_int,c_uchar,c_uint};
use libc::types::common::c95::{c_void};

//const OBJECTID_RAW_SIZE: uint = c::GIT_OID_RAWSZ;
//const OBJECTID_HEX_SIZE: uint = c::GIT_OID_HEXSZ;
// TODO: minprefixlen

// TODO what the fuck
//pub fn BLANK_OID() -> c::git_oid {
    //return { id: (0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar, 0 as c_uchar), };
//}
pub const BLANK_OID: c::git_oid = c::Struct__git_oid{ id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };


struct ObjectID {
    c_oid: *c::git_oid,

    drop {
        //c::git_object_free(self.c_commit as *c::git_object);
    }

    // TODO not done...
}
fn ObjectID(c_oid: *c::git_oid) -> ObjectID {
    return ObjectID{ c_oid: c_oid };
}

impl ObjectID {
    // ...
}
