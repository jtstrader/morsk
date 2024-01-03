pub mod traits;

use traits::Morskable;

use std::ops::Deref;

/// A representation of a value that exposes the individual hex digits of the word and allows matching portions of the
/// word rather than the entire word (i.e., 0x9XX9 should match any word with 9s on the beginning and end of the word).
pub struct Word<T, const HEX_DIGITS: usize>
where
    T: Morskable<HEX_DIGITS>,
{
    word: T,
    hex_digits: [u8; HEX_DIGITS],
}

impl<T, const HEX_DIGITS: usize> Word<T, HEX_DIGITS> where T: Morskable<HEX_DIGITS> {}

impl<T, const HEX_DIGITS: usize> Deref for Word<T, HEX_DIGITS>
where
    T: Morskable<HEX_DIGITS>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_struct_from_u16() {
        let w = Word::from(0xABCD_u16);
        assert_eq!(w.word, 0xABCD);
        assert_eq!(w.hex_digits, [0xA, 0xB, 0xC, 0xD]);
    }

    #[test]
    fn deref_word() {
        let w = Word::from(0xABCD_u16);
        let new_w = *w + 1;
        assert_eq!(new_w, 0xABCE);
    }
}
