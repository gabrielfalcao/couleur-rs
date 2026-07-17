all: test

run: cls
	@printf "testing background\n\n"
	cargo run -q -- --bg FFCC00  --fg CCCCCC --detect test background
	cargo run -q -- --contrast web  --fg CCCCCC --detect test background
	cargo run -q -- --contrast web --bg FFCC00 --detect test background
	cargo run -q -- --contrast web --fg CCCCCC --detect test background

	@printf "\ntesting foreground\n\n"
	cargo run -q -- --fg FFCC00  --bg 333333 --detect test foreground
	cargo run -q -- --contrast web  --bg 333333 --detect test foreground
	cargo run -q -- --contrast web --fg FFCC00  --bg 333333 --detect test foreground
	cargo run -q -- --contrast web --bg 333333 --detect test foreground

test: cls
	cargo test -q

format:
	find src/ -type f -name '*.rs' -exec rustfmt {} \;

clean: format
	cargo clean
cls:
	1>&2 printf "\x1b[2J\x1b[3J\x1b[H"

.PHONY: run test format cls all clean
