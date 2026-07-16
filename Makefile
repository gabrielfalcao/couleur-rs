run:cls
	cargo run -- --bg FFCC00 test 123

test:cls

	cargo test


cls:
	1>&2 echo -en "\x1b[2J\x1b[3J\x1b[H"
.PHONY: test
