use std::ops::Deref;

pub struct Word {
    word: u16,
    hex_digits: [u8; 4],
}

impl Word {
    pub fn new(word: u16) -> Self {
        Self {
            word,
            hex_digits: get_hex(word),
        }
    }
}

impl Deref for Word {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.word
    }
}

/// Get the hex digits of a [`u16`].
fn get_hex(w: u16) -> [u8; 4] {
    let digit_1 = (w & 0xF000) >> 12;
    let digit_2 = (w & 0x0F00) >> 8;
    let digit_3 = (w & 0x00F0) >> 4;
    let digit_4 = w & 0x000F;

    [digit_1, digit_2, digit_3, digit_4].map(|digit| digit as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_from_u16() {
        let w = 0xABCD;
        let digits = get_hex(w);
        assert_eq!(digits, [0xA, 0xB, 0xC, 0xD]);
    }

    #[test]
    fn word_struct_from_u16() {
        let w = Word::new(0xABCD);
        assert_eq!(w.word, 0xABCD);
        assert_eq!(w.hex_digits, [0xA, 0xB, 0xC, 0xD]);
    }

    #[test]
    fn deref_word() {
        let w = Word::new(0xABCD);
        let new_w = *w + 1;
        assert_eq!(new_w, 0xABCE);
    }
}
