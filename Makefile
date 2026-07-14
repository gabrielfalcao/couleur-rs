test:
	1>&2 echo -en "\x1b[2J\x1b[3J\x1b[H"
	cargo test

.PHONY: test
