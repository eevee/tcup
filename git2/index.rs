extern mod std;

use mod c;
use mod objectid;
use libc::types::os::arch::c95::{c_char,c_int,c_uint};
use libc::types::common::c95::{c_void};

pub struct Index {
    c_index: *c::git_index,

    drop {
        if self.c_index != ptr::null() {
            c::git_index_free(self.c_index);
        }
    }
}
pub fn Index(c_index: *c::git_index) -> Index {
    return Index{ c_index: c_index };
}

impl Index {
    fn add(path: &str) {
        let mut retval: c_int = 0;
        do str::as_c_str(path) |bytes| {
            retval = c::git_index_add(self.c_index, bytes, 0);
        }

        if retval != 0 {
            fail;
        }
    }

    fn write() -> c::git_oid {
        let mut retval: c_int = 0;
        retval = c::git_index_write(self.c_index);

        if retval != 0 {
            fail;
        }


        let c_oid: c::git_oid = objectid::BLANK_OID;
        retval = c::git_tree_create_fromindex(ptr::addr_of(&c_oid), self.c_index);

        if retval != 0 {
            fail;
        }

        return c_oid;
    }
}
