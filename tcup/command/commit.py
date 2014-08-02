import argparse
from pathlib import Path

from tcup.repo import Repository


parser = argparse.ArgumentParser()
parser.add_argument('-m', '--message')


def run(ns, argv):
    repo = Repository.open(Path())
    ns = parser.parse_args(argv)

    # TODO atomic ref update
    repo._repo.create_commit(
        'refs/heads/master',
        repo._repo.default_signature,
        repo._repo.default_signature,
        ns.message,
        repo._repo.index.write_tree(),
        # TODO this should be empty if there's no HEAD
        [repo._repo.head.target],
    )
