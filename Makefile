.DEFAULT_GOAL := help
.PHONY: help
help:  ## Display this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


backtest: ## Run backtest
	cargo run backtest

paper: ## Run paper trading
	cargo run paper --broker ${broker}

# Unfinished
live: ## Run live trading
	cargo run live

csv: ## Download CSV data
	cargo run csv
