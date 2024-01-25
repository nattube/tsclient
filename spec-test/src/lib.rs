pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use ::axum::{extract::Query, Json};
    use serde::{Serialize, Deserialize};
    use tsclient::TypeScript;
    use tsclient::TypeScriptStrict;
    use tsclient::api::HTTPMethod;
    use tsclient::axum::Router;
    use tsclient::axum::routing::post;
    use tsclient::prelude::*;
    use tsclient::types::builder::{GlobalTypeRegistry, TypeBuilder};
    
    pub use tsclient::TypeScriptStrict as TS;

    use super::*;

    #[derive(Serialize, Deserialize, TS)]
    pub struct Test {
        field1: Vec<String>,
        field2: Vec<Test2>
    }

    #[derive(Serialize, Deserialize, TypeScript)]
    #[serde(tag = "typ", content = "value")]
    pub enum GestEnum {
        T1,
        T2(i32),
        T3(i32, i32),
        T4 {x: i32, y: i32}
    }
    
    #[derive(Serialize, Deserialize, TypeScript)]
    pub struct Test2 {
        field1: Result<u64, String>,
        field2: u64
    }

    #[derive(Serialize, Deserialize, TypeScript)]
    pub struct Test2A {
        field1: u64,
        field2: u64
    }

    #[derive(Serialize, Deserialize, TypeScript)]
    pub struct Test3 {
        field1: Vec<String>,
        field2: Option<LocactionId>,
        field3: Option<LocactionId>,
        field4: Option<Test>,
    }

    type LocactionId = u64;
    
    //#[ts]
    //#[derive(Serialize, Deserialize)]
    //#[serde(tag = "type", content = "value")]
    //pub enum TestEnum {
    //    Manage,
    //    Groups(Test2)
    //}
    
    async fn test_fn(q: Query<Test3>, Json(body): Json<Test>) -> Json<Test> {
        Json(body)
    }

    async fn test_fn3(q: Query<Test>) -> Result<Json<String>, Json<u64>> {
        Ok(Json(String::from("OK")))
    }

    async fn test_fn2(q: Query<Test3>, Json(body): Json<Test>) -> Result<Json<Test>, String> {
        Ok(Json(body))
    }

    #[test]
    fn it_works() {
        let mut registry = GlobalTypeRegistry::new();
        println!("{:?}", registry.components);
        println!();
        println!();

        let mut registry = GlobalTypeRegistry::new();
        println!("{:?}", GestEnum::get_definition(&mut registry));
        println!();
        println!("{:?}", registry.components);
        println!();
        println!();
        println!("{:?}", Test::get_definition(&mut registry));
        println!();
        println!("{:?}", registry.components);
        println!();
        println!();
        println!();
        println!();
        let builder = TypeBuilder::build(&registry);
        println!("{:#?}", builder.file_map);
        println!();
        println!();
        println!();
        println!();
        let api = Router::<()>::new()
            .route("/api/test", post(test_fn).get(test_fn3))
            .route("/api/test/deep/and/nested", post(test_fn2).rename_ts([(HTTPMethod::POST, "createNested")]));

        api.api.export_to(&std::path::PathBuf::from("test/api"), Some("/api")).unwrap();

        println!();
        let t7: Test3 = Test3 {
            field1: vec![String::from("String 1")],
            field2: None,
            field3: Some(1),
            field4: Some(Test {
                field1: vec![String::from("String 2")],
                field2: vec![Test2 { field1: Ok(2), field2: 3 }, Test2 { field1: Err(String::from("erorr string")), field2: 3 }],
            }),
        };

        println!("{}", serde_json::to_string_pretty(&t7).unwrap())
    }
}
