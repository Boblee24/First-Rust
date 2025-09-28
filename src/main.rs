fn main() {
    println!("Hello, Welcome to Rust 🦀, from cargo!");
    //Primitive data types we have, int, float, char and bool, here are the examples i just leatt from a video

    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    //boolean code
    let is_rich: bool = true;
    println!("Are you rich, {}", is_rich);


    //Now the compound data types: Which includes arrays, strings(slice string), slices and tuples

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("This is the number array: {:?} ",numbers);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Lost".to_string()];
    println!("These are the books in stock {:?} ", book_slices);

    let letters : String = String::from("Hello world");
    let slice : &str = &letters[0..4];
    println!("Sliced shit: {}", slice);
    human_id("Ayomiposi", 21, 178.2);

}


fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I'm {} of age and my height is {}", name, age, height);
}