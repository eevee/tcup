/* automatically generated by rust-bindgen */

use core::libc::*;
use core::libc::types::common::c99::*;


pub struct git_strarray {
    strings: **c_schar,
    count: size_t
}

pub type Enum_unnamed1 = c_int;
pub static GIT_OK: c_int = 0;
pub static GIT_ERROR: c_int = -1;
pub static GIT_ENOTFOUND: c_int = -3;
pub static GIT_EEXISTS: c_int = -4;
pub static GIT_EAMBIGUOUS: c_int = -5;
pub static GIT_EBUFS: c_int = -6;
pub static GIT_PASSTHROUGH: c_int = -30;
pub static GIT_REVWALKOVER: c_int = -31;


pub struct git_error {
    message: *c_schar,
    klass: c_int
}

pub type git_error_t = c_uint;
pub static GITERR_NOMEMORY: c_uint = 0;
pub static GITERR_OS: c_uint = 1;
pub static GITERR_INVALID: c_uint = 2;
pub static GITERR_REFERENCE: c_uint = 3;
pub static GITERR_ZLIB: c_uint = 4;
pub static GITERR_REPOSITORY: c_uint = 5;
pub static GITERR_CONFIG: c_uint = 6;
pub static GITERR_REGEX: c_uint = 7;
pub static GITERR_ODB: c_uint = 8;
pub static GITERR_INDEX: c_uint = 9;
pub static GITERR_OBJECT: c_uint = 10;
pub static GITERR_NET: c_uint = 11;
pub static GITERR_TAG: c_uint = 12;
pub static GITERR_TREE: c_uint = 13;
pub static GITERR_INDEXER: c_uint = 14;


pub type git_off_t = int64_t;

pub type git_time_t = int64_t;

pub type git_otype = c_int;
pub static GIT_OBJ_ANY: c_int = -2;
pub static GIT_OBJ_BAD: c_int = -1;
pub static GIT_OBJ__EXT1: c_int = 0;
pub static GIT_OBJ_COMMIT: c_int = 1;
pub static GIT_OBJ_TREE: c_int = 2;
pub static GIT_OBJ_BLOB: c_int = 3;
pub static GIT_OBJ_TAG: c_int = 4;
pub static GIT_OBJ__EXT2: c_int = 5;
pub static GIT_OBJ_OFS_DELTA: c_int = 6;
pub static GIT_OBJ_REF_DELTA: c_int = 7;


type Struct_git_odb = c_void;

pub type git_odb = Struct_git_odb;

pub type git_odb_backend = Struct_git_odb_backend;

type Struct_git_odb_object = c_void;

pub type git_odb_object = Struct_git_odb_object;

pub type git_odb_stream = Struct_git_odb_stream;

type Struct_git_repository = c_void;

pub type git_repository = Struct_git_repository;

type Struct_git_object = c_void;

pub type git_object = Struct_git_object;

type Struct_git_revwalk = c_void;

pub type git_revwalk = Struct_git_revwalk;

type Struct_git_tag = c_void;

pub type git_tag = Struct_git_tag;

type Struct_git_blob = c_void;

pub type git_blob = Struct_git_blob;

type Struct_git_commit = c_void;

pub type git_commit = Struct_git_commit;

type Struct_git_tree_entry = c_void;

pub type git_tree_entry = Struct_git_tree_entry;

type Struct_git_tree = c_void;

pub type git_tree = Struct_git_tree;

type Struct_git_treebuilder = c_void;

pub type git_treebuilder = Struct_git_treebuilder;

type Struct_git_index = c_void;

pub type git_index = Struct_git_index;

type Struct_git_config = c_void;

pub type git_config = Struct_git_config;

pub type git_config_file = Struct_git_config_file;

type Struct_git_reflog_entry = c_void;

pub type git_reflog_entry = Struct_git_reflog_entry;

type Struct_git_reflog = c_void;

pub type git_reflog = Struct_git_reflog;

type Struct_git_note = c_void;

pub type git_note = Struct_git_note;

struct Struct_git_time {
    time: git_time_t,
    offset: c_int
}

pub type git_time = Struct_git_time;

struct Struct_git_signature {
    name: *c_schar,
    email: *c_schar,
    when: git_time
}

pub type git_signature = Struct_git_signature;

type Struct_git_reference = c_void;

pub type git_reference = Struct_git_reference;

pub type git_ref_t = c_uint;
pub static GIT_REF_INVALID: c_uint = 0;
pub static GIT_REF_OID: c_uint = 1;
pub static GIT_REF_SYMBOLIC: c_uint = 2;
pub static GIT_REF_PACKED: c_uint = 4;
pub static GIT_REF_HAS_PEEL: c_uint = 8;
pub static GIT_REF_LISTALL: c_uint = 7;


pub type git_branch_t = c_uint;
pub static GIT_BRANCH_LOCAL: c_uint = 1;
pub static GIT_BRANCH_REMOTE: c_uint = 2;


type Struct_git_refspec = c_void;

pub type git_refspec = Struct_git_refspec;

type Struct_git_remote = c_void;

pub type git_remote = Struct_git_remote;

pub type git_remote_head = Struct_git_remote_head;

pub type git_oid = Struct__git_oid;

pub struct Struct__git_oid {
    id: [c_uchar, ..20]
}

type Struct_git_oid_shorten = c_void;

pub type git_oid_shorten = Struct_git_oid_shorten;

struct Struct_git_odb_backend {
    odb: *git_odb,
    read: *u8,
    read_prefix: *u8,
    read_header: *u8,
    write: *u8,
    writestream: *u8,
    readstream: *u8,
    exists: *u8,
    free: *u8
}

type Enum_unnamed2 = c_uint;
pub static GIT_STREAM_RDONLY: c_uint = 2;
pub static GIT_STREAM_WRONLY: c_uint = 4;
pub static GIT_STREAM_RW: c_uint = 6;


struct Struct_git_odb_stream {
    backend: *Struct_git_odb_backend,
    mode: c_int,
    read: *u8,
    write: *u8,
    finalize_write: *u8,
    free: *u8
}

type Enum_unnamed3 = c_uint;
pub static GIT_REPOSITORY_OPEN_NO_SEARCH: c_uint = 1;
pub static GIT_REPOSITORY_OPEN_CROSS_FS: c_uint = 2;


pub type git_treewalk_cb = *u8;

type Enum_git_treewalk_mode = c_uint;
pub static GIT_TREEWALK_PRE: c_uint = 0;
pub static GIT_TREEWALK_POST: c_uint = 1;


type Enum_unnamed4 = c_uint;
pub static GIT_DIFF_NORMAL: c_uint = 0;
pub static GIT_DIFF_REVERSE: c_uint = 1;
pub static GIT_DIFF_FORCE_TEXT: c_uint = 2;
pub static GIT_DIFF_IGNORE_WHITESPACE: c_uint = 4;
pub static GIT_DIFF_IGNORE_WHITESPACE_CHANGE: c_uint = 8;
pub static GIT_DIFF_IGNORE_WHITESPACE_EOL: c_uint = 16;
pub static GIT_DIFF_IGNORE_SUBMODULES: c_uint = 32;
pub static GIT_DIFF_PATIENCE: c_uint = 64;
pub static GIT_DIFF_INCLUDE_IGNORED: c_uint = 128;
pub static GIT_DIFF_INCLUDE_UNTRACKED: c_uint = 256;
pub static GIT_DIFF_INCLUDE_UNMODIFIED: c_uint = 512;
pub static GIT_DIFF_RECURSE_UNTRACKED_DIRS: c_uint = 1024;


pub struct git_diff_options {
    flags: uint32_t,
    context_lines: uint16_t,
    interhunk_lines: uint16_t,
    old_prefix: *c_schar,
    new_prefix: *c_schar,
    pathspec: git_strarray
}

type Struct_git_diff_list = c_void;

pub type git_diff_list = Struct_git_diff_list;

type Enum_unnamed5 = c_uint;
pub static GIT_DIFF_FILE_VALID_OID: c_uint = 1;
pub static GIT_DIFF_FILE_FREE_PATH: c_uint = 2;
pub static GIT_DIFF_FILE_BINARY: c_uint = 4;
pub static GIT_DIFF_FILE_NOT_BINARY: c_uint = 8;
pub static GIT_DIFF_FILE_FREE_DATA: c_uint = 16;
pub static GIT_DIFF_FILE_UNMAP_DATA: c_uint = 32;


pub type git_delta_t = c_uint;
pub static GIT_DELTA_UNMODIFIED: c_uint = 0;
pub static GIT_DELTA_ADDED: c_uint = 1;
pub static GIT_DELTA_DELETED: c_uint = 2;
pub static GIT_DELTA_MODIFIED: c_uint = 3;
pub static GIT_DELTA_RENAMED: c_uint = 4;
pub static GIT_DELTA_COPIED: c_uint = 5;
pub static GIT_DELTA_IGNORED: c_uint = 6;
pub static GIT_DELTA_UNTRACKED: c_uint = 7;


pub struct git_diff_file {
    oid: git_oid,
    path: *c_schar,
    mode: uint16_t,
    size: git_off_t,
    flags: c_uint
}

pub struct git_diff_delta {
    old_file: git_diff_file,
    new_file: git_diff_file,
    status: git_delta_t,
    similarity: c_uint,
    binary: c_int
}

pub type git_diff_file_fn = *u8;

pub struct git_diff_range {
    old_start: c_int,
    old_lines: c_int,
    new_start: c_int,
    new_lines: c_int
}

pub type git_diff_hunk_fn = *u8;

type Enum_unnamed6 = c_uint;
pub static GIT_DIFF_LINE_CONTEXT: c_uint = 32;
pub static GIT_DIFF_LINE_ADDITION: c_uint = 43;
pub static GIT_DIFF_LINE_DELETION: c_uint = 45;
pub static GIT_DIFF_LINE_ADD_EOFNL: c_uint = 10;
pub static GIT_DIFF_LINE_DEL_EOFNL: c_uint = 0;
pub static GIT_DIFF_LINE_FILE_HDR: c_uint = 70;
pub static GIT_DIFF_LINE_HUNK_HDR: c_uint = 72;
pub static GIT_DIFF_LINE_BINARY: c_uint = 66;


pub type git_diff_data_fn = *u8;

pub struct git_index_time {
    seconds: git_time_t,
    nanoseconds: c_uint
}

struct Struct_git_index_entry {
    ctime: git_index_time,
    mtime: git_index_time,
    dev: c_uint,
    ino: c_uint,
    mode: c_uint,
    uid: c_uint,
    gid: c_uint,
    file_size: git_off_t,
    oid: git_oid,
    flags: c_ushort,
    flags_extended: c_ushort,
    path: *c_schar
}

pub type git_index_entry = Struct_git_index_entry;

struct Struct_git_index_entry_unmerged {
    mode: [c_uint, ..3],
    oid: [git_oid, ..3],
    path: *c_schar
}

pub type git_index_entry_unmerged = Struct_git_index_entry_unmerged;

struct Struct_git_config_file {
    cfg: *Struct_git_config,
    open: *u8,
    get: *u8,
    get_multivar: *u8,
    set: *u8,
    set_multivar: *u8,
    del: *u8,
    foreach: *u8,
    free: *u8
}

pub type git_cvar_t = c_uint;
pub static GIT_CVAR_FALSE: c_uint = 0;
pub static GIT_CVAR_TRUE: c_uint = 1;
pub static GIT_CVAR_INT32: c_uint = 2;
pub static GIT_CVAR_STRING: c_uint = 3;


pub struct git_cvar_map {
    cvar_type: git_cvar_t,
    str_match: *c_schar,
    map_value: c_int
}

struct Struct_git_remote_head {
    local: c_int,
    oid: git_oid,
    loid: git_oid,
    name: *c_schar
}

pub type git_headlist_cb = *u8;

struct Struct_git_indexer_stats {
    total: c_uint,
    processed: c_uint
}

pub type git_indexer_stats = Struct_git_indexer_stats;

type Struct_git_indexer = c_void;

pub type git_indexer = Struct_git_indexer;

type Struct_git_indexer_stream = c_void;

pub type git_indexer_stream = Struct_git_indexer_stream;

type Enum_unnamed7 = c_uint;
pub static GIT_STATUS_CURRENT: c_uint = 0;
pub static GIT_STATUS_INDEX_NEW: c_uint = 1;
pub static GIT_STATUS_INDEX_MODIFIED: c_uint = 2;
pub static GIT_STATUS_INDEX_DELETED: c_uint = 4;
pub static GIT_STATUS_WT_NEW: c_uint = 8;
pub static GIT_STATUS_WT_MODIFIED: c_uint = 16;
pub static GIT_STATUS_WT_DELETED: c_uint = 32;
pub static GIT_STATUS_IGNORED: c_uint = 64;


pub type git_status_show_t = c_uint;
pub static GIT_STATUS_SHOW_INDEX_AND_WORKDIR: c_uint = 0;
pub static GIT_STATUS_SHOW_INDEX_ONLY: c_uint = 1;
pub static GIT_STATUS_SHOW_WORKDIR_ONLY: c_uint = 2;
pub static GIT_STATUS_SHOW_INDEX_THEN_WORKDIR: c_uint = 3;


type Enum_unnamed8 = c_uint;
pub static GIT_STATUS_OPT_INCLUDE_UNTRACKED: c_uint = 1;
pub static GIT_STATUS_OPT_INCLUDE_IGNORED: c_uint = 2;
pub static GIT_STATUS_OPT_INCLUDE_UNMODIFIED: c_uint = 4;
pub static GIT_STATUS_OPT_EXCLUDE_SUBMODULED: c_uint = 8;
pub static GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS: c_uint = 16;


pub struct git_status_options {
    show: git_status_show_t,
    flags: c_uint,
    pathspec: git_strarray
}

pub type git_submodule_update_t = c_uint;
pub static GIT_SUBMODULE_UPDATE_CHECKOUT: c_uint = 0;
pub static GIT_SUBMODULE_UPDATE_REBASE: c_uint = 1;
pub static GIT_SUBMODULE_UPDATE_MERGE: c_uint = 2;


pub type git_submodule_ignore_t = c_uint;
pub static GIT_SUBMODULE_IGNORE_ALL: c_uint = 0;
pub static GIT_SUBMODULE_IGNORE_DIRTY: c_uint = 1;
pub static GIT_SUBMODULE_IGNORE_UNTRACKED: c_uint = 2;
pub static GIT_SUBMODULE_IGNORE_NONE: c_uint = 3;


pub struct git_submodule {
    name: *c_schar,
    path: *c_schar,
    url: *c_schar,
    oid: git_oid,
    update: git_submodule_update_t,
    ignore: git_submodule_ignore_t,
    fetch_recurse: c_int,
    refcount: c_int
}

pub struct git_note_data {
    blob_oid: git_oid,
    annotated_object_oid: git_oid
}

#[link_args="-lgit2"]
pub extern {

pub fn git_strarray_free(++array: *git_strarray);

pub fn git_strarray_copy(++tgt: *git_strarray, ++src: *git_strarray) -> c_int;

pub fn git_libgit2_version(++major: *c_int, ++minor: *c_int, ++rev: *c_int);

pub fn git_threads_init();

pub fn git_threads_shutdown();

pub fn giterr_last() -> *git_error;

pub fn giterr_clear();

pub fn git_oid_fromstr(++out: *git_oid, ++str: *c_schar) -> c_int;

pub fn git_oid_fromstrn(++out: *git_oid, ++str: *c_schar, ++length: size_t) -> c_int;

pub fn git_oid_fromraw(++out: *git_oid, ++raw: *c_uchar);

pub fn git_oid_fmt(++str: *c_schar, ++oid: *git_oid);

pub fn git_oid_pathfmt(++str: *c_schar, ++oid: *git_oid);

pub fn git_oid_allocfmt(++oid: *git_oid) -> *c_schar;

pub fn git_oid_tostr(++out: *c_schar, ++n: size_t, ++oid: *git_oid) -> *c_schar;

pub fn git_oid_cpy(++out: *git_oid, ++src: *git_oid);

pub fn git_oid_cmp(++a: *git_oid, ++b: *git_oid) -> c_int;

pub fn git_oid_ncmp(++a: *git_oid, ++b: *git_oid, ++len: c_uint) -> c_int;

pub fn git_oid_streq(++a: *git_oid, ++str: *c_schar) -> c_int;

pub fn git_oid_iszero(++a: *git_oid) -> c_int;

pub fn git_oid_shorten_new(++min_length: size_t) -> *git_oid_shorten;

pub fn git_oid_shorten_add(++os: *git_oid_shorten, ++text_oid: *c_schar) -> c_int;

pub fn git_oid_shorten_free(++os: *git_oid_shorten);

pub fn git_signature_new(++sig_out: **git_signature, ++name: *c_schar, ++email: *c_schar, ++time: git_time_t, ++offset: c_int) -> c_int;

pub fn git_signature_now(++sig_out: **git_signature, ++name: *c_schar, ++email: *c_schar) -> c_int;

pub fn git_signature_dup(++sig: *git_signature) -> *git_signature;

pub fn git_signature_free(++sig: *git_signature);

pub fn git_odb_backend_pack(++backend_out: **git_odb_backend, ++objects_dir: *c_schar) -> c_int;

pub fn git_odb_backend_loose(++backend_out: **git_odb_backend, ++objects_dir: *c_schar, ++compression_level: c_int, ++do_fsync: c_int) -> c_int;

pub fn git_odb_new(++out: **git_odb) -> c_int;

pub fn git_odb_open(++out: **git_odb, ++objects_dir: *c_schar) -> c_int;

pub fn git_odb_add_backend(++odb: *git_odb, ++backend: *git_odb_backend, ++priority: c_int) -> c_int;

pub fn git_odb_add_alternate(++odb: *git_odb, ++backend: *git_odb_backend, ++priority: c_int) -> c_int;

pub fn git_odb_free(++db: *git_odb);

pub fn git_odb_read(++out: **git_odb_object, ++db: *git_odb, ++id: *git_oid) -> c_int;

pub fn git_odb_read_prefix(++out: **git_odb_object, ++db: *git_odb, ++short_id: *git_oid, ++len: c_uint) -> c_int;

pub fn git_odb_read_header(++len_p: *size_t, ++type_p: *git_otype, ++db: *git_odb, ++id: *git_oid) -> c_int;

pub fn git_odb_exists(++db: *git_odb, ++id: *git_oid) -> c_int;

pub fn git_odb_write(++oid: *git_oid, ++odb: *git_odb, ++data: *c_void, ++len: size_t, ++_type: git_otype) -> c_int;

pub fn git_odb_open_wstream(++stream: **git_odb_stream, ++db: *git_odb, ++size: size_t, ++_type: git_otype) -> c_int;

pub fn git_odb_open_rstream(++stream: **git_odb_stream, ++db: *git_odb, ++oid: *git_oid) -> c_int;

pub fn git_odb_hash(++id: *git_oid, ++data: *c_void, ++len: size_t, ++_type: git_otype) -> c_int;

pub fn git_odb_hashfile(++out: *git_oid, ++path: *c_schar, ++_type: git_otype) -> c_int;

pub fn git_odb_object_free(++object: *git_odb_object);

pub fn git_odb_object_id(++object: *git_odb_object) -> *git_oid;

pub fn git_odb_object_data(++object: *git_odb_object) -> *c_void;

pub fn git_odb_object_size(++object: *git_odb_object) -> size_t;

pub fn git_odb_object_type(++object: *git_odb_object) -> git_otype;

pub fn git_repository_open(++repository: **git_repository, ++path: *c_schar) -> c_int;

pub fn git_repository_discover(++repository_path: *c_schar, ++size: size_t, ++start_path: *c_schar, ++across_fs: c_int, ++ceiling_dirs: *c_schar) -> c_int;

pub fn git_repository_open_ext(++repo: **git_repository, ++start_path: *c_schar, ++flags: uint32_t, ++ceiling_dirs: *c_schar) -> c_int;

pub fn git_repository_free(++repo: *git_repository);

pub fn git_repository_init(++repo_out: **git_repository, ++path: *c_schar, ++is_bare: c_uint) -> c_int;

pub fn git_repository_head(++head_out: **git_reference, ++repo: *git_repository) -> c_int;

pub fn git_repository_head_detached(++repo: *git_repository) -> c_int;

pub fn git_repository_head_orphan(++repo: *git_repository) -> c_int;

pub fn git_repository_is_empty(++repo: *git_repository) -> c_int;

pub fn git_repository_path(++repo: *git_repository) -> *c_schar;

pub fn git_repository_workdir(++repo: *git_repository) -> *c_schar;

pub fn git_repository_set_workdir(++repo: *git_repository, ++workdir: *c_schar) -> c_int;

pub fn git_repository_is_bare(++repo: *git_repository) -> c_int;

pub fn git_repository_config(++out: **git_config, ++repo: *git_repository) -> c_int;

pub fn git_repository_set_config(++repo: *git_repository, ++config: *git_config);

pub fn git_repository_odb(++out: **git_odb, ++repo: *git_repository) -> c_int;

pub fn git_repository_set_odb(++repo: *git_repository, ++odb: *git_odb);

pub fn git_repository_index(++out: **git_index, ++repo: *git_repository) -> c_int;

pub fn git_repository_set_index(++repo: *git_repository, ++index: *git_index);

pub fn git_revwalk_new(++walker: **git_revwalk, ++repo: *git_repository) -> c_int;

pub fn git_revwalk_reset(++walker: *git_revwalk);

pub fn git_revwalk_push(++walk: *git_revwalk, ++oid: *git_oid) -> c_int;

pub fn git_revwalk_push_glob(++walk: *git_revwalk, ++glob: *c_schar) -> c_int;

pub fn git_revwalk_push_head(++walk: *git_revwalk) -> c_int;

pub fn git_revwalk_hide(++walk: *git_revwalk, ++oid: *git_oid) -> c_int;

pub fn git_revwalk_hide_glob(++walk: *git_revwalk, ++glob: *c_schar) -> c_int;

pub fn git_revwalk_hide_head(++walk: *git_revwalk) -> c_int;

pub fn git_revwalk_push_ref(++walk: *git_revwalk, ++refname: *c_schar) -> c_int;

pub fn git_revwalk_hide_ref(++walk: *git_revwalk, ++refname: *c_schar) -> c_int;

pub fn git_revwalk_next(++oid: *git_oid, ++walk: *git_revwalk) -> c_int;

pub fn git_revwalk_sorting(++walk: *git_revwalk, ++sort_mode: c_uint);

pub fn git_revwalk_free(++walk: *git_revwalk);

pub fn git_revwalk_repository(++walk: *git_revwalk) -> *git_repository;

pub fn git_merge_base(++out: *git_oid, ++repo: *git_repository, ++one: *git_oid, ++two: *git_oid) -> c_int;

pub fn git_reference_lookup(++reference_out: **git_reference, ++repo: *git_repository, ++name: *c_schar) -> c_int;

pub fn git_reference_name_to_oid(++out: *git_oid, ++repo: *git_repository, ++name: *c_schar) -> c_int;

pub fn git_reference_create_symbolic(++ref_out: **git_reference, ++repo: *git_repository, ++name: *c_schar, ++target: *c_schar, ++force: c_int) -> c_int;

pub fn git_reference_create_oid(++ref_out: **git_reference, ++repo: *git_repository, ++name: *c_schar, ++id: *git_oid, ++force: c_int) -> c_int;

pub fn git_reference_oid(++_ref: *git_reference) -> *git_oid;

pub fn git_reference_target(++_ref: *git_reference) -> *c_schar;

pub fn git_reference_type(++_ref: *git_reference) -> git_ref_t;

pub fn git_reference_name(++_ref: *git_reference) -> *c_schar;

pub fn git_reference_resolve(++resolved_ref: **git_reference, ++_ref: *git_reference) -> c_int;

pub fn git_reference_owner(++_ref: *git_reference) -> *git_repository;

pub fn git_reference_set_target(++_ref: *git_reference, ++target: *c_schar) -> c_int;

pub fn git_reference_set_oid(++_ref: *git_reference, ++id: *git_oid) -> c_int;

pub fn git_reference_rename(++_ref: *git_reference, ++new_name: *c_schar, ++force: c_int) -> c_int;

pub fn git_reference_delete(++_ref: *git_reference) -> c_int;

pub fn git_reference_packall(++repo: *git_repository) -> c_int;

pub fn git_reference_list(++array: *git_strarray, ++repo: *git_repository, ++list_flags: c_uint) -> c_int;

pub fn git_reference_foreach(++repo: *git_repository, ++list_flags: c_uint, ++callback: *u8, ++payload: *c_void) -> c_int;

pub fn git_reference_is_packed(++_ref: *git_reference) -> c_int;

pub fn git_reference_reload(++_ref: *git_reference) -> c_int;

pub fn git_reference_free(++_ref: *git_reference);

pub fn git_reference_cmp(++ref1: *git_reference, ++ref2: *git_reference) -> c_int;

pub fn git_reflog_read(++reflog: **git_reflog, ++_ref: *git_reference) -> c_int;

pub fn git_reflog_write(++_ref: *git_reference, ++oid_old: *git_oid, ++committer: *git_signature, ++msg: *c_schar) -> c_int;

pub fn git_reflog_rename(++_ref: *git_reference, ++new_name: *c_schar) -> c_int;

pub fn git_reflog_delete(++_ref: *git_reference) -> c_int;

pub fn git_reflog_entrycount(++reflog: *git_reflog) -> c_uint;

pub fn git_reflog_entry_byindex(++reflog: *git_reflog, ++idx: c_uint) -> *git_reflog_entry;

pub fn git_reflog_entry_oidold(++entry: *git_reflog_entry) -> *git_oid;

pub fn git_reflog_entry_oidnew(++entry: *git_reflog_entry) -> *git_oid;

pub fn git_reflog_entry_committer(++entry: *git_reflog_entry) -> *git_signature;

pub fn git_reflog_entry_msg(++entry: *git_reflog_entry) -> *c_schar;

pub fn git_reflog_free(++reflog: *git_reflog);

pub fn git_object_lookup(++object: **git_object, ++repo: *git_repository, ++id: *git_oid, ++_type: git_otype) -> c_int;

pub fn git_object_lookup_prefix(++object_out: **git_object, ++repo: *git_repository, ++id: *git_oid, ++len: c_uint, ++_type: git_otype) -> c_int;

pub fn git_object_id(++obj: *git_object) -> *git_oid;

pub fn git_object_type(++obj: *git_object) -> git_otype;

pub fn git_object_owner(++obj: *git_object) -> *git_repository;

pub fn git_object_free(++object: *git_object);

pub fn git_object_type2string(++_type: git_otype) -> *c_schar;

pub fn git_object_string2type(++str: *c_schar) -> git_otype;

pub fn git_object_typeisloose(++_type: git_otype) -> c_int;

pub fn git_object__size(++_type: git_otype) -> size_t;

pub fn git_blob_rawcontent(++blob: *git_blob) -> *c_void;

pub fn git_blob_rawsize(++blob: *git_blob) -> size_t;

pub fn git_blob_create_fromfile(++oid: *git_oid, ++repo: *git_repository, ++path: *c_schar) -> c_int;

pub fn git_blob_create_fromdisk(++oid: *git_oid, ++repo: *git_repository, ++path: *c_schar) -> c_int;

pub fn git_blob_create_frombuffer(++oid: *git_oid, ++repo: *git_repository, ++buffer: *c_void, ++len: size_t) -> c_int;

pub fn git_commit_id(++commit: *git_commit) -> *git_oid;

pub fn git_commit_message_encoding(++commit: *git_commit) -> *c_schar;

pub fn git_commit_message(++commit: *git_commit) -> *c_schar;

pub fn git_commit_time(++commit: *git_commit) -> git_time_t;

pub fn git_commit_time_offset(++commit: *git_commit) -> c_int;

pub fn git_commit_committer(++commit: *git_commit) -> *git_signature;

pub fn git_commit_author(++commit: *git_commit) -> *git_signature;

pub fn git_commit_tree(++tree_out: **git_tree, ++commit: *git_commit) -> c_int;

pub fn git_commit_tree_oid(++commit: *git_commit) -> *git_oid;

pub fn git_commit_parentcount(++commit: *git_commit) -> c_uint;

pub fn git_commit_parent(++parent: **git_commit, ++commit: *git_commit, ++n: c_uint) -> c_int;

pub fn git_commit_parent_oid(++commit: *git_commit, ++n: c_uint) -> *git_oid;

pub fn git_commit_create(++oid: *git_oid, ++repo: *git_repository, ++update_ref: *c_schar, ++author: *git_signature, ++committer: *git_signature, ++message_encoding: *c_schar, ++message: *c_schar, ++tree: *git_tree, ++parent_count: c_int, ++parents: **git_commit) -> c_int;

pub fn git_commit_create_v(++oid: *git_oid, ++repo: *git_repository, ++update_ref: *c_schar, ++author: *git_signature, ++committer: *git_signature, ++message_encoding: *c_schar, ++message: *c_schar, ++tree: *git_tree, ++parent_count: c_int) -> c_int; /* FIXME: variadic function */

pub fn git_tag_id(++tag: *git_tag) -> *git_oid;

pub fn git_tag_target(++target: **git_object, ++tag: *git_tag) -> c_int;

pub fn git_tag_target_oid(++tag: *git_tag) -> *git_oid;

pub fn git_tag_type(++tag: *git_tag) -> git_otype;

pub fn git_tag_name(++tag: *git_tag) -> *c_schar;

pub fn git_tag_tagger(++tag: *git_tag) -> *git_signature;

pub fn git_tag_message(++tag: *git_tag) -> *c_schar;

pub fn git_tag_create(++oid: *git_oid, ++repo: *git_repository, ++tag_name: *c_schar, ++target: *git_object, ++tagger: *git_signature, ++message: *c_schar, ++force: c_int) -> c_int;

pub fn git_tag_create_frombuffer(++oid: *git_oid, ++repo: *git_repository, ++buffer: *c_schar, ++force: c_int) -> c_int;

pub fn git_tag_create_lightweight(++oid: *git_oid, ++repo: *git_repository, ++tag_name: *c_schar, ++target: *git_object, ++force: c_int) -> c_int;

pub fn git_tag_delete(++repo: *git_repository, ++tag_name: *c_schar) -> c_int;

pub fn git_tag_list(++tag_names: *git_strarray, ++repo: *git_repository) -> c_int;

pub fn git_tag_list_match(++tag_names: *git_strarray, ++pattern: *c_schar, ++repo: *git_repository) -> c_int;

pub fn git_tag_peel(++tag_target: **git_object, ++tag: *git_tag) -> c_int;

pub fn git_tree_id(++tree: *git_tree) -> *git_oid;

pub fn git_tree_entrycount(++tree: *git_tree) -> c_uint;

pub fn git_tree_entry_byname(++tree: *git_tree, ++filename: *c_schar) -> *git_tree_entry;

pub fn git_tree_entry_byindex(++tree: *git_tree, ++idx: c_uint) -> *git_tree_entry;

pub fn git_tree_entry_attributes(++entry: *git_tree_entry) -> c_uint;

pub fn git_tree_entry_name(++entry: *git_tree_entry) -> *c_schar;

pub fn git_tree_entry_id(++entry: *git_tree_entry) -> *git_oid;

pub fn git_tree_entry_type(++entry: *git_tree_entry) -> git_otype;

pub fn git_tree_entry_to_object(++object_out: **git_object, ++repo: *git_repository, ++entry: *git_tree_entry) -> c_int;

pub fn git_tree_create_fromindex(++oid: *git_oid, ++index: *git_index) -> c_int;

pub fn git_treebuilder_create(++builder_p: **git_treebuilder, ++source: *git_tree) -> c_int;

pub fn git_treebuilder_clear(++bld: *git_treebuilder);

pub fn git_treebuilder_free(++bld: *git_treebuilder);

pub fn git_treebuilder_get(++bld: *git_treebuilder, ++filename: *c_schar) -> *git_tree_entry;

pub fn git_treebuilder_insert(++entry_out: **git_tree_entry, ++bld: *git_treebuilder, ++filename: *c_schar, ++id: *git_oid, ++attributes: c_uint) -> c_int;

pub fn git_treebuilder_remove(++bld: *git_treebuilder, ++filename: *c_schar) -> c_int;

pub fn git_treebuilder_filter(++bld: *git_treebuilder, ++filter: *u8, ++payload: *c_void);

pub fn git_treebuilder_write(++oid: *git_oid, ++repo: *git_repository, ++bld: *git_treebuilder) -> c_int;

pub fn git_tree_get_subtree(++subtree: **git_tree, ++root: *git_tree, ++subtree_path: *c_schar) -> c_int;

pub fn git_tree_walk(++tree: *git_tree, ++callback: git_treewalk_cb, ++mode: c_int, ++payload: *c_void) -> c_int;

pub fn git_diff_list_free(++diff: *git_diff_list);

pub fn git_diff_tree_to_tree(++repo: *git_repository, ++opts: *git_diff_options, ++old_tree: *git_tree, ++new_tree: *git_tree, ++diff: **git_diff_list) -> c_int;

pub fn git_diff_index_to_tree(++repo: *git_repository, ++opts: *git_diff_options, ++old_tree: *git_tree, ++diff: **git_diff_list) -> c_int;

pub fn git_diff_workdir_to_index(++repo: *git_repository, ++opts: *git_diff_options, ++diff: **git_diff_list) -> c_int;

pub fn git_diff_workdir_to_tree(++repo: *git_repository, ++opts: *git_diff_options, ++old_tree: *git_tree, ++diff: **git_diff_list) -> c_int;

pub fn git_diff_merge(++onto: *git_diff_list, ++from: *git_diff_list) -> c_int;

pub fn git_diff_foreach(++diff: *git_diff_list, ++cb_data: *c_void, ++file_cb: git_diff_file_fn, ++hunk_cb: git_diff_hunk_fn, ++line_cb: git_diff_data_fn) -> c_int;

pub fn git_diff_print_compact(++diff: *git_diff_list, ++cb_data: *c_void, ++print_cb: git_diff_data_fn) -> c_int;

pub fn git_diff_print_patch(++diff: *git_diff_list, ++cb_data: *c_void, ++print_cb: git_diff_data_fn) -> c_int;

pub fn git_diff_blobs(++old_blob: *git_blob, ++new_blob: *git_blob, ++options: *git_diff_options, ++cb_data: *c_void, ++file_cb: git_diff_file_fn, ++hunk_cb: git_diff_hunk_fn, ++line_cb: git_diff_data_fn) -> c_int;

pub fn git_index_open(++index: **git_index, ++index_path: *c_schar) -> c_int;

pub fn git_index_clear(++index: *git_index);

pub fn git_index_free(++index: *git_index);

pub fn git_index_read(++index: *git_index) -> c_int;

pub fn git_index_write(++index: *git_index) -> c_int;

pub fn git_index_find(++index: *git_index, ++path: *c_schar) -> c_int;

pub fn git_index_uniq(++index: *git_index);

pub fn git_index_add(++index: *git_index, ++path: *c_schar, ++stage: c_int) -> c_int;

pub fn git_index_add2(++index: *git_index, ++source_entry: *git_index_entry) -> c_int;

pub fn git_index_append(++index: *git_index, ++path: *c_schar, ++stage: c_int) -> c_int;

pub fn git_index_append2(++index: *git_index, ++source_entry: *git_index_entry) -> c_int;

pub fn git_index_remove(++index: *git_index, ++position: c_int) -> c_int;

pub fn git_index_get(++index: *git_index, ++n: c_uint) -> *git_index_entry;

pub fn git_index_entrycount(++index: *git_index) -> c_uint;

pub fn git_index_entrycount_unmerged(++index: *git_index) -> c_uint;

pub fn git_index_get_unmerged_bypath(++index: *git_index, ++path: *c_schar) -> *git_index_entry_unmerged;

pub fn git_index_get_unmerged_byindex(++index: *git_index, ++n: c_uint) -> *git_index_entry_unmerged;

pub fn git_index_entry_stage(++entry: *git_index_entry) -> c_int;

pub fn git_index_read_tree(++index: *git_index, ++tree: *git_tree) -> c_int;

pub fn git_config_find_global(++global_config_path: *c_schar, ++length: size_t) -> c_int;

pub fn git_config_find_system(++system_config_path: *c_schar, ++length: size_t) -> c_int;

pub fn git_config_open_global(++out: **git_config) -> c_int;

pub fn git_config_file__ondisk(++out: **Struct_git_config_file, ++path: *c_schar) -> c_int;

pub fn git_config_new(++out: **git_config) -> c_int;

pub fn git_config_add_file(++cfg: *git_config, ++file: *git_config_file, ++priority: c_int) -> c_int;

pub fn git_config_add_file_ondisk(++cfg: *git_config, ++path: *c_schar, ++priority: c_int) -> c_int;

pub fn git_config_open_ondisk(++cfg: **git_config, ++path: *c_schar) -> c_int;

pub fn git_config_free(++cfg: *git_config);

pub fn git_config_get_int32(++out: *int32_t, ++cfg: *git_config, ++name: *c_schar) -> c_int;

pub fn git_config_get_int64(++out: *int64_t, ++cfg: *git_config, ++name: *c_schar) -> c_int;

pub fn git_config_get_bool(++out: *c_int, ++cfg: *git_config, ++name: *c_schar) -> c_int;

pub fn git_config_get_string(++out: **c_schar, ++cfg: *git_config, ++name: *c_schar) -> c_int;

pub fn git_config_get_multivar(++cfg: *git_config, ++name: *c_schar, ++regexp: *c_schar, ++_fn: *u8, ++data: *c_void) -> c_int;

pub fn git_config_set_int32(++cfg: *git_config, ++name: *c_schar, ++value: int32_t) -> c_int;

pub fn git_config_set_int64(++cfg: *git_config, ++name: *c_schar, ++value: int64_t) -> c_int;

pub fn git_config_set_bool(++cfg: *git_config, ++name: *c_schar, ++value: c_int) -> c_int;

pub fn git_config_set_string(++cfg: *git_config, ++name: *c_schar, ++value: *c_schar) -> c_int;

pub fn git_config_set_multivar(++cfg: *git_config, ++name: *c_schar, ++regexp: *c_schar, ++value: *c_schar) -> c_int;

pub fn git_config_delete(++cfg: *git_config, ++name: *c_schar) -> c_int;

pub fn git_config_foreach(++cfg: *git_config, ++callback: *u8, ++payload: *c_void) -> c_int;

pub fn git_config_get_mapped(++out: *c_int, ++cfg: *git_config, ++name: *c_schar, ++maps: *git_cvar_map, ++map_n: size_t) -> c_int;

pub fn git_refspec_src(++refspec: *git_refspec) -> *c_schar;

pub fn git_refspec_dst(++refspec: *git_refspec) -> *c_schar;

pub fn git_refspec_src_matches(++refspec: *git_refspec, ++refname: *c_schar) -> c_int;

pub fn git_refspec_transform(++out: *c_schar, ++outlen: size_t, ++spec: *git_refspec, ++name: *c_schar) -> c_int;

pub fn git_indexer_stream_new(++out: **git_indexer_stream, ++gitdir: *c_schar) -> c_int;

pub fn git_indexer_stream_add(++idx: *git_indexer_stream, ++data: *c_void, ++size: size_t, ++stats: *git_indexer_stats) -> c_int;

pub fn git_indexer_stream_finalize(++idx: *git_indexer_stream, ++stats: *git_indexer_stats) -> c_int;

pub fn git_indexer_stream_hash(++idx: *git_indexer_stream) -> *git_oid;

pub fn git_indexer_stream_free(++idx: *git_indexer_stream);

pub fn git_indexer_new(++out: **git_indexer, ++packname: *c_schar) -> c_int;

pub fn git_indexer_run(++idx: *git_indexer, ++stats: *git_indexer_stats) -> c_int;

pub fn git_indexer_write(++idx: *git_indexer) -> c_int;

pub fn git_indexer_hash(++idx: *git_indexer) -> *git_oid;

pub fn git_indexer_free(++idx: *git_indexer);

pub fn git_remote_new(++out: **git_remote, ++repo: *git_repository, ++name: *c_schar, ++url: *c_schar, ++fetch: *c_schar) -> c_int;

pub fn git_remote_load(++out: **git_remote, ++repo: *git_repository, ++name: *c_schar) -> c_int;

pub fn git_remote_save(++remote: *git_remote) -> c_int;

pub fn git_remote_name(++remote: *git_remote) -> *c_schar;

pub fn git_remote_url(++remote: *git_remote) -> *c_schar;

pub fn git_remote_set_fetchspec(++remote: *git_remote, ++spec: *c_schar) -> c_int;

pub fn git_remote_fetchspec(++remote: *git_remote) -> *git_refspec;

pub fn git_remote_set_pushspec(++remote: *git_remote, ++spec: *c_schar) -> c_int;

pub fn git_remote_pushspec(++remote: *git_remote) -> *git_refspec;

pub fn git_remote_connect(++remote: *git_remote, ++direction: c_int) -> c_int;

pub fn git_remote_ls(++remote: *git_remote, ++list_cb: git_headlist_cb, ++payload: *c_void) -> c_int;

pub fn git_remote_download(++remote: *git_remote, ++bytes: *git_off_t, ++stats: *git_indexer_stats) -> c_int;

pub fn git_remote_connected(++remote: *git_remote) -> c_int;

pub fn git_remote_disconnect(++remote: *git_remote);

pub fn git_remote_free(++remote: *git_remote);

pub fn git_remote_update_tips(++remote: *git_remote, ++cb: *u8) -> c_int;

pub fn git_remote_valid_url(++url: *c_schar) -> c_int;

pub fn git_remote_supported_url(++url: *c_schar) -> c_int;

pub fn git_remote_list(++remotes_list: *git_strarray, ++repo: *git_repository) -> c_int;

pub fn git_remote_add(++out: **git_remote, ++repo: *git_repository, ++name: *c_schar, ++url: *c_schar) -> c_int;

pub fn git_status_foreach(++repo: *git_repository, ++callback: *u8, ++payload: *c_void) -> c_int;

pub fn git_status_foreach_ext(++repo: *git_repository, ++opts: *git_status_options, ++callback: *u8, ++payload: *c_void) -> c_int;

pub fn git_status_file(++status_flags: *c_uint, ++repo: *git_repository, ++path: *c_schar) -> c_int;

pub fn git_status_should_ignore(++ignored: *c_int, ++repo: *git_repository, ++path: *c_schar) -> c_int;

pub fn git_submodule_foreach(++repo: *git_repository, ++callback: *u8, ++payload: *c_void) -> c_int;

pub fn git_submodule_lookup(++submodule: **git_submodule, ++repo: *git_repository, ++name: *c_schar) -> c_int;

pub fn git_note_read(++note: **git_note, ++repo: *git_repository, ++notes_ref: *c_schar, ++oid: *git_oid) -> c_int;

pub fn git_note_message(++note: *git_note) -> *c_schar;

pub fn git_note_oid(++note: *git_note) -> *git_oid;

pub fn git_note_create(++out: *git_oid, ++repo: *git_repository, ++author: *git_signature, ++committer: *git_signature, ++notes_ref: *c_schar, ++oid: *git_oid, ++note: *c_schar) -> c_int;

pub fn git_note_remove(++repo: *git_repository, ++notes_ref: *c_schar, ++author: *git_signature, ++committer: *git_signature, ++oid: *git_oid) -> c_int;

pub fn git_note_free(++note: *git_note);

pub fn git_note_default_ref(++out: **c_schar, ++repo: *git_repository) -> c_int;

pub fn git_note_foreach(++repo: *git_repository, ++notes_ref: *c_schar, ++note_cb: *u8, ++payload: *c_void) -> c_int;

}


// Ports of inline functions, which bindgen can't handle automatically
pub fn git_commit_lookup(commit: **git_commit, repo: *git_repository, oid: *git_oid) -> c_int {
    unsafe {
        return git_object_lookup(commit as **git_object, repo, oid, GIT_OBJ_COMMIT);
    }
}

pub fn git_commit_lookup_prefix(commit: **git_commit, repo: *git_repository, oid: *git_oid, len: c_uint) -> c_int {
    unsafe {
        return git_object_lookup_prefix(commit as **git_object, repo, oid, len, GIT_OBJ_COMMIT);
    }
}

pub fn git_commit_free(commit: *git_commit) {
    unsafe {
        git_object_free(commit as *git_object);
    }
}

pub fn git_tree_lookup(tree: **git_tree, repo: *git_repository, oid: *git_oid) -> c_int {
    unsafe {
        return git_object_lookup(tree as **git_object, repo, oid, GIT_OBJ_TREE);
    }
}

pub fn git_tree_lookup_prefix(tree: **git_tree, repo: *git_repository, oid: *git_oid, len: c_uint) -> c_int {
    unsafe {
        return git_object_lookup_prefix(tree as **git_object, repo, oid, len, GIT_OBJ_TREE);
    }
}

pub fn git_tree_free(tree: *git_tree) {
    unsafe {
        git_object_free(tree as *git_object);
    }
}
