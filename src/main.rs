extern crate hyper;
extern crate markov;
extern crate rustc_serialize;

use std::io::{Error, ErrorKind, Result};
use std::io::prelude::*;
use hyper::Url;
use hyper::client::Client;
use markov::Chain;
use rustc_serialize::json::decode;

fn main() {
    let key = ""; // populate me with a tumblr API key.
    let blogs = [""]; // populate me with tumblr blogs.
    let mut chain: Chain<String> = Chain::new();
    let mut client = Client::new();
    println!("Populating chain...");
    for blog in blogs.iter() {
        let url = format!("http://api.tumblr.com/v2/blog/{}/posts/text?api_key={}&filter=text",
                          blog, key);
        let mut res = String::new();
        client.get(Url::parse(&url).unwrap()).send().unwrap().read_to_string(&mut res).unwrap();
        if let Ok(resp) = TumblrResponse::decode(&res) {
            if let Some(resp) = resp.response {
                for post in resp.posts.iter() {
                    let cleaned = post.body.replace("\n", ".").replace("(", ".")
                                           .replace(")", ".").replace("\"", ".");
                    for sentence in cleaned.split(".") {
                        chain.feed_str(sentence);
                    }
                }
            }
        }
    }
    println!("Saving chain...");
    // FIXME no serialization support :(
    // chain.save_utf8("output.json").unwrap();
    println!("Done.");
}

#[derive(RustcDecodable)]
struct TumblrResponse {
    response: Option<Response>,
}

impl TumblrResponse {
    pub fn decode(string: &str) -> Result<TumblrResponse> {
        decode(string).map_err(|_|
            Error::new(ErrorKind::InvalidInput, "Failed to decode response.")
        )
    }
}

#[derive(RustcDecodable)]
struct Response {
    posts: Vec<Post>
}

#[derive(RustcDecodable)]
struct Post {
    body: String
}
