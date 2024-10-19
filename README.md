[alpha version or pre-alpha still]

early testing notes:
```
in toml
[profile.release-small]
inherits = "release"
lto = "thin"
codegen-units = 1
strip = "symbols"
panic = "abort"

for small build:

run 
```bash
$ cargo build --profile release-small 
```

Test with:
$ curl -X POST -d "This is the request body" http://127.0.0.1:8080/

```


## fiddler_crab

vanilla-Rust serial load-proof https minimal server 
- https://github.com/lineality/fiddler_crab

#### "Edit in the camera."
~ John Ford

## Manage loads by making the server load-impervious, crash-proof, and load-ignoring.



# Slim Rust Server

The project is to make an extremely minimal Rust server.

## A very basic server to: 
- get a post request
- that uses a minimal load-management system
- has https support
- is crash-resistant
- that calls an another program e.g. python or llama cpp, though there is a rust_llamacpp if that works.
-- python: that runs a run script as an external operation e.g. run $python script.py -{"hell world"}

Not requirements:
- the this is not a normal server
- this server does not need many async 'worker' or other threads

requirements:
- a can-fail-routinely loop to make a new disposable_handoff_queue
- a can-fail-routinely loop to make a new single request-handler
- an extremely strict way to regulate that the server is not taxed with any work at all when excess requests come in: passively ignore them and do nothing at all: no prints, no logs, no responses, DO NOTHING and move on. (the optimal design for this may not be in place, if not, request better solutions for this, that is important. is checking the length of the array 'easy' enough as server work, or should there be a better method?, possibly even a time-lag on even checking for the size of the queue if it has been found to be full, and dropping, ignoring, any requests that come in in the mean time.)
- the server is designed to handle one maybe two requests at a time and be indestructible, ignoring failures and ignoring request floods and slowly marching on.

## principles:
- 'fail and try again'
- minimal, as much 'vanilla Rust' with no other packages as possible.

# minimal load management system:
1. queue:
- all incoming requests are first put into a queue
- the queue has a max length, perhaps 500 or 1000, after which requests are just dropped. This should be coded in a such a way as to protect the server from getting overloaded with requests, however that is done. perhaps: if len(queue)>500 {drop}; or a struct that only holds that many and attempts to add more are ignored, whatever is least-work for the server.

2. the queue is handled one item at a time with some pace-wait time to be set as a constant.

3. this is not a multi-worker-thread server, this is a FIFO queue server that may be slow but should be impossible to over-load. e.g. in k8s this could be scaled in various ways.

4. crash resistance: the disposable_handoff_queue should be free to fail and be disposed of and a new fresh (however empty) queue being created without bother. e.g. two loops: a queue loops that makes a clean queue if the old one fails for any reason. and a FIFO queue handler thread that takes the next item in the queue (and when if fails, and new thread handler is made)

5. If the server is overwhelmed by requests it is imperative that the server do as little as possible, ideally absolutely nothing, to ignore the flood of requests. no prints, no logs, no response, nothing; not even an action to check the size of the queue if possible (e.g. if the queue is a fixed size and adding to it fails, whatever is the least effort). 

## use-case example:
- a process that takes most of the cpu-gpu of the server cannot be multiplied in threads, but must be run more serially. speed is not the goal, parallel is not the goal, not-crashing is the goal, resource-balancing is the goal, not crashing is the goal


please start with initial MVP code that runs gradually adding features,
not all features need to exist in the first version

even though we will try (again) with tokio and hyper,
remember this is not a generic async multi-thread react server
this is a minimal server, review the specs and requirements.

see:
- https://docs.rs/llama_cpp/latest/llama_cpp/
- https://docs.rs/bounded-vec-deque/latest/bounded_vec_deque/ 


# # Variations
1. A first mvp version of this should be tried with standard library non-async as the scope, e.g. for very resource intensive operations (data science endpoints).
2. But a more fully async version for smaller operations, basic micro endpoints but still load/crash resistant.


# Queue Handoff
The length-count is not the only thing requiring access to the queue. The main listening loop also needs to add items to the queue when new requests arrive.


Having the handler loop take ownership of the whole queue and then having the main listener create a new queue is sometimes referred to as a "queue handoff" or "queue exchange."

Benefits of Queue Handoff:

Simplified Ownership: The ownership of the queue is clearly defined at each point in time. The handler loop owns the queue while it's processing it, and the main listener owns the queue while it's receiving requests.

Reduced Contention: There's no need for locks or mutexes to protect the queue because only one thread accesses it at any given time.

Potential for Parallelism: If you want to introduce a limited degree of concurrency (e.g., with a small thread pool), you could have multiple handler loops, each taking ownership of a separate queue.


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

Note:  handler will NOT be concurrently accessing the same queue
Pay attention to the specific design of this system.
This is not a thread-sharing updated-queue where one queue is both updated and processed, this is Queue Handoff.

e.g.
If the handler is never busy when the next request comes in (if the traffic is slow), then the server simply passes each new request (and an empty queue) into the handler.

If the handler is busy (and the queue is not full) the stream-loop keep adding (up to MAX quantity) new requests to a disposable Handoff_Queue


