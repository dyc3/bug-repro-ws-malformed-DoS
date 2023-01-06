# bug-repro-ws-malformed-DoS

This repository contains a minimal reproduction of a bug in the `ws` package.

See: https://github.com/websockets/ws/issues/1354

## Steps to reproduce

1. Clone this repository
2. Make sure you have yarn installed, and the rust toolchain installed.

Both of these should succeed:
```bash
which yarn
which cargo
```

3. Run `./run.sh`. It will install the dependencies, build the rust client code, and run the node server code. The result will output the following stack trace:

```
node:events:498
      throw er; // Unhandled 'error' event
      ^

RangeError: Invalid WebSocket frame: RSV2 and RSV3 must be clear
    at Receiver.getInfo (/home/carson/Documents/code/bug-repro-ws-malformed-DoS/server/node_modules/ws/lib/receiver.js:176:14)
    at Receiver.startLoop (/home/carson/Documents/code/bug-repro-ws-malformed-DoS/server/node_modules/ws/lib/receiver.js:136:22)
    at Receiver._write (/home/carson/Documents/code/bug-repro-ws-malformed-DoS/server/node_modules/ws/lib/receiver.js:83:10)
    at writeOrBuffer (node:internal/streams/writable:389:12)
    at _write (node:internal/streams/writable:330:10)
    at Receiver.Writable.write (node:internal/streams/writable:334:10)
    at Socket.socketOnData (/home/carson/Documents/code/bug-repro-ws-malformed-DoS/server/node_modules/ws/lib/websocket.js:1272:35)
    at Socket.emit (node:events:520:28)
    at addChunk (node:internal/streams/readable:315:12)
    at readableAddChunk (node:internal/streams/readable:289:9)
Emitted 'error' event on WebSocket instance at:
    at Receiver.receiverOnError (/home/carson/Documents/code/bug-repro-ws-malformed-DoS/server/node_modules/ws/lib/websocket.js:1158:13)
    at Receiver.emit (node:events:520:28)
    at emitErrorNT (node:internal/streams/destroy:157:8)
    at emitErrorCloseNT (node:internal/streams/destroy:122:3)
    at processTicksAndRejections (node:internal/process/task_queues:83:21) {
  code: 'WS_ERR_UNEXPECTED_RSV_2_3',
  [Symbol(status-code)]: 1002
}
npm ERR! code ELIFECYCLE
npm ERR! errno 1
npm ERR! server@1.0.0 start: `node main.mjs`
npm ERR! Exit status 1
npm ERR!
npm ERR! Failed at the server@1.0.0 start script.
npm ERR! This is probably not a problem with npm. There is likely additional logging output above.
```
