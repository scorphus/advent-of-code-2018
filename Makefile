setup:
	@cargo install grcov
	@echo "Install lcov using your package manager of choice"

coverage: export CARGO_INCREMENTAL := 0
coverage: export RUSTFLAGS := -Zprofile -Ccodegen-units=1 -Copt-level=0 \
	-Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
coverage: export RUSTDOCFLAGS := -Cpanic=abort
coverage:
	@rm -rf target
	@cargo build
	@cargo test
	@grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing \
		-o ./target/debug/grcov/
	@grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing \
		-o ./target/debug/lcov.info
	@lcov -e ./target/debug/lcov.info 'src/*' > ./target/debug/lcov.src.info
	@genhtml -o ./target/debug/lcov/ --show-details --highlight \
		--ignore-errors source --legend ./target/debug/lcov.src.info > /dev/null
	@echo "grcov: file://$$(pwd)/target/debug/grcov/src/index.html"
	@echo "lcov: file://$$(pwd)/target/debug/lcov/src/index.html"
