run: cls
	cargo run -q -- --bg FFCC00  --fg CCCCCC test background
	cargo run -q -- --contrast web  --fg CCCCCC test background
	cargo run -q -- --contrast web --bg FFCC00 test background
	cargo run -q -- --contrast web --fg CCCCCC test background

	cargo run -q -- --fg FFCC00  --bg 333333 test foreground
	cargo run -q -- --contrast web  --bg 333333 test foreground
	cargo run -q -- --contrast web --fg FFCC00  --bg 333333 test foreground
	cargo run -q -- --contrast web --bg 333333 test foreground

test: cls

	cargo test

all: run test


format:
	find src/ -type f -name '*.rs' -exec rustfmt {} \;

clean: format
	cargo clean
cls:
	1>&2 echo -en "\x1b[2J\x1b[3J\x1b[H"

.PHONY: run test format cls all clean
