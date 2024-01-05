# Morsk - Match or Skip

Morsk is a matching library for collections of bits designated as words. A word is an unsigned integer, and can be
matched against strings of *variadic input*. Morsk also supports wildcards in the matching, and returns a result from
the matching that can be used to extract the desired values.

```rust
use morsk::{self, Word};

fn main() -> morsk::Result<()> {
    let w = Word::from(0xAF25_u16);

    // Check to see if the provided word matches any hex
    // with an A at the beginning and a 5 at the end.
    let match_res = mvh!(w | 0xAXY5)?;
    if let Some(res) = match_res {
        println!("{}", res['X']); // F
        println!("{}", res['Y']); // 2
    }
}
```

The primary intent of this library is to solve problems with working with low level systems that require the extraction
of information from words of data. In writing things such as emulators, it's common to have instructions with op-codes
and "parameters" for these operations in the word itself. For instance, in
the [Chip8 instruction set](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table) there are multiple instructions such as
`4XNN` for skipping instructions that require the extraction of the `X` and `NN` bits. This library attempts to offer
convenience for operations such as this where you want to match on a certain type of word and extract out values that
are relevant to the next set of operations that need to be completed.