
debug:
	RUST_BACKTRACE=1 cargo run backtest --symbol ${symbol}

backtest:
	cargo run backtest --symbol ${symbol}

paper:
	cargo run paper-trading

live:
	cargo run live-trading