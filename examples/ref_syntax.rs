fn main()
{
    // I thought Rust was weird about the syntax for references but maybe it's not actually that
    // weird.
    //
    // First we create a string. As we've mentioned, that creates a buffer on the heap, then a
    // String which consists of a buffer, length, and capacity.
    let string = String::from("boo");
    // Then we borrow the string. str here is a &String by default, but it could just as easily be
    // a &str. In this case, &String is a reference to the String, which is/has a reference to the
    // buffer. So it's smaller than a &str, weirdly, but also requires two pointer indirections to
    // get to the contents of the string itself.
    let str = &string;
    // I'm not 100% sure how the conversion happens? I just know it does, here, and the &str is
    // what is passed into the function. 
    //
    // So we don't actually need to mark that this is a reference here, because the type of the
    // variable is already a reference. We would need to use &string in order to *get* a &String
    // (which is implicitly converted to &str to be passed into borrows str.)
    borrows_str(str);
}

fn borrows_str(a: &str)
{
    println!("Borrowing str! {}", a);
}
