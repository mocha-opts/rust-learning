use std::io;

fn main() {
    println!("Hello, world!");
    let a = [1, 2, 3, 4];
    let b = [3; 5];
    let first = a[0];
    let second = a[1];
    let array: [String; 8] = std::array::from_fn(|i| String::from("rust is good!"));
    // outVisit();
    // arraySlice();
    summary();
}

fn outVisit() {
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();
    //读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}

fn arraySlice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn summary() {
    let one = [1, 2, 3];

    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}:", a);

        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
