use quote::{format_ident, ToTokens as _};
use syn::{parse_quote, Fields, FieldsNamed, Field, FieldsUnnamed};

pub(crate) fn parse_tuple(fields: FieldsUnnamed, holder: syn::Path) -> syn::Block {
    let block = fields.unnamed.iter().enumerate().map(|(i, field)| {
        let ident = format_ident!("d{}", i);
        let ty = field.ty.clone();
        let field_type_name: String = field.ty.to_token_stream().to_string().split_whitespace().collect();
        
        parse_quote![{
            let #ident = (&mut &#holder::<#ty>::new()).get_definition(registry);
            let type_name: ::std::string::String = (&mut &#holder::<#ty>::new()).name().split_whitespace().collect();

            let renamed = if &type_name == "any" || #field_type_name == &type_name {
                None
            } else {
                Some(String::from(#field_type_name))
            };

            let comp = ::tsclient::types::model::ComponentReference {
                id: #ident,
                renamed
            };

            __block.push(comp);
        }]
    }).collect::<Vec<syn::Block>>();

    parse_quote!({
        let mut __block = ::std::vec::Vec::new();
        #(#block)*

        ::tsclient::types::model::InnerType::Tuple(__block)
    })
}

pub (crate)fn parse_newtype(field: Field, holder: syn::Path) -> syn::Block {
    let ty = field.ty.clone();
    let field_type_name: String = field.ty.to_token_stream().to_string().split_whitespace().collect();

    let block = parse_quote!({
        let inner_def = (&mut &#holder::<#ty>::new()).get_definition(registry);
        let type_name: ::std::string::String = (&mut &#holder::<#ty>::new()).name().split_whitespace().collect();
        
        let renamed = if &type_name == "any" || #field_type_name == &type_name {
            None
        } else {
            Some(String::from(#field_type_name))
        };
        
        let comp = ::tsclient::types::model::ComponentReference {
            id: inner_def,
            renamed
        };

        ::tsclient::types::model::InnerType::NewType(comp)
    });

    return block
}

pub(crate) fn parse_object(fields: FieldsNamed, holder: syn::Path) -> syn::Block {
    let block = fields.named.iter().enumerate().map(|(i, field)| {
        let ident = format_ident!("d{}", i);
        let ty = field.ty.clone();
        let field_type_name: String = field.ty.to_token_stream().to_string().split_whitespace().collect();
        
        let field_name = field.ident.clone().map(|f| syn::Member::Named(f)).expect("Parser error named fields").to_token_stream().to_string();
        parse_quote![{
            let #ident = (&mut &#holder::<#ty>::new()).get_definition(registry);
            let type_name: ::std::string::String = (&mut &#holder::<#ty>::new()).name().split_whitespace().collect();

            let renamed = if &type_name == "any" || #field_type_name == &type_name {
                None
            } else {
                Some(String::from(#field_type_name))
            };

            let comp = ::tsclient::types::model::ComponentReference {
                id: #ident,
                renamed
            };

            __block.push((String::from(#field_name), comp));
        }]
    }).collect::<Vec<syn::Block>>();

    parse_quote!({
        let mut __block = ::std::vec::Vec::new();
        #(#block)*

        ::tsclient::types::model::InnerType::Object(__block)
    })
}