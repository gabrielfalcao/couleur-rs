MAKEFILE_PATH		:= $(realpath $(firstword $(MAKEFILE_LIST)))
PROJECT_ROOT		:= $(shell dirname $(MAKEFILE_PATH))
SRC_ROOT		:= $(PROJECT_ROOT)/src
TEST_ROOT		:= $(PROJECT_ROOT)/tests


all: cls run-couleur #test
# all: cls run-couleur test


hex-to-bin: cls
	cargo run --bin hex-to-bin ./palettes/cs108.hex

contrast: cls
	cargo run --bin contrast web "#E83B3B" "Hello World"

run: cls run-couleur

run-couleur: cls
	cargo run --bin couleur -- "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World"

run-contrast: cls
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
	cargo test --all-features
	cargo test

nextest: cls
	cargo nextest run

format:
	find $(SRC_ROOT) $(TEST_ROOT) -type f -name '*.rs' -exec rustfmt {} \;

clean: format
	cargo clean
cls:
	@1>&2 printf "\x1b[2J\x1b[3J\x1b[H"
	@rm -f couleur.log

.PHONY: run test format cls all clean nextest hex-to-bin run-contrast run-couleur
