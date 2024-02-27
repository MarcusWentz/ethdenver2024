use http::{Response, StatusCode};

fn main() {
    let response = Response::builder()
      .status(StatusCode::MOVED_PERMANENTLY)
      .header("Location", "https://www.rust-lang.org/install.html")
      .body(())
      .unwrap();
    println!("{:?}",response);
}