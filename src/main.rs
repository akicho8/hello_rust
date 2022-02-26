/// ファイルの先頭のコメント
mod file1;
// use crate::func2;

/// (cargo doc でコメントに現れる部分)
/// C-c C-c C-v で cargo doc
// (ここはドキュメントに現れない)
/// markdown で記述する `foo`
///
/// 行1
/// 行2 (空行が改行になる)
///
/// 行3
/// 行4
///
/// # 例
/// ```
/// println!("Hello Rust!");
/// ```
fn main() {
    println!("Hello Rust!");
    file1::func2();
}

pub fn func1() {
    println!("func1");
}

pub fn ret1() -> isize {
    1
}

// mod tests 外のテストは cargo build のときもビルド対象となるらしいが実際のバイナリサイズに違いはないので本当か怪しい
#[test]
fn outer_test1() {
    assert_eq!(1 + 2, 3);
}

// cargo test -- --test-threads=1
// cargo test -- --nocapture
// cargo test test_func1
// cargo test func
// cargo test -- --ignored

#[cfg(test)]
mod tests {
    use super::*;
    // C-c C-c C-o で実行
    // C-c C-c C-f で一つだけ実行
    #[test]
    fn test_func1() {
        assert_eq!(1 + 2, 3);
        assert_ne!(1 + 2, 0);
        assert!(true, "{}", 1); // format! の引数と同じ
        assert!(true);
    }
    #[test]
    fn test_func2() {
        assert_eq!(ret1(), 1);
    }
    #[test]
    #[should_panic]
    fn test_func3() {
        panic!();
    }
    #[test]
    #[should_panic(expected = "foo")] // エラーメッセージの一致も確認する場合
    fn test_func4() {
        panic!("foo");
    }
    #[test]
    fn test_func5() -> Result<(), String> {
        if true {
            Ok(())
        } else {
            Err(String::from("(message)"))
        }
    }
    #[test]
    #[ignore]
    fn test_func6() {
        assert!(false);
    }
}
