use std::future::Future;

use axum::{extract::{Query, State}, Json, Extension};

use crate::{types::{TypescriptType, builder::GlobalTypeRegistry}, api_router::{Buildable, Postion, RouteComponentType, ApiBuildable}};

impl<T> Buildable for State<T> {
    fn build(_registry: &mut GlobalTypeRegistry, _pos: Postion) -> Option<RouteComponentType> {
        return None
    }
}

impl<T> Buildable for Extension<T> {
    fn build(_registry: &mut GlobalTypeRegistry, _pos: Postion) -> Option<RouteComponentType> {
        return None
    }
}


impl<T: TypescriptType> Buildable for Query<T> {
    fn build(registry: &mut GlobalTypeRegistry, _pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Query(T::get_definition(registry)))
    }
}

impl<T: TypescriptType> Buildable for Json<T> {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Json(pos, T::get_definition(registry)))
    }
}


impl<T: TypescriptType + 'static, R: TypescriptType + 'static> Buildable for Result<T, R> {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Json(pos, Result::<T, R>::get_definition(registry)))
    }
}

impl<T: TypescriptType + 'static, R: TypescriptType + 'static> Buildable for Result<Json<T>, R> {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Json(pos, Result::<T, R>::get_definition(registry)))
    }
}

impl<T: TypescriptType + 'static, R: TypescriptType + 'static> Buildable for Result<Json<T>, Json<R>> {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Json(pos, Result::<T, R>::get_definition(registry)))
    }
}


impl<T: TypescriptType + 'static> Buildable for Option<Json<T>> {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Json(pos, Option::<T>::get_definition(registry)))
    }
}

impl<T: TypescriptType + 'static> Buildable for Option<T> {
    fn build(registry: &mut GlobalTypeRegistry, pos: Postion) -> Option<RouteComponentType> {
        return Some(RouteComponentType::Json(pos, Option::<T>::get_definition(registry)))
    }
}


impl<F, Fut, Res, M> ApiBuildable<(M,)> for F 
where
F: FnOnce() -> Fut + Clone + Send + 'static,
Fut: Future<Output = Res> + Send,
Res: Buildable,
{
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType> {
        let res = Res::build(registry, Postion::Result);

        return vec![res].into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}


impl<F, Fut, Res, M, T1> ApiBuildable<(M, T1)> for F 
where
F: FnOnce(T1) -> Fut + Clone + Send + 'static,
Fut: Future<Output = Res> + Send,
Res: Buildable,
T1: Buildable, 
{
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType> {
        let t1 = T1::build(registry, Postion::Body);
        let res = Res::build(registry, Postion::Result);

        return vec![t1, res].into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}


impl<F, Fut, Res, M, T1, T2> ApiBuildable<(M, T1, T2)> for F 
where
F: FnOnce(T1, T2) -> Fut + Clone + Send + 'static,
Fut: Future<Output = Res> + Send,
Res: Buildable,
T1: Buildable,
T2: Buildable, 
{
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType> {
        let t1 = T1::build(registry, Postion::Body);
        let t2 = T2::build(registry, Postion::Body); 
        let res = Res::build(registry, Postion::Result);

        return vec![t1, t2, res].into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}


impl<F, Fut, Res, M, T1, T2, T3> ApiBuildable<(M, T1, T2, T3)> for F 
where
F: FnOnce(T1, T2, T3) -> Fut + Clone + Send + 'static,
Fut: Future<Output = Res> + Send,
Res: Buildable,
T1: Buildable,
T2: Buildable, 
T3: Buildable, 
{
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType> {
        let t1 = T1::build(registry, Postion::Body);
        let t2 = T2::build(registry, Postion::Body); 
        let t3 = T3::build(registry, Postion::Body); 
        let res = Res::build(registry, Postion::Result);

        return vec![t1, t2, t3, res].into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}


impl<F, Fut, Res, M, T1, T2, T3, T4> ApiBuildable<(M, T1, T2, T3, T4)> for F 
where
F: FnOnce(T1, T2, T3, T4) -> Fut + Clone + Send + 'static,
Fut: Future<Output = Res> + Send,
Res: Buildable,
T1: Buildable,
T2: Buildable, 
T3: Buildable, 
T4: Buildable, 
{
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType> {
        let t1 = T1::build(registry, Postion::Body);
        let t2 = T2::build(registry, Postion::Body); 
        let t3 = T3::build(registry, Postion::Body); 
        let t4 = T4::build(registry, Postion::Body); 
        let res = Res::build(registry, Postion::Result);

        return vec![t1, t2, t3, t4, res].into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}


impl<F, Fut, Res, M, T1, T2, T3, T4, T5> ApiBuildable<(M, T1, T2, T3, T4, T5)> for F 
where
F: FnOnce(T1, T2, T3, T4, T5) -> Fut + Clone + Send + 'static,
Fut: Future<Output = Res> + Send,
Res: Buildable,
T1: Buildable,
T2: Buildable, 
T3: Buildable, 
T4: Buildable, 
T5: Buildable, 
{
    fn build(registry: &mut GlobalTypeRegistry) -> Vec<RouteComponentType> {
        let t1 = T1::build(registry, Postion::Body);
        let t2 = T2::build(registry, Postion::Body); 
        let t3 = T3::build(registry, Postion::Body); 
        let t4 = T4::build(registry, Postion::Body); 
        let t5 = T5::build(registry, Postion::Body); 
        let res = Res::build(registry, Postion::Result);

        return vec![t1, t2, t3, t4, t5, res].into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}