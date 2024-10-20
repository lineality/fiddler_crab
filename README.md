[under construction pre-alpha]

# fiddler_crab: A Slim Rust Server
vanilla-Rust serial load-proof https minimal server 
- https://github.com/lineality/fiddler_crab

#### "Edit in the camera."
~ John Ford

# TODO:
1. TODO: response (to sender of request)
code exists to send a 200 response code and that is working

```Rust
let response = "HTTP/1.1 200 OK\r\n\r\n";
stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

But an endpoint server needs to send the result of the endpoint based on the input, not just an arbitrary 200 status code regardless of input or function without any function result.


- it may be the single-slim-fiddlecrab vs. many-jellyfish will need entirely different systems, that is fine. Because fiddler-crab is linear-serial, sharing multiple threads is not an issue because there is only one active thread. 


The current plan for response is that the request data are turned into a struct for each request and passed through queue processing and back to main-stream to response-return to end-user.
There is debugging to do after making the general changes for this.
(e.g. many attempts to move stream into the handler failed and decoupling nearly worked but interrupted the 'same-session')


2. TODO: Error Handling
- Cover every possible area for error handling.
- primarily the server needs to survive errors; the server does not primarily need to report on errors.
- every process, every thread, needs to be able to fail casually without stopping or burdening the server in any way.
3. TODO: all unwrap will be removed

4. TODO: get-request testing

5. TODO: load testing (with existing async python endpoint testers)

6. TODO: cloud deployment testing (EC2)


## fiddler_crab vs. jellyfish:
- fiddler_crab is single non-async server for gradual but robust endpoints, managing load by ignoring overflow; e.g. for big requests that take most server resources
- jellyfish (branch from this project) is a variation that puts a 'traffic-light green-yellow-red' system-resource check before picking one of two modes for concurrent/parallel processing of small requests: low-traffic-green: use fast cpu mode, or 2: yellow: high traffic, low resource availability, use more ram-intensive but less cpu intensive parallel-concurrent method for that queue. This is an additional layer where each queue gets handed off to be handled concurrently/in-parallel.


## Manage loads by making the server load-impervious, crash-proof, and load-ignoring.
The project is to make an extremely minimal Rust server.

## A very basic server to: 
- efficiently manage resources
- be small: version 2 is less than 500 kb; keep it light.
- strictly insulate against overwhelming workloads
- get a post request
- use a minimal load-management system
- have https support
- be crash-resistant: fail casually by default and move on
- be able to call another program: this may be a crucial modular part of a simple fiddlercrab server, with other programs and even other rust server modules being called in fail-ok disposable threads. Possible examples:
-- llama cpp for deep learning models (ideally embedding models too)
-- python for data science & NLP
-- modular specific request handling

## Not requirements:
- this is not a normal server
- this server does not need many async 'worker' or other threads
- this server does not need serde
- this server does not need arc mutexes

## Aims:
- Vanilla is the ideal, few or no dependencies or 3rd party libraries, etc. Do everything In-house.
- balance flexibility with do-one-thing-well. 


## Requirements, Firm Goals
- a can-fail-routinely loop to make a new disposable_handoff_queue
- a can-fail-routinely loop to make a new single request-handler
- an extremely strict way to regulate that the server is not taxed with any work at all when excess requests come in: passively ignore them and do nothing at all: no prints, no logs, no responses, DO NOTHING and move on. (the optimal design for this may not be in place, if not, request better solutions for this, that is important. is checking the length of the array 'easy' enough as server work, or should there be a better method?, possibly even a time-lag on even checking for the size of the queue if it has been found to be full, and dropping, ignoring, any requests that come in in the mean time.)
- the server is designed to handle one maybe two requests at a time and be indestructible, ignoring failures and ignoring request floods and slowly marching on.
- Cover every possible area for error handling.
- primarily the server needs to survive errors; the server does not primarily need to report on errors.
- every process, every thread, needs to be able to fail casually without stopping or burdening the server in any way.


## Principles:
- 'fail and try again'


## minimal load management system:
1. queue:
- all incoming requests are first put into a queue
- the queue has a max length, perhaps 500 or 1000, after which requests are just dropped. This should be coded in a such a way as to protect the server from getting overloaded with requests, however that is done. perhaps: if len(queue)>500 {drop}; or a struct that only holds that many and attempts to add more are ignored, whatever is least-work for the server.

2. the queue is handled one item at a time with some pace-wait time to be set as a constant.

3. this is not a multi-worker-thread server, this is a FIFO queue server that may be slow but should be impossible to over-load. e.g. in k8s this could be scaled in various ways.

4. crash resistance: the disposable_handoff_queue should be free to fail and be disposed of and a new fresh (however empty) queue being created without bother. e.g. two loops: a queue loops that makes a clean queue if the old one fails for any reason. and a FIFO queue handler thread that takes the next item in the queue (and when if fails, and new thread handler is made)

5. if the server is overwhelmed by requests it is imperative that the server do as little as possible, ideally absolutely nothing, to ignore the flood of requests. no prints, no logs, no response, nothing; not even an action to check the size of the queue if possible (e.g. if the queue is a fixed size and adding to it fails, whatever is the least effort). 

## Use-Case Example:
- a process that takes most of the cpu-gpu of the server cannot be multiplied in threads, but must be run more serially. speed is not the goal, parallel is not the goal, not-crashing is the goal, resource-balancing is the goal, not crashing is the goal

## Later Variations
1. A first mvp version of this should be tried with standard library non-async as the scope, e.g. for very resource intensive operations (data science endpoints).
2. But a more fully async version for smaller operations, basic micro endpoints but still load/crash resistant. Thoough alternative methods should be tried before continuing the bad-status-quo for network mismanagement.


# Queue Handoff
Having the handler loop take ownership of the whole queue and then having the main listener create a new queue is sometimes referred to as a "queue handoff" or "queue exchange."

Benefits of Queue Handoff:

Simplified Ownership: The ownership of the queue is clearly defined at each point in time. The handler loop owns the queue while it's processing it, and the main listener owns the queue while it's receiving requests.

Reduced Contention: There's no need for locks or mutexes to protect the queue because only one thread accesses it at any given time.

Potential for Parallelism: If you want to introduce a limited degree of concurrency (e.g., with a small thread pool), you could have multiple handler loops, each taking ownership of a separate queue. (Jellyfish?)







# Overall Design:
headline: The request streamloop and the request_handler will NOT be concurrently accessing the same disposable_handoff_queue.

loop 1: 
application runs in a loop that restarts when it fails

stream-loop 2:
Requests ~are added to a max-sized queue 
(tracked with a counter).
when queue is full (if counter > MAX): 
server ignores additional requests (zero action taken).

if request_handler state is idle: pass along request and Queue Handoff


process loop 3: in request_handler
process request and queue in thread with 3 states:
1. busy
2. idle
3. failed

