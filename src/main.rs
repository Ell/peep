use futures::executor::block_on;

use crate::application::Application;

mod application;
mod renderer;
mod texture;

fn main() {
    env_logger::init();

    let application = block_on(Application::create());
    application.run();
}
