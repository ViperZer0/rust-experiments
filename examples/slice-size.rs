fn main()
{
    // String size is 24, i.e 8 bytes for a pointer to a buffer, 8 bytes for a usize length, and 8
    // bytes for a usize capacity.
    println!("string has size {}", std::mem::size_of::<String>());
    // &str size is 16 bytes, 8 bytes for a pointer to a buffer and 8 bytes for a usize length.
    // No capacity is needed since a &str can't be resized.
    println!("str has size {}", std::mem::size_of::<&str>());

    // What is the difference between a str and a &str?
    // A str is basically black magic. 
    // A slice in general is a "sequence of T of unchangeable length".
    // It is unsized, but when an unsized type is put behind a reference or pointer, the pointer
    // *automatically* becomes wide/fat, i.e there is additional metadata included with the
    // pointer, in this case the length. But yeah, [T] more or less represents the *underlying*
    // heap memory.
    
    // String creates an area on the heap and inserts the contents of the string onto it.
    // The String object on the stack thus becomes a pointer, a usize length, and a usize capacity.
    let string = String::from("Hello World!");

    // As mentioned, &String can automatically be coerced to &str, which is really cool! 
    // It basically replaces this double-indirection where &String is a pointer to a String
    // which is a pointer to the heap with a &str as a pointer directly to the buffer in the heap,
    // With an extra 8 bytes for the length of the buffer!
    let str_slice: &str = &string;

    // Note that this is a trivial, implicit copy. This would not
    // work for types that don't implement copy.
    let str_slice_1: &str = str_slice;
    let str_slice_2: &str = str_slice;
    let str_slice_3: &str = str_slice;

    // Here, we use & to say we want the *address* of str_slice_1, not the value of it (which is
    // itself an address. It's weird bc the "value" of a &str is just the address of the buffer, so
    // it doesn't actually include the length, so the length feels almost invisible.
    //
    // Each of these addresses are different: I.e &str is *trivially* copyable, since it's just 16
    // bytes, but each one contains the same address to str, i.e the buffer in memory. 
    // The value of str_slice, str_slice_1, etc... is all the same, but their addresses are
    // different.
    println!("str_slice address: {:p}", &str_slice);
    println!("str_slice value: {:p}", str_slice);
    println!("str_slice_1 address: {:p}", &str_slice_1);
    println!("str_slice_1 value: {:p}", str_slice_1);
    println!("str_slice_2 address: {:p}", &str_slice_2);
    println!("str_slice_2 value: {:p}", str_slice_2);
    println!("str_slice_3 address: {:p}", &str_slice_3);
    println!("str_slice_3 value: {:p}", str_slice_3);

    unsafe
    {
        let ptr: *const u8 = str_slice.as_ptr();
        // Follow the ptr, get the u8, convert it to a char.
        println!("The first character of the string slice is {}", *ptr as char)
    }
    // This transparent converting a pointer to a container into a pointer to the underlying data
    // happens with other containers too. Strings, Vecs, Boxes, all convert a pointer to themselves
    // into a pointer to the underlying data.
    //
    // This operation is made possible because of the Deref<U> trait.
    // If Deref<Target=U> is implemented for T, &T can be implicity converted to &U.
    // So String implements Deref<Target=str>,
    // VecT> implements Deref<Target=[T]>,
    // Box<T> implemenst Deref<Target=T>.
    
    let number_box: Box<i64> = Box::new(100);
    // Again, we sort of sneakily convert a &Box<i64> into a &i64 here.
    let box_ref: &i64 = &number_box;

    // Both of these print statements print the same thing, bc Display has a transparent
    // impl<T: Display> for &T.
    //
    // The source code doesn't actually include this, but it uses a macro called fmt_refs!
    // Took me a while to find it but here is the full thing:
    /*
     * macro_rules! fmt_refs {
     *    ($($tr:ident),*) => {
     *        $(
     *        #[stable(feature = "rust1", since = "1.0.0")]
     *        impl<T: ?Sized + $tr> $tr for &T {
     *            // This is the important bit, you can see it basically just calls fmt on the
     *            dereffed value. Idk why it needs the whole &** thing instead of just * but i'm
     *            sure there's a good reason.
     *            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
     *        }
     *        #[stable(feature = "rust1", since = "1.0.0")]
     *        impl<T: ?Sized + $tr> $tr for &mut T {
     *            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
     *        }
     *        )*
     *    }
     * }
     */
    println!("box_ref value: {}", box_ref);
    println!("box_ref value: {}", *box_ref);
}
