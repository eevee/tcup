all: libgit2 tcup

libgit2:
	rustc git2/git2.rc

tcup: libgit2
	rustc -o tcup -L git2 tcup.rs


clean:
	rm -f git2/libgit2-*.so
	rm -f tcup
