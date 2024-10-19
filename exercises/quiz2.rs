// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // 定义输出为一个存储 String 的 Vec
        let mut output: Vec<String> = vec![];
        
        // 遍历输入，处理每个 (String, Command) 对
        for (string, command) in input.iter() {
            let result = match command {
                // Uppercase 命令：将字符串转换为大写
                Command::Uppercase => string.to_uppercase(),
                // Trim 命令：去掉字符串的前后空格
                Command::Trim => string.trim().to_string(),
                // Append 命令：将字符串附加指定次数的 "bar"
                Command::Append(times) => {
                    let mut result = string.clone();
                    for _ in 0..*times {
                        result.push_str("bar");  // 修改这里，将 "bar" 附加到原字符串后面
                    }
                    result
                }
            };
            // 将结果存入输出向量
            output.push(result);
        }
        output
    
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
