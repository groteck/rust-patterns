// Example of the builder pattern in Rust
// I want to have the interface:
// let user = User::build()
//    .name("Jon Doe")
//    .email("jondoe@example.com")
//    .create();

#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

impl User {
    fn build() -> UserBuilder {
        UserBuilder::new()
    }
}

struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
}

impl UserBuilder {
    fn new() -> UserBuilder {
        UserBuilder {
            name: None,
            email: None,
        }
    }

    fn name(&mut self, name: &str) -> &mut UserBuilder {
        self.name = Some(name.to_string());
        self
    }

    fn email(&mut self, email: &str) -> &mut UserBuilder {
        self.email = Some(email.to_string());
        self
    }

    fn create(&mut self) -> User {
        User {
            name: self.name.take().expect("Name is required"),
            email: self.email.take().expect("Email is required"),
        }
    }
}

fn main() {
    let user = User::build()
        .name("Jon Doe")
        .email("jondoe@example.com")
        .create();

    println!("User name:{0:?}", user.name);
    println!("User email:{0:?}", user.email);
}
