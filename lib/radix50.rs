// PDP-10/PDP-11/VAX RADIX-50 encoding/decoding functions
//
// Copyright © 2023 David Caldwell <david@porkrind.org>
// License: MIT (see LICENSE.md file)

#![doc = include_str!(env!("CARGO_PKG_README"))]

use const_for::const_for;

// https://en.wikipedia.org/wiki/DEC_RADIX_50

pub mod pdp10 {
    use super::Error;
    /// The RADIX-50 character set used on the PDP-10, PDP-6, DECsystem-10, and DECSYSTEM-20.
    ///
    /// |Char |Dec| Hex| Oct|Binary|
    /// |-----|---|----|----|------|
    /// |space|  0|0x00|0o00|000000|
    /// |0    |  1|0x01|0o01|000001|
    /// |1    |  2|0x02|0o02|000010|
    /// |2    |  3|0x03|0o03|000011|
    /// |3    |  4|0x04|0o04|000100|
    /// |4    |  5|0x05|0o05|000101|
    /// |5    |  6|0x06|0o06|000110|
    /// |6    |  7|0x07|0o07|000111|
    /// |7    |  8|0x08|0o10|001000|
    /// |8    |  9|0x09|0o11|001001|
    /// |9    | 10|0x0a|0o12|001010|
    /// |A    | 11|0x0b|0o13|001011|
    /// |B    | 12|0x0c|0o14|001100|
    /// |C    | 13|0x0d|0o15|001101|
    /// |D    | 14|0x0e|0o16|001110|
    /// |E    | 15|0x0f|0o17|001111|
    /// |F    | 16|0x10|0o20|010000|
    /// |G    | 17|0x11|0o21|010001|
    /// |H    | 18|0x12|0o22|010010|
    /// |I    | 19|0x13|0o23|010011|
    /// |J    | 20|0x14|0o24|010100|
    /// |K    | 21|0x15|0o25|010101|
    /// |L    | 22|0x16|0o26|010110|
    /// |M    | 23|0x17|0o27|010111|
    /// |N    | 24|0x18|0o30|011000|
    /// |O    | 25|0x19|0o31|011001|
    /// |P    | 26|0x1a|0o32|011010|
    /// |Q    | 27|0x1b|0o33|011011|
    /// |R    | 28|0x1c|0o34|011100|
    /// |S    | 29|0x1d|0o35|011101|
    /// |T    | 30|0x1e|0o36|011110|
    /// |U    | 31|0x1f|0o37|011111|
    /// |V    | 32|0x20|0o40|100000|
    /// |W    | 33|0x21|0o41|100001|
    /// |X    | 34|0x22|0o42|100010|
    /// |Y    | 35|0x23|0o43|100011|
    /// |Z    | 36|0x24|0o44|100100|
    /// |.    | 37|0x25|0o45|100101|
    /// |$    | 38|0x26|0o46|100110|
    /// |%    | 39|0x27|0o47|100111|
    pub const RADIX50_DECODE: [char; 40] = [' ', '0', '1', '2', '3', '4', '5', '6',
                                            '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
                                            'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
                                            'V', 'W', 'X', 'Y', 'Z', '.', '$', '%'];

    const RADIX50_ENCODE: [Option<u8>; 128] = super::invert(&RADIX50_DECODE);

    /// Encode a string into [PDP-10 RADIX-50 format][`RADIX50_DECODE`].
    ///
    /// The input string will be space padded to a multiple of 3 characters before encoding. This is because
    /// RADIX-50 encodes 3 charaters into a single 16 bit word.
    ///
    /// The output is a [`std::vec::Vec`] of 16-bit words.
    ///
    /// It will return an [Error] if any of the input characters are not part of the [valid RADIX-50 character
    /// set][`RADIX50_DECODE`].
    ///
    /// # Examples
    /// ```
    /// # use radix50::{Error,pdp10::encode};
    /// let pdp10_encoded = encode("THIS IS A TEST").unwrap();
    /// assert_eq!(pdp10_encoded, [48739, 46419, 46411, 1215, 47600]);
    ///
    /// let result = encode("This contains invalid characters");
    /// assert_eq!(result, Err(Error::IllegalChar { char: 'h', pos: 2 }));
    ///
    /// assert_eq!(encode("PADDING12").unwrap(), encode("PADDING12").unwrap());
    /// assert_eq!(encode("PADDING1").unwrap(),  encode("PADDING1 ").unwrap());
    /// assert_eq!(encode("PADDING").unwrap(),   encode("PADDING  ").unwrap());
    /// ```
    pub fn encode(s: &str) -> Result<Vec<u16>, Error> { super::encode(&RADIX50_ENCODE, s) }

    /// Encode 3 characters into a [PDP-10 RADIX-50 formatted][`RADIX50_DECODE`] word.
    ///
    /// If the string is shorter than 3 characters then the missing characters are assumed to be spaces.
    ///
    /// The output is a single 16-bit word.
    ///
    /// It will return an [Error] if any of the input characters are not part of the [valid RADIX-50 character
    /// set][`RADIX50_DECODE`].
    ///
    /// # Examples
    /// ```
    /// # use radix50::{Error,pdp10::encode_word};
    /// let pdp10_encoded = encode_word("ABC").unwrap();
    /// assert_eq!(pdp10_encoded, 18093);
    ///
    /// assert_eq!(encode_word("AA").unwrap(), encode_word("AA ").unwrap());
    /// assert_eq!(encode_word("A").unwrap(),  encode_word("A  ").unwrap());
    /// assert_eq!(encode_word("").unwrap(),   encode_word("   ").unwrap());
    ///
    /// let result = encode_word("AB-");
    /// assert_eq!(result, Err(Error::IllegalChar { char: '-', pos: 3 }))
    /// ```
    pub fn encode_word(s: &str) -> Result<u16, Error> { super::encode_word(&RADIX50_ENCODE, s) }

    /// Decode a [`Vec`] of [PDP-10 RADIX-50 encoded][`RADIX50_DECODE`] words into a string.
    ///
    /// The output is a String.
    ///
    /// # Examples
    /// ```
    /// # use radix50::pdp10::decode;
    /// assert_eq!(decode(&[48739, 46419, 46411, 1215, 47600]), "THIS IS A TEST ");
    /// ```
    pub fn decode(words: &[u16]) -> String { super::decode(&RADIX50_DECODE, words) }

    /// Decode a [PDP-10 RADIX-50 encoded][`RADIX50_DECODE`] word into a 3 character string.
    ///
    /// The output is a String.
    ///
    /// # Examples
    /// ```
    /// # use radix50::pdp10::decode_word;
    /// assert_eq!(decode_word(3324), "123");
    /// ```
    pub fn decode_word(word: u16) -> String { super::decode_word(&RADIX50_DECODE, word) }
}

pub mod pdp11 {
    use super::Error;
    /// The RADIX-50 character set used on the PDP-11 and VAX.
    ///
    /// |Char |Dec| Hex| Oct|Binary|
    /// |-----|---|----|----|------|
    /// |space|  0|0x00|0o00|000000|
    /// |A    |  1|0x01|0o01|000001|
    /// |B    |  2|0x02|0o02|000010|
    /// |C    |  3|0x03|0o03|000011|
    /// |D    |  4|0x04|0o04|000100|
    /// |E    |  5|0x05|0o05|000101|
    /// |F    |  6|0x06|0o06|000110|
    /// |G    |  7|0x07|0o07|000111|
    /// |H    |  8|0x08|0o10|001000|
    /// |I    |  9|0x09|0o11|001001|
    /// |J    | 10|0x0a|0o12|001010|
    /// |K    | 11|0x0b|0o13|001011|
    /// |L    | 12|0x0c|0o14|001100|
    /// |M    | 13|0x0d|0o15|001101|
    /// |N    | 14|0x0e|0o16|001110|
    /// |O    | 15|0x0f|0o17|001111|
    /// |P    | 16|0x10|0o20|010000|
    /// |Q    | 17|0x11|0o21|010001|
    /// |R    | 18|0x12|0o22|010010|
    /// |S    | 19|0x13|0o23|010011|
    /// |T    | 20|0x14|0o24|010100|
    /// |U    | 21|0x15|0o25|010101|
    /// |V    | 22|0x16|0o26|010110|
    /// |W    | 23|0x17|0o27|010111|
    /// |X    | 24|0x18|0o30|011000|
    /// |Y    | 25|0x19|0o31|011001|
    /// |Z    | 26|0x1a|0o32|011010|
    /// |$    | 27|0x1b|0o33|011011|
    /// |.    | 28|0x1c|0o34|011100|
    /// |%    | 29|0x1d|0o35|011101|
    /// |0    | 30|0x1e|0o36|011110|
    /// |1    | 31|0x1f|0o37|011111|
    /// |2    | 32|0x20|0o40|100000|
    /// |3    | 33|0x21|0o41|100001|
    /// |4    | 34|0x22|0o42|100010|
    /// |5    | 35|0x23|0o43|100011|
    /// |6    | 36|0x24|0o44|100100|
    /// |7    | 37|0x25|0o45|100101|
    /// |8    | 38|0x26|0o46|100110|
    /// |9    | 39|0x27|0o47|100111|
    pub const RADIX50_DECODE: [char; 40] = [' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
                                            'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                                            'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
                                            'X', 'Y', 'Z', '$', '.', '%', '0', '1',
                                            '2', '3', '4', '5', '6', '7', '8', '9'];

    const RADIX50_ENCODE: [Option<u8>; 128] = super::invert(&RADIX50_DECODE);

    /// Encode a string into [PDP-11 RADIX-50 format][`RADIX50_DECODE`].
    ///
    /// The input string will be space padded to a multiple of 3 characters before encoding. This is because
    /// RADIX-50 encodes 3 charaters into a single 16 bit word.
    ///
    /// The output is a [`std::vec::Vec`] of 16-bit words.
    ///
    /// It will return an [Error] if any of the input characters are not part of the [valid RADIX-50 character
    /// set][`RADIX50_DECODE`].
    ///
    /// # Examples
    /// ```
    /// # use radix50::{Error,pdp11::encode};
    /// let pdp11_encoded = encode("THIS IS A TEST").unwrap();
    /// assert_eq!(pdp11_encoded, [32329, 30409, 30401, 805, 31200]);
    ///
    /// let result = encode("This contains invalid characters");
    /// assert_eq!(result, Err(Error::IllegalChar { char: 'h', pos: 2 }));
    ///
    /// assert_eq!(encode("PADDING12").unwrap(), encode("PADDING12").unwrap());
    /// assert_eq!(encode("PADDING1").unwrap(),  encode("PADDING1 ").unwrap());
    /// assert_eq!(encode("PADDING").unwrap(),   encode("PADDING  ").unwrap());
    /// ```
    pub fn encode(s: &str) -> Result<Vec<u16>, Error> { super::encode(&RADIX50_ENCODE, s) }

    /// Encode 3 characters into a [PDP-11 RADIX-50 formatted][`RADIX50_DECODE`] word.
    ///
    /// If the string is shorter than 3 characters then the missing characters are assumed to be spaces.
    ///
    /// The output is a single 16-bit word.
    ///
    /// It will return an [Error] if any of the input characters are not part of the [valid RADIX-50 character
    /// set][`RADIX50_DECODE`].
    ///
    /// # Examples
    /// ```
    /// # use radix50::{Error,pdp11::encode_word};
    /// let pdp11_encoded = encode_word("ABC").unwrap();
    /// assert_eq!(pdp11_encoded, 1683);
    ///
    /// assert_eq!(encode_word("AA").unwrap(), encode_word("AA ").unwrap());
    /// assert_eq!(encode_word("A").unwrap(),  encode_word("A  ").unwrap());
    /// assert_eq!(encode_word("").unwrap(),   encode_word("   ").unwrap());
    ///
    /// let result = encode_word("AB-");
    /// assert_eq!(result, Err(Error::IllegalChar { char: '-', pos: 3 }))
    /// ```
    pub fn encode_word(s: &str) -> Result<u16, Error> { super::encode_word(&RADIX50_ENCODE, s) }

    /// Decode a [`Vec`] of [PDP-11 RADIX-50 encoded][`RADIX50_DECODE`] words into a string.
    ///
    /// The output is a String.
    ///
    /// # Examples
    /// ```
    /// # use radix50::pdp11::decode;
    /// assert_eq!(decode(&[32329, 30409, 30401, 805, 31200]), "THIS IS A TEST ");
    /// ```
    pub fn decode(words: &[u16]) -> String { super::decode(&RADIX50_DECODE, words) }

    /// Decode a [PDP-11 RADIX-50 encoded][`RADIX50_DECODE`] word into a 3 character string.
    ///
    /// The output is a String.
    ///
    /// # Examples
    /// ```
    /// # use radix50::pdp11::decode_word;
    /// assert_eq!(decode_word(50913), "123");
    /// ```
    pub fn decode_word(word: u16) -> String { super::decode_word(&RADIX50_DECODE, word) }
}

const fn invert(radix50_table: &[char; 40]) -> [Option<u8>; 128] {
    let mut out = [None; 128];
    const_for!(i in 0..40 => {
        out[radix50_table[i] as usize] = Some(i as u8);
    });
    out
}

fn encode(encode_table: &[Option<u8>; 128], s: &str) -> Result<Vec<u16>, Error> {
    let mut out = Vec::with_capacity(s.len()/3);
    let mut i=0;
    for (i, chunk) in s.split_inclusive(|_| { i+=1; i % 3 == 0 }).enumerate() {
        out.push(encode_word(encode_table, chunk).map_err(|e| match e { Error::IllegalChar { char, pos } => Error::IllegalChar{char, pos: i*3 + pos} })?);
    }
    Ok(out)
}

fn encode_word(encode_table: &[Option<u8>; 128], s: &str) -> Result<u16, Error> {
    let mut it = s.chars();
    let c = [radix50_from_char(encode_table, it.next().unwrap_or(' '), 1)?,
             radix50_from_char(encode_table, it.next().unwrap_or(' '), 2)?,
             radix50_from_char(encode_table, it.next().unwrap_or(' '), 3)?];
    Ok(c[0] as u16 * 40_u16.pow(2) +
       c[1] as u16 * 40_u16.pow(1) +
       c[2] as u16 * 40_u16.pow(0))
}

fn radix50_from_char(encode_table: &[Option<u8>; 128], c: char, pos: usize) -> Result<u8, Error> {
    if c > '\u{7f}' {
        Err(Error::IllegalChar { char: c, pos })?;
    }
    match encode_table[c as usize] {
        Some(v) => Ok(v),
        None => Err(Error::IllegalChar { char: c, pos }),
    }
}

fn decode(decode_table: &[char; 40], words: &[u16]) -> String {
    words.iter().fold(String::new(), |mut s, w| { s.push_str(&decode_word(decode_table, *w)); s })
}

fn decode_word(decode_table: &[char; 40], w: u16) -> String {
    // Unsafe rationalization: bytes can only come from the RADIX50_PDP11 look up table and so are guaranteed
    // to be ASCII (and therefore valid utf8).
    unsafe { String::from_utf8_unchecked(vec![decode_table[(w / 40_u16.pow(2) % 40) as usize] as u8,
                                              decode_table[(w / 40_u16.pow(1) % 40) as usize] as u8,
                                              decode_table[(w / 40_u16.pow(0) % 40) as usize] as u8])
    }
}

/// RADIX-50 Encoding Errors
#[derive(Debug,Clone,PartialEq)]
pub enum Error {
    /// The given character (at `pos` offset (1-based) in the original string) isn't part of the valid
    /// RADIX-50 character set ([pdp-10][`pdp10::RADIX50_DECODE`]/[pdp-11][`pdp11::RADIX50_DECODE`])
    IllegalChar { char: char, pos: usize }
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IllegalChar {char, pos} => write!(f, "Illegal character '{}' ({}) at position {}", char, *char as u32, pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_words() {
        assert_eq!(pdp11::encode_word("999").expect("bad char"), 63999);
        assert_eq!(pdp11::encode_word("_BC"), Err(Error::IllegalChar{ char: '_', pos: 1 }));
        assert_eq!(pdp11::encode_word("A_C"), Err(Error::IllegalChar{ char: '_', pos: 2 }));
        assert_eq!(pdp11::encode_word("AB_"), Err(Error::IllegalChar{ char: '_', pos: 3 }));

        assert_eq!(pdp11::encode_word("ABC").expect("bad char"), 1683);
        assert_eq!(pdp11::encode_word("DEF").expect("bad char"), 6606);
        assert_eq!(pdp11::encode_word("GHI").expect("bad char"), 11529);
        assert_eq!(pdp11::encode_word("JKL").expect("bad char"), 16452);
        assert_eq!(pdp11::encode_word("MNO").expect("bad char"), 21375);
        assert_eq!(pdp11::encode_word("PQR").expect("bad char"), 26298);
        assert_eq!(pdp11::encode_word("STU").expect("bad char"), 31221);
        assert_eq!(pdp11::encode_word("VWX").expect("bad char"), 36144);
        assert_eq!(pdp11::encode_word("YZ ").expect("bad char"), 41040);
        assert_eq!(pdp11::encode_word("012").expect("bad char"), 49272);
        assert_eq!(pdp11::encode_word("345").expect("bad char"), 54195);
        assert_eq!(pdp11::encode_word("678").expect("bad char"), 59118);
        assert_eq!(pdp11::encode_word("9.$").expect("bad char"), 63547);
        assert_eq!(pdp11::encode_word("%")  .expect("bad char"), 46400);

        assert_eq!(pdp10::encode_word("ABC").expect("bad char"), 18093);
        assert_eq!(pdp10::encode_word("DEF").expect("bad char"), 23016);
        assert_eq!(pdp10::encode_word("GHI").expect("bad char"), 27939);
        assert_eq!(pdp10::encode_word("JKL").expect("bad char"), 32862);
        assert_eq!(pdp10::encode_word("MNO").expect("bad char"), 37785);
        assert_eq!(pdp10::encode_word("PQR").expect("bad char"), 42708);
        assert_eq!(pdp10::encode_word("STU").expect("bad char"), 47631);
        assert_eq!(pdp10::encode_word("VWX").expect("bad char"), 52554);
        assert_eq!(pdp10::encode_word("YZ ").expect("bad char"), 57440);
        assert_eq!(pdp10::encode_word("012").expect("bad char"),  1683);
        assert_eq!(pdp10::encode_word("345").expect("bad char"),  6606);
        assert_eq!(pdp10::encode_word("678").expect("bad char"), 11529);
        assert_eq!(pdp10::encode_word("9.$").expect("bad char"), 17518);
        assert_eq!(pdp10::encode_word("%")  .expect("bad char"), 62400);
    }

    #[test]
    fn decode_words() {
        assert_eq!(pdp11::decode_word( 1683),"ABC");
        assert_eq!(pdp11::decode_word( 6606),"DEF");
        assert_eq!(pdp11::decode_word(11529),"GHI");
        assert_eq!(pdp11::decode_word(16452),"JKL");
        assert_eq!(pdp11::decode_word(21375),"MNO");
        assert_eq!(pdp11::decode_word(26298),"PQR");
        assert_eq!(pdp11::decode_word(31221),"STU");
        assert_eq!(pdp11::decode_word(36144),"VWX");
        assert_eq!(pdp11::decode_word(41040),"YZ ");
        assert_eq!(pdp11::decode_word(49272),"012");
        assert_eq!(pdp11::decode_word(54195),"345");
        assert_eq!(pdp11::decode_word(59118),"678");
        assert_eq!(pdp11::decode_word(63547),"9.$");
        assert_eq!(pdp11::decode_word(46400),"%  ");

        assert_eq!(pdp10::decode_word(18093),"ABC");
        assert_eq!(pdp10::decode_word(23016),"DEF");
        assert_eq!(pdp10::decode_word(27939),"GHI");
        assert_eq!(pdp10::decode_word(32862),"JKL");
        assert_eq!(pdp10::decode_word(37785),"MNO");
        assert_eq!(pdp10::decode_word(42708),"PQR");
        assert_eq!(pdp10::decode_word(47631),"STU");
        assert_eq!(pdp10::decode_word(52554),"VWX");
        assert_eq!(pdp10::decode_word(57440),"YZ ");
        assert_eq!(pdp10::decode_word( 1683),"012");
        assert_eq!(pdp10::decode_word( 6606),"345");
        assert_eq!(pdp10::decode_word(11529),"678");
        assert_eq!(pdp10::decode_word(17518),"9.$");
        assert_eq!(pdp10::decode_word(62400),"%  ");

        // Section 2.6 of "Getting DOS On The Air" https://archive.org/details/bitsavers_decpdp11dotingDOSontheAirAug71_3085688/page/n37/mode/2up
        assert_eq!(pdp11::decode_word(0o14760), "DF ");
        assert_eq!(pdp11::decode_word(0o15270), "DK ");
        assert_eq!(pdp11::decode_word(0o14570), "DC ");
        assert_eq!(pdp11::decode_word(0o42420), "KB ");
        assert_eq!(pdp11::decode_word(0o63320), "PR ");
        assert_eq!(pdp11::decode_word(0o63200), "PP ");
        assert_eq!(pdp11::decode_word(0o46600), "LP ");
        assert_eq!(pdp11::decode_word(0o16040), "DT ");
        assert_eq!(pdp11::decode_word(0o52140), "MT ");
        assert_eq!(pdp11::decode_word(0o12620), "CR ");
        assert_eq!(pdp11::decode_word(0o63440), "PT ");
    }

    #[test]
    fn encode_strings() {
        assert_eq!(pdp10::encode("THIS IS A TEST").expect("bad char"), [48739, 46419, 46411, 1215, 47600]);
        assert_eq!(pdp11::encode("THIS IS A TEST").expect("bad char"), [32329, 30409, 30401, 805, 31200]);
        assert_eq!(pdp11::encode("_HIS IS A TEST"), Err(Error::IllegalChar{ char: '_', pos:  1 }));
        assert_eq!(pdp11::encode("THIS _S A TEST"), Err(Error::IllegalChar{ char: '_', pos:  6 }));
        assert_eq!(pdp11::encode("THIS IS A TES_"), Err(Error::IllegalChar{ char: '_', pos: 14 }));
    }

    #[test]
    fn decode_strings() {
        assert_eq!(pdp10::decode(&[48739, 46419, 46411, 1215, 47600]), "THIS IS A TEST ");
        assert_eq!(pdp11::decode(&[32329, 30409, 30401, 805, 31200]), "THIS IS A TEST ");
    }
}
