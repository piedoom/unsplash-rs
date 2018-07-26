//! This simple source client grabs images without need to use the official API.
//! This way we can avoid rate limiting for simple random image tasks
use std::fmt;

const BASE_URL = "https://source.unsplash.com"

/// Used to query specific image sizes
struct ImageSize {
    x: usize,
    y: usize,
}

/// Configure options for the request
struct RequestOptions {
    image_size: Option<ImageSize>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        RequestOptions {
            image_size: None,
        }
    }
}

impl fmt::Display for ImageSize {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("#{}x#{}", self.x, self.y);
        Ok(())
    }
}

struct Client {
    http_client: hyper::Client;

    /// return a random image with an optional size parameter
    pub fn get_random(size: Option<ImageSize>) -> Image {

    }

    /// perform a generic request
    fn get(options: Option<RequestOptions>) -> String {
        // perform the request
        let mut response = http_client.get("#{}", BASE_URL)
            .header(Connection::close()).send().unwrap();
        // read the response to memory
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        body
    } 
}

impl Default for Client {
    fn default() -> Self {
        Client {
            http_client: hyper::Client::new(),
        }
    }
}