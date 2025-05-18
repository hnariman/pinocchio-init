build:
	cargo build-sbf

test:
	cargo test

get-program-key:
	solana address -k target/deploy/__PROGRAM_NAME_NORMALIZED__-keypair.json

deploy:
	solana program deploy ./target/deploy/__PROGRAM_NAME_NORMALIZED__.so --program-id ./target/deploy/__PROGRAM_NAME_NORMALIZED__-keypair.json