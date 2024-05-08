
backtest:
	@RUST_LOG=info cargo run backtest --project buy_and_hold

data:
	@RUST_LOG=info cargo run data -d SPY