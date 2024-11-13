


// fn main() {
//     let num =3;

//     if num <20 {
//         println!("condition was true")
//     } else{
//         println!("false")
//     }
//     println!("Hello, world!");
// }




// fn main() {
//     let num =3;

//     if num !=0 {
//         println!("number is true")
//     }
// }



// fn main(){
//     let number =2;

//     if number %4 ==0{
//         println!("number is divisible by 4")
//     } else if number % 3 ==0{
//         println!("number is divisible by 3")
//     } else{
//         println!("number is divisible by 0")
//     }
// }


//Using if in a let Statement

// fn main(){
//     let condition =true;
//     let number =if condition {10} else {5};

//     println!("The value of number {number}")
// }

// fn main(){
//     let falsecondition =false;

//     let number =if falsecondition (199) else {0};

//     println!("false number {number}")
// }




//Repeating Code with loop


// fn main(){
//     loop {
//         println!("loop")
//     }
// }




// fn main() {
//     let mut counter =0;

//     let result = loop {
//         counter +=1;
         
//          if counter ==10 {
//             break counter *2;

//          }
//     };
//     println!("The result is {result}")
// }


// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }



// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }



// fn main() {
//     let a =[12, 33, 34, 45, 56];

//     for element in a{
//         println!("the value is {element}")
//     }
// }



// fn main(){
//     for number in (1..10).rev(){
//         println!("{number}");
//     }
// }






fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remainings = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}