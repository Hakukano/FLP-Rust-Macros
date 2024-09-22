use std::collections::HashMap;

use darling::{
    ast::Data,
    util::{Ignored, PathList},
    FromDeriveInput, FromField,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Type, Visibility};

#[derive(FromField)]
#[darling(attributes(partial))]
struct Field {
    ident: Option<Ident>,
    vis: Visibility,
    ty: Type,

    #[darling(default)]
    included: PathList,
}

#[derive(FromDeriveInput)]
#[darling(attributes(partial))]
struct Options {
    vis: Visibility,
    data: Data<Ignored, Field>,

    #[darling(default)]
    structs: PathList,

    #[darling(default)]
    derives: PathList,
}

pub fn handle(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token);
    let options = Options::from_derive_input(&input).expect("Wrong options");

    let struct_vis = options.vis;

    let fields = options
        .data
        .take_struct()
        .expect("Only struct is supported");

    let struct_fields_map = fields.into_iter().fold(
        HashMap::<String, Vec<proc_macro2::TokenStream>>::new(),
        |mut acc,
         Field {
             ident,
             vis,
             ty,
             included,
         }| {
            included.iter().for_each(|new_struct| {
                let new_struct = new_struct
                    .get_ident()
                    .expect("Only ident is supported for included")
                    .to_string();
                let ident = ident.clone().expect("Ident is needed for the field");
                let field = quote! {
                    #vis #ident: #ty,
                };
                if let Some(v) = acc.get_mut(&new_struct) {
                    v.push(field);
                } else {
                    acc.insert(new_struct, vec![field]);
                }
            });
            acc
        },
    );

    let new_structs = options.structs.iter().map(|name| {
        let derives = options
            .derives
            .iter()
            .map(|derive| {
                quote! {
                    #derive,
                }
            })
            .collect::<Vec<_>>();
        let derives = if derives.is_empty() {
            quote! {}
        } else {
            quote! {
                #[derive(#(#derives)*)]
            }
        };
        let k = name
            .get_ident()
            .expect("Only ident is supported for structs")
            .to_string();
        let v = Vec::new();
        let fields = struct_fields_map.get(&k).unwrap_or(&v);
        quote! {
            #derives
            #struct_vis struct #name {
                #(#fields)*
            }
        }
    });

    let output = quote! {
        #(#new_structs)*
    };
    output.into()
}
