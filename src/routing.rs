use std::collections::HashMap;

use axum::handler::Handler;

use crate::{api_router::{ApiMethodRouter, ApiBuildable}, api::{Method, HTTPMethod, Route}, GLOBAL_TYPE_REGISTRY};



fn create_method_router<H, T, S>(handler: H, http_method: HTTPMethod) -> ApiMethodRouter<S> 
where
    H: Handler<T, S> + ApiBuildable<T>,
    S: Clone + Send + Sync + 'static,
    T: 'static {


    let mut content = {
        let mut registry = GLOBAL_TYPE_REGISTRY.lock().unwrap();
        H::build(&mut *registry)
    };

    let method = Method {
        content,
        name: None
    };
    ApiMethodRouter {
        route: Route {
            methods: HashMap::from([(http_method, method)])
        },
        rename_map: HashMap::new(),
        header: HashMap::new(),
        router: http_method.create_axum_route(handler),
    }
}

pub fn get<H, T, S>(handler: H) -> ApiMethodRouter<S>
where
    H: Handler<T, S> + ApiBuildable<T>,
    S: Clone + Send + Sync + 'static,
    T: 'static {

        create_method_router(handler, HTTPMethod::GET)
}

pub fn post<H, T, S>(handler: H) -> ApiMethodRouter<S>
where
    H: Handler<T, S> + ApiBuildable<T>,
    S: Clone + Send + Sync + 'static,
    T: 'static {

        create_method_router(handler, HTTPMethod::POST)
}

pub fn put<H, T, S>(handler: H) -> ApiMethodRouter<S>
where
    H: Handler<T, S> + ApiBuildable<T>,
    S: Clone + Send + Sync + 'static,
    T: 'static {

        create_method_router(handler, HTTPMethod::PUT)
}

pub fn delete<H, T, S>(handler: H) -> ApiMethodRouter<S>
where
    H: Handler<T, S> + ApiBuildable<T>,
    S: Clone + Send + Sync + 'static,
    T: 'static {

        create_method_router(handler, HTTPMethod::DELETE)
}