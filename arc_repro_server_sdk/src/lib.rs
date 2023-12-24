//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
#[allow(unused_imports)]
use std as alloc;
struct ServerState {
    router: matchit::Router<u32>,
    application_state: ApplicationState,
}
pub struct ApplicationState {
    s0: alloc::sync::Arc<arc_repro::ExampleDependency>,
}
pub async fn build_application_state() -> crate::ApplicationState {
    let v0 = arc_repro::dependency_with_arc();
    crate::ApplicationState { s0: v0 }
}
pub fn run(
    server_builder: pavex::server::Server,
    application_state: ApplicationState,
) -> pavex::server::ServerHandle {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router(),
        application_state,
    });
    server_builder.serve(route_request, server_state)
}
fn build_router() -> matchit::Router<u32> {
    let mut router = matchit::Router::new();
    router.insert("/api/ping", 0u32).unwrap();
    router
}
async fn route_request(
    request: http::Request<hyper::body::Incoming>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    let (request_head, request_body) = request.into_parts();
    #[allow(unused)]
    let request_body = pavex::request::body::RawIncomingBody::from(request_body);
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.uri.path()) {
        Ok(m) => m,
        Err(_) => {
            let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter(
                    vec![],
                )
                .into();
            let matched_route_template = pavex::request::route::MatchedRouteTemplate::new(
                "*",
            );
            return route_1::middleware_0(
                    matched_route_template,
                    &allowed_methods,
                    &request_head,
                )
                .await;
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::request::route::RawRouteParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            let matched_route_template = pavex::request::route::MatchedRouteTemplate::new(
                "/api/ping",
            );
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_0::middleware_0(
                            matched_route_template,
                            server_state.application_state.s0.clone(),
                            &request_head,
                        )
                        .await
                }
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_1::middleware_0(
                            matched_route_template,
                            &allowed_methods,
                            &request_head,
                        )
                        .await
                }
            }
        }
        i => unreachable!("Unknown route id: {}", i),
    }
}
pub mod route_0 {
    pub async fn middleware_0(
        v0: pavex::request::route::MatchedRouteTemplate,
        v1: alloc::sync::Arc<arc_repro::ExampleDependency>,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = arc_repro::telemetry::RootSpan::new(v2, v0);
        let v4 = crate::route_0::Next0 {
            s_0: v1,
            next: handler,
        };
        let v5 = pavex::middleware::Next::new(v4);
        arc_repro::telemetry::logger(v5, v3).await
    }
    pub async fn handler(
        v0: alloc::sync::Arc<arc_repro::ExampleDependency>,
    ) -> pavex::response::Response {
        let v1 = arc_repro::routes::status::ping(v0);
        <http::StatusCode as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: alloc::sync::Arc<arc_repro::ExampleDependency>,
        next: fn(alloc::sync::Arc<arc_repro::ExampleDependency>) -> T,
    }
    impl<T> std::future::IntoFuture for Next0<T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}
pub mod route_1 {
    pub async fn middleware_0(
        v0: pavex::request::route::MatchedRouteTemplate,
        v1: &pavex::router::AllowedMethods,
        v2: &pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v3 = arc_repro::telemetry::RootSpan::new(v2, v0);
        let v4 = crate::route_1::Next0 {
            s_0: v1,
            next: handler,
        };
        let v5 = pavex::middleware::Next::new(v4);
        arc_repro::telemetry::logger(v5, v3).await
    }
    pub async fn handler(
        v0: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v1 = pavex::router::default_fallback(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        next: fn(&'a pavex::router::AllowedMethods) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}
