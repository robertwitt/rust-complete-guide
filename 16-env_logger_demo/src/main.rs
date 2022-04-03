use log::debug;
use user_auth::User;

fn main() {
    std::env::set_var("RUST_LOG", "user_auth=info,env_logger_demo=info");
    env_logger::init();
    debug!("env logger demo started");
    let user = User::new("bob", "super_sekret");
    user.sign_in("super_secret");
    user.sign_in("super_sekret");
}
