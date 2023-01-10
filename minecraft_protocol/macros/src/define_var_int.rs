use syn::parse::{Parse, ParseStream};
use syn::LitInt;

pub struct VarInt {
    pub(crate) value: LitInt,
}

impl Parse for VarInt {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse::<LitInt>()?;
        Ok(VarInt { value })
    }
}
