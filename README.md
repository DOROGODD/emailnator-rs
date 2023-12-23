```rust

#[tokio::main]
async fn main() {
    let response = emailnator_rs::CreateClient::new(None)
            .get_email(true, true, true, true)
            .await.unwrap()
        println!("{}", response);
}

```