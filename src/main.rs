use std::env;

fn main() {
    let key = "QT_QPA_PLATFORM";
    env::set_var(key, "wayland");
    assert_eq!(env::var(key), Ok("wayland".to_string()));
    println!("{}",env::var("QT_QPA_PLATFORM").unwrap());
}
