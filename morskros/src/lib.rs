extern crate proc_macro;
use proc_macro::TokenStream;
use std::convert::{TryFrom, TryInto};

use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, BinOp, Ident, LitInt, LitStr, Result,
};

enum MvhType {
    Inclusive,
    Exclusive,
    Single,
}

impl TryFrom<BinOp> for MvhType {
    type Error = syn::Error;

    fn try_from(value: BinOp) -> std::result::Result<Self, Self::Error> {
        match value {
            BinOp::BitOr(_) => Ok(MvhType::Inclusive),
            BinOp::BitXor(_) => Ok(MvhType::Exclusive),
            BinOp::BitAnd(_) => Ok(MvhType::Single),
            _ => Err(syn::Error::new(
                value.span(),
                "only the '|', '^', and '&' operators are supported",
            )),
        }
    }
}

struct MvhInput {
    input_word: Ident,
    variadic_hex: LitStr,
    ty: MvhType,
}

impl Parse for MvhInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if !lookahead.peek(Ident) {
            return Err(lookahead.error());
        }

        let input_word: Ident = input.parse()?;
        let mvh_type: BinOp = input.parse()?;

        // Attempt to parse the variadic hex as an Integer. If this cannot be done,
        // parse it instead as a LitString which is the only other supported type.
        let variadic_hex: LitStr = extract_variadic_hex(input)?;

        Ok(MvhInput {
            input_word,
            variadic_hex,
            ty: mvh_type.try_into()?,
        })
    }
}

/// Extract the variadic hex string or integer literal from the macro input. Error on
/// invalid type or invalid string (e.g., `0xAB0+_-4XY` is not parsable).
fn extract_variadic_hex(input: ParseStream) -> Result<LitStr> {
    let lookahead = input.lookahead1();
    if lookahead.peek(LitInt) {
        let parsed_input: LitInt = input.parse()?;
        let span = parsed_input.span();

        // Digits: Anything before strange/erroneous characters such as _, X, Y, etc.
        // Suffix: Anything after strange/erroneous character (cannot be first character)
        //
        // Example: 0xFX13 -> 0xF is digits, X13 is suffix
        Ok(LitStr::new(
            &format!("{}{}", parsed_input.base10_digits(), parsed_input.suffix()),
            span,
        ))
    } else if lookahead.peek(LitStr) {
        validate_hex(input.parse()?)
    } else {
        Err(lookahead.error())
    }
}

/// Validate a provided LitStr to ensure it is a valid hex string. Error on invalid hex input.
fn validate_hex(hex: LitStr) -> Result<LitStr> {
    // Ensure that string begins with 0x for signifying hex.
    let value = hex.value();
    if value.len() < 2 || value[0..2] != *"0x" {
        return Err(syn::Error::new(
            hex.span(),
            "provided hex did not begin with '0x'",
        ));
    } else if value.len() < 3 {
        return Err(syn::Error::new(
            hex.span(),
            "provided hex has no value, only prefix '0x'",
        ));
    }

    // Non-alphanumeric chars.
    match value.chars().all(char::is_alphanumeric) {
        true => Ok(hex),
        false => Err(syn::Error::new(
            hex.span(),
            "provided hex included non-alphanumeric characters",
        )),
    }
}

/// A macro for matching variadic hex.
///
/// This macro takes a word on the left-hand side and a hexadecimal on the right hand side. The
/// hexadecimal can contain any alphanumeric character. Any character used that is not in Base16 will
/// be treated as a *wildcard*, and will be matched with any other of the same wildcard. `mvh`, like
/// other Morsk matching macros, support three modes: inclusive, exclusive, and single wildcare matching.
///   - Inclusive matching (`word | HEX`) allows two or more unique wildcards to represent the same
///     value in hex.
///   - Exclusive matching (`word ^ HEX`) rejects two or more unique wildcards attempting to represent
///     the same value in hex.
///   - Single matching (`word & HEX`) allows only one wildcard to be used, but each instance
///     of the wildcard can represent any value.
///
/// Details of how these matches work can be found under the documentation on the [`Word`](morsk::Word)
/// struct.
///
/// ## Caveats
/// Due to Rust's lexicographical grammar, matching on `0xX...` for any non-hex character `X` is invalid syntax,
/// even for procedural macros. Therefore, if you want to match the left-most bit, you will need to put it in
/// quotes. In any other case, you can use your wildcards wherever while writing the number as a series of hex
/// digits (`0xABXYZDEF1234`).
///
/// # Examples
///
/// Using inclusive matching with `mvh`.
/// ```
/// use morsk::{mvh, Word};
///
/// let w16 = Word::from(0xABCD_u16);
/// let w32 = Word::from(0xAAB1249E_u32);
///
/// assert!(mvh!(w16 | 0xABXY));
/// assert!(mvh!(w32 | "0xXXB12349Y"));
/// ```
///
/// Using exclusive matching with `mvh`.
#[proc_macro]
pub fn mvh(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MvhInput);
    let (word, hex) = (input.input_word, input.variadic_hex);
    match input.ty {
        MvhType::Inclusive => quote!(#word.inclusive_morsk(#hex)).into(),
        MvhType::Exclusive => quote!(#word.exclusive_morsk(#hex)).into(),
        MvhType::Single => quote!(#word.single_morsk(#hex)).into(),
    }
}
