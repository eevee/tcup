from pathlib import Path

import pygit2


class NoSuchRepository(Exception):
    """No repository was found here or in any parent directory."""


def _discover_repository(where, *, follow_links=False):
    try:
        repo_path = pygit2.discover_repository(str(where), follow_links)
    except KeyError:
        # KeyError is GIT_ENOTFOUND, which in this case means this doesn't
        # appear to be a repository
        raise NoSuchRepository
    else:
        return Path(repo_path)


class Repository:
    def __init__(self, *, _pygit2_repo=None):
        if _pygit2_repo is None:
            raise TypeError("Repository can't be instantiated directly")

        self._repo = _pygit2_repo

    @classmethod
    def initialize(cls, where, **kwargs):
        try:
            repo_path = _discover_repository(where, **kwargs)
        except NoSuchRepository:
            # This is fine, since we're going to init it anyway
            repo_path = where

        repo = pygit2.init_repository(str(repo_path))
        return cls(_pygit2_repo=repo)

    @classmethod
    def open(cls, where, **kwargs):
        repo_path = _discover_repository(where, **kwargs)
        repo = pygit2.init_repository(str(repo_path))
        return cls(_pygit2_repo=repo)

    @property
    def path(self):
        return Path(self._repo.path)
