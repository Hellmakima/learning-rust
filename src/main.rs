struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("sufiyanattar"),
        email: String::from("sufiyanhattar@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;

    user1.username = String::from("hellmakima");
    let user2 = build_user(String::from("zenintoji"), String::from("toji@zenin.clan"));
    
    let user3 = User {
        username: String::from("RyoumenSukuna"),
        email: String::from("ryoumensukuna@heien.era"),
        ..user2
    };
}

fn build_user(username: String, email: String) -> User {
    User {
        // username: username, // or that, coz same name
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
