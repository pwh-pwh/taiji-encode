# Taiji Encode for Rust
太极编码器


## 安装
Cargo.toml添加项目依赖

## 使用
```rust
use taiji_encode::{taiji_decode, taiji_encode};

fn main() {
    let str = "hello world!";
    let encode_result = taiji_encode(str);
    println!("result: {encode_result}");
    let result = taiji_decode(&encode_result);
    if let Ok(r) = result {
        println!("{r}");
    } else {
        eprintln!("Failed to decode");
    }
}
//output result
/**
result: ䷮䷭䷾䷷䷹䷭䷠䷖䷰䷸䷌䷺䷹䷭䷇䷚
hello world!
*/
```

## 项目灵感来源
[taiji-encode](https://github.com/Cat7373/taiji-encode)

## License
[MIT](LICENSE)

