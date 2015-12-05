.PHONY: test

all: test index.html

index.html: index-template.md sync-index.py $(wildcard src/*rs)
	markdown index-template.md | python sync-index.py > index.html

test:
	rustc --test src/structs.rs -o src/structs && src/structs