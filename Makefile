.PHONY: test

all: test index.html

index.html: index-template.md sync-index.py $(wildcard src/*rs)
	markdown index-template.md | python sync-index.py > index.html

test:
	cd src && rustc --test basics.rs && ./basics
