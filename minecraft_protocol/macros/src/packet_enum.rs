use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, LitStr, Type};

/// Will parse the repr attribute
/// ```no_compile
/// #[repr(u8)]
/// ```
pub struct Represents(pub Type);

impl Parse for Represents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Type::parse(input).map(Represents)
    }
}
impl ToTokens for Represents {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}
/// The type must implement From<PacketType> into Represents
/// ```no_compile
/// #[packet_type(u8)]
/// ```
pub struct PacketType(pub Type);

impl Parse for PacketType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Type::parse(input).map(PacketType)
    }
}
impl ToTokens for PacketType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}
/// Will parse the packet attribute
/// #[error("Invalid packet id")]
pub struct ErrorMessage(pub LitStr);
impl Parse for ErrorMessage {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        <LitStr as Parse>::parse(input).map(ErrorMessage)
    }
}
impl ToTokens for ErrorMessage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub fn parse_packet_enum(input: DeriveInput) -> syn::Result<TokenStream> {
    let  Data::Enum(ref data) = input.data else {
        return Err(Error::new(input.span(), "Expected enum"))
    };
    let enum_ident = &input.ident;
    let error_name = format_ident!("{}Error", input.ident);

    let inner_type = match input.attrs.iter().find(|attr| attr.path.is_ident("repr")) {
        Some(attr) => attr.parse_args::<Represents>()?,
        None => return Err(Error::new(input.span(), "Expected #[repr(i32)]")),
    };
    let error: ErrorMessage = match input.attrs.iter().find(|attr| attr.path.is_ident("error")) {
        Some(attr) => attr.parse_args::<ErrorMessage>()?,
        None => {
            return Err(Error::new(
                input.span(),
                "Expected #[error(\"Invalid Data Type {}\")]",
            ))
        }
    };
    let packet_type = match input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("packet_type"))
    {
        Some(attr) => Some(attr.parse_args::<PacketType>()?),
        None => None,
    }
    .map(|packet_type| {
        quote! {
                impl PacketDataType for #enum_ident {
                    fn read<R: std::io::Read>(buf: &mut R) -> std::io::Result<Self>
                        where
                            Self: Sized,
                    {
                        let packet_type: #inner_type = <#packet_type as PacketDataType>::read(buf)?.into();
                        Self::try_from(packet_type).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, #error))
                    }
                    fn write<W: std::io::Write>(self, write: &mut W) -> std::io::Result<()>
                        where
                            Self: Sized,
                    {
                        let packet_type = (self as #inner_type).into();
                        <#packet_type as PacketDataType>::write(packet_type, write)?;
                        Ok(())
                    }
                }
            }
    });
    let mut variants = Vec::new();
    for variant in &data.variants {
        let variant_name = &variant.ident;
        let Some((_, value)) = &variant.discriminant else {
            return Err(Error::new(variant.span(), "Expected discriminant"))
        };

        variants.push(quote! {
            #value => Ok(#enum_ident::#variant_name)
        })
    }
    let result = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct #error_name(pub #inner_type);
        impl std::fmt::Display for #error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #error, self.0)
            }
        }
        impl std::error::Error for #error_name {}

        impl std::convert::TryFrom<#inner_type> for #enum_ident {
            type Error = #error_name;
            fn try_from(value: #inner_type) -> Result<Self, Self::Error> {
                match value {
                    #(#variants,)*
                    _ => Err(#error_name(value)),
                }
            }
        }
        // If packet_type is defined, implement PacketDataType
        #packet_type

    };
    return Ok(result);
}
