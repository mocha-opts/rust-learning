fn main() {
    println!("Hello, world!");
    expressionLoop();
}

fn expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; //if块是表达式，返回的类型需要一致
    println!("The value of number is :{}", number)
}

fn forMethod() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

fn forGetIndex() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn forNoVariable() {
    for _ in 0..10 {
        //...
    }
}

fn forCompare() {
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        //
    }

    for item in collection {}
}

fn forContinue() {
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }
}

fn forBreak() {
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i)
    }
}

fn whileContinue() {
    let mut n = 0;
    while n <= 5 {
        println!("{}!", n);

        n = n + 1;
    }
    println!("out")
}

fn loopFlow() {
    let mut n = 0;
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }
    println!("out loop")
}

fn whileVsFor() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("value is:{} ", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn expressionLoop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is :{}", result)
}
