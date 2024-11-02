use super::router::ApiEntry;

mod command;
mod dialogue;

pub fn register_routes(router: &mut matchit::Router<ApiEntry>) {
    command::register_routes(router);
    dialogue::register_routes(router);
}
