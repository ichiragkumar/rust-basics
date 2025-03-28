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



}


