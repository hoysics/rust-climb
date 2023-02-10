// fn is_copy<T:Copy>(){}

// fn types_impl_copy_trait(){
//     is_copy::<bool>();
//     is_copy::<char>();
    
//     is_copy::<i8>();
//     is_copy::<u64>();
//     is_copy::<usize>();

//     is_copy::<fn()>();

//     is_copy::<*const String>();
//     is_copy::<*mut String>();
    
//     is_copy::<&[Vec<u8>]>();
//     is_copy::<&String>();

//     is_copy::<[u8;4]>();
//     is_copy::<(&str,&str)>();
// }

// fn types_not_impl_copy_trait() {
//     is_copy::<str>();
//     is_copy::<[u8]>();
//     is_copy::<Vec<u8>>();
//     is_copy::<String>();

//     is_copy::<&mut String>();

//     is_copy::<[Vec<u8>;4]>();
//     is_copy::<String,u32>();
// }

// fn main() {
//    types_impl_copy_trait();
//    types_not_impl_copy_trait(); 
// }


// fn main() {
//     let data = vec![1, 2, 3, 4];
//     let data1 = &data;
//     // 值的地址是什么？引用的地址又是什么？
//     println!(
//         "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
//         &data, data1, &&data, &data1
//     );
//     println!("sum of data1: {}", sum(data1));

//     // 堆上数据的地址是什么？
//     println!(
//         "addr of items: [{:p}, {:p}, {:p}, {:p}]",
//         &data[0], &data[1], &data[2], &data[3]
//     );
// }

// fn sum(data: &Vec<u32>) -> u32 {
//     // 值的地址会改变么？引用的地址会改变么？
//     println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
//     data.iter().fold(0, |acc, x| acc + x)
// }

// fn main() {
//     let mut data = vec![1, 2, 3];
//     let data1 = vec![&data[0]];
//     println!("data[0]: {:p}", &data[0]);

//     for i in 0..100 {
//         data.push(i);
//     }

//     println!("data[0]: {:p}", &data[0]);
//     println!("boxed: {:p}", &data1);
// }

// fn main() {
//   let mut arr = vec![1, 2, 3];
//   // cache the last item
//   arr.push(4);
//   // consume previously stored last item
//   let last = arr.last();
//   println!("last: {:?}", last);
// }

use std::mem;

fn main() {
    // capacity 是 1, len 是 0
    let mut v = vec![1];
    // capacity 是 8, len 是 0
    let v1: Vec<i32> = Vec::with_capacity(8);

    print_vec("v1", v1);

    // 我们先打印 heap 地址，然后看看添加内容是否会导致堆重分配
    println!("heap start: {:p}", &v[0] as *const i32);

    extend_vec(&mut v);

    // heap 地址改变了！这就是为什么可变引用和不可变引用不能共存的原因
    println!("new heap start: {:p}", &v[0] as *const i32);

    print_vec("v", v);
}

fn extend_vec(v: &mut Vec<i32>) {
    // Vec<T> 堆内存里 T 的个数是指数增长的，我们让它恰好 push 33 个元素
    // capacity 会变成 64
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    let p: [usize; 3] = unsafe { mem::transmute(data) };
    // 打印 Vec<T> 的堆地址，capacity，len
    println!("{}: 0x{:x}, {}, {}", name, p[0], p[1], p[2]);
}