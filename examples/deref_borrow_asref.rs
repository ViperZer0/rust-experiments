// Okay, let's figure out Borrow vs AsRef vs Deref.
//
// # AsRef
// AsRef is a "cheap reference-to-reference" conversion. Basically, 
// AsRef<T> for some type U means that U.as_ref() returns &T.
// So String and str both implement as_ref(). 
//
// We can use AsRef<str> to take any type and convert it to a reference.
// See print_as_ref.
//
// # Borrow 
// Borrow is basically AsRef, it has the same signature, buuuut...
// 1. Borrow has a blanket impl for any T, and can be used to accept either a reference or a value
//    (???? Not sure what that means)
// 2. Borrow also requires that Hash, Eq, and Ord for a borrowed value are equivalent to those of
//    the owned value.
//      - This means that if you want to borrow only a single field of a struct you can implement
//      AsRef but not Borrow.
//
// The big thing that Borrow does is let HashMaps work on both String keys and &str keys.
//
// # Deref
// Deref is the only one (?) that can happen automatically via deref coercion. AsRef is explicit,
// while Deref is implicit. Oh, Deref doesn't have a generic type, so you can't ask for a Deref<T>
// or anything. Makes sense!
//
fn main()
{
    // &'static str
    let s1 = "hello";
    print_as_ref(s1);
    let s2 = String::from("hello");
    // One downside of passing in the string this way is that this function
    // *does* take ownership of s2 even though it doesn't have to.
    //
    // Fortunately, this is pretty easy to fix, we can just... pass in &s2 instead of s2.
    //
    // No ownership transferred here!
    print_as_ref(&s2);
    print_as_ref(s2);
}

fn print_as_ref<T: AsRef<str>>(s: T)
{
    println!("{}", s.as_ref());
}
