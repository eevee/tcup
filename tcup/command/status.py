from pathlib import Path

from tcup.repo import Repository


def run(ns, argv):
    # TODO handle NoSuchRepository, probably in the arg parser somehow?
    repo = Repository.open(Path())

    # TODO this doesn't work in bare repositories...  but maybe it can do
    # something useful nonetheless?
    # TODO wrap this in something a bit nicer probably -- for example, this
    # lists all files, rather than collapsing unadded directories into a single
    # item
    import pygit2
    possible_untracked_dirs = set()
    impossible_untracked_dirs = set()
    untracked_files = []
    untracked = []
    staged = []
    modified = []

    for fn, flag in sorted(repo._repo.status().items()):
        if flag & pygit2.GIT_STATUS_IGNORED:
            continue

        path = Path(fn)
        if flag & (
                pygit2.GIT_STATUS_CURRENT
                | pygit2.GIT_STATUS_INDEX_NEW
                | pygit2.GIT_STATUS_INDEX_MODIFIED
                | pygit2.GIT_STATUS_INDEX_DELETED):
            # This file is tracked, so none of the parent directories can
            # possibly be untracked
            impossible_untracked_dirs.update(path.parents)

        if flag & (
                pygit2.GIT_STATUS_INDEX_NEW
                | pygit2.GIT_STATUS_INDEX_MODIFIED
                | pygit2.GIT_STATUS_INDEX_DELETED):
            staged.append(path)

        if flag & (
                pygit2.GIT_STATUS_WT_MODIFIED
                | pygit2.GIT_STATUS_WT_DELETED
        ):
            modified.append(path)

        if flag & pygit2.GIT_STATUS_WT_NEW:
            untracked_files.append(path)

        if flag & (pygit2.GIT_STATUS_INDEX_NEW | pygit2.GIT_STATUS_WT_NEW):
            possible_untracked_dirs.update(path.parents)

    possible_untracked_dirs.discard(Path())
    untracked_dirs = possible_untracked_dirs - impossible_untracked_dirs
    untracked.extend(sorted(untracked_dirs))

    for path in sorted(untracked_files):
        if frozenset(path.parents) & untracked_dirs:
            continue
        else:
            untracked.append(path)

    untracked.sort()

    # TODO branch
    # TODO tracking, ahead/behind
    # TODO age of last commit, age of upstream?

    if staged:
        print("Staged:")
        for path in staged:
            print("   ", path)
        print()

    if modified:
        print("Modified:")
        for path in modified:
            print("   ", path)
        print()

    if untracked:
        print("Untracked:")
        # TODO trailing / for directories
        for path in untracked:
            print("   ", path)
        print()

    # TODO "clean!"
