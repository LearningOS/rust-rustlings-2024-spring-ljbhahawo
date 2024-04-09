// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {}

#[cfg(test)]
// 只有在执行test时才会编译
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        // 使用了条件编译指令 #[cfg(feature = "pass")]。如果在编译时启用了 pass 功能，那么函数将直接返回，测试会被视为通过。
        return;

        panic!("no cfg set");
    }
}
