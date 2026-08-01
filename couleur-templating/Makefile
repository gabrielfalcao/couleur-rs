MAKEFILE_PATH		:= $(realpath $(firstword $(MAKEFILE_LIST)))
PROJECT_ROOT		:= $(shell dirname $(MAKEFILE_PATH))
SRC_ROOT		:= $(PROJECT_ROOT)/src
TEST_ROOT		:= $(PROJECT_ROOT)/tests


# all: run test
all: cls test

test: cls
	cargo test -q

nextest: cls
	cargo nextest run

format:
	find $(SRC_ROOT) $(TEST_ROOT) -type f -name '*.rs' -exec rustfmt {} \;

clean: format
	cargo clean
cls:
	1>&2 printf "\x1b[2J\x1b[3J\x1b[H"

.PHONY: run test format cls all clean nextest hex-to-bin
