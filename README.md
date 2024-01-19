# 1. Simple Client
Build a simple client that can run the two following commands:
./simple --mode=cache --times=10
./simple --mode=read
The cache mode should listen to a websocket for given number of times(seconds) only for the USD
prices of BTC. Example is given here
https://binance-docs.github.io/apidocs/websocket_api/en/#symbol-price-ticker, any other websocket is
also fine like kucoin, gemini, gateio, bybit etc. Calculate the average of these prices, say XXX. Then
print "Cache complete. The average USD price of BTC is: XXX"
Save both the result of the aggregate and the data points used to create the aggregate to a file.
The read mode should just read from the file and print the values to the terminal.

------------------------------------------------------------------------------------------------------------------------------






To run the Rust program with the provided code, you can use the Cargo build and run commands. Assuming you are in the project directory, open a terminal and run the following commands:

## Build the project:
```
cargo build
```
Run the project with the desired mode and optional parameters:

## For the "cache" mode with the default times value (10):
```
cargo run -- --mode cache
```
## For the "cache" mode with a specific times value (replace <times> with the desired value):

```
cargo run -- --mode cache --times=<times>
```
## For the "read" mode:
```
cargo run -- --mode read
```
Replace <times> with the actual number of seconds you want the WebSocket connection to run in the "cache" mode.

Make sure you have Rust and Cargo installed on your system. If not, you can install them by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install

Additionally, ensure that you have an active internet connection to connect to the Binance WebSocket API in the "cache" mode.
