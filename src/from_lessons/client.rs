#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();

    let address = std::fs::read_to_string("adress.conf")?;
    let address = address.lines().next().unwrap().to_string();
    println!("{}", address);

    let res = client
        .get(address.parse().unwrap())
        .await?;
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    
    Ok(())
}
