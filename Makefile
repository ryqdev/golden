backtest:
	cargo run backtest --symbol ${symbol}

paper:
	cargo run paper --broker ${broker}

live:
	cargo run live