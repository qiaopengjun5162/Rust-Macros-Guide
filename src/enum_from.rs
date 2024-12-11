use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    // get the ident
    let ident = input.ident;
    // get the generics
    let generics = input.generics;
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
                                impl #generics  From<#ty> for #ident #generics  {
                                    fn from(value: #ty) -> Self {
                                        #ident::#var(value)
                                    }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    // quote return proc-macro2 TokenStream so we need to convert it to TokenStream
    quote! { #(#variant_from_impls)* }
}
