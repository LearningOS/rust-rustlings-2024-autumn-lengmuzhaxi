// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    // 定义一个默认的 licensing_info 方法
    fn licensing_info(&self) -> String {
        let version = self.get_version();
        format!("Some information for version {}", version)
    }

    
    fn get_version(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {
    fn get_version(&self) -> String {
        self.version_number.to_string() // 将 i32 转换为 String
    }
}

// 为 OtherSoftware 实现 Licensed trait
impl Licensed for OtherSoftware {
    fn get_version(&self) -> String {
        self.version_number.clone() // 直接返回 String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info_some = String::from("Some information for version 1");
        let licensing_info_other = String::from("Some information for version v2.0.0");

        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };

        assert_eq!(some_software.licensing_info(), licensing_info_some);
        assert_eq!(other_software.licensing_info(), licensing_info_other);
    }
}
