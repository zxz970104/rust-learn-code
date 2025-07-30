fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,
    Invalid,56
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个fields变量，类型是Vec， 类似于C++的vector
        // <_>表示Vec中的元素类型由编译器自行推断
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())  // 函数可以作为参数也可以作为返回值，这里map方法中的闭包函数（匿名函数/lambda函数）就是参数
            .collect();

        if cfg!(debug_assertions) {  // 条件编译，debug模式下才会执行
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把fields[1]转换为f32类型的浮点数，如果成功则把f32值赋给length变量
        /* 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
                1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，
                   if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
                2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        */
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok (length) = fields[1].parse::<f32>() {  // ::<f32>表示parse方法的返回值类型是f32, 这种类型标注不是很常用，但是在编译器无法推断出你的数据类型时,就很有用了
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        } else {
            // 输出到标准错误输出
            eprintln!("error: failed to parse length for '{}'", name);
        }

        panic!("oh no!");

        let a: u16 = 1;
        println!("a = {}", a);
    }
}
