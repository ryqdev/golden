backtest:
	cargo run backtest --symbol ${symbol}

paper:
	cargo run paper-trading

live:
	cargo run live-trading