use std::marker::PhantomData;
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;

use mime;
use gotham::http::response::create_response;
use gotham::state::{State, FromState};
use gotham::handler::{Handler, NewHandler, HandlerFuture, IntoHandlerFuture, IntoHandlerError};
use hyper::{Response, StatusCode, Body};
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};
use futures::{future, Future, Stream};
use handlebars::{Handlebars, to_json};
use serde_json::value::{Map};
use serde_json;
use unicase::Ascii;

use resource::{default_scripts, default_styles};
use logger::log_handler_err;
use update::{LayoutUpdate, GenerateUpdate, ClientMessage};
use layout::Layout;
use ChartState;

#[derive(Clone, Debug)]
pub struct NewIndexHandler {
    page: String,
}
impl NewIndexHandler {
    pub fn new() -> NewIndexHandler {
        let mut hbs = Handlebars::new();
        let template_file = "./templates/base.hbs";
        hbs.register_template_file("base", template_file)
            .expect(&format!("unrecoverable error: unable to register template file {}",
                template_file));

        let mut data = Map::new();
        data.insert("title".to_string(), to_json(&"Graph!".to_string()));
        data.insert("scripts".to_string(), to_json(&default_scripts()));
        data.insert("styles".to_string(), to_json(&default_styles()));

        let page = hbs.render("base", &data)
            .expect(&format!("unrecoverable error: unable to render template file {}",
                template_file));
        NewIndexHandler { page: page }
    }
}

impl NewHandler for NewIndexHandler {
    type Instance = IndexHandler;

    fn new_handler(&self) -> io::Result<IndexHandler> {
        Ok(IndexHandler::new(self.page.clone()))
    }
}

pub struct IndexHandler {
    page: String,
}
impl IndexHandler {
    pub fn new(page: String) -> IndexHandler {
        IndexHandler { page: page }
    }
}
impl Handler for IndexHandler {
    fn handle(self, state: State) -> Box<HandlerFuture> {
        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((self.page.into_bytes(), mime::TEXT_HTML)),
        );
        (state, res).into_handler_future()
    }
}

macro_rules! serve_file {
    ($handler_name:ident, $file_name:expr, $mime_type:expr) => {
        serve_file!(_inner $handler_name, $file_name, $mime_type, |_| {});
    };
    (_inner $handler_name:ident, $file_name:expr, $mime_type:expr, $res_manip:expr) => {

pub fn $handler_name(state: State) -> (State, Response) {
    let mut buf = vec![];
    let res = match File::open($file_name).and_then(|mut f| f.read_to_end(&mut buf)) {
        Ok(_) => {
            let mut res = create_response(
                &state,
                StatusCode::Ok,
                Some((buf, $mime_type))
            );
            $res_manip(&mut res);
            res
        },
        Err(e) => {
            log_handler_err(stringify!($handler_name), e);
            create_response(
                &state,
                StatusCode::NotFound,
                None
            )
        }
    };
    (state, res)
}

    }
}

macro_rules! serve_file_to_anyone {
    ($handler_name:ident, $file_name:expr, $mime_type:expr) => {
        serve_file!(_inner $handler_name, $file_name, $mime_type, |res: &mut Response| {
            let headers = res.headers_mut();
            headers.set(AccessControlAllowOrigin::Any);
        });
    };
}

serve_file!(app_bundle_js, "./assets/bundle.js", mime::TEXT_JAVASCRIPT);
serve_file!(app_bundle_css, "./assets/bundle.css", mime::TEXT_CSS);
serve_file_to_anyone!(test_json, "./assets/test.json", mime::APPLICATION_JSON);

#[derive(Clone, Debug)]
pub struct NewGraphHandler<St, Gen: GenerateUpdate<St>> {
    graph_gen: Gen,
    layout: Arc<Layout<St>>,
    phantom: PhantomData<St>
}
impl<St, Gen: GenerateUpdate<St>> NewGraphHandler<St, Gen> {
    pub fn new(gen: Gen, layout: Arc<Layout<St>>) -> NewGraphHandler<St, Gen> {
        NewGraphHandler {
            graph_gen: gen,
            layout,
            phantom: PhantomData
        }
    }
}

impl<St, Gen> NewHandler for NewGraphHandler<St, Gen>
    where St: ChartState, Gen: 'static + GenerateUpdate<St>
{
    type Instance = GraphHandler<St, Gen>;

    fn new_handler(&self) -> io::Result<GraphHandler<St, Gen>> {
        Ok(GraphHandler::new(self.graph_gen.clone(), self.layout.clone()))
    }
}

pub struct GraphHandler<St, Gen: GenerateUpdate<St>> {
    updater: Gen,
    layout: Arc<Layout<St>>,
    phantom: PhantomData<St>
}
impl<St, Gen: GenerateUpdate<St>> GraphHandler<St, Gen> {
    pub fn new(gen: Gen, layout: Arc<Layout<St>>) -> GraphHandler<St, Gen> {
        GraphHandler {
            updater: gen,
            layout: layout,
            phantom: PhantomData
        }
    }
}

impl<St, Gen> Handler for GraphHandler<St, Gen>
    where St: ChartState, Gen: GenerateUpdate<St> + 'static
{
    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let future = Body::take_from(&mut state)
            .concat2()
            .then(move |full_body| {

                // parse the body into the UI state object (or default if unparsed)
                let body_content = match full_body {
                    Ok(valid_body) => {
                        String::from_utf8(valid_body.to_vec()).unwrap()
                    },
                    Err(e) => return future::err((state, e.into_handler_error())),
                };
                let (prev_ui_state, ui_state): (Option<St>, St) = if body_content.len() == 0 {
                    (None, St::default())
                } else {
                    match serde_json::from_str(&body_content) {
                        Ok(cm) => {
                            let ClientMessage { ui_state, event_message }: ClientMessage<St> = cm;
                            let prev_state = ui_state.clone();
                            let new_state = match self.layout.handle_event(event_message,
                                ui_state)
                            {
                                Ok(new_state) => new_state,
                                Err(e) => {
                                    return future::err((state, e.into_handler_error()));
                                }
                            };
                            (Some(prev_state), new_state)
                        },
                        Err(e) => {
                            println!("failed deserialize: {}", e);
                            return future::err((state, e.into_handler_error()));
                        }
                    }
                };
                let do_send_layout = prev_ui_state.is_none();

                // call the provided container generator function
                let update_result = self.updater.update(&self.layout, prev_ui_state, ui_state)
                    .and_then(|update| {
                        // add layout to (initial) update message if required, and serialize
                        if do_send_layout {
                            let update = LayoutUpdate::new(update, &self.layout);
                            // println!("{}", serde_json::to_string_pretty(&update).unwrap());
                            serde_json::to_vec(&update).map_err(|e| e.into())
                        } else {
                            // println!("{}", serde_json::to_string_pretty(&update).unwrap());
                            serde_json::to_vec(&update).map_err(|e| e.into())
                        }
                    }
                );
                match update_result {
                    Ok(bytes) => {
                        let mut res = create_response(
                            &state,
                            StatusCode::Ok,
                            Some((bytes, mime::APPLICATION_JSON))
                        );
                        //TODO: break this out into configurable header manipulation component
                        res.headers_mut().set(AccessControlAllowOrigin::Value(
                            "http://localhost:4200".into()));
                        future::ok((state, res))
                    },
                    Err(e) => {
                        log_handler_err("graph", e);
                        let res = create_response(
                            &state,
                            StatusCode::InternalServerError,
                            None
                        );
                        future::ok((state, res))
                    }
                }
            });
        Box::new(future)
    }
}

pub fn options_origin(state: State) -> (State, Response) {
    let mut res = create_response(&state, StatusCode::Ok, Some((vec![], mime::TEXT_PLAIN)));
    {
        let headers = res.headers_mut();
        headers.set(AccessControlAllowOrigin::Value("http://localhost:4200".into()));
        headers.set(AccessControlAllowHeaders(vec![Ascii::new("Content-Type".into())]));
    }
    (state, res)
}
