// Copyright © 2024 David Caldwell <david@porkrind.org>

use predicates::prelude::*;
use assert_cmd::Command;

fn run<'a, I, O, P>(args: &[ &str ], stdin: Option<I>, stdout: &'a O) -> Result<(), Box<dyn std::error::Error>>
where
    O: std::fmt::Debug + std::cmp::Eq + ?Sized,
    predicates::ord::EqPredicate<&'a O>: assert_cmd::assert::IntoOutputPredicate<P>,
    P: predicates::Predicate<[u8]>,
    Vec<u8>: From<I>,
{
    let mut cmd = Command::cargo_bin("radix50")?;

    cmd.args(args);
    if let Some(stdin) = stdin {
        cmd.write_stdin(stdin);
    }
    cmd.assert()
        .success()
        .stdout(predicate::eq(stdout))
        .stderr(predicate::str::is_empty());
    Ok(())
}

const NONE: Option<&str> = None; // Work around None carrying no type information

#[test]
fn decode_args_basic() -> Result<(), Box<dyn std::error::Error>> {
    run(&["decode", "32329", "30409", "30401", "805", "31200"], NONE, "THIS IS A TEST \n")?;
    run(&["decode", "--pdp10", "3119342419", "2970305215", "3046400000"], NONE, "THIS IS A TEST    \n")?;
    Ok(())
}

#[test]
fn decode_args_pdp_11_bases() -> Result<(), Box<dyn std::error::Error>> {
    run(&["decode", "32329",   "30409",   "30401",   "805",    "31200"],   NONE, "THIS IS A TEST \n")?;
    run(&["decode", "0x7e49",  "0x76c9",  "0x76c1",  "0x325",  "0x79e0"],  NONE, "THIS IS A TEST \n")?;
    run(&["decode", "0o77111", "0o73311", "0o73301", "0o1445", "0o74740"], NONE, "THIS IS A TEST \n")?;
    run(&["decode", "0b111111001001001", "0b111011011001001",
                    "0b111011011000001", "0b1100100101", "0b111100111100000"], NONE,"THIS IS A TEST \n")?;
    Ok(())
}

#[test]
fn decode_args_pdp_10_bases() -> Result<(), Box<dyn std::error::Error>> {
    run(&["decode", "--pdp10", "3119342419",    "2970305215",    "3046400000"],    NONE, "THIS IS A TEST    \n")?;
    run(&["decode", "--pdp10", "0xb9ed6353",    "0xb10b42bf",    "0xb5946000"],    NONE, "THIS IS A TEST    \n")?;
    run(&["decode", "--pdp10", "0o27173261523", "0o26102641277", "0o26545060000"], NONE, "THIS IS A TEST    \n")?;
    run(&["decode", "--pdp10", "0b10111001111011010110001101010011",
                               "0b10110001000010110100001010111111",
                               "0b10110101100101000110000000000000"], NONE,"THIS IS A TEST    \n")?;
    Ok(())
}

#[test]
fn decode_stdin() -> Result<(), Box<dyn std::error::Error>> {
    run(&["decode"],            Some(&[0x7e, 0x49, 0x76, 0xc9, 0x76, 0xc1, 0x03, 0x25, 0x79, 0xe0]),             "THIS IS A TEST \n")?;
    run(&["decode", "--pdp10"], Some(&[0xb9, 0xed, 0x63, 0x53, 0xb1, 0x0b, 0x42, 0xbf, 0xb5, 0x94, 0x60, 0x00]), "THIS IS A TEST    \n")?;
    Ok(())
}

#[test]
fn encode_args_basic() -> Result<(), Box<dyn std::error::Error>> {
    run(&["encode", "THIS IS A TEST"], NONE, "32329 30409 30401 805 31200\n")?;
    run(&["encode", "--pdp10", "THIS IS A TEST"], NONE, "3119342419 2970305215 3046400000\n")?;
    Ok(())
}

#[test]
fn encode_args_pdp_11_bases() -> Result<(), Box<dyn std::error::Error>> {
    run(&["encode", "--format=dec", "THIS IS A TEST"], NONE, "32329 30409 30401 805 31200\n")?;
    run(&["encode", "--format=hex", "THIS IS A TEST"], NONE, "7e49 76c9 76c1 325 79e0\n")?;
    run(&["encode", "--format=oct", "THIS IS A TEST"], NONE, "77111 73311 73301 1445 74740\n")?;
    run(&["encode", "--format=bin", "THIS IS A TEST"], NONE,
        "111111001001001 111011011001001 111011011000001 1100100101 111100111100000\n")?;
    run(&["encode", "--format=raw", "THIS IS A TEST"], NONE, &[0x7e, 0x49, 0x76, 0xc9, 0x76, 0xc1, 0x03, 0x25, 0x79, 0xe0][..])?;
    Ok(())
}

#[test]
fn encode_args_pdp_10_bases() -> Result<(), Box<dyn std::error::Error>> {
    run(&["encode", "--pdp10", "--format=dec", "THIS IS A TEST"], NONE, "3119342419 2970305215 3046400000\n")?;
    run(&["encode", "--pdp10", "--format=hex", "THIS IS A TEST"], NONE, "b9ed6353 b10b42bf b5946000\n")?;
    run(&["encode", "--pdp10", "--format=oct", "THIS IS A TEST"], NONE, "27173261523 26102641277 26545060000\n")?;
    run(&["encode", "--pdp10", "--format=bin", "THIS IS A TEST"], NONE,
        "10111001111011010110001101010011 10110001000010110100001010111111 10110101100101000110000000000000\n")?;
    run(&["encode", "--pdp10", "--format=raw", "THIS IS A TEST"], NONE, &[0xb9, 0xed, 0x63, 0x53, 0xb1, 0x0b, 0x42, 0xbf, 0xb5, 0x94, 0x60, 0x00][..])?;
    Ok(())
}

#[test]
fn encode_stdin() -> Result<(), Box<dyn std::error::Error>> {
    run(&["encode"],            Some("THIS IS A TEST"), "32329 30409 30401 805 31200\n")?;
    run(&["encode", "--pdp10"], Some("THIS IS A TEST"), "3119342419 2970305215 3046400000\n")?;
    Ok(())
}
