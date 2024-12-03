all: simul

TARGET=$(shell rustc -vV | grep ^host | cut -d' ' -f 2)

simul:
	RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" \
		cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort,optimize_for_size \
		--target $(TARGET) --release
	cp target/$(TARGET)/release/jordanbray-chess simul
	strip simul

clean:
	cargo clean
	rm -f simul simul.gz

run: simul
	gzip -c simul > simul.gz
	ls -l simul simul.gz
	./simul 10000
