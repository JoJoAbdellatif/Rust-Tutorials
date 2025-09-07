// Tuples can be returned from functions as the following.
pub fn tuple_return() -> (String , String , i32) {
    ("Hello".to_string() , "World".to_string() , 25)
}

pub fn tuple_testing() {
    let (hello , _ , _) = tuple_return();
    println!("{hello}");
}