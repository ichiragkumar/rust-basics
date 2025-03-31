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


    


    let is_logged_in : bool = true;

    let is_user_paid : bool = true;

    if is_logged_in {
        print!(" User is logged in");
    }else if is_user_paid {
        print!(" User has paid");
    }

    if is_logged_in && is_user_paid {
        print!(" User is logged in and paid");
    }


    // lops in go lnag
    let mut counter : i32 = 0;

    loop {
        print!("counter is : {} \n", counter);
        counter = counter + 1;
        if counter == 10 {
            break;
        }
    }   

    // for loop in rust
    for i in 0..10{
        print!("i is : {} \n", i);
    }

    for i in 0..10{
        print!("i is : {} \n", i);
    }


    // while loop


    // iterate over
    // array
    // map
    // string


    let sentence = String::from("Hello rusty, welcome to world of rust");

    let first_word_from = get_first_word(&sentence);
    print!("First word from sentence is : {}", first_word_from);

    let first_letter_from = get_first_letter(&sentence);
    print!("First letter from sentence is : {}", first_letter_from);

    let return_asked_index = get_index_of(&sentence, 2);
    print!("Index of rust in sentence is : {}", return_asked_index);


    // learn why we use used &str here [moved and boroow up]
    // and then updated the function call with &str

    // let sentence = String::from("Hello rusty, welcome to world of rust");

    // let first_word_from = get_first_word(&sentence);
    // print!("First word from sentence is : {}", first_word_from);

    // let first_letter_from = get_first_letter(&sentence);
    // print!("First letter from sentence is : {}", first_letter_from);


    // sum of two number in rust
    let sum1:i32 = 10;
    let sum2:i32 = 20;

    let ans = sum_of_two_numbers(sum1, sum2);
    print!("Sum of two numbers is : {}", ans);

    // sum of n number_n is
    let result:i128 = sum_of_numbers(10000);
    print!("Sum of n number is : {}", result);






}


fn get_first_word(sentence: &str) -> String {
    let mut ans = String::new();
    
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }
    
    return ans
}


fn get_first_letter(sentence: &str) -> String {


    
    return sentence.chars().nth(0).unwrap().to_string();
}

fn get_index_of(sentence: &str, index: usize) -> String {
    if let Some(c) = sentence.chars().nth(index) {
        return c.to_string();
    }
    return String::from("chirag"); 
}


fn sum_of_two_numbers(num1:i32, num2:i32) -> i32{
    return num1 + num2;
}

fn sum_of_numbers(index:i128) -> i128{
    // let mut total_count_is : i128 = 0;
    // for i in 0..index{
    //     total_count_is = total_count_is + i;
    // }
    // return total_count_is;


    // ideal way
    // (n * (n - 1)) / 2

    return  (index * (index - 1)) / 2;

}