use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

use crate::define_var_int::VarInt;
use crate::packet_io::PacketIO;

pub(crate) mod define_var_int;
pub(crate) mod packet_enum;
pub(crate) mod packet_io;

/// Defines a byte array to represent a var int. This s good for PacketID's to prevent converting at runtime
///
///
/// # Example
/// ```rust
/// use minecraft_protocol_macros::define_var_int;
///
/// assert_eq!(define_var_int!(0x00), [0x00]);
/// assert_eq!(define_var_int!(0x01), [0x01]);
/// assert_eq!(define_var_int!(154), [0x9A, 0x01]);
/// assert_eq!(define_var_int!(255), [0xFF, 0x01]);
/// ```
#[proc_macro]
pub fn define_var_int(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as VarInt);
    let x: i32 = input.value.base10_parse().unwrap();
    let mut bytes = Vec::with_capacity(4);
    let mut value = x;

    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
    let v = quote! {
        [ #(#bytes),* ]
    };

    v.into()
}

#[proc_macro]
pub fn define_io(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as PacketIO);
    match packet_io::handle(input) {
        Ok(ok) => ok.into(),
        Err(err) => Error::new(err.span(), err.to_string())
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_derive(PacketImplDebug)]
pub fn packet_impl_debug(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let v = quote! {
        #[automatically_derived]
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{name:?} Protocol: {protocol:?} State: {stage:?} ID: {id:?}",
                    name = stringify!(#name),
                    protocol = <#name as Packet>::protocol(),
                    stage = <#name as Packet>::stage(),
                    id = <#name as Packet>::packet_id(),
                )
            }
        }
    };
    v.into()
}

#[proc_macro_derive(PacketContentType)]
pub fn packet_content_type(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let v = quote! {
        #[automatically_derived]
        impl PacketContent for #name {}
    };
    v.into()
}
///
/// This will add a try_from, an error type, and a read/write impl for the enum
/// # Example
/// ```rust, no_compile
/// use minecraft_protocol_macros::{PacketEnum};
/// #[derive(Debug, Clone, PartialEq, PacketEnum)]
/// #[packet_type(VarInt)]
/// #[repr(i32)]
/// #[error("Invalid Main Hand {}")]
/// pub enum MainHand {
///     Left = 0,
///     Right = 1,
/// }
/// ```
#[proc_macro_derive(PacketEnum, attributes(error, packet_type))]
pub fn packet_enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    packet_enum::parse_packet_enum(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
