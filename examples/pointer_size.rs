use std::rc::Rc;

fn main()
{
    println!("Size of Box<u32>: {} bytes", std::mem::size_of::<Box<u32>>()); // 8
    println!("Size of Box<[u32]>: {} bytes", std::mem::size_of::<Box<[u32]>>()); // 16
    println!("Size of Box<str>: {} bytes", std::mem::size_of::<Box<str>>()); // 16

    println!("Size of Rc<u32>: {} bytes", std::mem::size_of::<Rc<u32>>()); // 8
    println!("Size of Rc<[u32]>: {} bytes", std::mem::size_of::<Rc<[u32]>>()); // 16
    println!("Size of Rc<str>: {} bytes", std::mem::size_of::<Rc<str>>()); // 16

    println!("Size of *u32: {} bytes", std::mem::size_of::<*const u32>()); // 8
    println!("Size of *[u32]: {} bytes", std::mem::size_of::<*const [u32]>()); // 16
}

