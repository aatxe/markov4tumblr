#![allow(unstable)]
#![feature(slicing_syntax)]

extern crate hyper;
extern crate markov;
extern crate "rustc-serialize" as rustc_serialize;

use std::borrow::ToOwned;
use std::error::Error;
use std::io::{IoError, IoErrorKind, IoResult};
use hyper::Url;
use hyper::client::Client;
use markov::Chain;
use rustc_serialize::json::decode;

fn main() {
    let key = ""; // populate me with a tumblr API key.
    let blogs = [""]; // populate me with tumblr blogs.
    let mut chain = Chain::for_strings();
    let mut client = Client::new();
    println!("Populating chain...");
    for blog in blogs.iter() {
        let url = format!("http://api.tumblr.com/v2/blog/{}/posts/text?api_key={}&filter=text", 
                          blog, key);
        let res = client.get(Url::parse(&url[]).unwrap()).send().unwrap().read_to_string().unwrap();
        if let Ok(resp) = TumblrResponse::decode(&res[]) {
            if let Some(resp) = resp.response {
                for post in resp.posts.iter() {
                    let cleaned = post.body.replace("\n", ".").replace("(", ".")
                                           .replace(")", ".").replace("\"", ".");
                    for sentence in cleaned.split_str(".") {
                        chain.feed_str(sentence);
                    }
                }
            }
        }
    }
    println!("Saving chain...");
    chain.save_utf8("output.json").unwrap();
    println!("Done.");
}

#[derive(RustcDecodable)]
struct TumblrResponse {
    response: Option<Response>,
}

impl TumblrResponse {
    pub fn decode(string: &str) -> IoResult<TumblrResponse> {
        decode(string).map_err(|e| IoError {
            kind: IoErrorKind::InvalidInput,
            desc: "Failed to decode response.",
            detail: Some(e.description().to_owned()),
        })
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
