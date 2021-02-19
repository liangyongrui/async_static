use async_static::async_static;

async fn get_num() -> i32 {
    println!("hello world");
    123
}

async_static! {
    static ref FOO:i32 = get_num().await;
}

/// run print
/// ```
/// hello world
/// The result of the first call: 123
/// The result of the second call: 123
/// ```
#[tokio::test]
async fn test() {
    // The first call, print hello world
    let n = FOO.await;
    println!("The result of the first call: {}", n);

    // The second call, nothing print
    let n = FOO.await;
    println!("The result of the second call: {}", n);
}
