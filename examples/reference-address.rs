fn main()
{
    let number: i32 = 0;
    // We use {:p} to format a reference as a pointer/address. This only works on & types, so &i32
    // not i32.
    println!("Number value: {}\tAddress: {:p}", number, &number);

    // Static sized array is allocated on the stack. A reference to an array *is* a slice,
    // which is basically just a pointer with an extra byte for a length or w/e.
    let array = [0,1,2,3,4];
    println!("Array: {:?}\tAddress: {:p}", array, &array);
    println!("Array[1..] Address: {:p}", &array[1..]);
    println!("Array[2..] Address: {:p}", &array[2..]);
    println!("Array[3..] Address: {:p}", &array[3..]);

    // String buffer is allocated on the heap, and the String is allocated on the stack with the
    // pointer to the buffer, the length, and the capacity.
    let string = String::from("Bada Booey AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    // This gives us a pointer to the string object on the STACK.
    // Essentially, this is a double indirected pointer. &string -> string -> heap.
    //
    // By default, &string will give us a &str, which is a string slice. A string slice just points
    // to the underlying data on the heap.
    //
    // &str   -> heap 
    // string -> heap
    print_string_reference(&string);
    // Note that the ouput of this function is probably going to be a much lower address, because
    // it exists on the heap!
    print_string_slice_reference(&string);
    unsafe
    {
        print_string_slice_components(&string);
    }
    print_size_of("&string", &string);
    print_size_of("string", string);
}

fn print_string_reference(str: &String) 
{
    println!("&String address {:p}", str);
}

fn print_string_slice_reference(str: &str)
{
    println!("&str address {:p}", str);
}

unsafe fn print_string_slice_components(str_slice: &str)
{
    // So, we're gonna make a pointer TO the pointer to the string slice.
    let ptr: *const &str = &str_slice;
    println!("String slice pointer: {:p}", *ptr);
    // Now in theory, we can offset this to get the length of the string slice
    unsafe {
        // A pointer is 64 bits, so 8 bytes.
        // And that should leave us pointing to only a usize field, which is the length of the
        // string slice.
        let ptr_2: *const usize = ptr.byte_add(8).cast::<usize>();
        println!("String slice length: {}", *ptr_2);
        assert_eq!(str_slice.len(), *ptr_2);
    }
    println!("Assertion passed!");
}

fn print_size_of<T>(name: &str, _: T)
{
    println!("Size of {} is {}", name, std::mem::size_of::<T>());
}
