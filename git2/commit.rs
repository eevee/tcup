use core::libc::types::os::arch::c95::{c_char,c_int,c_uint};
use core::libc::types::common::c95::{c_void};

use std::time::Tm;

use mod c;
use mod signature;


pub struct Commit {
    c_commit: *c::git_commit,
}
pub fn Commit(c_commit: *c::git_commit) -> Commit {
    return Commit{ c_commit: c_commit };
}

impl Drop for Commit {
    fn finalize(&self) {
        if self.c_commit != ptr::null() {
            unsafe {
                c::git_object_free(self.c_commit as *c::git_object);
            }
        }
    }
}
impl Commit {
    pub fn id(&self) -> ~str {
        let c_oid = unsafe {
            c::git_commit_id(self.c_commit)
        };
        // TODO
        fail!(~"NotImplementedError");

    }

    pub fn timestamp(&self) -> Tm {
        // TODO finish me off
        unsafe {
            let time = c::git_commit_time(self.c_commit);
            let time_offset = c::git_commit_time_offset(self.c_commit);
        }
        // TODO offset is minutes from UTC, remember
        fail!(~"NotImplementedError");
    }

    pub fn message(&self) -> ~str {
        unsafe {
            // TODO: git_commit_message_encoding, should decode this
            return unsafe {
                str::raw::from_c_str(c::git_commit_message(self.c_commit))
            }
        }
    }

    pub fn author(&self) -> signature::Signature {
        return signature::Signature(unsafe {
            c::git_commit_author(self.c_commit)
        });
    }

    pub fn committer(&self) -> signature::Signature {
        return signature::Signature(unsafe {
            c::git_commit_committer(self.c_commit)
        });
    }

    pub fn parents(&self) -> ~[Commit] {
        let parent_ct = unsafe {
            c::git_commit_parentcount(self.c_commit) as uint
        };
        //let parents: ~[Commit] = ~[];
        let c_commit: *c::git_commit = ptr::null();
        let mut retval: c_int = 0;

        //vec::reserve(parents, parent_ct);
        // TODO wish this were a map to avoid the mut
        //return vec::from_fn(parent_ct, |i| {
        let parents = do vec::from_fn(parent_ct) |i| {
            retval = unsafe {
                c::git_commit_parent(ptr::addr_of(&c_commit), self.c_commit, i as c_uint)
            };
            if retval != 0 {
                fail!(~"got an error from C code that i should handle better");
            }

            //return Commit(c_commit);
            Commit(c_commit)

            // TODO lazy retrieval?
            // GIT_EXTERN(const git_oid *) git_commit_parent_oid(git_commit *commit, unsigned int n);
        };

        return parents;
    }

    // TODO
    // GIT_EXTERN(int) git_commit_tree(git_tree **tree_out, git_commit *commit);
    // GIT_EXTERN(const git_oid *) git_commit_tree_oid(git_commit *commit);
}
