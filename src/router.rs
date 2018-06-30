use std::sync::Arc;

use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

use handler;
use layout::Layout;
use update::GenerateUpdate;
use hyper::{Get, Post};

use ChartState;

pub(crate) fn router<St, Gen>(gen: Gen, layout: Arc<Layout<St>>) -> Router
    where Gen: GenerateUpdate<St> + 'static, St: ChartState
{
    build_simple_router(|route| {
        route.get("/").to_new_handler(handler::NewIndexHandler::new());
        route.scope("/app_bundle", |route| {
            route.get("/bundle.js").to(handler::app_bundle_js);
            route.get("/bundle.css").to(handler::app_bundle_css);
        });
        route.get("/test.json").to(handler::test_json);
        route.associate("/graph", |assoc| {
            assoc.request(vec![Get, Post]).to_new_handler(handler::NewGraphHandler::new(gen,
                layout));
            assoc.options().to(handler::options_origin);
        });
    })
}
