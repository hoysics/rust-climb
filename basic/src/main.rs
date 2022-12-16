// fn main() {
//     println!("Hello, world!");
//     println!("apply square: {}",apply(2,square));
//     println!("apply cube: {}",apply(2,cube));
// }
// // fn as param
// fn apply(value: i32,f: fn(i32)->i32)->i32{
//     f(value)
// }

// fn square(value: i32)->i32{
//     value * value
// }

// fn cube(value: i32)->i32{
//     value*value*value
// }
//----varaierbal and param
// fn pi()->f64{
//     3.1415926
// }

// fn not_pi(){
//     3.1415926;
// }

// fn main(){
//     let is_pi=pi();
//     let is_unit1=not_pi();
//     let is_unit2={
//         pi();
//     };

//     println!("is_pi: {:?}, is_unit: {:?}, iss_unit3: {:?}",is_pi,is_unit1,is_unit2);
// }
// //-------data struct
// #[derive(Debug)]
// enum Gender {
//     Unspecified=0,
//     Female=1,
//     Male=2,
// }

// #[derive(Debug,Copy,Clone)]
// struct UserId(u64);

// #[derive(Debug,Copy,Clone)]
// struct TopicId(u64);

// #[derive(Debug)]
// struct User{
//     id: UserId,
//     name: String,
//     gender:Gender,
// }
// #[derive(Debug)]
// struct Topic{
//     id: TopicId,
//     name: String,
//     owner: UserId,
// }
// #[derive(Debug)]
// enum Event{
//     Join((UserId,TopicId)),
//     Leave((UserId,TopicId)),
//     Message((UserId,TopicId,String)),
// }

// fn main(){
//     let alice=User{id:UserId(1),name:"Alice".into(),gender:Gender::Female};
//     let bob=User{id:UserId(1),name:"Bob".into(),gender:Gender::Male};

//     let topic=Topic{id:TopicId(1),name:"rust".into(),owner:UserId(1)};
//     let event1=Event::Join((alice.id,topic.id));
//     let event2=Event::Join((bob.id,topic.id));
//     let event3=Event::Message((alice.id,topic.id,"Hello world!".into()));

//     println!("event1: {:?}, event2: {:?}, event3: {:?}",event1,event2,event3);
// }
//-------controle pattern
// fn fib_loop(n: u8) {
//     let mut a = 1;
//     let mut b = 1;
//     let mut i = 2u8;

//     loop {
//         let c = a + b;
//         a = b;
//         b = c;
//         i += 1;
//         println!("next val is {}", b);

//         if i >= n {
//             break;
//         }
//     }
// }

// fn fib_while(n: u8) {
//     let (mut a, mut b, mut i) = (1, 1, 2);

//     while i < n {
//         let c = a + b;
//         a = b;
//         b = c;
//         i += 1;

//         println!("next val is {}", b);
//     }
// }
// fn fib_for(n: u8) {
//     let (mut a, mut b) = (1, 1);
//     for _i in 2..n {
//         let c = a + b;
//         a = b;
//         b = c;
//         println!("next val is {}", b);
//     }
// }

// fn main() {
//     let n = 10;
//     fib_loop(n);
//     fib_while(n);
//     fib_for(n);
// }
//-------pattern match
// fn process_event(event: &Event){
//     match event {
//         Event::Join((uid,_tid))=>println!("user {:?} joined",uid),
//         Event::Leave((uid,tid))=>println!("user {:?} left {:?}",uid,tid),
//         Event::Message((_,_,msg))=>println!("broadcast: {}",msg),
//     }
// }
// fn process_event(event: &Event){
//     if let Event::Message((_,_,msg)) =event  {
//         println!("broadcast: {}",msg)
//     }
// }

//------ fib sec
// fn count_next(a:&mut i32,b: &mut i32){
//      let c = *a + *b;
//         *a = *b;
//         *b = c;
// }
// fn fib_loop(n: u8) {
//     let mut a = 1;
//     let mut b = 1;
//     let mut i = 2u8;

//     loop {
//         count_next(&mut a,&mut b);
//         i += 1;
//         println!("next val is {}", b);

//         if i >= n {
//             break;
//         }
//     }
// }

// fn fib_while(n: u8) {
//     let (mut a, mut b, mut i) = (1, 1, 2);

//     while i < n {
//         count_next(&mut a,&mut b);
//         i += 1;

//         println!("next val is {}", b);
//     }
// }
// fn fib_for(n: u8) {
//     let (mut a, mut b) = (1, 1);
//     for _i in 2..n {
//         count_next(&mut a,&mut b);
//         println!("next val is {}", b);
//     }
// }

// fn main() {
//     let n = 10;
//     fib_loop(n);
//     fib_while(n);
//     fib_for(n);
// }

fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
