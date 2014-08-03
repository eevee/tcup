from pathlib import Path

import pygit2

from tcup.repo import Repository


def run(ns, argv):
    # TODO handle NoSuchRepository, probably in the arg parser somehow?
    repo = Repository.open(Path())

    for commit in repo._repo.walk(repo._repo.head.target, pygit2.GIT_SORT_TOPOLOGICAL):
        print(commit.id, commit.commit_time, commit.message)
