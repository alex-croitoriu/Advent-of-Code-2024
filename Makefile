ifeq ($(OS),Windows_NT)
    DAY := $(shell powershell -Command "(Get-Date).ToString('dd')")
else
    DAY := $(shell date +%d)
endif

all:
	@echo DAY $(DAY)
	cd day$(DAY)
	cargo run --release --manifest-path=day$(DAY)/Cargo.toml