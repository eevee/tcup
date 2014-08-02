from pathlib import Path

from tcup.repo import Repository


def run(ns):
    # TODO what if it's already a repo
    repo = Repository.initialize(Path())

    print("Created a new Git repository in {}".format(repo.path))
