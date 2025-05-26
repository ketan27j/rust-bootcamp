// fn main() {
//     let v = vec![1, 2, 3];

//     for i in v {
//         println!("{}", i);
//     }

// }

// fn main() {
//     let v = vec![1, 2, 3];
//     let mut v_iter = v.iter();

//     while let Some(i) = v_iter.next() {
//         print!("{}", i);
//     }
    
//     println!("{:?}", v);
// }
// WILL NOT WORK
// fn main() {
//     let v = vec![1, 2, 3];
//     let mut v_iter = v.into_iter();

//     while let Some(i) = v_iter.next() {
//         print!("{}", i);
//     }
    
//     println!("{:?}", v);
// }
//---------------------------------------
// fn main() {
//     let v = vec![1, 2, 3];
//     let v_iter = v.into_iter();
//     let v2: Vec<i32> = v_iter.map(|x| x + 1).collect();
//     println!("{:?}", v2);
// }
//-----------------------------------------
fn main() {
    let v = vec![1, 2, 3];
    let v_iter = v.into_iter();
    let sum: i32 = v_iter.sum();
    println!("{}", sum);
    // try using the iterator again here.
}