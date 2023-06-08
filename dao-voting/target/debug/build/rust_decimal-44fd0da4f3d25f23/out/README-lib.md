
A Decimal number implementation written in pure Rust suitable for financial calculations that require significant integral and fractional digits with no round-off errors.

The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign. Because of this representation, trailing zeros are preserved and may be exposed when in string form. These can be truncated using the `normalize` or `round_dp` functions.

## Getting started

To get started, add `rust_decimal` and optionally `rust_decimal_macros` to your `Cargo.toml`:

```toml
[dependencies]
rust_decimal = "1.26"
rust_decimal_macros = "1.26"
```

## Usage

Decimal numbers can be created in a few distinct ways. The easiest and most efficient method of creating a Decimal is to use the procedural macro within the `rust_decimal_macros` crate:

```rust
// Procedural macros need importing directly
use rust_decimal_macros::dec;

let number = dec!(-1.23);
assert_eq!("-1.23", number.to_string());
```

Alternatively you can also use one of the Decimal number convenience functions:

```rust
// Using the prelude can help importing trait based functions (e.g. core::str::FromStr).
use rust_decimal::prelude::*;

// Using an integer followed by the decimal points
let scaled = Decimal::new(202, 2);
assert_eq!("2.02", scaled.to_string());

// From a string representation
let from_string = Decimal::from_str("2.02").unwrap();
assert_eq!("2.02", from_string.to_string());

// From a string representation in a different base
let from_string_base16 = Decimal::from_str_radix("ffff", 16).unwrap();
assert_eq!("65535", from_string_base16.to_string());

// Using the `Into` trait
let my_int: Decimal = 3i32.into();
assert_eq!("3", my_int.to_string());

// Using the raw decimal representation
let pi = Decimal::from_parts(1102470952, 185874565, 1703060790, false, 28);
assert_eq!("3.1415926535897932384626433832", pi.to_string());
```

Once you have instantiated your `Decimal` number you can perform calculations with it just like any other number:

```rust
use rust_decimal::prelude::*;

let amount = Decimal::from_str("25.12").unwrap();
let tax = Decimal::from_str("0.085").unwrap();
let total = amount + (amount * tax).round_dp(2);
assert_eq!(total.to_string(), "27.26");
```

## Features

**Behavior / Functionality**

* [borsh](#borsh)
* [c-repr](#c-repr)
* [legacy-ops](#legacy-ops)
* [maths](#maths)
* [rkyv](#rkyv)
* [rocket-traits](#rocket-traits)
* [rust-fuzz](#rust-fuzz)
* [std](#std)

**Database**

* [db-postgres](#db-postgres)
* [db-tokio-postgres](#db-tokio-postgres)
* [db-diesel-postgres](#db-diesel-postgres)
* [db-diesel-mysql](#db-diesel-mysql)

**Serde**

* [serde-float](#serde-float)
* [serde-str](#serde-str)
* [serde-arbitrary-precision](#serde-arbitrary-precision)
* [serde-with-float](#serde-with-float)
* [serde-with-str](#serde-with-str)
* [serde-with-arbitrary-precision](#serde-with-arbitrary-precision)

### `borsh`

Enables [Borsh](https://borsh.io/) serialization for `Decimal`.

### `c-repr`

Forces `Decimal` to use `[repr(C)]`. The corresponding target layout is 128 bit aligned.

### `db-postgres`

Enables a PostgreSQL communication module. It allows for reading and writing the `Decimal`
type by transparently serializing/deserializing into the `NUMERIC` data type within PostgreSQL.

### `db-tokio-postgres`

Enables the tokio postgres module allowing for async communication with PostgreSQL.

### `db-diesel-postgres`

Enable `diesel` PostgreSQL support. By default, this enables version `1.4` of `diesel`. If you wish to use the `diesel`
release candidates then you can do so by using `db-diesel2-postgres`. Please note, the `db-diesel2-*` features are considered
unstable and will be upgraded as and when new `diesel 2.x` features are released.

### `db-diesel-mysql`

Enable `diesel` MySQL support. By default, this enables version `1.4` of `diesel`. If you wish to use the `diesel`
release candidates then you can do so by using `db-diesel2-mysql`. Please note, the `db-diesel2-*` features are considered
unstable and will be upgraded as and when new `diesel 2.x` features are released.

### `legacy-ops`

**Warning:** This is deprecated and will be removed from a future versions.

As of `1.10` the algorithms used to perform basic operations have changed which has benefits of significant speed improvements.
To maintain backwards compatibility this can be opted out of by enabling the `legacy-ops` feature.

### `maths`

The `maths` feature enables additional complex mathematical functions such as `pow`, `ln`, `enf`, `exp` etc.
Documentation detailing the additional functions can be found on the
[`MathematicalOps`](https://docs.rs/rust_decimal/latest/rust_decimal/trait.MathematicalOps.html) trait.  

Please note that `ln` and `log10` will panic on invalid input with `checked_ln` and `checked_log10` the preferred functions
to curb against this. When the `maths` feature was first developed the library would return `0` on invalid input. To re-enable this
non-panicking behavior, please use the feature: `maths-nopanic`.

### `rand`

Implements `rand::distributions::Distribution<Decimal>` to allow the creation of random instances.

Note: When using `rand::Rng` trait to generate a decimal between a range of two other decimals, the scale of the randomly-generated
decimal will be the same as the scale of the input decimals (or, if the inputs have different scales, the higher of the two).

### `rkyv`
Enables [rkyv](https://github.com/rkyv/rkyv) serialization for `Decimal`.
Supports rkyv's safe API when the `rkyv-safe` feature is enabled as well.

### `rocket-traits`

Enable support for Rocket forms by implementing the `FromFormField` trait.

### `rust-fuzz`

Enable `rust-fuzz` support by implementing the `Arbitrary` trait.

### `serde-float`

**Note:** it is recommended to use the `serde-with-*` features for greater control. This allows configurability at the data
level.

Enable this so that JSON serialization of `Decimal` types are sent as a float instead of a string (default).

e.g. with this turned on, JSON serialization would output:
```json
{
  "value": 1.234
}
```

### `serde-str`

**Note:** it is recommended to use the `serde-with-*` features for greater control. This allows configurability at the data
level.

This is typically useful for `bincode` or `csv` like implementations.

Since `bincode` does not specify type information, we need to ensure that a type hint is provided in order to
correctly be able to deserialize. Enabling this feature on its own will force deserialization to use `deserialize_str`
instead of `deserialize_any`.

If, for some reason, you also have `serde-float` enabled then this will use `deserialize_f64` as a type hint. Because
converting to `f64` _loses_ precision, it's highly recommended that you do NOT enable this feature when working with
`bincode`. That being said, this will only use 8 bytes so is slightly more efficient in terms of storage size.

### `serde-arbitrary-precision`

**Note:** it is recommended to use the `serde-with-*` features for greater control. This allows configurability at the data
level.

This is used primarily with `serde_json` and consequently adds it as a "weak dependency". This supports the
`arbitrary_precision` feature inside `serde_json` when parsing decimals.

This is recommended when parsing "float" looking data as it will prevent data loss.

### `serde-with-float`

Enable this to access the module for serializing `Decimal` types to a float. This can be use in `struct` definitions like so:

```rust
# use rust_decimal::Decimal;
# use serde::{Serialize, Deserialize};
# #[cfg(features = "serde-with-float")]
#[derive(Serialize, Deserialize)]
pub struct FloatExample {
    #[serde(with = "rust_decimal::serde::float")]
    value: Decimal,
}
```
```rust
# use rust_decimal::Decimal;
# use serde::{Serialize, Deserialize};
# #[cfg(features = "serde-with-float")]
#[derive(Serialize, Deserialize)]
pub struct OptionFloatExample {
    #[serde(with = "rust_decimal::serde::float_option")]
    value: Option<Decimal>,
}
```

### `serde-with-str`

Enable this to access the module for serializing `Decimal` types to a `String`. This can be use in `struct` definitions like so:

```rust
# use rust_decimal::Decimal;
# use serde::{Serialize, Deserialize};
# #[cfg(features = "serde-with-str")]
#[derive(Serialize, Deserialize)]
pub struct StrExample {
    #[serde(with = "rust_decimal::serde::str")]
    value: Decimal,
}
```
```rust
# use rust_decimal::Decimal;
# use serde::{Serialize, Deserialize};
# #[cfg(features = "serde-with-str")]
#[derive(Serialize, Deserialize)]
pub struct OptionStrExample {
    #[serde(with = "rust_decimal::serde::str_option")]
    value: Option<Decimal>,
}
```

### `serde-with-arbitrary-precision`

Enable this to access the module for serializing `Decimal` types to a `String`. This can be use in `struct` definitions like so:

```rust
# use rust_decimal::Decimal;
# use serde::{Serialize, Deserialize};
# #[cfg(features = "serde-with-arbitrary-precision")]
#[derive(Serialize, Deserialize)]
pub struct ArbitraryExample {
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    value: Decimal,
}
```
```rust
# use rust_decimal::Decimal;
# use serde::{Serialize, Deserialize};
# #[cfg(features = "serde-with-arbitrary-precision")]
#[derive(Serialize, Deserialize)]
pub struct OptionArbitraryExample {
    #[serde(with = "rust_decimal::serde::arbitrary_precision_option")]
    value: Option<Decimal>,
}
```

### `std`

Enable `std` library support. This is enabled by default, however in the future will be opt in. For now, to support `no_std`
libraries, this crate can be compiled with `--no-default-features`.

## Building

Please refer to the [Build document](https://github.com/paupino/rust-decimal/blob/master/BUILD.md) for more information on building and testing Rust Decimal.

## Minimum Rust Compiler Version

The current _minimum_ compiler version is [`1.56.0`](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1560-2021-10-21)
which was released on `2021-10-21`.

This library maintains support for rust compiler versions that are 4 minor versions away from the current stable rust compiler version.
For example, if the current stable compiler version is `1.50.0` then we will guarantee support up to and including `1.46.0`.
Of note, we will only update the minimum supported version if and when required.
