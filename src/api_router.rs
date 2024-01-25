use std::{convert::Infallible, collections::HashMap, future::Future};

use axum::{Router, routing::{MethodRouter, IntoMakeService}, handler::Handler, response::IntoResponse, extract::{FromRequestParts, FromRequest, Query}, Json, body::{HttpBody, Body}, http::Request};
use serde::Serialize;
use tower_layer::Layer;
use tower_service::Service;

use crate::{api::{Api, Route, HTTPMethod, Method}, types::{model::ComponentReference, builder::{HasIndexed, GlobalTypeRegistry}, TypescriptType}, GLOBAL_TYPE_REGISTRY};

pub trait IntoMethodRouter<S,B,E> {
    fn into_method_router(self) -> MethodRouter<S,B,E>;
}

impl<S,B,E> IntoMethodRouter<S,B,E> for ApiMethodRouter<S,B,E> {
    fn into_method_router(self) -> MethodRouter<S,B,E> {
        self.router
    }
}

impl<S,B,E> IntoMethodRouter<S,B,E> for MethodRouter<S,B,E> {
    fn into_method_router(self) -> MethodRouter<S,B,E> {
        self
    }
}

pub struct ApiRouter<S = (), B = Body> where 
S: Clone + Send + Sync + 'static,
B: HttpBody + Send + 'static, {
    pub api: Api,
    pub router: Router<S, B>,
}
impl<S: Clone + Send + Sync + 'static, B: HttpBody + Send + 'static> ApiRouter<S,B> {
    pub fn new() -> Self {
        Self {
            api: Api {
                components: &GLOBAL_TYPE_REGISTRY,
                routes: HashMap::new(),
            },
            router: Router::new()
        }
    }

    pub fn nest(mut self, prefix_route: &str, nested: ApiRouter<S, B>) -> Self {
        for (path, route) in nested.api.routes {
            self.api.routes.insert(format!("{}{}", prefix_route, path), route);
        }
        self.router = self.router.nest(prefix_route, nested.router);

        self
    }

    pub fn route(mut self, route: &str, method: ApiMethodRouter<S, B>) -> Self {
        self.api.routes.insert(route.to_owned(), method.route);
        self.router = self.router.route(route, method.router);

        return self
    }

    pub fn route_without_api<R: IntoMethodRouter<S, B, Infallible>>(mut self, route: &str, method: R) -> Self {
        self.router = self.router.route(route, method.into_method_router());

        return self
    }

    pub fn with_state<S2: Clone + Send + Sync + 'static>(self, state: S) -> ApiRouter<S2, B> {
        let router = self.router.with_state(state);

        ApiRouter::<S2, B> {
            api: self.api,
            router,
        } 
    }

    pub fn layer<L, NewReqBody>(self, layer: L) -> ApiRouter<S, NewReqBody> 
    where
        L: Layer<axum::routing::Route<B>> + Clone + Send + 'static,
        L::Service: Service<Request<NewReqBody>> + Clone + Send + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request<NewReqBody>>>::Future: Send + 'static,
        NewReqBody: HttpBody + Send + 'static, {

        let router = self.router.layer(layer);

        return ApiRouter::<S, NewReqBody> {
            api: self.api,
            router,
        }
    }

    pub fn fallback<H,T>(mut self, handler: H) -> Self
    where
        H: Handler<T, S, B>,
        T: 'static, {

        self.router = self.router.fallback(handler);

        return self
    }
}

impl<B: HttpBody + Send + 'static> ApiRouter<(), B> {
    pub fn into_make_service(self) -> IntoMakeService<Router<(), B>> {
        self.router.into_make_service()
    }
}

pub struct ApiMethodRouter<S = (), B = Body, E=Infallible> {
    pub(crate) route: Route,
    pub header: HashMap<String, String>,
    pub(crate) rename_map: HashMap<HTTPMethod, String>,
    pub(crate) router: MethodRouter<S, B, E>
}

impl<S: Clone + Send,B: HttpBody + Send + 'static> ApiMethodRouter<S,B,Infallible> {
    /// This method overrides all previously set renames for this MethodRouter
    pub fn rename_ts<'a, T: Into<HashMap<HTTPMethod, &'a str>>>(mut self, rename: T) -> Self {
        let map = <T as Into<HashMap::<HTTPMethod, &str>>>::into(rename).into_iter()
            .map(|(k,v)|(k, v.to_owned()))
            .collect();

        self.route.rename_ts_methods(map);

        return self
    }

    pub fn on<H,T>(mut self, http_method: HTTPMethod, handler: H) -> Self 
    where
    H: Handler<T, S, B> + ApiBuildable<T>,
    T: 'static,
    S: Send + Sync + 'static, {
        let mut content = {
            let mut registry = GLOBAL_TYPE_REGISTRY.lock().unwrap();
            H::build(&mut *registry)
        };
    
        let method = Method {
            content,
            name: None
        };

        self.route.methods.insert(http_method, method);
        self.router = self.router.on(http_method.to_axum_method_filter(), handler);

        return self
    }

    pub fn get<H,T>(mut self, handler: H) -> Self 
    where
    H: Handler<T, S, B> + ApiBuildable<T>,
    T: 'static,
    S: Send + Sync + 'static, {

        self.on(HTTPMethod::GET, handler)
    }

    pub fn post<H,T>(mut self, handler: H) -> Self 
    where
    H: Handler<T, S, B> + ApiBuildable<T>,
    T: 'static,
    S: Send + Sync + 'static, {

        self.on(HTTPMethod::POST, handler)
    }

    pub fn put<H,T>(mut self, handler: H) -> Self 
    where
    H: Handler<T, S, B> + ApiBuildable<T>,
    T: 'static,
    S: Send + Sync + 'static, {

        self.on(HTTPMethod::PUT, handler)
    }

    pub fn delete<H,T>(mut self, handler: H) -> Self 
    where
    H: Handler<T, S, B> + ApiBuildable<T>,
    T: 'static,
    S: Send + Sync + 'static, {

        self.on(HTTPMethod::DELETE, handler)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Postion {
    Body,
    Result
}

#[derive(Debug)]
pub enum RouteComponentType {
    Query(HasIndexed),
    Path(HasIndexed),
    Json(Postion, HasIndexed),
    Raw(Postion, HasIndexed),
}

impl RouteComponentType {
    pub fn get_indexed(&self) -> &HasIndexed {
        match self {
            RouteComponentType::Query(x) => x,
            RouteComponentType::Path(x) => x,
            RouteComponentType::Json(_, x) => x,
            RouteComponentType::Raw(_, x) => x,
        }
    }

    pub fn get_default_header(&self) -> Option<(String, String)> {
        match self {
            RouteComponentType::Query(_) => None,
            RouteComponentType::Path(_) => None,
            RouteComponentType::Json(pos, _) => {
                if let Postion::Body = pos {
                    Some((String::from("Content-Type"), String::from("application/json")))
                } else {
                    None
                }
            },
            RouteComponentType::Raw(pos, _) => {
                if let Postion::Body = pos {
                    Some((String::from("Content-Type"), String::from("text/plain")))
                } else {
                    None
                }
            }
        }
    }
}

pub trait ApiBuildable<T> {
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType>;
}

pub trait Buildable {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType>;
}