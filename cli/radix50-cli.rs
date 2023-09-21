// Copyright © 2023 David Caldwell <david@porkrind.org>

#![feature(iter_intersperse)]
#![feature(array_chunks)]

use std::error::Error;

use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
Usage:
  radix50 -h
  radix50 [-h] decode  [--pdp10] [<word>...]
  radix50 [-h] encode  [--pdp10] [--format=<format>] [<string>]
  radix50 [-h] charset [--pdp10]

Options:
  -h --help              Show this screen.
  -f --format=<format>   Output in a specific format [default: dec].
                         <format> can be: hex, oct, dec, bin, raw.
                         \"raw\" is a raw binary byte stream.
  --pdp10                Use the PDP-10 radix-50 encoding instead
                         of the default PDP-11 encoding.

<word> is a 16 bit word in decimal, hex, or octal (123, 0x7b, 0o173,
and 0b1111011 are the same)

If <string> or <word> is omitted, stdin is read as input.
When decoding from stdin, stdin is read as a binary stream.

The \"charset\" command will dump the radix-50 charset table.
";
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
        use std::io::Write;
        let to_encode = args.arg_string.map(|s| Ok(s)).unwrap_or_else(|| stdin_to_string())?;
        let encoded = match args.flag_pdp10 { true  => radix50::pdp10::encode(&to_encode)?,
                                              false => radix50::pdp11::encode(&to_encode)?, };
        match args.flag_format {
            Format::Raw => {
                // I like how compact this is, but I dislike the extra vec contructions and flattens:
                // std::io::stdout().write(&encoded.iter().map(|w| vec![(w >> 8) as u8, (w & 0xff) as u8]).flatten().collect::<Vec<u8>>() );
                let mut buffer: Vec<u8> = Vec::with_capacity(encoded.len() * 2);
                for w in encoded.iter() { buffer.push((w >> 8) as u8); buffer.push((w & 0xff) as u8); }
                std::io::stdout().write(&buffer)?;
            },
            Format::Hex | Format::Oct | Format::Dec | Format::Bin => {
                println!("{}", encoded.iter().map(|w| { match args.flag_format {
                                                            Format::Bin => format!("{:b}", w),
                                                            Format::Hex => format!("{:x}", w),
                                                            Format::Oct => format!("{:o}", w),
                                                            Format::Dec => format!("{}",   w),
                                                            _ => unreachable!(),
                                                        }})
                                             .intersperse(" ".to_string()).collect::<String>())
            },
        }
    }


    if args.cmd_decode {
        let words = if args.arg_word.len() > 0 {
            args.arg_word.iter().map(|s| Ok(match s {
                                                s if s.starts_with("0x") => u16::from_str_radix(&s[2..], 16),
                                                s if s.starts_with("0o") => u16::from_str_radix(&s[2..],  8),
                                                s if s.starts_with("0b") => u16::from_str_radix(&s[2..],  2),
                                                s                        => u16::from_str_radix(s,       10),
                                            }.map_err(|_| format!("Couldn't parse as integer: {}", s))?))
                .collect::<Result<Vec<u16>, String>>()?
        } else {
            stdin_to_bytes()?.array_chunks().map(|a: &[u8;2]| (a[0] as u16) << 8 | a[1] as u16).collect()
        };

        match args.flag_pdp10 {
            true  => println!("{}", radix50::pdp10::decode(&words)),
            false => println!("{}", radix50::pdp11::decode(&words)),
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
