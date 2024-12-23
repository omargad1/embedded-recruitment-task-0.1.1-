# solution

## **Server Implementation**

### **1. Non-blocking Mode with Event-driven Approach**
- **Issue**: The server used `self.listener.set_nonblocking(true)` with `thread::sleep`, which is inefficient.
- **Fix**: Implemented an event-driven approach using `tokio::select!` and asynchronous `TcpListener` and `TcpStream`.



### **2. Single-threaded Client Handling**
- **Issue**: The server processed client connections sequentially in a single thread.
- **Fix**: Added `tokio::spawn` to handle each client in a separate asynchronous task.


### **3. Infinite Loop in `run`**
- **Issue**: The server's `run` method ran indefinitely without a graceful shutdown mechanism.
- **Fix**: Introduced a `shutdown_notify` mechanism to signal tasks for graceful termination.



### **4. Dynamic Port Handling**
- **Issue**: The server used a fixed port, which could lead to conflicts or reduced flexibility in testing.
- **Fix**: Used dynamic port allocation (`localhost:0`) and retrieved the assigned port using `listener.local_addr()`.



### **5. Message Size Validation**
- **Issue**: The server did not validate incoming message sizes, leading to potential crashes on oversized payloads.
- **Fix**: Added validation for incoming messages to ensure they do not exceed the defined `MAX_MESSAGE_SIZE`.


### **6. Blocking I/O**
- **Issue**: Blocking I/O methods (`read`, `write_all`) limited scalability and responsiveness.
- **Fix**: Replaced blocking calls with asynchronous equivalents (`stream.read` and `stream.write_all`).



### **7. Error Handling**
- **Issue**: Errors during decoding, unsupported message types, and connection acceptance were not handled properly.
- **Fix**: Added detailed logging and handling for decoding errors, unsupported message types, and connection acceptance failures.


### **8. Resource Management**
- **Issue**: The server did not properly clean up resources when a client disconnected.
- **Fix**: Explicitly handled client disconnections by detecting when `bytes_read == 0`.



### **9. Concurrency Management**
- **Issue**: No limits were imposed on the number of concurrent tasks spawned.
- **Fix**: Added a placeholder for implementing concurrency limits using tools like `tokio::sync::Semaphore`.



### **10. Logging Improvements**
- **Issue**: Logs were verbose and repetitive under high load.
- **Fix**: Refined logging for readability and introduced more meaningful log messages.



## **Client handling Implementation**

### **1. Fixed Buffer Size**
- **Issue**: Hardcoded buffer size (`512 bytes`) caused issues with larger messages.
- **Fix**: Updated buffer to dynamically match the maximum allowed message size.



### **2. Lack of Continuous Communication**
- **Issue**: The `handle` method processed only one message per connection.
- **Fix**: Wrapped the `read` and `handle` logic in a loop for continuous communication.



### **3. No Encoding Error Handling**
- **Issue**: Errors during `encode_to_vec` were not handled.
- **Fix**: Added error handling for encoding failures.



### **4. Incomplete Error Logging for Decoding**
- **Issue**: Decoding errors were not fully logged.
- **Fix**: Enhanced error logging for decoding failures.



### **5. No Validation for Received Data**
- **Issue**: The client did not validate the type of received messages.
- **Fix**: Implemented validation for message types and added handling for unexpected messages.


### **6. Timeout Handling**
- **Issue**: The client could hang indefinitely if the server sent no data.
- **Fix**: Added timeouts for `read` and `write` operations.

------------------------------------------------------------------------------------------------------------------

## **Summary**

| **Component** |                 **Bug**                   |          **Fix**                    |
|---------------|-------------------------------------------|-------------------------------------|
| Server        | Non-blocking mode inefficiency            | Event-driven approach               |
| Server        | Single-threaded handling                  | Spawned async tasks for clients     |
| Server        | Infinite loop in `run`                    | Graceful shutdown with notifications|
| Server        | Dynamic port handling                     | Dynamic port allocation             |
| Server        | Message size validation                   | Added buffer size checks            |
| Client        | Fixed buffer size                         | Dynamically sized buffer            |
| Client        | Lack of continuous communication          | Wrapped `read` in a loop            |
| Client        | No validation for received data           | Added validation logic              |



# Added Tests

## 1. `test_malformed_message`
- Purpose: Validates the server's ability to handle malformed messages gracefully.
- What it Tests:
  - Ensures the server detects malformed data and does not crash.
  - Verifies appropriate logging or error messages are generated.
- Expected Outcome: The server rejects the malformed message without impacting other operations.



## 2. `test_add_response`
- Purpose: Tests the `AddRequest` and `AddResponse` functionality.
- What it Tests:
  - Validates that the server correctly processes an `AddRequest`.
  - Confirms the server returns an `AddResponse` with the correct sum.
- Expected Outcome: The server responds with the correct sum based on the operands provided in the `AddRequest`.



## 3. `test_concurrent_requests`
- Purpose: Tests the server's ability to handle multiple concurrent requests from different clients.
- What it Tests:
  - Validates the server's concurrency using `tokio::spawn`.
  - Ensures that each client request is processed independently and without conflicts.
- Expected Outcome: All clients receive correct responses simultaneously, with no delays or errors.



## 4. `test_high_client_load`
- Purpose: Simulates high client load to test server scalability and robustness.
- What it Tests:
  - Verifies the server's performance and resource management under high load.
  - Detects potential resource exhaustion or task scheduling bottlenecks.
- Expected Outcome: The server remains responsive, handling all client connections and requests correctly, even under heavy load.




# Test Cases and Results

## **Test Cases**

The following test cases were executed to ensure the robustness, scalability, and functionality of the server and client implementations:

1. **`test_client_connection`**  
   Validates the basic client connection to the server.

2. **`test_client_echo_message`**  
   Tests the echo message functionality of the server.

3. **`test_client_add_request`**  
   Verifies the server correctly processes an `AddRequest` and returns an `AddResponse`.

4. **`test_malformed_message`**  
   Ensures the server handles malformed messages gracefully without crashing.

5. **`test_add_response`**  
   Tests the correctness of the server's `AddResponse` functionality.

6. **`test_multiple_clients`**  
   Checks the server's ability to handle multiple simultaneous client connections.

7. **`test_multiple_echo_messages`**  
   Validates the server can process multiple echo messages within a single client connection.

8. **`test_concurrent_requests`**  
   Ensures the server can handle multiple concurrent requests efficiently.

9. **`test_high_client_load`**  
   Simulates high client load to test the server's performance and stability under stress.



## **Test Results**

- All tests were executed successfully, and no failures or errors were encountered.
- Test results have been logged in the file `test_results.log`.



### **How to View Test Results**
To verify the test results:
1. Open the `test_results.log` file in the project directory.
2. Review the detailed output for each test case, including pass/fail statuses and any relevant logs.



### **Command Used to Generate Logs**
The following command was used to run the tests and capture the results:

```bash
cargo test -- --nocapture > test_results.log
```