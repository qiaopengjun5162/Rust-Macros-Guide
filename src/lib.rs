// proc macro crate

use proc_macro::TokenStream;
use quote::quote;

// For enums, we want to generate From implementations for each variant.
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("input: {:#?}", input);
    // get the ident
    let ident = input.ident;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be used with enums"),
    };
    // for each variant, generate a From implementation
    let variant_from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one unnamed field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("only one unnamed field");
                    let ty = &field.ty;
                    quote! {
                                impl From<#ty> for #ident {
                                    fn from(value: #ty) -> Self {
                                        #ident::#var(value)
                                    }
                        }
                    }
                }
            }
            syn::Fields::Named(_fields) => quote! {},
            syn::Fields::Unit => quote! {},
        }
    });
    // quote return proc-macro2 TokenStream so we need to convert it to TokenStream
    quote! {
        #(#variant_from_impls)*
    }
    .into()
}
