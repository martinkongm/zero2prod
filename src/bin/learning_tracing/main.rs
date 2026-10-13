use tracing::{Level, event, span};
use tracing_subscriber::{self, util::SubscriberInitExt};

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .finish();
    subscriber.init();
    const RESOURCE_NAME: &str = "application-tracing";
    let user = User {
        name: "Martin".to_string(),
        email: "martin@gmail.com".to_string(),
    };
    let age = 29;
    let my_span = span!(Level::INFO, "my_span", name = user.name, email = user.email, age, { RESOURCE_NAME } = "enabled");

    let _enter = my_span.enter();

    event!(Level::WARN, "something has happened!");
}

struct User {
    name: String,
    email: String,
}
