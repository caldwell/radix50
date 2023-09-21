
use const_for::const_for;

// https://en.wikipedia.org/wiki/DEC_RADIX_50

pub mod pdp10 {
    pub const RADIX50_DECODE: [char; 40] = [' ', '0', '1', '2', '3', '4', '5', '6',
                                            '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
                                            'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
                                            'V', 'W', 'X', 'Y', 'Z', '.', '$', '%'];

    const RADIX50_ENCODE: [Option<u8>; 128] = super::invert(&RADIX50_DECODE);

    pub fn encode(s: &str) -> Result<Vec<u16>, super::Error> { super::encode(&RADIX50_ENCODE, s) }
    pub fn encode_word(s: &str) -> Result<u16, super::Error> { super::encode_word(&RADIX50_ENCODE, s) }
    pub fn decode(words: &[u16]) -> String { super::decode(&RADIX50_DECODE, words) }
    pub fn decode_word(word: u16) -> String { super::decode_word(&RADIX50_DECODE, word) }
}

pub mod pdp11 {
    pub const RADIX50_DECODE: [char; 40] = [' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
                                            'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                                            'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
                                            'X', 'Y', 'Z', '$', '.', '%', '0', '1',
                                            '2', '3', '4', '5', '6', '7', '8', '9'];

    const RADIX50_ENCODE: [Option<u8>; 128] = super::invert(&RADIX50_DECODE);

    pub fn encode(s: &str) -> Result<Vec<u16>, super::Error> { super::encode(&RADIX50_ENCODE, s) }
    pub fn encode_word(s: &str) -> Result<u16, super::Error> { super::encode_word(&RADIX50_ENCODE, s) }
    pub fn decode(words: &[u16]) -> String { super::decode(&RADIX50_DECODE, words) }
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


#[derive(Debug,Clone,PartialEq)]
pub enum Error {
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
