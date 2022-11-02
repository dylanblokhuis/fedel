function tealLiveReloadConnect(config) {
  let scriptTag = document.querySelector("script[data-port]")
  let port = scriptTag.dataset.port;
  let protocol = location.protocol === "https:" ? "wss:" : "ws:";
  let host = location.hostname;
  let socketPath = protocol + "//" + host + ":" + port + "/socket";

  let ws = new WebSocket(socketPath);
  ws.onmessage = (message) => {
    let event = JSON.parse(message.data);
    if (event.type === "LOG") {
      console.log(event.message);
    }
    if (event.type === "RELOAD") {
      console.log("💿 Reloading window ...");
      window.location.reload();
    }
  };
  ws.onopen = () => {
    if (config && typeof config.onOpen === "function") {
      config.onOpen();
    }
  };
  ws.onclose = (error) => {
    console.log("Reconnecting...");
    setTimeout(
      () =>
        tealLiveReloadConnect({
          onOpen: () => window.location.reload(),
        }),
      500
    );
  };
  ws.onerror = (error) => {
    console.log("Server web socket error:");
    console.error(error);
  };
}
tealLiveReloadConnect();