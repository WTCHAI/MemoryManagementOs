fn demo_stack_allocation (depth : u32){
    let x : u32 = 20 + depth ; 
    println!(
        "Stack depth: {}, Local variable value: {}, Address in hex: {:p}, Address in decimal: {}",
        depth, x, &x, &x as *const u32 as usize
    );    
    if depth < 5 {
        demo_stack_allocation(depth + 1) ; 
    }
}

fn main() { 
    println!("----------- Stack growth goes down --------- ") ;
    demo_stack_allocation(1);

    let heap_allocation1 : Box<u32> = Box::new(101) ; 
    let heap_allocation2 : Box<u32> = Box::new(102) ; 
    let heap_allocation3 : Box<u32> = Box::new(103) ; 
    let heap_allocation4 : Box<u32> = Box::new(104) ; 
    let heap_allocation5 : Box<u32> = Box::new(105) ; 

    println!("----------- Heap growth goes up --------- ") ;

    println!("Heap value after return: {}, Heap address after return: {:p}, Address in decimal : {}", *heap_allocation1, &*heap_allocation1 , heap_allocation1.as_ref() as *const u32 as usize);
    println!("Heap value after return: {}, Heap address after return: {:p}, Address in decimal : {}", *heap_allocation2, &*heap_allocation2 , heap_allocation2.as_ref() as *const u32 as usize);
    println!("Heap value after return: {}, Heap address after return: {:p}, Address in decimal : {}", *heap_allocation3, &*heap_allocation3 , heap_allocation3.as_ref() as *const u32 as usize);
    println!("Heap value after return: {}, Heap address after return: {:p}, Address in decimal : {}", *heap_allocation4, &*heap_allocation4 , heap_allocation4.as_ref() as *const u32 as usize);
    println!("Heap value after return: {}, Heap address after return: {:p}, Address in decimal : {}", *heap_allocation5, &*heap_allocation5 , heap_allocation5.as_ref() as *const u32 as usize);
}