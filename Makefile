###########################################################################
# CONFIGURATION

# Directory where the `index.html` will be loaded. Any reference to
# `http://home.url` is changed to this string.
HOME_URL ?= http://smallcultfollowing.com/rust-tutorials

# Path to give to `scp` for uploading the tutorials. This is an alias
# that I define locally, so I am pretty confident you will want to
# change it.
UPLOAD_PATH ?= scf:web/rust-tutorials

###########################################################################
# Make rules themselves.

.PHONY: test html upload

all: clean html test

upload: html
	scp html/*html $(UPLOAD_PATH)

clean:
	rm -f html/*.html
	rm -f src/*.exe

html: html/index.html html/hint-borrowing-1.html

html/%.html: md/%.md sync-index.py $(wildcard src/*rs)
	markdown $< | python sync-index.py "$(HOME_URL)" > $@

test:
	rustc src/ownership.rs -o src/ownership.exe && src/ownership.exe
	rustc --test src/structs.rs -o src/structs.exe && src/structs.exe
	rustc --test src/enums.rs -o src/enums.exe && src/enums.exe
	rustc --test src/options.rs -o src/options.exe && src/options.exe
	rustc src/threads.rs -o src/threads.exe && src/threads.exe
