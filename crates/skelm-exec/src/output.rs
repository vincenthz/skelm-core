use std::borrow::Cow;

use smallvec::{SmallVec, smallvec};

pub struct OutputUtf8 {
    bytes: SmallVec<[u8; 32]>,
}

pub enum Res<'a> {
    Ok(Cow<'a, str>),
    More,
    Error(ProcessingError),
}

impl<'a> From<String> for Res<'a> {
    fn from(value: String) -> Self {
        Self::Ok(Cow::Owned(value))
    }
}

pub enum InternalResult {
    Ok(String),
    More,
    Error(ProcessingError),
}

impl<'a> From<&'a str> for Res<'a> {
    fn from(value: &'a str) -> Self {
        Self::Ok(Cow::Borrowed(value))
    }
}

impl<'a> From<InternalResult> for Res<'a> {
    fn from(value: InternalResult) -> Self {
        match value {
            InternalResult::Ok(i) => Self::Ok(Cow::Owned(i)),
            InternalResult::More => Self::More,
            InternalResult::Error(processing_error) => Self::Error(processing_error),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ProcessingError {
    OnHeader(usize, [u8; 4]),
    OnCont(u8),
}

impl OutputUtf8 {
    pub fn new() -> Self {
        Self { bytes: smallvec![] }
    }

    // try to process the internal buffer
    fn process_internal(&mut self) -> InternalResult {
        assert!(!self.bytes.is_empty());
        match std::str::from_utf8(&self.bytes) {
            Ok(valid) => {
                let valid = valid.to_string();
                self.bytes.truncate(0);
                InternalResult::Ok(valid)
            }
            Err(e) => {
                if e.valid_up_to() > 0 {
                    let valid = std::str::from_utf8(&self.bytes[..e.valid_up_to()])
                        .unwrap()
                        .to_string();
                    self.bytes.copy_within(e.valid_up_to().., 0);
                    self.bytes.truncate(self.bytes.len() - e.valid_up_to());
                    InternalResult::Ok(valid)
                } else {
                    let first = self.bytes[0];
                    if utf8::is_cont(first) {
                        self.bytes.copy_within(1.., 0);
                        self.bytes.truncate(self.bytes.len() - 1);
                        InternalResult::Error(ProcessingError::OnCont(first))
                    } else {
                        let len = utf8::utf8_char_width(first);
                        if self.bytes.len() >= len {
                            let mut out = [0; 4];

                            out[0..len].copy_from_slice(&self.bytes[0..len]);
                            self.bytes.copy_within(len.., 0);
                            self.bytes.truncate(self.bytes.len() - len);

                            InternalResult::Error(ProcessingError::OnHeader(len, out))
                        } else {
                            InternalResult::More
                        }
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn process<'a>(&mut self, bytes: &'a [u8]) -> Res<'a> {
        if self.bytes.is_empty() {
            match std::str::from_utf8(bytes) {
                Ok(r) => r.into(),
                Err(e) => {
                    if e.valid_up_to() > 0 {
                        let (valid, queueable) = bytes.split_at(e.valid_up_to());
                        let valid = std::str::from_utf8(valid).unwrap();
                        self.bytes.extend_from_slice(queueable);
                        valid.into()
                    } else {
                        self.bytes.extend_from_slice(bytes);
                        Res::More
                    }
                }
            }
        } else {
            self.bytes.extend_from_slice(bytes);
            self.process_internal().into()
        }
    }

    pub fn finalize(mut self) -> Result<String, ()> {
        if self.bytes.is_empty() {
            return Ok(String::new());
        }
        match self.process_internal() {
            InternalResult::Ok(s) => Ok(s.to_string()),
            InternalResult::More => Err(()),
            InternalResult::Error(_processing_error) => Err(()),
        }
    }
}

mod utf8 {
    //! need utf8 parsing capability, as the tokenizer work on bytes slice
    //! but the utf8 module of rust is private / ongoing stabilisation,
    //! so borrow the table and the utf8_char_width function to
    //! write our own parser

    // * https://tools.ietf.org/html/rfc3629
    // * accessible tweaked copy of https://doc.rust-lang.org/src/core/str/validations.rs.html#246
    const UTF8_CHAR_WIDTH: [u8; 256] = [
        // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
        0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
        4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
    ];

    pub const fn utf8_char_width(b: u8) -> usize {
        UTF8_CHAR_WIDTH[b as usize] as usize
    }

    pub const fn is_cont(v: u8) -> bool {
        const CONT_MASK: u8 = 0b1100_0000;
        const CONT_ESEQ: u8 = 0b1000_0000;
        (v & CONT_MASK) == CONT_ESEQ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        const TESTS: &[&str] = &["A", "Ǝ", "ᛔ", "ꪄ", "AƎᛔꪄ", "AƎᛔꪄAƎᛔꪄ", "AƎᛔAƎᛔꪄꪄƎᛔꪄAƎᛔꪄ"];

        for test in TESTS {
            let bytes = test.bytes().collect::<Vec<_>>();
            let max = bytes.len();
            for chunk_size in 1..max {
                let mut out = String::new();
                let mut output = OutputUtf8::new();

                for b in bytes.chunks(chunk_size) {
                    match output.process(b) {
                        Res::Ok(s) => {
                            out.push_str(&s);
                        }
                        Res::More => {}
                        Res::Error(err) => {
                            panic!("got error in valid stream : {:?}", err);
                        }
                    }
                }

                match output.finalize() {
                    Ok(s) => out.push_str(&s),
                    Err(_) => panic!("should not error"),
                };

                assert_eq!(&out, test)
            }
        }
    }
}
