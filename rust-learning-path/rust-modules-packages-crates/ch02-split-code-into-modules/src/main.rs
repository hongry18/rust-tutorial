mod authentication {
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }

        pub fn get_username(&self) -> &String {
            &self.username
        }

        pub fn get_password(&self) -> &u64 {
            &self.password_hash
        }

        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(new_password)
        }
    }

    fn hash_password(input: &str) -> u64 {
        input.parse::<u64>().unwrap()
    }
}
fn main() {
    let mut user = authentication::User::new("jeremy", "1234");

    println!("The username is: {}", user.get_username());
    println!("The username is: {}", user.get_password());

    user.set_password("4321");

    println!("The username is: {}", user.get_password());
}
