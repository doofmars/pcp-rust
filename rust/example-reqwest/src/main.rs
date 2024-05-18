use std::collections::HashMap;

//Library usage example of reqwest (HTTP-Get requests) and tokio (Async communication)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the request
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        // Parse the json response to a hashmap
        .json::<HashMap<String, String>>()
        .await?;
    // Debug format print of the hash map
    println!("{resp:#?}");
    // Return the unit type, no value is produced
    Ok(())
}
