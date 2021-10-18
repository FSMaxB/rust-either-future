# Changelog

## 1.1.0
* Make the `Either` in `EitherFuture` publicly accessible
* Implement conversions to and from `futures_util::future::Either` behind the `futures03` feature flag
* Implement conversion from `EitherFuture` to `Either`
  * With `futures03` feature, implements `From`, otherwise only `Into` because of the [relaxed orphan rule restrictions in 1.41.0](https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html#relaxed-restrictions-when-implementing-traits)

## 1.0.0
* Enable `std_future` by default
* Rename `futures_future` to `futures01` and disable it by default
* Support `no_std`
* Introduce MSRV (minimum supported rust version). It's `1.15.0` with only `futures01` and `1.36.0` with `std_future`.
