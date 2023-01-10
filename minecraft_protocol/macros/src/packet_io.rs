use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error, LitInt, Token, Type};

pub struct PacketIO {
    pub(crate) generic: Type,
    pub(crate) variants: Punctuated<PacketType, Token![,]>,
}

pub struct PacketType {
    pub(crate) id: LitInt,
    pub(crate) type_name: Type,
    pub(crate) g_var: Ident,
}

pub type Colon = syn::Token![:];

impl Parse for PacketType {
    /// Parses a packet type  Example:
    /// ```no_compile
    /// 1 => {
    ///    type_name: ClientBoundEncryptionRequestImpl,
    ///    g_var: EncryptionRequest
    /// }
    /// ```
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id = input.parse::<LitInt>()?;
        input.parse::<Token![=>]>()?;
        let content;
        syn::braced!(content in input);
        let type_name = {
            let key = content.parse::<Ident>()?;
            if key != "type_name" {
                return Err(Error::new(key.span(), "Expected type_name"));
            }
            content.parse::<Colon>()?;
            let type_name = content.parse::<Type>()?;
            type_name
        };
        let g_var = {
            let key = content.parse::<Ident>()?;
            if key != "g_var" {
                return Err(Error::new(key.span(), "Expected g_var"));
            }
            content.parse::<Colon>()?;
            content.parse::<Ident>()?
        };
        Ok(PacketType {
            id,
            type_name,
            g_var,
        })
    }
}

impl Parse for PacketIO {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generic = input.parse::<Type>()?;
        let content;
        syn::braced!(content in input);
        let variants = content.parse_terminated(PacketType::parse)?;
        Ok(PacketIO { generic, variants })
    }
}

pub fn handle(io: PacketIO) -> Result<TokenStream, Error> {
    let PacketIO { generic, variants } = io;
    let mut writes: Vec<TokenStream> = Vec::with_capacity(variants.len());
    let mut reads: Vec<TokenStream> = Vec::with_capacity(variants.len());
    variants.iter().for_each(|v| {
        let ident = &v.g_var;
        let type_name = &v.type_name;
        let id = &v.id;
        writes.push(quote! {
            #generic::#ident(value) =>{
                <#type_name as Packet>::write(value, writer)?;
            }
        });
        reads.push(quote! {
            #id => {
                let value = <#type_name as Packet>::read_with_length(reader,len)?;
                Ok(#generic::#ident(value))
            }
        });
    });

    Ok(quote! {
        #[derive(Debug)]
        pub struct PacketIOImpl;
        impl PacketIO for PacketIOImpl {
            type Type = #generic;
            fn handle_read<R: Read>(packed_id: i32, len: usize, reader: &mut R) -> Result<Self::Type, PacketReadError>{
                match packed_id {
                    #(#reads),*
                    v => Err(PacketReadError::UnknownPacketId(v))
                }
            }
            fn handle_write<W: Write>(packet: Self::Type, writer: &mut W) -> Result<(), PacketWriteError>{
                match packet {
                    #(#writes)*
                    _ => {
                        return Err(PacketWriteError::InvalidPacketType);
                    }
                }
                Ok(())
            }
        }
    })
}
