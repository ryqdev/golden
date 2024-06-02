backtest:
	cargo run backtest --symbol ${symbol}

paper:
	cargo run paper --broker ${broker}

# Unfinished
live:
	cargo run live

csv:
	cargo run csv --symbol ${symbol}