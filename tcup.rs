extern mod std;
extern mod git2;

use git2::repository::Repository_discover;
use git2::repository::Repository_init;
use git2::signature::Signature_create;

fn main() {
    let args = os::args();
    match args[1] {
        ~"dummy" => {
            let repo = Repository_discover(".");
            io::println(repo.path());
        }
        ~"new" => {
            // TODO check for existing repo
            let repo = Repository_init(".");
            io::println("Created a new repository.");
        }
        ~"stage" => {
            if vec::len(args) == 2 {
                io::println("Need some filenames, chief.");
                return;
            }

            let repo = Repository_discover(".");
            let index = repo.index();

            for vec::each(vec::view(args, 2, args.len())) |filename| {
                index.add(*filename);
            }
            index.write();
        }
        ~"status" => {
            let repo = Repository_discover(".");
            // TODO this kinda sucks and is not amenable to `git status` style output
            do repo.for_status() |path, status| {
                io::println(#fmt("%s    (%d)", path, status as int));
            };
        }
        ~"commit" => {
            let repo = Repository_discover(".");
            let tree_oid = repo.index().write();
            let sig = Signature_create("Eevee", "git@veekun.com", std::time::now());
            let commit = repo.commit(tree_oid, "Commit from Rust!", sig);
            io::println("commit!");
        }
        _ => {
            io::println("error!");
        }
    }
}
