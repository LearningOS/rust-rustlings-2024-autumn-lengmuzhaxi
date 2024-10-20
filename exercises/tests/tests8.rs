// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// 这是您 tests8.rs 的修改版本
fn my_demo_function() {
    println!("这是 my_demo_function!");
}

fn my_demo_function_alias() {
    println!("这是 my_demo_function_alias!");
}

fn main() {
    my_demo_function();
    my_demo_function_alias();
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_demo_function() {
        my_demo_function(); // 测试函数调用
    }

    #[test]
    fn test_my_demo_function_alias() {
        my_demo_function_alias(); // 测试别名函数调用
    }

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
