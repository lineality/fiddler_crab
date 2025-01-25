/*

Endpoints:

make sure there is an endpoint_modules directory in src with main.rs

*/
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::collections::VecDeque;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use std::panic::AssertUnwindSafe;


const MAX_QUEUE_SIZE: usize = 500;
const PROCESSING_DELAY_MS: u64 = 100; // Adjust as needed
const REQUEST_HANDLER_PAUSE: u64 = 10; // millis

// For states of request_hanlder
enum HandlerState {
    // Busy,
    Idle,
    Failed,
}

/// Represents the state of the request handler with Integer Mapping
/// 
/// This atomic variable is used to track whether the handler is currently busy processing a request,
/// idle and available to handle a new request, or in a failed state.
/// 
/// The state is represented as a `usize` to be compatible with the `AtomicUsize` type.
/// The possible states are defined by the `HandlerState` enum:
/// - `Busy`: The handler is currently processing a request.
/// - `Idle`: The handler is available to process a new request.
/// - `Failed`: The handler has encountered an error and is not operational.
/// 
/// The initial state is set to `Idle`.
/// AtomicUsize
///
/// Represents an unsigned integer (usize) that can be safely accessed and modified by multiple threads concurrently.
///
/// It provides atomic operations (e.g., load, store, compare-and-swap) 
/// that guarantee that these operations are completed as a single, indivisible unit, preventing race conditions.
///
/// It is not designed to store strings directly.
///
/// Why usize for HANDLER_STATE?
/// In the previous code, we used AtomicUsize to represent the HANDLER_STATE because:
/// 1. Enum Representation: We defined the HandlerState enum with different states (Busy, Idle, Failed).
/// 2. Integer Mapping: We implicitly mapped these enum variants to integer values 
/// (e.g., Idle might be 0, Busy might be  1, and Failed might be 2). This mapping is done automatically by the compiler 
/// when you cast an enum to an integer (e.g., HandlerState::Idle as usize).
/// 3. Atomic Storage: We needed an atomic variable to store this integer representation of the state 
/// so that multiple threads could safely access and update it. 
/// AtomicUsize is suitable because it can store unsigned integers.
static HANDLER_STATE: AtomicUsize = AtomicUsize::new(HandlerState::Idle as usize); 

#[derive(Clone, Debug)]
struct RequestUnit {
    id: usize,
    endpoint_module_name: Option<String>,  // or module-function name, whatever
    body: String,
    output_for_response: Option<String>,
    stream_addr: std::net::SocketAddr, // Or a unique stream ID
    response_status: Option<u16>, 
    response_headers: Option<Vec<(String, String)>>,
    response_body: Option<String>,
}





/// Routes an incoming request to its specified endpoint-module for processing
/// and returns the processed result. This function is part of the modular
/// endpoint system where individual endpoint-modules handle specific types
/// of requests.
/// 
/// # Modular Endpoint System
/// Each endpoint-module:
/// - Lives in its own directory in endpoint_modules/
/// - Has its own input/output handling
/// - Is referenced in the endpoint lookup table
/// - Processes its specific type of request
/// 
/// # Function Steps
/// 1. Gets endpoint module name from request
/// 2. Validates module exists in lookup table
/// 3. Routes request to the module
/// 4. Returns processed result or error
/// 
/// # Arguments
/// * `request_unit_struct` - Contains:
///   - endpoint_module_name: name of module to process request
///   - body: request data to be processed
///   - fields for response data
/// 
/// # Returns
/// * `Result<RequestUnit, String>` 
///   - Ok: RequestUnit with processed response
///   - Err: Error message if routing or processing fails
///
/// # Error Handling
/// Returns Err if:
/// - No endpoint module is specified
/// - Specified endpoint module not found in lookup table
/// - Module processing fails
/// fn route_request_to_endpoint_module(mut request_unit_struct: RequestUnit) -> Result<RequestUnit, String> {
/// process_request_with_module
fn process_request_with_module(mut request_unit_struct: RequestUnit) -> Result<RequestUnit, String> {

    // Log incoming request for debugging
    println!("Routing request to endpoint module: {:?}", request_unit_struct);

    // 1. Get endpoint module name from request
    let endpoint_module_name = request_unit_struct.endpoint_module_name
        .as_ref()
        .ok_or("No endpoint module specified")?;

    // 2. Validate: check if endpoint module exists in lookup table
    // TODO: implement actual lookup table check
    if !validate_endpoint_module_exists(endpoint_module_name) {
        return Err(format!("Endpoint module not found among modules: {}", endpoint_module_name));
    }

    // 3. TODO: Route to Module
    // - Get module from lookup table
    // - Pass request data to module
    // - Get processed result from module

    // 4. For now, return error to force implementation discussion
    Err("Need concrete implementation of module lookup and processing".to_string())
}

// /// Processes a request by routing it to the appropriate module and handling the response
// /// 
// /// # Arguments
// /// * `request_unit_struct` - Contains request data including endpoint module name and body
// /// 
// /// # Returns
// /// * `Result<RequestUnit, String>` - Processed request unit with response or error
// fn process_request_with_module(mut request_unit_struct: RequestUnit) -> Result<RequestUnit, String> {
//     println!("Processing request with module: {:?}", request_unit_struct);

//     // 1. Get module name
//     let module_name = request_unit_struct.endpoint_module_name
//         .as_ref()
//         .ok_or("No module specified")?;

//     // 2. Check if module exists
//     // TODO: implement proper module lookup/validation
//     if !module_exists(module_name) {
//         return Err(format!("Module not found: {}", module_name));
//     }

//     // 3. Process with module
//     // TODO: implement proper module interface
//     let processed_output = match route_request_to_endpoint_module(&request_unit_struct.body, module_name) {
//         Ok(output) => output,
//         Err(e) => return Err(format!("Module processing error: {}", e)),
//     };

//     // 4. Package response
//     request_unit_struct.response_status = Some(200);
//     request_unit_struct.response_headers = Some(vec![
//         ("Content-Type".to_string(), "application/json".to_string())
//     ]);
//     request_unit_struct.response_body = Some(processed_output);

//     Ok(request_unit_struct)
// }

// /// Routes an incoming request to its specified endpoint-module for processing
// /// and returns the processed result.
// /// 
// /// # Arguments
// /// * `request_unit_struct` - Contains request data including endpoint module name and body
// /// 
// /// # Returns
// /// * `Result<RequestUnit, String>` - Processed request unit with response or error
// fn route_request_to_endpoint_module(mut request_unit_struct: RequestUnit) -> Result<RequestUnit, String> {
//     println!("Routing request to endpoint module: {:?}", request_unit_struct);

//     // 1. Get endpoint module name
//     let endpoint_module_name = request_unit_struct.endpoint_module_name
//         .as_ref()
//         .ok_or("No endpoint module specified")?;

//     // 2. Check if endpoint module exists in lookup table
//     // TODO: Actually check a real lookup table
//     if !endpoint_module_exists_in_lookup_table(endpoint_module_name) {
//         return Err(format!("Endpoint module not found in lookup table: {}", endpoint_module_name));
//     }

//     // TODO: Need concrete steps here for:
//     // - Getting the module
//     // - Running the module
//     // - Getting output from module

//     // For now, return error to force discussion of concrete next steps
//     Err("Need concrete implementation of module lookup and processing".to_string())
// }

// Helper functions that need to be implemented:
fn module_exists(module_name: &str) -> bool {
    // TODO: implement actual module checking
    true  // temporary
}

fn validate_endpoint_module_exists(endpoint_module_name: &str) -> bool {
    // Check if directory exists in endpoint_modules/
    let module_path = format!("endpoint_modules/{}", endpoint_module_name);
    std::path::Path::new(&module_path).is_dir()
}


// /// Processes an individual request by routing it to the appropriate endpoint module
// /// and handling the response.
// ///
// /// # Arguments
// /// * `request_unit_struct` - A mutable RequestUnit containing the request details
// ///   including endpoint name, request body, and fields for response data
// ///
// /// # Returns
// /// * `Result<RequestUnit, String>` - Returns either the processed RequestUnit with
// ///   response data or an error message
// ///
// /// # Processing Steps
// /// 1. Validates the endpoint module exists
// /// 2. Parses the request body according to endpoint specifications
// /// 3. Processes the request using the endpoint module
// /// 4. Formats the response
// ///
// /// # Error Handling
// /// * Returns error if endpoint module is not found
// /// * Returns error if request parsing fails
// /// * Returns error if processing fails
// /// * Returns error if response formatting fails
// fn process_a_request(mut request_unit_struct: RequestUnit) -> Result<RequestUnit, String> {
//     println!("Processing request: {:?}", request_unit_struct);

//     // 1. Validate endpoint module exists
//     let endpoint_name = match &request_unit_struct.endpoint_module_name {
//         Some(name) => name,
//         None => return Err("No endpoint module specified".to_string()),
//     };

//     // 2. Look up module and parse request
//     // This could be a match statement or HashMap lookup of registered endpoints
//     match endpoint_name.as_str() {
//         "endpoint_modules/return/module.rs" => {
//             // Parse input using the module's parse function
//             let parsed_input = match parse_return_module_input(&request_unit_struct.body) {
//                 Ok(input) => input,
//                 Err(e) => return Err(format!("Failed to parse input: {}", e)),
//             };

//             // Process using the module
//             let processed_output = match process_return_module(parsed_input) {
//                 Ok(output) => output,
//                 Err(e) => return Err(format!("Failed to process: {}", e)),
//             };

//             // Convert output to response format
//             request_unit_struct.output_for_response = Some(processed_output);
//         },
//         // Add other endpoints here
//         _ => return Err(format!("Unknown endpoint: {}", endpoint_name)),
//     }

//     // Convert the output to JSON string
//     let response_body = match convert_output_to_json_string(&request_unit_struct.output_for_response) {
//         Ok(json) => json,
//         Err(e) => return Err(format!("Failed to format response: {}", e)),
//     };

//     // Set up response data
//     request_unit_struct.response_status = Some(200);
//     request_unit_struct.response_headers = Some(vec![
//         ("Content-Type".to_string(), "application/json".to_string())
//     ]);
//     request_unit_struct.response_body = Some(response_body);

//     // Add intentional processing delay if configured
//     thread::sleep(Duration::from_millis(PROCESSING_DELAY_MS));

//     Ok(request_unit_struct)
// }

fn convert_output_to_json_string(output: &Option<String>) -> Result<String, String> {
    // Implementation needed
    unimplemented!()
}

// // for processing and functions on request data
// /// TODO Doc String Needed!!!
// fn process_a_request(mut request_unit_struct: RequestUnit) -> Result<RequestUnit, String> {
//     // likely now calling endpoint modules
    
//     println!("Processing request: {:?}", request_unit_struct);
    
    
    
//     // select parse_preprocess_function()
//     // e.g. extract and read fields from json
//     // process header, etc.
    
//     // note: there may be instruction about how the parse the request-body
//     // in a parse_instructions lookup table too

//     let endpoint_module_name = request_unit_struct.endpoint_module_name;
    
//     // look up enums and structs based on endpoint_module_name
//     // attempt to match request body to input fields (with safe error handling)
//     // add input field data to module_struct
//     // send module struct with inputs to the module
//     // module returns the struct with outputs (or error etc)
    
//     // turn struct into a json-string with meta-data fields
//     let result_of_function = convert_output_to_json_str(request_unit_struct.output_for_response);
    
//     // 2. select/run output_function()
//     // let result_of_function = json_string_of_struct;

//     // 3. Set up Response Data   
//     request_unit_struct.response_status = Some(200); 
//     request_unit_struct.response_headers = Some(vec![("Content-Type".to_string(), "text/plain".to_string())]); 
//     request_unit_struct.response_body = Some(result_of_function);

//     // intentional delay
//     thread::sleep(Duration::from_millis(PROCESSING_DELAY_MS));
    
//     // Return the modified RequestUnit or an error message
//     if /* processing successful */ true {
//         Ok(request_unit_struct) // Return the RequestUnit directly
//     } else {
//         Err("Processing failed".to_string())
//     }
// }

// module version draft
// fn process_a_request(mut request_unit: RequestUnit) -> Result<RequestUnit, String> {
//     // ... other code ...

//     // Extract endpoint name from module_path
//     let endpoint_name = // ... your logic to extract endpoint name from module_path ...

//     // Look up the module definition in the HashMap
//     if let Some(module_function) = ENDPOINT_MODULES.get(endpoint_name) {
//         // Based on the endpoint_name, create the correct ModuleData struct and parse the input
//         match endpoint_name {
//             "return" => {
//                 let input = // ... parse request_unit.body into ReturnInputFields ...
//                 let mut module_data = ReturnModuleData { input, output: ReturnOutputFields::ReturnedString(String::new()) };
                
//                 module_data = module_function(module_data)?; 

//                 // Extract the output from module_data.output and set request_unit.response_body
//                 // ...
//             },
//             "process_data" => {
//                 // ... Similar logic for "process_data" endpoint ...
//             },
//             _ => return Err(format!("Invalid endpoint: {}", endpoint_name)), 
//         }
//     } else {
//         return Err(format!("Invalid endpoint: {}", endpoint_name)); 
//     }

//     // ... other code ...
// }

// // old version does not iterate thorugh queue (oops!!)
// fn handler_of_request_and_queue(
//     request_unit_struct: RequestUnit, // Removed mut
//     mut disposable_handoff_queue: VecDeque<RequestUnit>,
//     sender: Sender<(usize, Result<RequestUnit, String>)>
// ) {
//     // Wrap the closure in AssertUnwindSafe
//     let closure = AssertUnwindSafe(|| {
//         // 1. Add request to the queue
//         disposable_handoff_queue.push_back(request_unit_struct);

//         // 2. Process the queue
//         loop {
//             if let Some(request_unit) = disposable_handoff_queue.pop_front() {
//                 // Process the request and handle the result
//                 match process_a_request(request_unit.clone()) {

//                     Ok(processed_request) => {
//                         // Send the processed RequestUnit back to the main thread 
//                         // using the channel. The main thread will then be responsible
//                         // for sending the HTTP response to the client.
//                         if let Err(e) = sender.send((processed_request.id, Ok(processed_request))) {
//                             eprintln!("Error sending processed request to main thread: {}", e);
//                             // TODO: Handle the error appropriately (e.g., log, retry, or exit)
//                         }
//                     }
//                     Err(error_message) => {
//                         // Send the error message back to the main thread
//                         if let Err(e) = sender.send((request_unit.id, Err(error_message))) {
//                             eprintln!("Error sending error message to main thread: {}", e);
//                             // TODO: Handle the error appropriately (e.g., log, retry, or exit)
//                         }
//                     } // Added Err case
//                 }
//             } else {
//                 // Queue is empty, wait a bit before checking again
//                 thread::sleep(Duration::from_millis(REQUEST_HANDLER_PAUSE)); 
//             }
//         }
//     });

//     // Call catch_unwind with the wrapped closure
//     std::panic::catch_unwind(closure).unwrap_or_else(|_| {
//         // Set HANDLER_STATE to Failed
//         HANDLER_STATE.store(HandlerState::Failed as usize, Ordering::Relaxed);
//         println!("Handler thread panicked!"); 
//         // You'll need to return a RequestUnit here as well
//     });
// }

fn handler_of_request_and_queue(
    request_unit_struct: RequestUnit,
    mut disposable_handoff_queue: VecDeque<RequestUnit>,
    sender: Sender<(usize, Result<RequestUnit, String>)>
) {
    // Wrap the closure in AssertUnwindSafe
    let closure = AssertUnwindSafe(|| {
        // 1. Add the initial request to the queue
        disposable_handoff_queue.push_back(request_unit_struct);

        // 2. Process the queue
        loop {
            if let Some(request_unit) = disposable_handoff_queue.pop_front() {
                // Process the request and handle the result
                match process_request_with_module(request_unit.clone()) {
                    Ok(processed_request) => {
                        // Send the processed RequestUnit back to the main thread
                        if let Err(e) = sender.send((processed_request.id, Ok(processed_request))) {
                            eprintln!("Error sending processed request to main thread: {}", e);
                            // TODO: Handle the error appropriately (e.g., log, retry, or exit)
                        }
                    }
                    Err(error_message) => {
                        // Send the error message back to the main thread
                        if let Err(e) = sender.send((request_unit.id, Err(error_message))) {
                            eprintln!("Error sending error message to main thread: {}", e);
                            // TODO: Handle the error appropriately (e.g., log, retry, or exit)
                        }
                    }
                }
            } else {
                // Queue is empty, wait a bit before checking again
                thread::sleep(Duration::from_millis(REQUEST_HANDLER_PAUSE));
            }
        }
    });

    // Call catch_unwind with the wrapped closure
    std::panic::catch_unwind(closure).unwrap_or_else(|_| {
        HANDLER_STATE.store(HandlerState::Failed as usize, Ordering::Relaxed);
        println!("Handler thread panicked!");
        // You'll need to return a RequestUnit here as well
    });
}


// TODO Add explanation here, in detail
/*
text
*/
static QUEUE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static REQUEST_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);


fn main() {
    
    // Main loop for crash resistance, 'Let it fail, and try again.'
    // Main Loop:
    // Purpose: The main loop is responsible for the overall lifecycle of the server. 
    // It initializes components, starts the stream-loop, and handles potential 
    // restarts if the stream-loop encounters errors.
    //
    // Execution: The main loop typically runs only once when the server starts and continues 
    // running indefinitely until the server is intentionally shut down.
    //
    // Responsibility for Queues: The main loop is responsible for creating the initial disposable handoff 
    // queue when the server starts. It might also handle the creation of a new queue if the handler thread 
    // encounters an error, but this logic might also be delegated to the stream-loop.
    loop {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        // TODO error handling, when fails: signal_to_restart = True, restart loop

        // Create a channel for communication between the main thread and the handler thread
        let (sender, receiver): (Sender<(usize, Result<RequestUnit, String>)>, Receiver<(usize, Result<RequestUnit, String>)>) = std::sync::mpsc::channel();

        // Create a mapping to store streams by request ID
        let mut stream_map: HashMap<usize, TcpStream> = HashMap::new();

        // Clone the sender before entering the loop
        let sender_clone = sender.clone();

        
        // Purpose: The stream-loop is responsible for listening for incoming requests, 
        // handling the request queue, and passing requests to the handler.
        // Execution: The stream-loop runs continuously within the main loop, 
        // accepting and processing incoming requests.
        // Responsibility for Queues: The stream-loop is primarily responsible 
        // for creating new disposable handoff queues immediately after handing 
        // off the previous queue to the handler. It also manages adding requests 
        // to the current queue and checking if the queue is full.
        // Additionally, the stream-loop can signal a restart of the main loop in case of bad failures.
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    
                    // Initial creation (in the main loop)
                    let mut disposable_handoff_queue: Option<VecDeque<RequestUnit>> = Some(VecDeque::with_capacity(MAX_QUEUE_SIZE));
                
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).unwrap();
                    // TODO when fails, error restart_signal = True / or drop and continue stream

                    let request_string = String::from_utf8_lossy(&buffer[..]);
                    // TODO when fails, error restart_signal = True / or drop and continue stream

                    // Very basic parsing of the request (assuming POST)
                    if request_string.starts_with("POST") {
                        // let body_start = request_string.find("\r\n\r\n").unwrap_or(0) + 4;
                        // let request_body = request_string[body_start..].to_string();
                        let body_start = match request_string.find("\r\n\r\n") {
                            Some(index) => index + 4,
                            None => {
                                eprintln!("Error: Invalid request format. Could not find end of headers.");
                                // Handle the error appropriately (e.g., return an error response to the client)
                                // For now, let's just return 0 to avoid a crash, but you should replace this 
                                // with more robust error handling
                                0 
                            }
                        };
                        
                        let request_body = request_string[body_start..].to_string();
                        // TODO when fails, error restart_signal = True / or drop and continue stream

                        // Generate a unique request ID
                        let request_id = REQUEST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

                        // for endpoint-module: After parsing the request path
                        // let endpoint_name = &request_path[1..request_path.find('/').unwrap_or(request_path.len())]; // Remove leading '/'
                        let endpoint_name = &request_body[1..request_body.find('/').unwrap_or(request_body.len())]; // Remove leading '/'
                        
                        // for endpoint-module
                        let endpoint_module_name = format!("endpoint_modules/{}/module.rs", endpoint_name);

                        // Stream Decoupling: Store stream address in RequestUnit
                        let stream_addr = stream.peer_addr().unwrap(); 
                        let request_unit_struct = RequestUnit {
                            id: request_id,
                            endpoint_module_name: Some(endpoint_module_name),
                            body: request_body,
                            output_for_response: None,
                            stream_addr: stream_addr,
                            response_status: None, // Initialize response fields to None
                            response_headers: None,
                            response_body: None,
                        };
                        
                        // Insert the stream into the map
                        stream_map.insert(request_id, stream);

                        // if Idle
                        // Checks if the request handler is currently in the `Idle` state.
                        //
                        // This function loads the current state of the handler from the `HANDLER_STATE` atomic variable
                        // and compares it with the integer representation of the `Idle` state. 
                        // It returns `true` if the handler is `Idle` and `false` otherwise (if it's `Busy` or `Failed`).
                        //
                        // Note: This check only explicitly distinguishes between `Idle` and non-`Idle` states.
                        // It does not differentiate between `Busy` and `Failed` within this specific check. 
                        // However, the `else` block that follows this check handles both `Busy` and `Failed` states 
                        // by attempting to add the incoming request to the queue.
                        if HANDLER_STATE.load(Ordering::Relaxed) == HandlerState::Idle as usize {
                            /*
                            handler can be: 1 busy, 2. not_busy 3. failed
                            
                            A. look for quit-signal_to_restart (optional, if needed later)
                            B. if handler is not busy, give request+queue to handler & reset counter to 0
                            C. if handler is busy, check counter
                            E. if counter > MAX: drop request
                            F. if counter < MAX: check if there is an existing queue
                            G. if there is an existing queue: add request to quque
                            H: if there is no queue: make a queue and add request to queue
                            loop back 
                            */
                            // Request processing oc
                            // when this fails (everything will fail at some point)
                            // this should output a signal to set a 'restart' flag 
                            // Spawn the handler thread and pass the sender channel
                            // In main
                            
                            // Clone sender_clone inside the loop
                            let sender_for_thread = sender_clone.clone(); 
                            
                            thread::spawn(move || {
                                handler_of_request_and_queue(
                                    request_unit_struct,
                                    disposable_handoff_queue.take().unwrap(),
                                    sender_for_thread, // Pass a clone of sender_clone
                                );
                            });

                            // 1. handler_of_request_and_queue(request, quque)

                            
                            // Double Tap: make sure queue is removed
                            // When the handler finishes or fails (in the handler thread or stream-loop):
                            // let disposable_handoff_queue: Option<VecDeque<String>> = None; // Indicate that a new queue needs to be created 
                            
                            // 2. counter = zero
                            // Reset the queue counter
                            QUEUE_COUNTER.store(0, Ordering::Relaxed);

                            // 3. make a new empty disposable_handoff_queue
                            // let mut disposable_handoff_queue: Option<VecDeque<String>> = Some(VecDeque::with_capacity(MAX_QUEUE_SIZE));
                            
                            // if faile:
                            // exit/continue/break stream-loop/quit/reboot
                            
                            
                        } else {  // if NOT Idle: elif busy, elif failed
                            
                            // Handle busy/failed state (e.g., add to queue)
                            // Check if a queue exists and add requests to it
                            if let Some(queue) = &mut disposable_handoff_queue {
                                // ... (add requests to the queue) ...
                                // if queue is not full: add request to queue
                                if QUEUE_COUNTER.load(Ordering::Relaxed) < MAX_QUEUE_SIZE {

                                    // add request to queue!
                                    queue.push_back(request_unit_struct); // Call push_back on the VecDeque inside the Option
                     
                                    QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
                            } else {
                                // No queue available, create a new one 
                                // let mut disposable_handoff_queue: Option<VecDeque<String>> = Some(VecDeque::with_capacity(MAX_QUEUE_SIZE));
                                // ... (potentially add the current request to the new queue) ...
                            }
                            

                            } else {
                                // Ignore the request (queue is full)
                            }
                            
                            // increment counter
                            QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
                        }
                        
                        
                        if HANDLER_STATE.load(Ordering::Relaxed) == HandlerState::Failed as usize {
                            println!("Handler thread failed. Restarting..."); // Log the failure
                            break; // Exit the stream-loop to signal a restart 
                        }
                    }
                    // } else if request_string.starts_with("GET") {
                    //     /*
                    //     TODO: this "works" but it returns a document containing
                    //     the post-request format server return...
                    //     which is something...but should be...optimized...
                    //     */
                        
                    //     // let body_start = request_string.find("\r\n\r\n").unwrap_or(0) + 4;
                    //     // let request_body = request_string[body_start..].to_string();
                    //     let request_body = if let Some(index) = request_string.find('X') {
                    //         &request_string[index + 1..] 
                    //     } else {
                    //         "" // No query string found
                    //     };
                        
                    //     // // Parse query_string into parameters (e.g., using a library or manual splitting)
                    //     // let parameters: HashMap<String, String> = /* ... parsing logic ... */;
                        
                    //     // let request_body = query_string;
                    
                    //     // Generate a unique request ID
                    //     let request_id = REQUEST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                    
                    //     // Stream Decoupling: Store stream address in RequestUnit
                    //     let stream_addr = stream.peer_addr().unwrap(); 
                    //     let mut request_unit_struct = RequestUnit {
                    //         id: request_id,
                    //         endpoint_module_name: "".to_string(),
                    //         body: request_body.to_string(),
                    //         output_for_response: None,
                    //         stream_addr: stream_addr,
                    //         response_status: None, // Initialize response fields to None
                    //         response_headers: None,
                    //         response_body: None,
                    //     };
                        
                    //     // Insert the stream into the map
                    //     stream_map.insert(request_id, stream);

                    //     // if Idle
                    //     // Checks if the request handler is currently in the `Idle` state.
                    //     //
                    //     // This function loads the current state of the handler from the `HANDLER_STATE` atomic variable
                    //     // and compares it with the integer representation of the `Idle` state. 
                    //     // It returns `true` if the handler is `Idle` and `false` otherwise (if it's `Busy` or `Failed`).
                    //     //
                    //     // Note: This check only explicitly distinguishes between `Idle` and non-`Idle` states.
                    //     // It does not differentiate between `Busy` and `Failed` within this specific check. 
                    //     // However, the `else` block that follows this check handles both `Busy` and `Failed` states 
                    //     // by attempting to add the incoming request to the queue.
                    //     if HANDLER_STATE.load(Ordering::Relaxed) == HandlerState::Idle as usize {
                    //         /*
                    //         handler can be: 1 busy, 2. not_busy 3. failed
                            
                    //         A. look for quit-signal_to_restart (optional, if needed later)
                    //         B. if handler is not busy, give request+queue to handler & reset counter to 0
                    //         C. if handler is busy, check counter
                    //         E. if counter > MAX: drop request
                    //         F. if counter < MAX: check if there is an existing queue
                    //         G. if there is an existing queue: add request to quque
                    //         H: if there is no queue: make a queue and add request to queue
                    //         loop back 
                    //         */
                    //         // Request processing oc
                    //         // when this fails (everything will fail at some point)
                    //         // this should output a signal to set a 'restart' flag 
                    //         // Spawn the handler thread and pass the sender channel
                    //         // In main
                            
                    //         // Clone sender_clone inside the loop
                    //         let sender_for_thread = sender_clone.clone(); 
                            
                    //         thread::spawn(move || {
                    //             handler_of_request_and_queue(
                    //                 request_unit_struct,
                    //                 disposable_handoff_queue.take().unwrap(),
                    //                 sender_for_thread, // Pass a clone of sender_clone
                    //             );
                    //         });

                    //         // 1. handler_of_request_and_queue(request, quque)

                            
                    //         // Double Tap: make sure queue is removed
                    //         // When the handler finishes or fails (in the handler thread or stream-loop):
                    //         // let disposable_handoff_queue: Option<VecDeque<String>> = None; // Indicate that a new queue needs to be created 
                            
                    //         // 2. counter = zero
                    //         // Reset the queue counter
                    //         QUEUE_COUNTER.store(0, Ordering::Relaxed);

                    //         // 3. make a new empty disposable_handoff_queue
                    //         // let mut disposable_handoff_queue: Option<VecDeque<String>> = Some(VecDeque::with_capacity(MAX_QUEUE_SIZE));
                            
                    //         // if faile:
                    //         // exit/continue/break stream-loop/quit/reboot
                            
                            
                    //     } else {  // if NOT Idle: elif busy, elif failed
                            
                    //         // Handle busy/failed state (e.g., add to queue)
                    //         // Check if a queue exists and add requests to it
                    //         if let Some(queue) = &mut disposable_handoff_queue {
                    //             // ... (add requests to the queue) ...
                    //             // if queue is not full: add request to queue
                    //             if QUEUE_COUNTER.load(Ordering::Relaxed) < MAX_QUEUE_SIZE {

                    //                 // add request to queue!
                    //                 queue.push_back(request_unit_struct); // Call push_back on the VecDeque inside the Option
                     
                    //                 QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
                    //         } else {
                    //             // No queue available, create a new one 
                    //             // let mut disposable_handoff_queue: Option<VecDeque<String>> = Some(VecDeque::with_capacity(MAX_QUEUE_SIZE));
                    //             // ... (potentially add the current request to the new queue) ...
                    //         }
                            

                    //         } else {
                    //             // Ignore the request (queue is full)
                    //         }
                            
                    //         // increment counter
                    //         QUEUE_COUNTER.fetch_add(1, Ordering::Relaxed);
                    //     }
                        
                        
                    //     if HANDLER_STATE.load(Ordering::Relaxed) == HandlerState::Failed as usize {
                    //         println!("Handler thread failed. Restarting..."); // Log the failure
                    //         break; // Exit the stream-loop to signal a restart 
                    //     }
                    // }

                    // (maybe too old of a text comment block)4. Respond
                    /*
                    Check for Valid Response Data: We first check if the response_status, response_headers, 
                    and response_body fields in the RequestUnit have been populated.
                
                    Establish Connection: We use TcpStream::connect to establish a new connection 
                    to the client using the stream_addr stored in the RequestUnit.
                
                    Send Status Line: We construct the HTTP status line (e.g., "HTTP/1.1 200 OK\r\n") 
                    and send it through the stream.
                
                    Send Headers: We iterate through the response_headers and send each 
                    header line (e.g., "Content-Type: text/plain\r\n").
                
                    Send Empty Line: We send an empty line ("\r\n") to indicate the end of the headers.
                
                    Send Response Body: Finally, we send the response_body through the stream.
                    */
                    // 4?
                    // Receive the processed RequestUnit or error from the channel
                    if let Ok((request_id, result)) = receiver.recv() {
                        // Find the corresponding stream using the request ID
                        if let Some(mut stream) = stream_map.remove(&request_id) {
                            // Handle the result from the handler
                            match result {
                                Ok(processed_request) => {
                                    // Send successful response
                                    let response = format!(
                                        "HTTP/1.1 {} OK\r\nContent-Type: text/plain\r\n\r\n{}",
                                        processed_request.response_status.unwrap_or(200), // Get status or default to 200
                                        processed_request.response_body.unwrap_or_default() // Get body or default to empty
                                    );
                                    stream.write_all(response.as_bytes()).unwrap();
                                    stream.flush().unwrap(); // Flush the stream to ensure data is sent
                                }
                                Err(error_message) => {
                                    // Send error response
                                    let response = format!("HTTP/1.1 500 Internal Server Error\r\n\r\n{}", error_message);
                                    stream.write_all(response.as_bytes()).unwrap();
                                    stream.flush().unwrap(); // Flush the stream
                                }
                            }
                        } else {
                            eprintln!("Stream not found for request ID: {}", request_id);
                            // Handle the case where the stream is not found
                        }
                    } else {
                        eprintln!("Error receiving data from handler thread.");
                        // Handle the error appropriately
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
                // look for restart-flag from failure and signal larger restart exit
            }
        }

        // If the code reaches here, it means the listener loop has exited (e.g., due to an error)
        // The outer loop will restart, creating a fresh disposable_handoff_queue and listener
    } 
}
