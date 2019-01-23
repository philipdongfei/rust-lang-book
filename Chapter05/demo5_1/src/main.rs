fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,

    }
    // demo5_2
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // demo5_3
    let mut user2 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");
    // demo5-6
    let user3 = User{
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User{
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    //tuple struct
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
/*//demo 5_4
fn build_user(email: String, username: String) -> User{
    User {
        email:email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
*/
    fn build_user(email: String, username:String) -> User{
        User{
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

