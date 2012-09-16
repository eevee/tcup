use std;

import c;
import signature;
import libc::types::os::arch::c95::{c_char,c_int,c_uint};
import libc::types::common::c95::{c_void};


class Commit {
    let c_commit: *c::git_commit;

    new(c_commit: *c::git_commit) {
        self.c_commit = c_commit;
    }

    drop {
        c::bindgen::git_object_free(self.c_commit as *c::git_object);
    }

    fn id() -> ~str {
        let c_oid = c::bindgen::git_commit_id(self.c_commit);
        // TODO
        fail;

    }

    fn timestamp() -> std::time::tm {
        // TODO finish me off
        let time = c::bindgen::git_commit_time(self.c_commit);
        let time_offset = c::bindgen::git_commit_time_offset(self.c_commit);
        // TODO offset is minutes from UTC, remember
        fail;
    }

    fn message() -> str unsafe {
        // TODO: git_commit_message_encoding, should decode this
        ret str::unsafe::from_c_str(c::bindgen::git_commit_message(self.c_commit));
    }

    fn author() -> signature::Signature {
        ret signature::Signature(c::bindgen::git_commit_author(self.c_commit));
    }

    fn committer() -> signature::Signature {
        ret signature::Signature(c::bindgen::git_commit_committer(self.c_commit));
    }

    fn parents() -> ~[Commit] {
        let parent_ct = c::bindgen::git_commit_parentcount(self.c_commit) as uint;
        //let parents: ~[Commit] = ~[];
        let c_commit: *c::git_commit = ptr::null();
        let mut retval: c_int = 0;

        //vec::reserve(parents, parent_ct);
        // TODO wish this were a map to avoid the mut
        //ret vec::from_fn(parent_ct, |i| {
        let parents = do vec::from_fn(parent_ct) |i| {
            retval = c::bindgen::git_commit_parent(ptr::addr_of(c_commit), self.c_commit, i as c_uint);
            if retval != 0 {
                fail;
            }

            //ret Commit(c_commit);
            Commit(c_commit)

            // TODO lazy retrieval?
            // GIT_EXTERN(const git_oid *) git_commit_parent_oid(git_commit *commit, unsigned int n);
        };

        ret parents;
    }

    // TODO
    // GIT_EXTERN(int) git_commit_tree(git_tree **tree_out, git_commit *commit);
    // GIT_EXTERN(const git_oid *) git_commit_tree_oid(git_commit *commit);
}
