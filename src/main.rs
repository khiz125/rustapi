mod domain;
use domain::user::User;

fn main() {
    let user = User {
        id: 1,
        name: "rust".to_string(),
    };

    println!("Hello, {:?}!", user.name);
}
