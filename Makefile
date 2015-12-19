.PHONY: test

all: clean test html/index.html html/hint-borrowing-1.html

clean:
    rm -f html/*.html
    rm src/*.exe

html/%.html: md/%.md sync-index.py $(wildcard src/*rs)
	markdown $< | python sync-index.py > $@

test:
	rustc src/ownership.rs -o src/ownership.exe && src/ownership.exe
	rustc --test src/structs.rs -o src/structs.exe && src/structs.exe
	rustc --test src/enums.rs -o src/enums.exe && src/enums.exe
	rustc --test src/options.rs -o src/options.exe && src/options.exe
	rustc src/threads.rs -o src/threads.exe && src/threads.exe
