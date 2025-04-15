solana-local-run:
	cd ~/
	solana-test-validator

solana-set-dev-net:
	solana config set --url devnet

solana-set-local-net:
	solana config set --url localhost