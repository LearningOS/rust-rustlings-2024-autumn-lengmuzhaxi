// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;

    // 使用 is_some() 方法检查 Option
    if my_option.is_some() {
       // 这里可以处理 Some 的情况
    }

    let my_arr = &[
        -1, -2, -3, // 添加缺少的逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 创建空 Vec
    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 交换值
    std::mem::swap(&mut value_a, &mut value_b);
    
    println!("value a: {}; value b: {}", value_a, value_b);
}

