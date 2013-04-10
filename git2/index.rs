use core::libc::types::os::arch::c95::{c_char,c_int,c_uint};
use core::libc::types::common::c95::{c_void};

use mod c;
use objectid;

pub struct Index {
    c_index: *c::git_index,
}
pub fn Index(c_index: *c::git_index) -> Index {
    return Index{ c_index: c_index };
}

impl Drop for Index {
    fn finalize(&self) {
        if self.c_index != ptr::null() {
            unsafe {
                c::git_index_free(self.c_index);
            }
        }
    }
}

impl Index {
    pub fn add(&self, path: &str) {
        let mut retval: c_int = 0;
        do str::as_c_str(path) |bytes| {
            retval = unsafe {
                c::git_index_add(self.c_index, bytes, 0)
            };
        }

        if retval != 0 {
            fail!(~"got an error from C code that i should handle better");
        }
    }

    pub fn write(&self) -> c::git_oid {
        let mut retval: c_int = 0;
        retval = unsafe {
            c::git_index_write(self.c_index)
        };

        if retval != 0 {
            fail!(~"got an error from C code that i should handle better");
        }


        let c_oid: c::git_oid = objectid::BLANK_OID;
        retval = unsafe {
            c::git_tree_create_fromindex(ptr::addr_of(&c_oid), self.c_index)
        };

        if retval != 0 {
            fail!(~"got an error from C code that i should handle better");
        }

        return c_oid;
    }
}
