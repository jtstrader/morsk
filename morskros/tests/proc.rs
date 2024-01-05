use morsk::Word;
use morskros::mvh;

#[test]
fn inclusive_morsk() {
    let w = Word::from(0xABCD_u16);
    assert!(mvh!(w | 0xAXYD));
}

#[test]
fn exclusive_morsk() {
    let w = Word::from(0xABBD_u16);
    assert!(mvh!(w ^ 0xAXXD));
}
