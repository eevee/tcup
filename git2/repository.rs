use std;

import c;
import commit;
import index;
import objectid;
import signature;
import libc::types::os::arch::c95::{c_char,c_int,c_uint};
import libc::types::common::c95::{c_void};

// TODO:
// - ...

// TODO yeah move me outta here
fn maybe_fail(retval: int) {}

extern fn status_foreach_callback(path: *c_char, stat: c_uint, payload: *c_void) -> c_int unsafe {
    let path_str = str::unsafe::from_c_str(path);
    let f: *fn(str, uint) = unsafe::reinterpret_cast(payload);

    (*f)(path_str, stat as uint);

    ret 0;
}

class Repository {
    let c_repository: *c::git_repository;

    new(c_repository: *c::git_repository) {
        self.c_repository = c_repository;
    }

    drop {
        c::bindgen::git_repository_free(self.c_repository);
    }

    fn path() -> str unsafe {
        ret str::unsafe::from_c_str(
            c::bindgen::git_repository_path(self.c_repository));
    }

    fn for_status(f: fn(str, uint)) unsafe {
        c::bindgen::git_status_foreach(self.c_repository, status_foreach_callback, unsafe::reinterpret_cast(ptr::addr_of(f)));
    }

    // TODO this should probably return the same index object every time?
    fn index() -> index::Index {
        let mut retval: c_int = 0;
        let c_index: *c::git_index = ptr::null();

        retval = c::bindgen::git_repository_index(ptr::addr_of(c_index), self.c_repository);

        if retval != 0 {
            fail;
        }

        ret index::Index(c_index);
    }

    // TODO should this accept a tree, or use current index automatically...?
    fn commit(tree_oid: c::git_oid, message: str, author: signature::Signature) -> commit::Commit {
        let c_oid: c::git_oid = objectid::BLANK_OID();

        let mut retval: c_int = 0;
        let c_tree: *c::git_tree = ptr::null();
        retval = c::inline::git_tree_lookup(ptr::addr_of(c_tree), self.c_repository, ptr::addr_of(tree_oid));

        if retval != 0 {
            fail;
        }

        do str::as_c_str(message) |message_bytes| {
            do str::as_c_str("HEAD") |ref_name_bytes| {
                retval = c::bindgen::git_commit_create(
                    ptr::addr_of(c_oid),
                    self.c_repository,
                    ref_name_bytes,  // TODO (update_ref)
                    author.c_sig,
                    author.c_sig,  // TODO committer
                    ptr::null(),  // TODO encoding
                    message_bytes,

                    c_tree,


                    // parent count
                    0,
                    ptr::null()
                );
            }
        }

        if retval != 0 {
            fail;
        }

        let c_commit: *c::git_commit = ptr::null();
        retval = c::inline::git_commit_lookup(ptr::addr_of(c_commit), self.c_repository, ptr::addr_of(c_oid));

        if retval != 0 {
            fail;
        }

        ret commit::Commit(c_commit);
    }

    fn find_commit(hash: &str) -> commit::Commit {
        // TODO
// GIT_INLINE(int) git_commit_lookup(git_commit **commit, git_repository *repo, const git_oid *id)
// GIT_INLINE(int) git_commit_lookup_prefix(git_commit **commit, git_repository *repo, const git_oid *id, unsigned len)
        fail;
    }
}

fn Repository_init(path: str) -> Repository {
    // TODO verify this isn't already an initialized repo?
    let mut retval: c_int = 0;
    let c_repository: *c::git_repository = ptr::null();

    do str::as_c_str(path) |bytes| {
        // TODO bare vs not
        retval = c::bindgen::git_repository_init(ptr::addr_of(c_repository), bytes, 0);
    };

    if retval != 0 {
        fail;
    }

    ret Repository(c_repository);
}

fn Repository_discover(path: str) -> Repository {
    let mut retval: c_int = 0;
    let c_repository: *c::git_repository = ptr::null();

    do str::as_c_str(path) |bytes| {
        retval = c::bindgen::git_repository_open_ext(ptr::addr_of(c_repository), bytes, 0, ptr::null());
    };

    if retval != 0 {
        fail;
    }

    ret Repository(c_repository);
}
