# golden
`golden` is all in one trading engine built with Rust.

There are many public algorithmic trading platform like [QuantConnect](https://www.quantconnect.com/) and [JoinQuant](https://www.joinquant.com/) for individual trader for learning, researching, backtesting and live-trading.
However, we still private algorithmic trading platform for many reasons such as strategy privacy, system stability and other customed features.

One of the well-known private trading library is [Backtrader](https://github.com/mementum/backtrader), but the commmunity is not active recently. It hasn't updated since Apr 19, 2023.
I also build a [simple project](https://github.com/ryqdev/silver) based on [backtrader](https://github.com/mementum/backtrader) and [ib_insync](https://github.com/ultra1971/backtrader_ib_insync), but it still need a lot of work to do to achieve above goals.

Inspired by `backtrader`, `golden` aims to build all in one trading engine supporting backtesting, analyzing, paper-trading and live-trading.


## Why Rust
- As my first Rust practice project
- Modern programming language
- High performance 
- Powerful compiler
- The trending


## Project feature list
- [x] Command Line Argument Parser
- [ ] Set configuration with a local toml file
- [ ] Set configuration with UI
- [x] Publish it to crate.io
- [ ] Customed Strategy writing


### Backtest features list
- [x] Single data feed
- [x] Single strategy 
- [x] Download CSV data from yahoo finance
- [ ] Set time range while downloading data from Yahoo finance
- [x] Load CSV data from local file
- [ ] Multiple data feed
- [ ] Multiple strategy
- [ ] Set time range in backtest
- [ ] Basic Indicators (such as SMA)
- [ ] More analyzer


### Visualization feature list
- [x] Create the basic layer
- [x] Implement candlestick chart
- [x] Implement line chart
- [ ] Add trading orders list
- [ ] Add side panel controller
- [ ] Add top menu bar
- [ ] Set window size and position
- [ ] Add benchmark
- [ ] Add more details


### Paper trading & live trading feature list
- [x] Single live broker: IBKR
- [x] Live trading with single broker
- [ ] Risk module
- [ ] Online monitor module


## Demo
```shell
make backtest symbol=SPY
```

TODO: add more

## Quick Install
```shell
cargo install golden
```

## Commands
```shell
# download csv to data/
golden csv --symbol SPY

# backtest
golden backtest --symbol SPY

# paper trading
golden paper --broker ibkr
```