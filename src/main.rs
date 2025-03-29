fn main() {
    println!("hello rust , welcome to world of rusty!");



    // Numbers are in rust can be of
    // integers
        // signed integers
            // i8 , i16 , i32 , i64
        // unsigned integers
            // u8 , u16 , u32 , u64
        // floats
            // f32 , f64

        

    // variable in rust
    let x : i32 = 10;
    let y : i32 = 20;

    print!("Numbers are : {} , {}", x, y);

   


    // float number in rust
    let x : f32 = 10.0;
    let y : f32 = 20.0;

    print!("Float numbers are : {} , {}", x, y);

    // unisgned integer in rust
    let x : u32 = 10;
    let y : u32 = 20;


    print!(" Unsigned integers are : {} , {}", x, y);



    // compiler never runs code, they check statically and when you build it
    // and started comapring the code with the machine 
    // and throw erorr 

    //example:




    // boolena in rust
    print!(" Booleans are true and false");
    let mut is_false : bool = false;
    let is_true : bool = true;

    if is_false{
        print!(" is false");
     }else{
        print!(" is true");
     }

     if is_true && is_false {
         print!(" All are true");
     }else{
        print!(" All are false");
     }



     // all variables are immutable by default
     // if you want to use, you have to use mut

     // like this
     // let mut is_false : bool = false;

     is_false= true;
     print!(" is_false is now , i have muted the variable declaration and updated it: {}", is_false);



     // strings in rust 

      // string in rust is a sequence of characters
      // in rust you can use double quotes to declare a string
      // or single quotes
      // like this


      // general string you can define in rust
      let greetings: String = String::from("Hello rusty, i am your world");
      print!(" Greeting from chirag is : {}", greetings);


      let x : &str = "Hello rusty";
      let y : &str = "Hello rusty";

      print!(" Strings are : {} , {}", x, y);   


      let my_essage:String = String::from("Hello rust, welcome me");


      // should avoid unwrap
      print!(" myMessage is : {}", my_essage.chars().nth(0).unwrap());

      let her_message:String = String::from("Hello rust, welcome me");

      let char1: Option<char> = her_message.chars().nth(0);

      match  char1{
          Some(x) => print!("char1 is : {}", x),
          None => print!(" None"),
      }

      print!(" char1 is : {} \n", char1.unwrap());

    

    // conditional statements in rust
    let is_user_logged_in : bool = true;
    let is_user_paid : bool = true;
    
   
   if is_user_logged_in {
       print!(" User is logged in");
   }

   if is_user_logged_in && is_user_paid {
       print!(" User is logged in and paid \n ");
   }

   if !is_user_logged_in {
       print!(" User is not logged in");
   }


   // loops in rust
   for i in 0..10{
       print!("i is : {} \n", i);
   }

   // print even number in rust , in given range
   let r1: i32 = 0;
   let r2 : i32 = 10;

   for i in r1..r2{
       if i % 2 == 0 {
           print!("Even number is : {} \n", i);
       }
   }


   // prime numbers are in rust in given range
   let p1r : i32 = 0;
   let p2r : i32 = 10;

   for i in p1r..p2r{
    if i % 2 != 0 {
        print!("Prime number is : {} \n", i);
    }
   }
   

   for i in (p1r + 2)..(p2r -2 ) {
    if i % 2 !=0 {
        print!("----------");
        println!("Prime number is: {}", i);
    }
}


}


