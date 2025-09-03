# Concurrency

Run the server, open <http://localhost:8080/> and observe that requests take ~2s to load.
In the main.rs `req_handler`, you will observe that we're fetching API results sequentially. 

Goals: execute the requests concurrently using any method you wish. The page load should be down to 0.6s if you've done this correctly.
