

### **Simple Single Threaded HTTP Server in Rust**

This repository contains a basic HTTP server implementation written in Rust. It demonstrates how to:

* Handle incoming TCP connections
* Parse HTTP requests (limited to GET methods)
* Handle basic errors (404 Not Found, 405 Method Not Allowed, 500 Internal Server Error)

**Error Handling:** 

I have done error handling for the foloowing errors:
1. **404 Not Found**: Try typing random address in the url and you will get 404 Not Found error. Example: `http://127.0.0.1:7878/not_exitsts` 
2. **405 Method Not Found**: Try submitting Login credentials in the Login page.
3. **500 Internal Server error**: Will get this error if the requested file does not exist in the server.Try clicking on `Click me to get 500 error`

**How to Run:**


1. **Clone the Repository:**
3. **Navigate to the Project Directory:**
4. **Compile and Run the Server:**
   ```bash
   cd src #This is necessary for the server to access the html files, else will give internal server error.
   cargo run
   ```

This will start the server on port `7878` by default.

**Using the Server:**

* Once the server is running, you can access it using a web browser by visiting `http://localhost:7878/`.
* You can navigate to specific pages by appending the filename to the URL, e.g., `http://localhost:7878/index.html`, `http://localhost:7878/About.html`, and so on.
