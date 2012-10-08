extern mod std;

use mod c;
use mod commit;
use mod index;
use mod objectid;
use mod signature;
use libc::types::os::arch::c95::{c_char,c_int,c_uint};
use libc::types::common::c95::{c_void};

// TODO:
// - ...

// TODO yeah move me outta here
fn maybe_fail(retval: int) {}

extern fn status_foreach_callback(path: *c_char, stat: c_uint, payload: *c_void) -> c_int unsafe {
    let path_str = str::raw::from_c_str(path);
    let f: *fn(&str, uint) = cast::reinterpret_cast(&payload);

    (*f)(path_str, stat as uint);

    return 0;
}

pub struct Repository {
    c_repository: *c::git_repository,

    drop {
        if self.c_repository != ptr::null() {
            c::git_repository_free(self.c_repository);
        }
    }
}
pub fn Repository(c_repository: *c::git_repository) -> Repository {
    return Repository{ c_repository: c_repository };
}

impl Repository {
    fn path() -> ~str unsafe {
        return str::raw::from_c_str(
            c::git_repository_path(self.c_repository));
    }

    fn for_status(f: fn(&str, uint)) unsafe {
        c::git_status_foreach(self.c_repository, status_foreach_callback, cast::reinterpret_cast(&ptr::addr_of(&f)));
    }

    // TODO this should probably return the same index object every time?
    fn index() -> index::Index {
        let mut retval: c_int = 0;
        let c_index: *c::git_index = ptr::null();

        retval = c::git_repository_index(ptr::addr_of(&c_index), self.c_repository);

        if retval != 0 {
            fail;
        }

        return index::Index(c_index);
    }

    // TODO should this accept a tree, or use current index automatically...?
    fn commit(tree_oid: c::git_oid, message: &str, author: signature::Signature) -> commit::Commit {
        let c_oid: c::git_oid = objectid::BLANK_OID;

        let mut retval: c_int = 0;
        let c_tree: *c::git_tree = ptr::null();
        retval = c::git_tree_lookup(ptr::addr_of(&c_tree), self.c_repository, ptr::addr_of(&tree_oid));

        if retval != 0 {
            fail;
        }

        do str::as_c_str(message) |message_bytes| {
            do str::as_c_str("HEAD") |ref_name_bytes| {
                retval = c::git_commit_create(
                    ptr::addr_of(&c_oid),
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
        retval = c::git_commit_lookup(ptr::addr_of(&c_commit), self.c_repository, ptr::addr_of(&c_oid));

        if retval != 0 {
            fail;
        }

        return commit::Commit(c_commit);
    }

    fn find_commit(hash: &str) -> commit::Commit {
        // TODO
// GIT_INLINE(int) git_commit_lookup(git_commit **commit, git_repository *repo, const git_oid *id)
// GIT_INLINE(int) git_commit_lookup_prefix(git_commit **commit, git_repository *repo, const git_oid *id, unsigned len)
        fail;
    }
}

pub fn Repository_init(path: &str) -> Repository {
    // TODO verify this isn't already an initialized repo?
    let mut retval: c_int = 0;
    let c_repository: *c::git_repository = ptr::null();

    do str::as_c_str(path) |bytes| {
        // TODO bare vs not
        retval = c::git_repository_init(ptr::addr_of(&c_repository), bytes, 0);
    };

    if retval != 0 {
        fail;
    }

    return Repository(c_repository);
}

pub fn Repository_discover(path: &str) -> Repository {
    let mut retval: c_int = 0;
    let c_repository: *c::git_repository = ptr::null();

    do str::as_c_str(path) |bytes| {
        retval = c::git_repository_open_ext(ptr::addr_of(&c_repository), bytes, 0, ptr::null());
    };

    if retval != 0 {
        fail;
    }

    return Repository(c_repository);
}
