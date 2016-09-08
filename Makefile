###########################################################################
# CONFIGURATION

# Directory where the `index.html` will be loaded. Any reference to
# `http://home.url` is changed to this string.
HOME_URL ?= http://rust-tutorials.com/exercises

# Path to give to `scp` for uploading the tutorials. This is an alias
# that I define locally, so I am pretty confident you will want to
# change it.
UPLOAD_PATH ?= scf:rust-tutorials.com/exercises

###########################################################################
# Make rules themselves.

.PHONY: test html upload

all: clean html test

upload: html
	scp html/*html $(UPLOAD_PATH)

clean:
	rm -f html/*.html
	rm -f src/*.exe

html: html/index.html html/hint-mutable-borrow-1.html html/hint-struct-1.html

html/%.html: md/%.md sync-index.py $(wildcard src/*rs)
	markdown $< | python sync-index.py "$(HOME_URL)" > $@

test:
	rustc src/ownership.rs -o src/ownership.exe && src/ownership.exe
	rustc --test src/structs.rs -o src/structs.exe && src/structs.exe
	rustc --test src/enums.rs -o src/enums.exe && src/enums.exe
	rustc --test src/options.rs -o src/options.exe && src/options.exe
	rustc src/threads.rs -o src/threads.exe && src/threads.exe
	rustc --test src/named_lifetime_parameters.rs -o src/named_lifetime_parameters.exe && src/named_lifetime_parameters.exe
	rustc --test src/lifetimes_as_part_of_type.rs -o src/lifetimes_as_part_of_type.exe && src/lifetimes_as_part_of_type.exe
	rustc --test src/successful_borrowing.rs -o src/successful_borrowing.exe && src/successful_borrowing.exe
	rustc --test src/entry.rs -o src/entry.exe && src/entry.exe
	rustc --test src/sharing_and_mutability.rs -o src/sharing_and_mutability.exe && src/sharing_and_mutability.exe


