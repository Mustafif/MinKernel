TARG = --target x86_64-os.json

build:
	cargo build $(TARG)
run:
	cargo run $(TARG)
test:
	cargo test $(TARG)
test_alloc:
	cargo test $(TARG) --test heap_allocation
