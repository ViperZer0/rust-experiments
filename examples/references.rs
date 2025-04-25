use std::ops::{Deref, DerefMut};

struct Dereffable
{
    num: i32
}

impl Deref for Dereffable
{
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

impl DerefMut for Dereffable
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.num
    }
}

fn main()
{
    let mut number: i32 = 50;
    let mut dereffable = Dereffable {
        num: 80
    };

    mutate_number(&mut number);
    println!("Number: {}", number);
    mutate_number(&mut dereffable);
    println!("Dereffable Number: {}", dereffable.num);

    println!("--------------------------------");
    print_type_of("&number", &number);
    print_type_of("&mut number", &mut number);
}

fn mutate_number(number: &mut i32)
{
    // Even though numbers are trivially copyable, we *can* pass them by reference
    *number += 5;
    // Though apparently we do need to manually dereference, since i32 doesn't implement Deref. (I
    // think.)
}

// Lets us print the type name of something YAY!
fn print_type_of<T>(variable_name: &str, _: &T)  {
    println!("Type of {} is {}", variable_name, std::any::type_name::<T>());
}

