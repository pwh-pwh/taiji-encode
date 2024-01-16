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