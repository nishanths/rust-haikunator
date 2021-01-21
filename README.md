# [rust-haikunator](https://github.com/nishanths/rust-haikunator)

[![Build Status](https://travis-ci.org/nishanths/rust-haikunator.svg?branch=master)](https://travis-ci.org/nishanths/rust-haikunator)
[![Coverage Status](https://coveralls.io/repos/nishanths/rust-haikunator/badge.svg?branch=master&service=github)](https://coveralls.io/github/nishanths/rust-haikunator?branch=master)
[![Docs](https://docs.rs/haikunator/badge.svg)](https://docs.rs/haikunator)

Generate Heroku-like random names to use in your Rust applications.

```
snowy-meadow-1347
delicate-haze-8496
cold-wildflower-3957
```

[crates.io](https://crates.io/crates/haikunator/)

## Installation

Add to your `Config.toml` dependencies

```toml
[dependencies]

haikunator = "0.1.0"
```

## Usage

Include the crate; create a `Haikunator` instance; then call `haikunate`.

```rust
extern crate haikunator;

use haikunator::{Haikunator};

fn main() {
    // normal usage
    let haikunator = Haikunator::default();    
    println!("{}", haikunator.haikunate()); // => "fancy-cloud-7181"

    // custom length (default=4)
    let mut haikunator = Haikunator::default();
    haikunator.token_length = 9;
    println!("{}", haikunator.haikunate()); // => "rapid-mode-572457286"

    // use hex instead of numbers
    let mut haikunator = Haikunator::default();
    haikunator.token_hex = true;
    println!("{}", haikunator.haikunate()); // => "misty-boat-bd01"

    // use custom chars instead of numbers/hex
    // unicode works too
    let mut haikunator = Haikunator::default();
    haikunator.token_chars = "HAIKUNATE忠犬ハチ公";
    println!("{}", haikunator.haikunate()); // => "divine-tiger-NKKチ"

    // don't include a token
    let mut haikunator = Haikunator::default();
    haikunator.token_length = 0;
    println!("{}", haikunator.haikunate()); // => "lingering-term"

    // use a different delimiter
    let mut haikunator = Haikunator::default();
    haikunator.delimiter = ":";
    println!("{}", haikunator.haikunate()); // => "young:cell:5426"

    // no token, space delimiter
    let mut haikunator = Haikunator::default();
    haikunator.token_length = 0;
    haikunator.delimiter = " ";
    println!("{}", haikunator.haikunate()); // => "wandering coke"

    // no token, empty delimiter
    let mut haikunator = Haikunator::default();
    haikunator.token_length = 0;
    haikunator.delimiter = "";
    println!("{}", haikunator.haikunate()); // => "freetooth"

    // custom nouns and/or adjectives
    let haikunator = Haikunator {
        adjectives: &["dandy", "froody", "happy"],
        nouns: &["whale", "towel", "earth"],
        delimiter: "-",
        token_length: 3,
        token_hex: false,
        token_chars: "24",
    };
    println!("{}", haikunator.haikunate()); // => "happy-earth-444"
}
```

See the test files at [`tests/lib.rs`](https://github.com/nishanths/rust-haikunator/blob/master/tests/lib.rs) for more examples.

## Options

The following options are available:

```rust
pub struct Haikunator<'a> {
    pub adjectives: &'a [&'a str],
    pub nouns: &'a [&'a str],
    pub delimiter: &'a str,
    pub token_length: usize,
    pub token_hex: bool,
    pub token_chars: &'a str,
}
```

**Note**: If `token_hex` is true, the value of `token_chars` is ignored.

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/nishanths/rust-haikunator/issues)
- Fix bugs and [submit pull requests](https://github.com/nishanths/rust-haikunator/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

## Other Languages

Haikunator is also available in other languages. Check them out:

- Go: <https://github.com/Atrox/haikunatorgo>
- Node: <https://github.com/Atrox/haikunatorjs>
- .NET: <https://github.com/Atrox/haikunator.net>
- Python: <https://github.com/Atrox/haikunatorpy>
- PHP: <https://github.com/Atrox/haikunatorphp>
- Java: <https://github.com/Atrox/haikunatorjava>
- Dart: <https://github.com/Atrox/haikunatordart>
- Ruby: <https://github.com/usmanbashir/haikunator>
- R: <https://github.com/amrrs/haikunator>

## License

rust-haikunator is available under the [MIT License](https://github.com/nishanths/rust-haikunator/blob/master/LICENSE).
