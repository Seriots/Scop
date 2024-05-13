
FILE = $(shell basename $(CURDIR))

all: build copy

build: 
	@cargo build --release

copy: 
	@cp target/release/$(FILE) .

clean:
	@rm -f $(FILE)
	@cargo clean

test:
	@cargo test

.PHONY: copy build test clean