use quote::{format_ident, ToTokens as _};
use syn::{parse_quote, Fields, FieldsNamed, Field, FieldsUnnamed};

fn get_easy_name(typ: &syn::Type) -> String {
    typ
        .to_token_stream()
        .to_string()
        .split_whitespace()
        .collect::<String>()
        .chars()
        .take_while(|c| *c != '<')
        .collect()
}

fn get_type_name_and_rename_check(typ: &syn::Type) -> (String, bool) {
    match typ {
        syn::Type::Paren(x) => get_type_name_and_rename_check(&*x.elem),
        syn::Type::Path(x) => {
            let name = get_easy_name(typ);
            
            /*x.path.get_ident()
                .map(|x| x.to_string())
                .unwrap_or(x.path.segments.last()
                    .map(|x| x.ident.to_string())
                    .unwrap_or(get_easy_name(typ)));*/

            (name, true)
        },
        syn::Type::Reference(x) => get_type_name_and_rename_check(&*x.elem),
        _ => (String::new(), false),
    }
}

pub(crate) fn parse_tuple(fields: FieldsUnnamed, holder: syn::Path) -> syn::Block {
    let block = fields.unnamed.iter().enumerate().map(|(i, field)| {
        let ident = format_ident!("d{}", i);
        
        let ty = field.ty.clone();
        let (field_type_name, check_rename) = get_type_name_and_rename_check(&field.ty);
        
        parse_quote![{
            let #ident = (&mut &#holder::<#ty>::new()).get_definition(registry);
            let type_name: ::std::string::String = (&mut &#holder::<#ty>::new()).name().split_whitespace().collect();

            let renamed = if !#check_rename || &type_name == "any" || #field_type_name == &type_name {
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
    let (field_type_name, check_rename) = get_type_name_and_rename_check(&field.ty);

    let block = parse_quote!({
        let inner_def = (&mut &#holder::<#ty>::new()).get_definition(registry);
        let type_name: ::std::string::String = (&mut &#holder::<#ty>::new()).name().split_whitespace().collect();
        
        let renamed = if !#check_rename || &type_name == "any" || #field_type_name == &type_name {
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
        let (field_type_name, check_rename) = get_type_name_and_rename_check(&field.ty);
        
        let field_name = field.ident.clone().map(|f| syn::Member::Named(f)).expect("Parser error named fields").to_token_stream().to_string();
        parse_quote![{
            let #ident = (&mut &#holder::<#ty>::new()).get_definition(registry);
            let type_name: ::std::string::String = (&mut &#holder::<#ty>::new()).name().split_whitespace().collect();

            let renamed = if !#check_rename || &type_name == "any" || #field_type_name == &type_name {
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