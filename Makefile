.PHONY: backtest paper live csv clean

backtest:
	@cargo run backtest

paper:
	@cargo run paper --broker ${broker}

# TODO: Unfinished
live:
	@cargo run live

csv:
	@cargo run csv

clean:
	@cargo clean