<!-- cargo-rdme start -->

radix50
=======

Rust library and cli for encoding and decoding [DEC PDP-11 and PDP-10
RADIX-50 word streams][wikipedia].

[wikipedia]: https://en.wikipedia.org/wiki/DEC_RADIX_50

Library Usage
-------------

Add this to your `Cargo.toml`:

```toml
[dependencies]
radix50 = "0.1.0"
```

### Example

```rust
use radix50::{pdp10,pdp11};

let pdp10_encoded = pdp10::encode("THIS IS A TEST").unwrap();
let pdp11_encoded = pdp11::encode("THIS IS A TEST").unwrap();

assert_eq!(pdp10_encoded, [48739, 46419, 46411, 1215, 47600]);
assert_eq!(pdp11_encoded, [32329, 30409, 30401, 805, 31200]);

let pdp10_decoded = pdp10::decode(&[48739, 46419, 46411, 1215, 47600]);
let pdp11_decoded = pdp11::decode(&[32329, 30409, 30401, 805, 31200]);

assert_eq!(pdp10_decoded, "THIS IS A TEST ");
assert_eq!(pdp11_decoded, "THIS IS A TEST ");
```

<!-- cargo-rdme end -->

CLI
---

The [code repo][repo] contains a cli utility for encoding or decoding (also
published as the `radix50-cli` crate).

[repo]: https://github.com/caldwell/radix50

### Installing from Cargo

```shell-session
cargo install radix50-cli
```

### Building From Source

```shell-session
cargo build --release
```

The output executable will be create in `./target/release/radix50`.

### Running

```shell-session
$ radix50 encode "ENCODE THIS"
```

or

```shell-session
$ echo -n "ENCODE THIS" | radix50 encode
```

will output a list of 16-bit words in decimal:

```shell-session
8563 24165 808 15160
```

Add the `--format' flag to output something other than decimal:

```shell-session
$ radix50 encode --format=hex "ENCODE THIS"
2173 5e65 328 3b38
$ radix50 encode --format=oct "ENCODE THIS"
20563 57145 1450 35470
$ radix50 encode --format=bin "ENCODE THIS"
10000101110011 101111001100101 1100101000 11101100111000
$ radix50 encode --format=raw "ENCODE THIS" | xxd
00000000: 2173 5e65 0328 3b38                      !s^e.(;8
```

Decoding:

```shell-session
$ radix50 decode 6603 24165 808 15188
DECODE THIS.
$ radix50 decode 0x3b60 0x7a18 0x666a 0x7ff8 0x32e0 0x32f 0x5dc0
IT SUPPORTS HEX TOO
$ radix50 decode 0o4164 0o1133 0o76464
AND OCTAL
$ radix50 decode 0b10001010110101 0b101011110000010 0b11101001110001 0b111010001101000
EVEN BINARY
```

Decoding from `stdin` will assume a raw bytestream format:

```shell-session
$ printf "\x79\x18\x70\xbf" | radix50 decode
SO RAW
```

The default uses PDP-11/VAX encoding. Use the `--pdp10` flag to use the
PDP-10 encoding (also used for PDP-6, DECsystem-10, DECSYSTEM-20).

Display the RADIX-50 character set:

```shell-session
$ radix50 charset
```

License
-------
Copyright © 2023 David Caldwell <david@porkrind.org>

MIT Licensed. See [LICENSE.md](LICENSE.md) for details.
