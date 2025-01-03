ifeq ($(OS),Windows_NT)
    DAY := $(shell powershell -Command "(Get-Date).ToString('dd')")
else
    DAY := $(shell date +%d)
endif

all:
	@echo Building and running solution for day $(DAY)
	@cargo run --release --manifest-path=days/day$(DAY)/Cargo.toml