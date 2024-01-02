use crate::Word;

pub(crate) trait Morskable<const DIGIT_COUNT: usize>: Sized {
    fn hex_digits(&self) -> [u8; DIGIT_COUNT];
}

macro_rules! impl_morskable {
    ([$(($t:ty, $digit_count:expr)),*]) => {
        $(impl Morskable<$digit_count> for $t {
            fn hex_digits(&self) -> [u8; $digit_count] {
                let mut digits = [0_u8; $digit_count];
                let mut start: i32 = 4 * ($digit_count - 1);
                digits.iter_mut().for_each(|digit| {
                    *digit = ((self & ((0xF as $t) << start)) >> start) as u8;
                    start -= 4;
                });
                digits
            }
        })*
    };
}

macro_rules! impl_morsk_froms {
    ([$(($t:ty, $size:expr)),*]) => {
        $(impl From<$t> for Word<$t, $size> where $t: Morskable<$size> {
            fn from(value: $t) -> Self {
                Self {
                    word: value,
                    hex_digits: value.hex_digits(),
                }
            }
        })*
    };
}

impl_morskable!([(u8, 2), (u16, 4), (u32, 8), (u64, 16), (u128, 32)]);
impl_morsk_froms!([(u8, 2), (u16, 4), (u32, 8), (u64, 16), (u128, 32)]);

#[cfg(test)]
mod tests {
    use super::*;

    mod morskable {
        use super::*;

        #[test]
        fn morskable_u8() {
            let word: u8 = 0xAB;
            assert_eq!(word.hex_digits(), [0xA, 0xB]);
        }

        #[test]
        fn morskable_u16() {
            let word: u16 = 0xABCD;
            assert_eq!(word.hex_digits(), [0xA, 0xB, 0xC, 0xD]);
        }

        #[test]
        fn morskable_u32() {
            let word: u32 = 0x12345678;
            assert_eq!(word.hex_digits(), [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8]);
        }

        #[test]
        fn morskable_u64() {
            let word: u64 = 0x123456789ABCDEF0;
            assert_eq!(
                word.hex_digits(),
                [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0]
            );
        }

        #[test]
        fn morskable_u128() {
            let word: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
            assert_eq!(word.hex_digits(), [0xF; 32]);
        }
    }

    mod morsk_froms {
        use super::*;

        #[test]
        fn word_from_u8() {
            let word = Word::from(0xAB_u8);
            assert_eq!(word.word, 0xAB);
            assert_eq!(word.hex_digits, [0xA, 0xB]);
        }

        #[test]
        fn word_from_u16() {
            let word = Word::from(0xABCD_u16);
            assert_eq!(word.word, 0xABCD);
            assert_eq!(word.hex_digits, [0xA, 0xB, 0xC, 0xD]);
        }

        #[test]
        fn word_from_u32() {
            let word = Word::from(0x12345678_u32);
            assert_eq!(word.word, 0x12345678);
            assert_eq!(word.hex_digits, [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8]);
        }

        #[test]
        fn word_from_u64() {
            let word = Word::from(0x123456789ABCDEF0_u64);
            assert_eq!(word.word, 0x123456789ABCDEF0);
            assert_eq!(
                word.hex_digits,
                [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0]
            );
        }

        #[test]
        fn word_from_u128() {
            let word = Word::from(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF_u128);
            assert_eq!(word.word, u128::MAX);
            assert_eq!(word.hex_digits, [0xF; 32]);
        }
    }
}
