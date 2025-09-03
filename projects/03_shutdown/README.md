# Graceful shutdowns

Run the server, open <http://localhost:8080/> and observe that requests take 5s to load. Refresh the page and then ctrl-C the server
and observe your browser will eventually show an error page.

Goals: When we ctrl-C, the server should not close immediately. 
1. We should wait until all inflight requests have completed before closing each connection.
2. We should wait until all connections have closed before closing the server.
3. We should not accept any new requests on existing connections.
4. We should not accept any new connections into the server.

Various tricks documented in the notes. We will also make use of the graceful_shutdown function in hyper-util: <https://docs.rs/hyper-util/latest/hyper_util/server/graceful/trait.GracefulConnection.html#tymethod.graceful_shutdown>
