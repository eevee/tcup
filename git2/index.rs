use std;

import c;
import objectid;
import libc::types::os::arch::c95::{c_char,c_int,c_uint};
import libc::types::common::c95::{c_void};

class Index {
    let c_index: *c::git_index;

    new(c_index: *c::git_index) {
        self.c_index = c_index;
    }

    fn add(path: str) {
        let mut retval: c_int = 0;
        do str::as_c_str(path) |bytes| {
            retval = c::bindgen::git_index_add(self.c_index, bytes, 0);
        }

        if retval != 0 {
            fail;
        }
    }

    fn write() -> c::git_oid {
        let mut retval: c_int = 0;
        retval = c::bindgen::git_index_write(self.c_index);

        if retval != 0 {
            fail;
        }


        let c_oid: c::git_oid = objectid::BLANK_OID();
        retval = c::bindgen::git_tree_create_fromindex(ptr::addr_of(c_oid), self.c_index);

        if retval != 0 {
            fail;
        }

        ret c_oid;
    }

    drop {
        c::bindgen::git_index_free(self.c_index);
    }
}
