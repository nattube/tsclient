use std::{collections::hash_map::DefaultHasher, hash::{Hasher as _, Hash as _}, collections::{HashMap, HashSet}};

use crate::Postion;

use super::builder::{HasIndexed, TypeBuilder, GlobalTypeRegistry};

/// Flow:
/// Check TypeId existence
/// Does not exist? Build -> return reference
///     build self with all subtypes
/// Does exist? don't build -> return reference
/// 

pub struct AnyType;

pub struct BuildTypeInfos {
    name: String,
    imports: Vec<String>
}

#[derive(Clone, Debug)]
pub struct Component {
    pub name: String,
    pub generics: String,
    pub typ: Type,
    pub hash: u64,
}

impl Component {
    pub fn any() -> Self {
        let mut hasher = DefaultHasher::new();
        "any".hash(&mut hasher);

        Self {
            name: String::from("any"),
            generics: String::new(),
            typ: Type::Any,
            hash: hasher.finish(),
        }
    }

    pub fn get_import_component(&self, registry: &GlobalTypeRegistry, pos: Postion) -> Option<Vec<Component>> {
        match &self.typ {
            Type::Array(x) => registry.get_indexed(&x.id).get_import_component(registry, pos),
            Type::Struct(_) => Some(vec![self.clone()]),
            Type::Enum(_, vals) => {
                if let Postion::Body = pos {
                    return Some(vec![self.clone()])
                }

                if vals.len() == 2 && vals[0].0 == "Ok" && vals[1].0 == "Err" {
                    if let (InnerType::NewType(ok), InnerType::NewType(err)) = (&vals[0].1, &vals[1].1) {
                        return match (registry.get_indexed(&ok.id).get_import_component(registry, pos), registry.get_indexed(&err.id).get_import_component(registry, pos)) {
                            (None, None) => None,
                            (None, Some(x)) => Some(x),
                            (Some(x), None) => Some(x),
                            (Some(mut x), Some(y)) => {
                                x.extend(y);
                                Some(x)
                            },
                        }
                    }
                }
                Some(vec![self.clone()])
            },
            Type::Generic(base, generics) => {  

                let base_comp = registry
                    .get_indexed(&base.id)
                    .get_import_component(registry, pos);

                let res = generics.iter()
                    .map(|x| {
                        registry
                            .get_indexed(&x.id)
                            .get_import_component(registry, pos)
                            .unwrap_or(Vec::new())
                    }).flatten()
                    .chain(base_comp.into_iter().flatten())
                    .collect();

                Some(res)
                    
            },
            Type::SimpleType(_) => None,
            Type::Any => None,
            Type::None => None,
        }
    }

    pub fn get_client_result(&self, registry: &GlobalTypeRegistry) -> String {
        match &self.typ {
            Type::Enum(_, vals) => {
                if vals.len() == 2 && vals[0].0 == "Ok" && vals[1].0 == "Err" {
                    if let (InnerType::NewType(ok), InnerType::NewType(err)) = (&vals[0].1, &vals[1].1) {
                        let ok = ok.renamed.as_ref().map(Clone::clone).unwrap_or(registry.get_indexed(&ok.id).get_ts_name(registry));
                        let err = err.renamed.as_ref().map(Clone::clone).unwrap_or(registry.get_indexed(&err.id).get_ts_name(registry));
                        format!("ApiResult<{}, {}>", ok, err)
                    } else {
                        format!("ApiResult<{}, any>", self.get_ts_name(registry))
                    }
                    
                } else {
                    format!("ApiResult<{}, any>", self.get_ts_name(registry))
                }
            },
            _=> format!("ApiResult<{}, any>", self.get_ts_name(registry))
        }
    }

    pub fn get_ts_name(&self, registry: &GlobalTypeRegistry) -> String {
        match &self.typ {
            Type::Array(x) => format!("Array<{}>", registry.get_indexed(&x.id).get_ts_name(registry)),
            Type::Struct(_) => self.name.clone(),
            Type::Enum(_, _) => self.name.clone(),
            Type::Generic(this, gen) => {
                //let generics = gen.iter()
                //    .map(|g| registry.get_indexed(&g.id).get_ts_name(registry))
                //    .collect::<Vec<String>>().join(", ");
                //
                //format!("{}<{}>", self.name, generics)
                registry.get_indexed(&this.id).get_ts_name(registry)
            }
            Type::SimpleType(x) => x.clone(),
            Type::Any => String::from("any"),
            Type::None => String::from("null"),
        }
    }

    pub fn build(&self, builder: &mut TypeBuilder, registry: &GlobalTypeRegistry) -> Option<BuildTypeInfos> {
        if self.name == "Result" {
            println!("Result hash: {}!!!!", self.hash)
        }
        self.typ.build(&self.name, &self.generics, self.hash, builder, registry)
    }
}

#[derive(Clone, Debug)]
pub struct ComponentReference {
    pub id: HasIndexed,
    pub renamed: Option<String>
}

#[derive(Clone, Debug)]
pub enum InnerType {
    Object(Vec<(String, ComponentReference)>),
    Tuple(Vec<ComponentReference>),
    NewType(ComponentReference),
    SimpleVariant(String),
    Null
}

fn update_declarations(declarations: &mut HashMap<String, String>, subcomponent: &Component, renamed: &Option<String>, registry: &GlobalTypeRegistry) {
    if renamed.is_some() {
        let new_name = renamed.as_ref().unwrap();
        if declarations.contains_key(new_name) {
            return
        };

        match subcomponent.typ {
            Type::Array(_) |     
            Type::SimpleType(_) |
            Type::Any |
            Type::None => {
                declarations.insert(new_name.to_owned(), format!("type {} = {};", new_name, subcomponent.get_ts_name(registry)));
            },

            Type::Struct(_) |
            Type::Generic(_,_) |
            Type::Enum(_, _) => return,
        }
    }
}

impl InnerType {

    pub fn inner_query_string_builder(&self, name: &str, registry: &GlobalTypeRegistry) -> String {
        match self {
            InnerType::Object(x) => {
                let mut assignments = Vec::new();
                for (field, r) in x.iter() {
                    let sub = registry.get_indexed(&r.id);
                    assignments.push(sub.typ.build_inner_query_string(name, &format!(".{}", field)));
                }
                return assignments.join("\n");
            },
            InnerType::Tuple(x) => {
                let mut assignments = Vec::new();
                for (field, r) in x.iter().enumerate() {
                    let sub = registry.get_indexed(&r.id);
                    assignments.push(sub.typ.build_inner_query_string(name, &format!("[{}]", field)));
                }
                return assignments.join("\n");
            },
            InnerType::NewType(x) => {
                let sub = registry.get_indexed(&x.id);
                sub.typ.build_query_string(name, registry)
            },
            _ => return String::new()
        }
    }

    pub fn build(&self, builder: &mut TypeBuilder, registry: &GlobalTypeRegistry, declarations: &mut HashMap<String, String>, repr: Option<(EnumRepresentation, &str)>) -> (String, HashMap<String,(String, Option<String>)>) {
        let (content, imports) = match self {
            InnerType::Object(fields) => {
                let mut result = String::from("{");
                let mut imports = HashMap::new();

                if let Some((EnumRepresentation::Internally(tag), typ)) = &repr {
                    result += &format!("\n\t{}: {}", tag, typ);
                }

                for (field, refr) in fields {
                    let sub_comp = registry.get_indexed(&refr.id);

                    let sub_res =  sub_comp.build(builder, registry);

                    let renamed_comp = match sub_res {
                        Some(import) => {
                            for imp in import.imports {
                                imports.insert(imp.clone(), (imp, None));
                            }
                            refr.renamed.clone()
                        },
                        None => refr.renamed.clone(),
                    };

                    update_declarations(declarations, sub_comp, &renamed_comp, registry);

                    if !result.ends_with('{') {
                        result += &format!(";");
                    }

                    result += &format!("\n\t{}: {}", field, renamed_comp.as_ref().unwrap_or(&sub_comp.get_ts_name(registry)));
                }

                result += "\n}";

                let result = match repr {
                    Some((EnumRepresentation::Adjacently(tag, var), typ)) => {
                        format!("{{\n\t{}: {};\n\t{}: {}\n}}", tag, typ, var, result)
                    }
                    Some((EnumRepresentation::Default, typ)) => {
                        format!("{{\n\t{}: {}\n}}", typ, result)
                    }
                    _ => result
                };

                (result, imports)
            }
            InnerType::Tuple(refs) => {
                let mut result = String::from("[");
                let mut imports = HashMap::new();

                for refr in refs {
                    let sub_comp = registry.get_indexed(&refr.id);

                    let sub_res =  sub_comp.build(builder, registry);

                    let renamed_comp = match sub_res {
                        Some(import) => {
                            for imp in import.imports {
                                imports.insert(imp.clone(), (imp, None));
                            }
                            refr.renamed.clone()
                        },
                        None => refr.renamed.clone(),
                    };

                    update_declarations(declarations, sub_comp, &refr.renamed, registry);

                    if !result.ends_with('[') {
                        result += &format!(",");
                    }

                    result += &format!(" {}", renamed_comp.as_ref().unwrap_or(&sub_comp.get_ts_name(registry)));
                }
                result += "]";

                let result = match repr {
                    Some((EnumRepresentation::Adjacently(tag, var), typ)) => {
                        format!("{{\n\t{}: {};\n\t{}: {}\n}}", tag, typ, var, result)
                    }
                    Some((EnumRepresentation::Default, typ)) => {
                        format!("{{\n\t{}: {}\n}}", typ, result)
                    }
                    _ => result
                };

                (result, imports)
            },
            InnerType::NewType(refr) => {
                let mut imports = HashMap::new();

                let sub_comp = registry.get_indexed(&refr.id);

                let sub_res =  sub_comp.build(builder, registry);

                let renamed_comp = match sub_res {
                    Some(import) => {
                        for imp in import.imports {
                            imports.insert(imp.clone(), (imp, None));
                        }
                        refr.renamed.clone()
                    },
                    None => refr.renamed.clone(),
                };

                let _alternative = sub_comp.get_ts_name(registry);

                let sub_name = renamed_comp.as_ref().unwrap_or(&_alternative);

                update_declarations(declarations, sub_comp, &renamed_comp, registry);

                let result = match repr {
                    Some((EnumRepresentation::Adjacently(tag, var), typ)) => {
                        format!("{{\n\t{}: \"{}\";\n\t{}: {}\n}}", tag, typ, var, sub_name)
                    }
                    Some((EnumRepresentation::Default, typ)) => {
                        format!("{{\n\t{}: {}\n}}", typ, sub_name)
                    }
                    _ => sub_name.to_string()
                };

                return (result, imports)
            },
            InnerType::SimpleVariant(x) => {
                let result = match repr {
                    Some((EnumRepresentation::Adjacently(tag, _), typ)) => {
                        format!("{{\n\t{}: \"{}\";\n}}", tag, typ)
                    },
                    _ => format!(r#"{}"#, x)
                };
                (format!(r#"{}"#, result), HashMap::new())
            },
            InnerType::Null => (String::from("null"), HashMap::new()),
        };

        return (content, imports)
    }

    pub fn get_decl_type(&self) -> &str {
        match self {
            InnerType::Object(_) => "type",
            InnerType::Tuple(_) => "type",
            InnerType::NewType(_) => "type",
            InnerType::SimpleVariant(_) => "type",
            InnerType::Null => "type",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Array(ComponentReference),
    Struct(InnerType),
    Enum(EnumRepresentation, Vec<(String, InnerType)>),
    Generic(ComponentReference, Vec<ComponentReference>),
    SimpleType(String),
    Any,
    None,
}

impl Type {

    fn build_inner_query_string(&self, main_name: &str, field_name: &str) -> String {
        match self {
            Type::Array(_) => format!("\tif({}?{} != null) {{ {}{}.forEach(val => __params.append('{}', val.toString())); }}", main_name, field_name, main_name, field_name, field_name.replace(".", "").replace("[", "").replace("]", "")),
            _ => format!("\tif({}?{} != null) {{ __params.append('{}', {}{}.toString()) }}", main_name, field_name, field_name.replace(".", "").replace("[", "").replace("]", ""), main_name, field_name),
        }
    }

    pub fn build_query_string(&self, name: &str, registry: &GlobalTypeRegistry) -> String {
        match self {
            Type::Struct(inner) => inner.inner_query_string_builder(name, registry),
            Type::Enum(repr, variants) => {
                let mut strs = Vec::new();
                for (_, variant) in variants {
                    strs.push(variant.inner_query_string_builder(name, registry))
                }

                strs.join("\n")
            }
            _=> return String::new()
        }
    }

    pub fn build(&self, name: &str, generics: &str, hash: u64, builder: &mut TypeBuilder, registry: &GlobalTypeRegistry) -> Option<BuildTypeInfos> {
        match self {
            Self::Struct(fields) => {
                let mut file = match builder.start_file(name, hash) {
                    Some(f) => f.lock().unwrap().clone(),
                    None => return Some(BuildTypeInfos {
                        name: format!("{}{}", name, generics),
                        imports: vec![name.to_owned()]
                    })
                };

                let (name,_) = builder.get_type_and_import(name, hash, 0);

                let decl = fields.get_decl_type();
                let ending = match decl {
                    "type" => "= ",
                    _ => ""
                };

                file.content += &format!("\n\nexport {} {} {}", decl, format!("{}{}", name, generics), ending);

                let mut type_declarations = HashMap::new();

                let (content, imports) = fields.build(builder, registry, &mut type_declarations, None);
                file.content += &content;

                if type_declarations.len() > 0 {
                    file.type_defs.extend(type_declarations.values().map(&String::to_owned));
                }

                if imports.len() > 0 {
                    file.imports.extend(imports.into_values());
                }

                file.exports.push(name.to_string());

                {
                    let mut guard = builder.get_file(&name, hash).expect("Present").lock().unwrap();
                    *guard = file.clone();
                }

                return Some(BuildTypeInfos {
                    name: format!("{}{}", name, generics),
                    imports: vec![name],
                })
            },
            Self::Generic(this, generics) => {
                let mut this_result = registry.get_indexed(&this.id).build(builder, registry).expect("BASE type");

                let generic_imports = generics
                    .iter()
                    .map(|g| registry.get_indexed(&g.id).build(builder, registry).into_iter().flat_map(|x| x.imports).collect::<Vec<String>>())
                    .flatten();

                this_result.imports.extend(generic_imports);

                return Some(this_result)
            }
            Self::Enum(repr, variants) => {
                let mut file = match builder.start_file(name, hash) {
                    Some(f) => f.lock().unwrap().clone(),
                    None => return Some(BuildTypeInfos {
                        name: format!("{}{}", name, generics),
                        imports: vec![name.to_owned()]
                    }),
                };

                let (name,_) = builder.get_type_and_import(name, hash, 0);

                let mut all_variant_type_names = Vec::new();

                for (variant, content) in variants {
                    let decl = content.get_decl_type();
                    let ending = match decl {
                        "type" => "= ",
                        _ => ""
                    };

                    all_variant_type_names.push(variant.clone());

                    file.content += &format!("\n\nexport {} {} {}", decl, variant, ending);

                    let mut type_declarations = HashMap::new();

                    let (content, imports) = content.build(builder, registry, &mut type_declarations, Some((repr.clone(), &variant)));
                    file.content += &content;
                    file.content += "\n";


                    if type_declarations.len() > 0 {
                        file.type_defs.extend(type_declarations.values().map(&String::to_owned));
                    }

                    if imports.len() > 0 {
                        file.imports.extend(imports.into_values());
                    }

                    file.exports.push(name.to_string());
                }

                file.content += &format!("\n\nexport type {} = {}", format!("{}{}", name, generics), all_variant_type_names.join(" | "));

                file.exports.push(name.to_string());

                {
                    if name == "Result" {
                        println!("{:#?}", builder.file_map);
                        println!("{:#?}", self);
                    }
                    let mut guard = builder.get_file(&name, hash)
                        .expect(&name).lock().unwrap();
                    *guard = file.clone();
                }

                return Some(BuildTypeInfos {
                    name: format!("{}{}", name, generics),
                    imports: vec![name]
                })
            }
            Self::Array(arr) => {
                let sub_comp = registry.get_indexed(&arr.id);

                return sub_comp.build(builder, registry);

            },
            Self::SimpleType(simple) => {
                return None
            },
            Self::Any => {
                return None
            },
            Self::None => {
                panic!("Failed finding None")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum EnumRepresentation {
    Default,
    Untagged,
    Internally(String),
    Adjacently(String, String),
}