use darling::{ast::Data, util::Ignored, FromDeriveInput};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Field, Ident, Path};

#[derive(FromDeriveInput)]
#[darling(attributes(duplicated))]
struct Options {
    ident: Ident,
    data: Data<Ignored, Field>,

    target: Path,
}

pub fn handle(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token);
    let options = Options::from_derive_input(&input).expect("Wrong options");

    let struct_name = options.ident;
    let fields = options
        .data
        .take_struct()
        .expect("Only struct is supported");

    let target = options.target;

    let copy_from = fields.iter().map(|field| {
        let field_name = field.ident.clone();
        quote! {
            #field_name: other.#field_name,
        }
    });

    let copy_into = fields.iter().map(|field| {
        let field_name = field.ident.clone();
        quote! {
            #field_name: self.#field_name,
        }
    });

    let output = quote! {
        impl From<#target> for #struct_name {
            fn from(other: #target) -> Self {
                Self {
                    #(#copy_from)*
                }
            }
        }

        impl Into<#target> for #struct_name {
            fn into(self) -> #target {
                #target {
                    #(#copy_into)*
                }
            }
        }
    };
    output.into()
}
