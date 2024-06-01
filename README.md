# golden
All in one trading engine

## Features
- [ ] Configuation files

### Backtest
- [ ] Learn and develop strategies
- [ ] Backtest configurations
- [x] Single data feed
- [x] Single strategy
- [ ] Multiple data feed
- [ ] Multiple strategy
- [ ] Customed backtest time range
- [x] Backtest UI
- [ ] Indicators

### Paper trading & live trading
- [x] Single live broker: IBKR
- [x] Live trading with single broker
- [ ] AsyncIO
- [ ] Websocket: Constant polling is not recommended to get real-time market data. Use websocket instead

## Demo
```shell
 make backtest symbol=SPY
```
