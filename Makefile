.DEFAULT_GOAL := help
.PHONY: help
help:  ## Display this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: backtest 
backtest: ## e.g.: make backtest
	@cargo run backtest

.PHONY: paper 
paper: ## Paper trading with brokers, e.g.: make paper --broker ibkr
	@cargo run paper --broker ${broker}

.PHONY: live 
live: ## Unfinished
	@cargo run live

.PHONY: csv 
csv: ## Download csv data, e.g.: make csv --symbol SPY
	@cargo run csv

.PHONY: clean 
clean: ## clean
	@cargo clean