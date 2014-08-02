import argparse
from pathlib import Path

from tcup.repo import Repository


parser = argparse.ArgumentParser()
parser.add_argument('paths', nargs='+')


def run(ns, argv):
    repo = Repository.open(Path())
    ns = parser.parse_args(argv)

    # TODO paths
    # TODO might be directories
    index = repo._repo.index
    for path in ns.paths:
        index.add(path)
    index.write()
