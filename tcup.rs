use std;
use git2;

import git2::repository::Repository_discover;
import git2::repository::Repository_init;
import git2::signature::Signature_create;

fn main(argv: ~[str]) {
    alt argv[1] {
        "dummy" {
            let repo = Repository_discover(".");
            io::println(repo.path());
        }
        "new" {
            // TODO check for existing repo
            let repo = Repository_init(".");
            io::println("Created a new repository.");
        }
        "stage" {
            if vec::len(argv) == 2 {
                io::println("Need some filenames, chief.");
                ret;
            }

            let repo = Repository_discover(".");
            let index = repo.index();

            do vec::iter_between(argv, 2, argv.len()) |filename| {
                index.add(filename);
            }
            index.write();
        }
        "status" {
            let repo = Repository_discover(".");
            // TODO this kinda sucks and is not amenable to `git status` style output
            do repo.for_status() |path, status| {
                io::println(#fmt("%s    (%d)", path, status as int));
            };
        }
        "commit" {
            let repo = Repository_discover(".");
            let tree_oid = repo.index().write();
            let sig = Signature_create("Eevee", "git@veekun.com", std::time::now());
            let commit = repo.commit(tree_oid, "Commit from Rust!", sig);
            io::println("commit!");
        }
        _ {
            io::println("error!");
        }
    }
}
