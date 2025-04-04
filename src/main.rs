struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count:u64,
}

fn main(){
    let name =  String::from("chirag kumar");
    let email = String::from("chireagory3030@gmail.com");
    let user = User {
        username:name,
        active:true,
        sign_in_count:10,
        email
    };

    print!("user name is {} and his email is {}", user.username, user.email)
}