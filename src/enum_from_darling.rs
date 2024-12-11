use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident};

#[derive(Debug, FromDeriveInput)]
pub struct EnumFromDarling {
    ident: Ident,
    generics: Generics,
    data: Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
pub struct EnumVariants {
    ident: Ident,
    fields: Fields<EnumVariantFields>,
}

#[derive(Debug, FromField)]
pub struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };
    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should be 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            #ident::#var(value)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! { #(#from_impls)* }
}
