#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

use http::{Request, Response, Version};
//use httparse::Request as ParsedRequest;
use httparse::Request as ParsedRequest;
use httparse::Response as ParsedResponse;
use json::parse;
use json_core::Outputs;

//fn parse_raw_request(raw_request: &str) -> http::request::Builder {
//fn parse_raw_request(raw_request: &str) -> Request<()> {
// fn parse_raw_request(raw_request: &str) -> ([httparse::Header<'_>; 64], ParsedRequest<'_, '_>) {
//     // Build HTTP request
//     // let mut request: http::request::Builder = Request::builder();
//     let mut headers: [httparse::Header<'_>; 64] = [httparse::EMPTY_HEADER; 64];
//     let mut req: ParsedRequest<'_, '_> = ParsedRequest::new(&mut headers);
//     let _ = req.parse(raw_request.as_bytes());
//     (headers, req)
// }

/*
fn parse_raw_request<'a>(raw_request: &'a str) -> ParsedRequest<'a, 'a> {
    // Build HTTP request
    //let mut request: http::request::Builder = Request::builder();
    let mut headers: [httparse::Header<'_>; 64] = [httparse::EMPTY_HEADER; 64];
    let mut req: ParsedRequest<'_, '_> = ParsedRequest::new(&mut headers);
    let _ = req.parse(raw_request.as_bytes());
    req
}
*/

// This works!
fn parse_raw_response_h<'a>(
    raw_response: &'a str,
    headers: &'a mut [httparse::Header<'a>; 64],
    //headers: [httparse::Header<'a>; 64],
) -> (
    ParsedResponse<'a, 'a>,
    Result<httparse::Status<usize>, httparse::Error>,
) {
    let mut req: ParsedResponse<'_, '_> = ParsedResponse::new(headers);
    let offset: Result<httparse::Status<usize>, httparse::Error> =
        req.parse(raw_response.as_bytes());
    (req, offset)
}

// This works!
fn parse_raw_request_h<'a>(
    raw_request: &'a str,
    headers: &'a mut [httparse::Header<'a>; 64],
    //headers: [httparse::Header<'a>; 64],
) -> ParsedRequest<'a, 'a> {
    let mut req: ParsedRequest<'_, '_> = ParsedRequest::new(headers);
    let _ = req.parse(raw_request.as_bytes());

    req
}

/*
   let method = req.method.unwrap();
   let path = req.path.unwrap();

   // let version = req.version.unwrap();
   //let z = Version::HTTP_11;

   // Extract method, path, and version
   // let method = req.method.ok_or("Method not found")?;
   // let path = parsed_request.path.ok_or("Path not found")?;
   let version = match req.version {
       Some(1) => Version::HTTP_11,
       //_ => return Err("Invalid HTTP version"),
       _ => return Request::builder(),
       //_ => return Request::builder().unwrap(),
       //_ => return Request::builder().body(()).unwrap(),
   };

   // let buf = b"GET /index.html HTTP/1.1\r\nHost";
   // assert!(req.parse(buf)?.is_partial());
   // // a partial request, so we try again once we have more data
   // let buf = b"GET /index.html HTTP/1.1\r\nHost: example.domain\r\n\r\n";
   // assert!(req.parse(buf)?.is_complete());

   /////////////////////////////////////////////
   // Parse the raw request
   //let mut headers = [httparse::EMPTY_HEADER; 64];
   //let mut parsed_request = ParsedRequest::new(&mut headers);
   // let _ = parsed_request
   //     .parse(raw_request.as_bytes())
   //     .map_err(|_| "Failed to parse HTTP request")?;

   // Extract method, path, and version
   // let method = parsed_request.method.ok_or("Method not found")?;
   // let path = parsed_request.path.ok_or("Path not found")?;
   // let version = match parsed_request.version {
   //     Some(1) => Version::HTTP_11,
   //     _ => return Err("Invalid HTTP version"),
   // };

   let mut request: http::request::Builder = Request::builder();

   request.method(method);
   request.uri(path);
   request.version(version);

   let request2 = Request::builder()
       .method(method)
       .uri(path)
       .version(version)
       //.header("User-Agent", "awesome/1.0")
       .body(())
       .unwrap();

   //let mut req_headers = req.headers;
   //for header in req_headers {
   //    // println!("{:?}", header);
   //    println!("HEADER");
   //    println!("{:?}", header.name);
   //    println!("{:?}", header.value);
   //    request.header(name, value);
   //}

   // for header in req.headers {
   //     header.

   //     }
   //     if Some(header.name) {}
   //     // if let Some(name) = header.name {
   //     //     if let Some(value) = header.value {
   //     //         request.header(name.unwrap(), value);
   //     //     }
   //     // }
   // }

   // Add headers
   // for header in parsed_request.headers {
   //     if let Some(name) = header.name {
   //         if let Some(value) = header.value {
   //             request.header(name, value);
   //         }
   //     }
   // }

   // Build the request
   // let request = request.body(()).map_err(|_| "Failed to build request")?;
   // Ok(request)
   request
*/
//}

fn main() {
    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());

    let raw_request = "GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Mozilla/5.0\r\n\r\n";

    let http_req = "GET /index.html HTTP/1.1
    Host: www.example.com
    User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
    Accept-Language: en-US,en;q=0.9
    Connection: close";
    //let req = parse_raw_request(raw_request);

    let mut req_headers: [httparse::Header<'_>; 64] = [httparse::EMPTY_HEADER; 64];
    let req = parse_raw_request_h(&raw_request, &mut req_headers);

    // let req = parse_raw_request(&raw_request);

    // let mut headers = [httparse::EMPTY_HEADER; 64];
    // let mut req = ParsedRequest::new(&mut headers);
    // let _ = req.parse(raw_request.as_bytes());
    let method = req.method.unwrap();
    let path = req.path.unwrap();
    let vers = req.version.unwrap();
    println!("----REQUEST DATA");
    println!("{:?}", method);
    println!("{:?}", path);
    println!("{:?}", vers);

    for header in req.headers {
        // println!("{:?}", header);
        // let val = header.value;
        let header_val = std::str::from_utf8(header.value).expect("invalid utf-8");
        println!("HEADER");
        println!("{:?}", header.name);
        // println!("{:?}", header.value);
        println!("{:?}", header_val);
        // request.header(name, value);
    }

    //request[0].assert!(request.uri() == "https://jsonplaceholder.typicode.com/todos/1");

    let raw_response = "HTTP/1.1 200 OK
Date: Sat, 28 Feb 2024 12:00:00 GMT
Server: Apache/2.4.41 (Unix)
Last-Modified: Thu, 20 Feb 2024 10:00:00 GMT
ETag: \"abcd-5a3e-dc532e5472b80\"
Content-Length: 128
Content-Type: text/html
Connection: close

<!DOCTYPE html>
<html>
<head>
  <title>Example Page</title>
</head>
<body>
  <h1>Hello, World!</h1>
</body>
</html>
";

    let mut resp_headers: [httparse::Header<'_>; 64] = [httparse::EMPTY_HEADER; 64];
    let (resp, offset) = parse_raw_response_h(&raw_response, &mut resp_headers);
    let offset_i: httparse::Status<usize> = offset.unwrap();
    let complete = offset_i.is_complete();
    let offset_i_val = offset_i.unwrap();
    let body = &raw_response[offset_i_val..];

    let code = resp.code.unwrap();
    let reason = resp.reason.unwrap();
    let version = resp.version.unwrap();

    println!("--- RESPONSE DATA");
    println!("{:?}", code);
    println!("{:?}", reason);
    println!("{:?}", version);
    println!("{:?}", body);

    for header in resp.headers {
        // println!("{:?}", header);
        // let val = header.value;
        let header_val = std::str::from_utf8(header.value).expect("invalid utf-8");
        println!("HEADER");
        println!("{:?}", header.name);
        // println!("{:?}", header.value);
        println!("{:?}", header_val);
        // request.header(name, value);
    }

    //let f = Request::from(http_req.to_string());

    // Example of creating a request
    // {'userId': 1, 'id': 1, 'title': 'delectus aut autem', 'completed': False}
    // let request = Request::builder()
    //     .uri("https://jsonplaceholder.typicode.com/todos/1")
    //     .header("User-Agent", "awesome/1.0")
    //     .body(())
    //     .unwrap();
    // println!("{:?}", request);

    //assert!(request.uri() == "https://jsonplaceholder.typicode.com/todos/1");

    // Key line - parsing json
    let data = parse(&data).unwrap();
    //let raw_data = &data["obj_field"]["string_subfield"];
    let raw_data = &data["obj_field"]["array_subfield"][1];

    let proven_val = raw_data.as_str().unwrap().to_string();
    let out = Outputs {
        data: proven_val,
        hash: sha,
    };
    env::commit(&out);
}
