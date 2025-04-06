// use chrono::prelude::*;

// import only Utc from complete library
use chrono::{Utc,  Local};
use dotenv::dotenv;
use std::{env};



fn main() {
    let utc = Utc::now();
    print!("current time is : {}", utc);


    let localtimeis = Local::now();
    print!("local time is {}", localtimeis);


    dotenv().ok();
    let reddis_value = env::var("REDIS_INTERNAL");
    match reddis_value{
        Ok(str) => println!("{}", str),
        Err(_e) => print!("Error while reading variable")
    }

    // use unwrap in rust
    // when you know, this is must exist, 
    // or else will it complain 
    dotenv().ok();
    let reddis_value_with_wrap = env::var("REDIS_INTERNAL").unwrap();
    print!("reddis value with uncapping {}", reddis_value_with_wrap);
  


    let positive_number = sum_of_two_number(20,20);
    println!("positive numver addition is {}", positive_number);


    let float_number =  sum_of_two_number(20.20,20.20);
    println!("addition of float point number {}", float_number);
    let user = User{
        name:String::from("ichiragkumar")
    };

   print_varialbe(1);
   print_varialbe("chirag");
   print_varialbe(2.9);
   print_varialbe(true);


   let result = biggest_element::<i32>(100, 200);
   println!("Biggest: {}", result);



}

// here are declaring two function , which is not ideal
// here comes, generic we have to solve this problem

fn sum_of_two_numer_u32(a1:u32, a2:u32)->u32{
    return  a1+a2;
}

fn sum_of_two_number_f32(b1:f32, b2:f32)->f32{
   return b1+b2;
}



// how to prevent this issue here
        // fn sum<T>(a:T, b:T) -> T{
        //     return  a+b;
        // }


// now add traits to fix this issue
fn sum_of_two_number<T:std::ops::Add<Output =T>>(a:T, b:T) -> T{
    return a+b;
}


struct User{
    name:String
}


fn print_varialbe<T: std::fmt::Display>(a:T){
    println!("{}", a)
}


// create a biggest_element using Ordering Trait

fn biggest_element<T: Ord>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}