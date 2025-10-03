// fn main() {
//     println!("Hello, Welcome to Rust 🦀, from cargo!");
//     //Primitive data types we have, int, float, char and bool, here are the examples i just leatt from a video

//     let x: i32 = -42;
//     let y: u64 = 100;
//     println!("Signed integer: {}", x);
//     println!("Unsigned integer: {}", y);

//     //boolean code
//     let is_rich: bool = true;
//     println!("Are you rich, {}", is_rich);


//     //Now the compound data types: Which includes arrays, strings(slice string), slices and tuples

//     let numbers: [i32; 5] = [1,2,3,4,5];
//     println!("This is the number array: {:?} ",numbers);

//     let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Lost".to_string()];
//     println!("These are the books in stock {:?} ", book_slices);

//     let letters : String = String::from("Hello world");
//     let slice : &str = &letters[0..4];
//     println!("Sliced shit: {}", slice);
//     human_id("Ayomiposi", 21, 178.2);
//     let s1 = String::from("RUST"); 
//     let s = calculate_length(&s1);
//     println!("The length of {s1} is {s} That kind thing, you suppose understand nahhhhh");


//     //Owner ship, borrowing and some other shit
//     let mut any_name: BankAccount = BankAccount {
//         owner: "Alice".to_string(),
//         balance: 160.04,
//     };

//     //Immutable borrow
//     any_name.check_balance();
//     //Muttable borrow
//     any_name.withdraw(30.03);
//     any_name.withdraw(33.03);

//     any_name.check_balance();

// }


// fn human_id(name: &str, age: u32, height: f32){
//     println!("My name is {}, I'm {} of age and my height is {}", name, age, height);
// }
// fn calculate_length(s: &String) -> usize{
//     s.len()
// }
// //Today, I leant about some basic rules of Rust 
// //Each value in Rust has a variable that's its owner
// //There can be only one owner at a Time
// //When the owner goes out of scope the value is dropped
// struct BankAccount {
//     owner : String,
//     balance : f64,
// }
// impl BankAccount {
//     fn withdraw (&mut self, amount:f64) {
//         println!("Withdrawing {} from account owned by {} ", amount, self.owner);
//         self.balance -= amount;
//     }
//     fn check_balance(&self) {
//         println!("Amount owned by {} has a balance of {}", self.owner, self.balance);
//     }
// }

// Now, Variable and Mutability


//loop in Rustttttt
fn main() {
    let mut favourite :u128 = 0;

    let result = loop {
        favourite += 1;

        if favourite == 10 {
            break favourite * 3
        }
    };
    print!("this is the result {result}");
}
//I was supposed to checkout enum today