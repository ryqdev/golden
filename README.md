# golden

`golden` is all in one trading engine built with Rust.

There are many public algorithmic trading platforms like [QuantConnect](https://www.quantconnect.com/) and [JoinQuant](https://www.joinquant.com/) for individual traders to learn, research, backtest and live-trading.
 However, we still need private algorithmic trading platform for many reasons such as strategy privacy, system stability and other customed features.

One of the well-known private trading libraries is [Backtrader](https://github.com/mementum/backtrader) with Python, but the community is not active recently, since It hasn't been updated since Apr 19, 2023. I also built a [simple project](https://github.com/ryqdev/silver) based on [backtrader](https://github.com/mementum/backtrader) and [ib_insync](https://github.com/ultra1971/backtrader_ib_insync), but it still need a lot of work to do to meet real world trading requirements.

Inspired by `backtrader`, `golden` aims to build all in one trading engine supporting backtesting, analyzing, paper-trading and live-trading.



## Why to use Rust

- As my first Rust practice project: Wonderful chance for me to learn Rust and write algorithmic trading system from scratch.
- Modern programming language: Rust is the modern programming language with a powerful package manager (`cargo`) and toolchain.
- High performance:  Well-written Rust programs can perform as well as C/C++. [1]



## Quick glance
### On windows:
Download csv data from yahoo finance:
![image](https://github.com/ryqdev/golden/assets/50010920/cd49ac08-6529-473a-90fb-c645b8154498)
Backtest with simple strategy:
![image](https://github.com/ryqdev/golden/assets/50010920/e03e639e-f4e2-41fb-b25e-09f3b5156cfd)

## Quick Install

```shell
cargo install golden
```

## Useful Commands

```shell
# download csv to data/
golden csv --symbol SPY

# backtest
golden backtest --symbol SPY

# paper trading
golden paper --broker ibkr
```



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
- [ ] Remote deployment
- [ ] Multi-strategy system design



## Reference

[1] https://rustmagazine.github.io/rust_magazine_2021/chapter_12/rust-perf.html
