use std::{collections::HashMap, sync::Mutex, path::{PathBuf}, error::Error, fs, convert::Infallible};

use axum::{http::method, routing::{MethodRouter, MethodFilter}, handler::Handler, body::HttpBody};
use regex::Regex;

use crate::{types::{builder::{GlobalTypeRegistry, TypeBuilder}, model::Component}, api_router::{RouteComponentType, Postion}, utils::{clean_var_name, capitalize_first_letter}, FILE_HEADER};

pub struct ClientObjectBuilder {
    obj: ClientObject,
    import_map: HashMap<String, Vec<String>>
}

impl ClientObjectBuilder {
    pub(crate) fn add(&mut self, segments: Vec<&str>, method: &str) {
        let path = segments.join("/");

        if self.import_map.contains_key(&format!("./{}", path)) {
            self.import_map.get_mut(&format!("./{}", path)).unwrap().push(method.to_owned());
        } else {
            self.import_map.insert(format!("./{}", path), vec![method.to_owned()]);
        }

        self.obj.add(&segments, method.to_owned())
    }

    pub(crate) fn get_ts_imports(&self) -> String {
        self.import_map.iter()
            .map(|(path, name)| format!("import {{ {} }} from \"{}\";", name.join(", "), path))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub enum ClientObject {
    Obj(HashMap<String, ClientObject>),
    Method(String)
}

impl ClientObject {
    fn add(&mut self, segments: &[&str], method: String) {
        if segments.len() == 0 {
            match self {
                ClientObject::Obj(fields) => {
                    fields.insert(method.clone(), ClientObject::Method(method));
                },
                ClientObject::Method(_) => todo!(),
            }

            return;
        }

        let next = if segments.len() <= 1 {
            &[]
        } else {
            &segments[1..]
        };

        match self {
            ClientObject::Obj(fields) => {
                if let Some(field) = fields.get_mut(segments[0]) {
                    field.add(next, method);
                } else {
                    fields.insert(segments[0].to_owned(), ClientObject::Obj(HashMap::new()));
                    fields.get_mut(segments[0]).unwrap().add(next, method);
                }
            },
            ClientObject::Method(_) => panic!("Bad implementation"),
        }
    }

    fn to_ts(&self, base_tabs: String) -> String {
        let mut result = String::from("{");

        match self {
            ClientObject::Obj(fields) => {
                for (field, ty) in fields.iter() {
                    if !result.ends_with("{") {
                        result += ",";
                    }
                    let nested = ty.to_ts(format!("{}\t", base_tabs));
                    result += &format!("\n\t{}{}: {}", base_tabs, field, nested);
                }
            },
            ClientObject::Method(m) => return m.to_owned(),
        }

        result += &format!("\n{}}}", base_tabs);

        return result
    }
}

#[derive(Debug)]
pub struct Api {
    pub components: &'static once_cell::sync::Lazy<Mutex<GlobalTypeRegistry>>,
    pub routes: HashMap<String, Route>
}

impl Api {
    pub fn export_to(&self, src_path: &PathBuf, remove_prefix: Option<&str>) -> Result<(), Box<dyn Error>> {
        let mut client_builder = ClientObjectBuilder {
            obj: ClientObject::Obj(HashMap::new()),
            import_map: HashMap::new(),
        };

        let dto_path = src_path.join("dto");
        let api_path = src_path.join("api");

        if !src_path.exists() {
            fs::create_dir_all(&src_path)?;
        }

        if dto_path.exists() {
            fs::remove_dir_all(&dto_path)?;
        }
        if !dto_path.exists() {
            fs::create_dir_all(&dto_path)?;
        }

        if api_path.exists() {
            fs::remove_dir_all(&api_path)?;
        }
        if !api_path.exists() {
            fs::create_dir_all(&api_path)?;
        }

        let builder = {
            let guard = self.components.lock()?;

            TypeBuilder::build(&*guard)
        };

        builder.export_to(&dto_path)?;

        for (path, route) in self.routes.iter() {
            
            let cleaned_path = match &remove_prefix {
                Some(prefix) => match path.strip_prefix(prefix) {
                    Some(x) => x,
                    None => &path,
                },
                None => &path,
            }.replace('{', "[")
            .replace('}', "]");

            let segments: Vec<_> = cleaned_path.split_terminator("/").filter(|x| !x.is_empty()).collect();

            let guard = self.components.lock()?;

            println!("{:#?}", segments);

            let (dir, name, level) = match segments.len() {
                0 => (api_path.clone(), "index.ts".to_owned(), 1),
                1 => {
                    if segments[0].split_whitespace().count() == 0 {
                        (api_path.clone(), "index.ts".to_owned(), 1)
                    } else {
                        (api_path.clone(), format!("{}.ts", segments[0]), 1)
                    }
                }
                level => {
                    let l = segments.len();

                    let p = segments[0..(l - 1)].join("/");
                    let wd = api_path.join(&p);
                    (wd.clone(), format!("{}.ts", segments[l-1]), level)
                }
            };

            let file = dir.join(&name);

            if !dir.exists() {
                fs::create_dir_all(&dir)?;
            }

            println!("{:?}", file);

            fs::write(file, route.build(&mut client_builder, path, segments, &name, level, &*guard, &builder))?;
        }

        let mut client = String::from(FILE_HEADER);

        client += &format!(r#"
{}

class Client {{
    BASE_PATH = "";

    API = {}

    setBasePath(path: string) {{
        this.BASE_PATH = path;
    }}
}}

const client = new Client();

export default client;

export type ApiResult<T, E> = T | {{isError: true, status_code: number,  error: E}};
        "#, client_builder.get_ts_imports(), client_builder.obj.to_ts("\t".to_string()));

        fs::write(api_path.join("client.ts"), client)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Copy, Clone)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE
}

impl HTTPMethod {
    pub fn to_name(&self) -> &str {
        match self {
            HTTPMethod::GET => "get",
            HTTPMethod::POST => "post",
            HTTPMethod::PUT => "put",
            HTTPMethod::DELETE => "delete",
        }
    }

    pub fn to_axum_method_filter(&self) -> MethodFilter {
        match self {
            HTTPMethod::GET => MethodFilter::GET,
            HTTPMethod::POST => MethodFilter::POST,
            HTTPMethod::PUT => MethodFilter::PUT,
            HTTPMethod::DELETE => MethodFilter::DELETE,
        }
    }

    pub fn create_axum_route<H,T,S,B>(&self, handler: H) -> MethodRouter<S, B, Infallible>
    where
        H: Handler<T, S, B>,
        B: HttpBody + Send + 'static,
        T: 'static,
        S: Clone + Send + Sync + 'static, {

        match self {
            HTTPMethod::GET =>axum::routing::get(handler),
            HTTPMethod::POST => axum::routing::post(handler),
            HTTPMethod::PUT => axum::routing::put(handler),
            HTTPMethod::DELETE => axum::routing::delete(handler),
        }
    }

    pub fn to_method(&self) -> &str {
        match self {
            HTTPMethod::GET => "GET",
            HTTPMethod::POST => "POST",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::DELETE => "DELETE",
        }
    }
}

#[derive(Debug)]
pub struct Route {
    pub methods: HashMap<HTTPMethod, Method>
}

#[derive(Default)]
pub struct RouteDestructured {
    pub path: Option<String>,
    pub query: Option<String>,
    pub body: Option<String>
}

impl Route {
    pub fn rename_ts_methods(&mut self, map: HashMap<HTTPMethod, String>) {
        for (k, v) in map.into_iter() {
            self.methods.get_mut(&k).and_then(|x| {
                x.name = Some(v);
                Some(x)
            });
        }
    }

    fn adjust_route_obj(&self, route: &mut RouteDestructured, part: &RouteComponentType, name: &str, full_route: &str, comp: &Component, registry: &GlobalTypeRegistry) {
        match part {
            RouteComponentType::Query(_) => {
                let query = format!(r#"
    const __params = new URLSearchParams();

{}

    const __queryString = "?" + __params.toString();
"#,         
                comp.typ.build_query_string(name, registry));

                route.query = Some(query);
            },
            RouteComponentType::Path(_) => {
                panic!("Unsupported atm.")
            },
            RouteComponentType::Json(Postion::Body, _) => {
                route.body = Some(format!("const __body = JSON.stringify({});", name))
            },
            RouteComponentType::Raw(Postion::Body, _) => {
                route.body = Some(format!("const __body = {};", name))
            },
            _=> return
        }
    }

    pub fn build(&self, client_builder: &mut ClientObjectBuilder, route: &str, segments: Vec<&str>, name: &str, level: usize, registry: &GlobalTypeRegistry, builder: &TypeBuilder) -> String {
        let mut imports = HashMap::new();
        let mut file_content = Vec::<String>::new();

        let mut client_imp = (0..(level-1)).map(|_|"..").collect::<Vec<_>>().join("/");
        if client_imp.is_empty() {
            client_imp += ".";
        }
        
        imports.insert("__client__".to_owned(), format!("import __client__, {{type ApiResult}} from \"{}/client\"", client_imp));

        for (http, method) in self.methods.iter() {
            let mut route_inputs_builder = Vec::new();
            let mut route_inputs_names = Vec::new();
            let mut route_result_builder = String::from("void");

            let mut route_obj = RouteDestructured::default();


            for content in method.content.iter() {
                let indexed = content.get_indexed();
                let main_component = registry.get_indexed(indexed);
                let component = main_component.get_import_component(registry);

                let clean_name = clean_var_name(&main_component.get_ts_name(registry));

                self.adjust_route_obj(&mut route_obj, content, &clean_name, route, &main_component, registry);

                match content {
                    RouteComponentType::Query(_) |
                    RouteComponentType::Path(_)  |
                    RouteComponentType::Json(Postion::Body, _) |
                    RouteComponentType::Raw(Postion::Body, _) => {
                        let (typ, _) = builder.get_type_and_import(&main_component.name, main_component.hash, level);
                        route_inputs_builder.push(format!("{}: {}", clean_name, typ));
                        route_inputs_names.push(clean_name.clone());
                    },
                    
                    RouteComponentType::Raw(Postion::Result, _) |
                    RouteComponentType::Json(Postion::Result, _) => {
                        route_result_builder = format!("Promise<{}>", main_component.get_client_result(registry));       
                    },
                }

                if let Some(comp) = component {
                    let (typ, import) = builder.get_type_and_import(&comp.name, comp.hash, level);
                    if import != "" {
                        imports.insert(typ, import);
                    }
                }
            }

            let method_base_name = clean_var_name( &method.name.clone()
                .unwrap_or(format!("{}{}", http.to_name(), capitalize_first_letter(name.strip_suffix(".ts").unwrap_or(name)))));

            client_builder.add(segments.clone(), &method_base_name);

            let inputs = route_inputs_builder.join(", ");
            let input_names = route_inputs_names.join(", ");

            let headers = method.content.iter()
                .map(|x| x.get_default_header())
                .filter(|x| x.is_some())
                .map(|x| {
                    let(k, v) = x.unwrap();
                    format!("'{}': '{}'", k, v)
                })
                .collect::<Vec<_>>()
                .join(",\n\t\t");

            let body = route_obj.body.unwrap_or(String::from("const __body = null;"));
            let query = route_obj.query.unwrap_or(String::from("const __queryString = \"\";"));

            file_content.push(format!(r#"
export async function {method_base_name}_RAW({inputs}): Promise<Response> {{
    const headers = new Headers({{
        {headers}
    }});

    {body_def}

    {query_def}

    let __result = await fetch(`${{__client__.BASE_PATH}}{route}${{__queryString}}`, {{
        method: '{http_method_cap}',
        headers: headers,
        body: __body
    }});

    return __result
}}
"#, 
                method_base_name = method_base_name, 
                headers = headers,
                body_def = body,
                query_def = query,
                http_method_cap = http.to_method(),
                inputs = inputs, 
                route = route));


            
            file_content.push(format!(r#"
export async function {method_base_name}({inputs}): {result} {{
    let __result = await {method_base_name}_RAW({imput_names});

    if(!__result.ok) {{
        let error = await __result.json();
        return {{
            isError: true, 
            status_code: __result.status,  
            error
        }}
    }} else {{
        return await __result.json()
    }}
}}
"#, 
                method_base_name = method_base_name, 
                inputs = inputs, 
                imput_names = input_names,
                result = route_result_builder)) 

        }



        let imports = imports.into_iter().map(|(_, x)| x).collect::<Vec<_>>().join("\n");


        format!("{}\n{}\n\n {}", FILE_HEADER, imports, file_content.join("\n\n"))
    }
}

#[derive(Debug)]
pub struct Method {
    pub content: Vec<RouteComponentType>,
    pub name: Option<String>
}