# binance-api

This project focuses on encapsulating Binance API so that making it easier to use in your Rust project.

This is the learning project while I study Rust language, so the features, code style, etc will
keep improving as my programming skill gets better.

**Note** : To use this sdk, you first neet to add your `APIKEY` and `SECRETKEY` to your
environment, which you applied from binance.com.

For example, in your .zshrc:

```shell
export APIKEY="your api key"
export SECRETKEY="your secret key"
```

Or use `std::env::set_var("APIKEY", "your api key")` and `std::env::set_var("SECRETKEY", "your
secret key")` in your Rust code before you instantiate binance sdk client.


