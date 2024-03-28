// PDP-10/PDP-11/VAX RADIX-50 encoding/decoding cli
//
// Copyright © 2023-2024 David Caldwell <david@porkrind.org>
// License: MIT (see LICENSE.md file)

#![feature(iter_intersperse)]

use std::error::Error;

use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = r#"
Usage:
  radix50 -h
  radix50 [-h] decode  [--pdp10] [<word>...]
  radix50 [-h] encode  [--pdp10] [--format=<format>] [<string>]
  radix50 [-h] charset [--pdp10]

Options:
  -h --help              Show this screen.
  -f --format=<format>   Output in a specific format [default: dec].
                         <format> can be: hex, oct, dec, bin, raw.
                         "raw" is a raw big endian binary byte stream.
  --pdp10                Use the PDP-10 radix-50 encoding instead
                         of the default PDP-11 encoding.

<word> is a word in decimal, hex, or octal (123, 0x7b, 0o173,
and 0b1111011 are the same). The default PDP-11 encoding uses 16-bit
words. PDP-10 encoding mode uses 32-bit words.

If <string> or <word> is omitted, stdin is read as input.
When decoding from stdin, stdin is read as a big endian binary stream.

The "charset" command will dump the radix-50 charset table.
"#;
#[derive(Debug, Deserialize)]
struct Args {
    flag_format:      Format,
    flag_pdp10:       bool,
    cmd_decode:       bool,
    cmd_encode:       bool,
    cmd_charset:      bool,
    arg_word:         Vec<String>,
    arg_string:       Option<String>,
}

#[derive(Debug, Deserialize)]
enum Format { Raw, Bin, Hex, Oct, Dec }


fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_encode {
        let to_encode = args.arg_string.map(|s| Ok(s)).unwrap_or_else(|| stdin_to_string())?;
        match args.flag_pdp10 { true  => output_with_format(&radix50::pdp10::encode(&to_encode)?, args.flag_format)?,
                                false => output_with_format(&radix50::pdp11::encode(&to_encode)?, args.flag_format)?}
    }


    if args.cmd_decode {
        match args.flag_pdp10 {
            true  => println!("{}", radix50::pdp10::decode(&get_input(&args.arg_word)?)),
            false => println!("{}", radix50::pdp11::decode(&get_input(&args.arg_word)?)),
        };
    }


    if args.cmd_charset {
        let header = format!("{:5} {:-3} {:>4} {:>4} {:>6}", "Char", "Dec", "Hex", "Oct", "Binary");
        println!("{}\n{:-<2$}", header, "", header.len());
        for (i, c) in if args.flag_pdp10 { radix50::pdp10::RADIX50_DECODE }
                                    else { radix50::pdp11::RADIX50_DECODE }.iter().enumerate() {
            println!("{:5} {:3} {:#04x} {:#04o} {:06b}",
                if *c == ' ' { "space".to_string() } else { c.to_string() },
                i, i, i, i);
        }
    }


    Ok(())
}

use std::{fmt::{Binary, LowerHex, Octal, Display}, mem::size_of};

fn output_with_format<T>(encoded: &Vec<T>, format: Format) -> Result<(), Box<dyn Error>>
where
    T: Binary+LowerHex+Octal+Display+Copy, u64:From<T>
{
    use std::io::Write;
    match format {
        Format::Raw => {
            let mut buffer: Vec<u8> = Vec::with_capacity(encoded.len() * size_of::<T>());
            for w in encoded.iter() {
                buffer.extend_from_slice(&(u64::from(*w)).to_be_bytes()[8-size_of::<T>()..]);
            }
            std::io::stdout().write(&buffer)?;
        },
        Format::Hex | Format::Oct | Format::Dec | Format::Bin => {
            println!("{}", encoded.iter().map(|w| { match format {
                                                        Format::Bin => format!("{:b}", w),
                                                        Format::Hex => format!("{:x}", w),
                                                        Format::Oct => format!("{:o}", w),
                                                        Format::Dec => format!("{}",   w),
                                                        _ => unreachable!(),
                                                    }})
                                         .intersperse(" ".to_string()).collect::<String>())
        },
    }

    Ok(())
}

fn get_input<T>(words: &Vec<String>) -> Result<Vec<T>, Box<dyn Error>>
where
    T: std::convert::TryFrom<u64, Error=std::num::TryFromIntError>,
{
    if words.len() > 0 {
        parse_words(words)
    } else {
        Ok(stdin_to_bytes()?.chunks_exact(size_of::<T>()).map(|a| {
            a.iter().fold(0u64, |w, b| w << 8 | *b as u64)
                .try_into().unwrap(/*Can't fail in chunk param is correct*/)
        }).collect())
    }
}

fn parse_words<T>(words: &Vec<String>) -> Result<Vec<T>, Box<dyn Error>>
where
    T: std::convert::TryFrom<u64, Error=std::num::TryFromIntError>,
{
    words.iter().map(|s| Ok(match s {
        s if s.starts_with("0x") => u64::from_str_radix(&s[2..], 16),
        s if s.starts_with("0o") => u64::from_str_radix(&s[2..],  8),
        s if s.starts_with("0b") => u64::from_str_radix(&s[2..],  2),
        s                        => u64::from_str_radix(s,       10),
    }.map_err(|_| format!("Couldn't parse as integer: {}", s))?
        .try_into().map_err(|_| format!("Couldn't convert {} to {}", s, std::any::type_name::<T>()))?))
        .collect()
}

fn stdin_to_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    use std::io::Read;
    let mut b = Vec::new();
    std::io::stdin().read_to_end(&mut b)?;
    Ok(b)
}

fn stdin_to_string() -> Result<String, Box<dyn Error>> {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}
