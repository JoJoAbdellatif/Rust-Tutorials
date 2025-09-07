// Rust Generics are similar in purpose to Templates in C++.
// They both allow writing code that works with different data types, which avoids code duplication and improves reusability.

// Say, for example, a "Point" needs to be created that takes coordinates as integer values.
pub struct Point {
    x: i32,
    y: i32
}

pub fn pre_generic_testing(){
    let p1 = Point {
        x: 10,
        y: 20
    };
    println!("({} , {})" , p1.x , p1.y);

    // What if that point is to accept floats as well?
    // The following code block, as expected, will fail.
    /*
    let _p2 = Point {
        x: 1.5,
        y: 2.25
    };
    */
}

// Enter, Generics!
// They can allow multiple an object to be initialized with multiple data types for the same fields.
// "<T>" is used to define a generic data type.
// "T" itself can actually be any name, letter, etc....
// "T" is just the convention used.
pub struct GenericPoint <T> {
    x: T,
    y: T
}

// Since Now "x" and "y" are assigned to any data type, an "GenericPoint" can be initialized as an integer, a float, or any other data type.
pub fn generic_testing() {
    let p1: GenericPoint<i32> = GenericPoint { x: 10 , y: 20 };
    println!("({} , {})" , p1.x , p1.y);

    let p2: GenericPoint<f32> = GenericPoint { x: 1.5 , y: 2.25 };
    println!("({} , {})" , p2.x , p2.y);

    // But now, there is a problem!
    // Creating a point of type string slices will also work.
    // It shouldn't make sense because coordinates, logically speaking, should be numbers.
    // They shouldn't be strings.
    let p3: GenericPoint<&str> = GenericPoint { x: "Hello" , y: "World" };
    println!("({} , {})" , p3.x , p3.y);

    
}

// This means the The struct "GenericPoint" is a point, yes!
// But it has been left unconstrained.
// The problem in details is what if, for example, the "GenericPoint" has an implementation that can move "x" and "y" by some value?
impl <T> GenericPoint<T> {

    // The following function will result in a compilation error.
    // This happens because at compile time, the compiler does not know the type of "T".
    // if "T" is of type "&str" then the function will fail.
    /*
    pub fn shift_point(&mut self , x: T , y: T){
        self.x += x;
        self.y += y;
    }
    */
}

// To fix this, rust needs to know exactly how to use the "+=" as above, or "+" in "self.x = self.x + x".
// If "+=" is used, then an "Trait" import needs to take place.
// This "Trait" is called the "AddAssign".
// This makes sure that the objects passed to be shifted all conform to the "AddAssign" trait.
use std::ops::AddAssign;

// If only a "+" is used as in "self.x = self.x + x", "Add" is imported.
use std::ops::Add;

// The use of "Copy" will not allow any object that does not implement "Copy" to be created.
pub struct GenericPoint2 <T: Copy> {
    x: T,
    y: T
}

impl <T: Copy> GenericPoint2<T> {
    
    pub fn shift_point_addassign(&mut self , x: T , y: T)
    where 
        T: AddAssign,
    {
        self.x += x;
        self.y += y;
    }
}

// Notice the implementation for the "Add".
// The "self.x = self.x + x" does the following.
    // Reads "self.x" -> consumes it (if "T" is not "Copy")
    // Reads "dx" -> consumes it (again)
    // Creates a new value and assigns it back to "self.x"
// If "T" is not "Copy", each value can only be used once before it is moved.

// This implementation means that if "<T>" conforms to "Add", then the function can be called without any issues. 
impl <T: Add<Output = T> + Copy> GenericPoint2<T>{
    pub fn shift_point_add(&mut self , x: T , y: T)
    where 
        T: Add,
    {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}

// What if 2 points are to be added together using the "+="?
// The AddAssign can be implemented to accomodate that.
// This will make "p1 += p2" work fine.
// "p1 += p2" is effectively automatically translated to "p1.add_assign(p2)".
impl <T: AddAssign + Copy> AddAssign for GenericPoint2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Not only can points be added together, but also they can be checked for equality using the "PartialEq"
impl <T: PartialEq + Copy> PartialEq for GenericPoint2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn generic_testing_after_fix() {
    let mut p1: GenericPoint2<i32> = GenericPoint2 { x: 10 , y: 20 };
    p1.shift_point_addassign(3, 15);
    println!("({} , {})" , p1.x , p1.y);

    let mut p2: GenericPoint2<f32> = GenericPoint2 { x: 1.5 , y: 2.25 };
    p2.shift_point_add(2.24, 4.89);
    println!("({} , {})" , p2.x , p2.y);

    // If a point is initialized as a "String", then the object creation will fail because "String" does not have "Copy".
    /*
    let mut p3: GenericPoint2<String> = GenericPoint2 { x: "Hello".to_string() , y: "World".to_string() };
    p3.shift_point_add(2.24, 4.89);
    println!("({} , {})" , p3.x , p3.y);
    */

    // To add 2 points together, now the "+=" can be used.
    // Since p4 will be only read, then it doesn't need to be mutable.
    let p4: GenericPoint2<i32> = GenericPoint2 { x: 3 , y: 5 };
    p1 += p4; // Note that p4 here is now consumed and therefore using it again is obsolete.
              // It will need to implement "Clone" so that the clone is consumed and not the original point.
    println!("({} , {})" , p1.x , p1.y);

    let p5: GenericPoint2<i32> = GenericPoint2 { x: 32 , y: 9 };
    let p6: GenericPoint2<i32> = GenericPoint2 { x: 15 , y: 11 };
    // Since the "PartialEq" is now implemented, 2 points can be checked if they are equal to each other.
    if p5 == p6 {
        println!("P1 and P4 Are Equal!");
    } else {
        println!("P1 and P4 Are NOT Equal!");
    }

}


