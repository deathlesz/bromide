import mitmproxy  # type: ignore

HOSTS_TO_INTERCEPT = ["www.boomlings.com"]


def request(flow: mitmproxy.http.HTTPFlow):
    request = flow.request

    if request.host in HOSTS_TO_INTERCEPT:
        request.path = request.path.replace("/database", "")
        request.host = "localhost"
        request.port = 8080
