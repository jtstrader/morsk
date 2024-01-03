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

impl<T, const HEX_DIGITS: usize> Word<T, HEX_DIGITS>
where
    T: Morskable<HEX_DIGITS>,
{
    pub fn morsk(&self, pattern: &str) -> bool {
        // Create table of values and

        for c in pattern.chars() {}
        true
    }
}

impl<T, const HEX_DIGITS: usize> Deref for Word<T, HEX_DIGITS>
where
    T: Morskable<HEX_DIGITS>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.word
    }
}

/// `mvh` - Match On Variadic Hex
///
/// This macro takes a word on the left-hand side and a hexadecimal on the right hand side. The
/// hexadecimal can contain any alphanumeric character. Any character used that is not in Base16 will
/// be treated as a *wildcard*, and will be matched with any other of the same wildcard. `mvh`, like
/// other Morsk matching macros, support two modes: inclusive and exclusive matching.
///   - Inclusive matching (`word | HEX`) allows two or more unique wildcards to represent the same
///     value in hex.
///   - Exclusive matching (`word ^ HEX`) rejects two or more unique wildcards attempting to represent
///     the same value in hex.
///
/// # Examples
///
/// Using inclusive matching with `mvh`.
/// ```
/// use morsk::{Word, mvh};
///
/// let w16 = Word::from(0xABCD_u16);
/// let w32 = Word::from(0xAAB1249E_u32);
///
/// assert!(mvh!(w16 | 0xABXY));
/// assert!(mvh!(w32 | 0xXXB12349Y));
/// ```
///
/// Using exclusive matching with `mvh.
#[macro_export]
macro_rules! mvh {
    /* Inclusive matching. */
    ($word:ident | $pattern:expr) => {
        $word.morsk(stringify!($pattern))
    };

    /* Exclusive matching. */
    ($word:ident ^ $pattern:expr) => {
        $word.morsk(stringify!($pattern))
    };
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

    #[test]
    fn morsk_u8() {
        let w = Word::from(0xAB_u8);

        // Instruction - Clear register X
        // 0xAA3X -> X ranges from 0-F

        if mvh!(w | 0xAA3X) {}

        // assert!(mvh!(w | 0xXA));
    }
}
