rijksdriehoek
==============

Convert coordinates between rijksdriehoek and wgs84.

[![Travis-CI Status](https://api.travis-ci.org/SahiBkom/rijksdriehoek.svg?branch=master)](https://travis-ci.org/SahiBkom/rijksdriehoek)
[![Latest version](https://img.shields.io/crates/v/rijksdriehoek.svg)](https://crates.io/crates/rijksdriehoek)
[![Documentation](https://docs.rs/rijksdriehoek/badge.svg)](https://docs.rs/rijksdriehoek)
[![License](https://img.shields.io/crates/l/rijksdriehoek.svg)](https://github.com/SahiBkom/rijksdriehoek#license)

# Getting Started

[rijksdriehoek is available on crates.io](https://crates.io/crates/rijksdriehoek).
It is recommended to look there for the newest released version, as well as links to the newest builds of the docs.

At the point of the last update of this README, the latest published version could be used like this:

Add the following dependency to your Cargo manifest...

```toml
[dependencies]
rijksdriehoek = "0.1.0"
```

...and see the [docs](https://docs.rs/rijksdriehoek) for how to use it.

# Example

```rust
use rijksdriehoek::*;

fn main() {
    let (x, y): (f32, f32) = wgs84_to_rijksdriehoek(52.37453253, 4.88352559);
    println!("Convert wgs84 lat=52.37453253, lon=4.88352559 to rijksdriehoek x={} y={}", x , y);

    let (lat, lon): (f32, f32) = rijksdriehoek_to_wgs84(120700.723, 487525.502);
    println!("convert rijksdriehoek x=120700.723, y=487525.502 to wgs84 lat={} lon={}", lat, lon);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
