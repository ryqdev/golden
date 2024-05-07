
backtest:
	@RUST_LOG=info cargo run backtest --project trade_spy

data:
	@RUST_LOG=info cargo run data -d SPY