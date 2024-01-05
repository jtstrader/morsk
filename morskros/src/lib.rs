extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, BinOp, Ident, LitInt, LitStr, Result,
};

enum MvhType {
    Inclusive,
    Exclusive,
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

        match mvh_type {
            BinOp::BitOr(_) => Ok(MvhInput {
                input_word,
                variadic_hex,
                ty: MvhType::Inclusive,
            }),
            BinOp::BitXor(_) => Ok(MvhInput {
                input_word,
                variadic_hex,
                ty: MvhType::Exclusive,
            }),
            _ => Err(syn::Error::new(
                mvh_type.span(),
                "only '|' and '^' are supported for mvh",
            )),
        }
    }
}

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
        input.parse()
    } else {
        Err(lookahead.error())
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
/// use morsk::{mvh, Word};
///
/// let w16 = Word::from(0xABCD_u16);
/// let w32 = Word::from(0xAAB1249E_u32);
///
/// assert!(mvh!(w16 | 0xABXY));
/// assert!(mvh!(w32 | "0xXXB12349Y"));
/// ```
///
/// Using exclusive matching with `mvh.
#[proc_macro]
pub fn mvh(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MvhInput);

    let (word, hex) = (input.input_word, input.variadic_hex);
    match input.ty {
        MvhType::Inclusive => quote!(#word.inclusive_morsk(#hex)).into(),
        MvhType::Exclusive => quote!(#word.exclusive_morsk(#hex)).into(),
    }
}
