LOG:=debug

test:
	RUST_LOG=${LOG} cargo test --all -- --nocapture --color=always

test_gizp:
	RUST_LOG=${LOG} cargo test --features=gzip.sqs -- --nocapture --color=always
