use super::router::ApiRequestHandler;

mod command;

pub fn register_routes(router: &mut matchit::Router<ApiRequestHandler>) {
    command::register_routes(router);
}
