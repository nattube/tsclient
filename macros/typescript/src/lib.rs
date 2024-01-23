#[macro_use]
extern crate darling;

use std::collections::HashSet;

use darling::FromMeta;
use inner::{parse_object, parse_newtype, parse_tuple};
use proc_macro::{TokenStream};
use proc_macro2::{TokenTree, Delimiter};
use proc_macro_error::{abort_call_site, abort, proc_macro_error};
use syn::{Item as SynItem, parse_macro_input, Attribute, Meta, punctuated::Punctuated, parse::Parser as _, parse_quote, Path, ItemStruct, Index, ItemEnum, Fields, DeriveInput, DataStruct, DataEnum};
use quote::{quote, ToTokens, format_ident};

mod inner;


#[derive(Default, Debug, FromMeta)] 
struct SerdeEnumTag {
    tag: Option<String>,
    content: Option<String>,
    untagged: Option<bool>,
}

#[proc_macro_error]
#[proc_macro_derive(TypeScriptStrict, attributes(serde))]
pub fn ts_strict(item: TokenStream,) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let holder = parse_quote!(::tsclient::types::TypeHolderStrict);

    ts_internal(input.data, input.generics, input.ident, input.attrs, holder)
}

#[proc_macro_error]
#[proc_macro_derive(TypeScript, attributes(serde))]
pub fn ts(item: TokenStream,) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let holder = parse_quote!(::tsclient::types::TypeHolder);

    ts_internal(input.data, input.generics, input.ident, input.attrs, holder)
}

fn ts_internal(parse: syn::Data, generics: syn::Generics, ident: syn::Ident, attrs: Vec<Attribute>, holder: syn::Path) -> TokenStream {
    let mut hashes = Vec::new();

    let serde_att = attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .map(|attr| SerdeEnumTag::from_meta(&attr.meta) )
        .find(|x| x.is_ok())
        .and_then(|x| Some(x.unwrap()));


    let repr: syn::Expr = match serde_att {
        Some( SerdeEnumTag { tag: Some(t), content: Some(c), .. }) => {
            parse_quote!(::tsclient::types::model::EnumRepresentation::Adjacently(#t.to_string(), #c.to_string()))
        },
        Some( SerdeEnumTag { tag: Some(t), content: None, .. }) => {
            parse_quote!(::tsclient::types::model::EnumRepresentation::Internally(#t.to_string()))
        },
        Some( SerdeEnumTag { tag: None, content: None, untagged: Some(true) }) => {
            parse_quote!(::tsclient::types::model::EnumRepresentation::Untagged)
        },
        _ => parse_quote!(::tsclient::types::model::EnumRepresentation::Default),
    };

    let (ident, typ, hash_lit) = match parse {
        syn::Data::Struct(mut x) => {
            hashes.extend(parse_hash_of_fields(x.fields.clone(), holder.clone()));
            (ident.clone(), parse_struct(x, holder), "Struct")
        },
        syn::Data::Enum(mut x) => {
            for variant in x.variants.iter() {
                hashes.extend(parse_hash_of_fields(variant.fields.clone(), holder.clone()));
            }

            (ident.clone(),parse_enum(x, holder, repr), "Enum")
        },
        _ => abort_call_site!("Only enums and structs can derive. Unions are not supported"),
    };

    let id_name = ident.to_string();

    let gen_inner = generics.params.iter();
    
    let output = quote! {

        impl <#(#gen_inner: ::tsclient::types::TypescriptType + 'static,)*> ::tsclient::types::TypescriptType for #ident #generics {
            fn get_definition(registry: &mut ::tsclient::types::builder::GlobalTypeRegistry) -> ::tsclient::types::builder::HasIndexed {
                let type_id = ::std::any::TypeId::of::<Self>();
                if let Some(existing) = registry.return_existing(type_id) {
                    return existing
                }
                registry.start(type_id);

                let hash = Self::hash(registry);
                let typ = #typ;

                let component = ::tsclient::types::model::Component {
                    name: #id_name.to_string(),
                    typ,
                    hash
                };

                return registry.finalize(type_id, component)
            }
            fn name() -> ::std::string::String {
                String::from(#id_name)
            }
            fn ts_name() -> ::std::string::String {
                String::from(#id_name)
            }
            fn hash(registry: &mut ::tsclient::types::builder::GlobalTypeRegistry) -> ::std::primitive::u64 {
                let type_id = ::std::any::TypeId::of::<Self>();

                if let Some(h) = registry.start_hash(type_id) {
                    return h
                }

                let mut __hash_block = ::std::vec::Vec::new();
                #(#hashes)*

                let mut __hasher = ::std::hash::DefaultHasher::new();
                __hash_block.sort();
                <::std::primitive::str as ::std::hash::Hash>::hash(#hash_lit, &mut __hasher);
                <::std::vec::Vec<::std::primitive::u64> as ::std::hash::Hash>::hash(&__hash_block, &mut __hasher);

                let hash = <::std::hash::DefaultHasher as ::std::hash::Hasher>::finish(&__hasher);
                registry.finalize_hash(type_id, hash);

                return hash;
            }
        }
    };

    output.into()
}

fn parse_hash_of_fields(fields: Fields, holder: syn::Path) -> Vec<syn::Block> {
    let res = fields.iter().enumerate().map(|(i, field)| {
        let ty = field.ty.clone();
        parse_quote![{
            let field_hash = (&mut &#holder::<#ty>::new()).hash(registry);

            __hash_block.push(field_hash);
        }]
    }).collect::<Vec<syn::Block>>();

    return res;
}

fn parse_enum(item: DataEnum, holder: syn::Path, repr: syn::Expr) -> syn::Block { 
    let mut blocks: Vec<syn::Block> = Vec::new();

    for variant in item.variants {
        let ident = variant.ident.to_string();

        let parsed = match variant.fields {
            syn::Fields::Named(named) => parse_object(named, holder.clone()),
            syn::Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
                0 => parse_quote!({::tsclient::types::model::InnerType::SimpleVariant(#ident.to_string())}),
                1 => parse_newtype(unnamed.unnamed[0].clone(), holder.clone()),
                _ => parse_tuple(unnamed, holder.clone())
            },
            syn::Fields::Unit => parse_quote!({::tsclient::types::model::InnerType::SimpleVariant(#ident.to_string())})
        };

        blocks.push(parse_quote!({
            let inner = #parsed;

            __block.push((#ident.to_string(), inner));
        }));
    }

    parse_quote!({
        let mut __block = ::std::vec::Vec::new();
        #(#blocks)*

        ::tsclient::types::model::Type::Enum(#repr, __block)
    })
}

fn parse_struct(item: DataStruct, holder: syn::Path) -> syn::Block { 
    let inner_typ = match item.fields {
        syn::Fields::Named(named) => parse_object(named, holder),
        syn::Fields::Unnamed(unnamed) => match unnamed.unnamed.len() {
            0 => todo!(),
            1 => parse_newtype(unnamed.unnamed[0].clone(), holder),
            _ => parse_tuple(unnamed, holder)
        }
        syn::Fields::Unit => parse_quote!(::tsclient::types::model::InnerType::Null),
        
    };

    parse_quote!({
        let inner = #inner_typ;

        ::tsclient::types::model::Type::Struct(inner)
    })
}
