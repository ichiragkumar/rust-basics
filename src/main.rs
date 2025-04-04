struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count:u64,
}

// implementation in rust structs


struct Rect {
    width : u32,
    height: u32,
}

impl  Rect {

    fn area(&self) -> u32{
        return self.height *  self.width;
    }   

    fn perimeter(&self) -> u32 {
        return  2 * ( self.height + self.width);
    }



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

    print!("user name is {} and his email is {}", user.username, user.email);

    print!("--------------");

    let rect = Rect{
        width:10,
        height:100
    };

    print!("area of rectangle is {}", rect.area());
    print!("peremeter of reactangle is {}", rect.perimeter());
}