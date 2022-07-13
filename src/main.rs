use std::collections::HashMap;

fn main() {
 /*    let a = 10;
    let b = 110;
    println!("Hello, world! a={}, b={}",a,b); 

    let unsigned  : u8= 10;
    let signed  : i8= -10;

    let float: f32= 1.2; 
    println!("unsign {}, sign {} float={}", unsigned, signed, float);

    let letter = "c";
    let emoji = "\u{1F600}";
    println!("letter {}, emoji {}", letter, emoji);
    let is_true:bool = true;
    println!("IsTrue: {}", is_true);

    // array

    let arr: [u8;3] = [1,2,3];
    let other_arr = [100,5];

    println!("Index: {}, length: {}", arr[0], other_arr.len());

    println!("{:?}",other_arr)


    // tuples

    let tuple: (u8,bool,f32) = (5,true,10.2);

    let tuple2 = (3,5);
    println!("First {}, second {}", tuple.0, tuple2.1);
    println!("{:?}",tuple2)


    // function 
    println!("{}",is_even(2));

    pub fn is_even(num: u8) -> bool {
        let digit = num % 2;
        // return value below 
        digit ==0 // retun bool

    }

// immutability

let mut num_ = 5;
num_ = 3;
println!("{}",num_);

//  arrays slices

let arr = [1,3,5,2,6]; // length
let slice= &arr[1.. 3]; // [3, 5]  dont know the length
println!("{:?}",slice)



// strings
let str: &str = " Helo world";
println!("{:?}",str);

let mut string : String = String::from("Hello world "); //
println!("{:?}",string);

let _slice = &string[..6];
let _ =_slice.len() == 6;
string.push('1');
string.push_str("! Bob");
string = string.replace("Hello","Bye ");
println!("{:?}",string);



let n= -1;

if n > 0 {
    println!("Greater than 0");
}else if n < 0 {
    println!("Less than 0");
}else{
    println!("Not greater than or Less than 0");
}



// for loop
for i in 0..5{
    println!("{:?}",i);
}


//  while loop

let mut i = 0 ;
while i < 5{
    println!("{:?}",i);
    i = i + 1;
    if i == 3 {
        println!("exit");
        break;
    }
}

//  match 

let i = 2;
match i {
    0 => println!("0"),
    1 | 2 => println!("1,2"),
    3..=4 => println!("3,4"),
    _ => println!("default")

}


struct Bird {
    name: String,
    attack : u64
}
impl Bird {
    fn print_name(&self){
        println!("{}", self.name);
    }
}
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool{
        true
    }
}



impl Animal for Bird{
    fn can_fly(&self) -> bool{
        false
    }
}


let name = String::from("Bird");
let bird = Bird { name, attack: 3};
bird.print_name();
println!("{} {}", bird.can_fly(), bird.is_animal());

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C{x:i32,y:i32}
}
let a: MyEnum = MyEnum::A;
let b : MyEnum = MyEnum::B(5);
let c : MyEnum = MyEnum::C{x:10,y:20};
println!("{:?}", a);
println!("{:?}", b);
println!("{:?}", c);


// vector
let mut vec: Vec<i64> = vec![1,2,3,4];
vec.len();
vec[0];
vec.push(6);
vec.remove(0);
println!("{:?}", vec)
*/

let mut map = HashMap::new();
map.insert(0,"H1");
map.insert(1,"H2");
println!("{:?}",map);


match map.get(&0){
    Some(str) => println!("{}", str),
    _ => println!("Doesnt exist in map"),
}

match map.get(&2){
    Some(str) => println!("{}", str),
    _ => println!("Doesnt exist in map"),
}

map.remove(&0);
println!("{:?}",map);




}


