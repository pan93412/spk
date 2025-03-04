use std::sync::{Arc};
use std::sync::atomic::{AtomicBool};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::h2tp::cfg::ATOMIC_ORDERING;
use crate::h2tp::message::Request;

pub struct Conn {
	stream: TcpStream,
	server_is_closing: Option<Arc<AtomicBool>>,
}

impl Conn {
	pub fn new(stream: TcpStream, server_is_closing: Option<Arc<AtomicBool>>) -> Self {
		return Self { stream, server_is_closing };
	}

	pub async fn handle(&mut self) {
		let mut req = Request::new();
		loop {
			match req.from(&mut self.stream).await {
				Some(e) => {
					if !e.is_empty() {
						println!("{:?}", e);
					}
					break;
				}
				None => {
					println!("{:?}", &req);
					self.stream.write(b"HTTP/1.0 200 OK\r\nContent-Length: 11\r\n\r\nHello World").await.err();
				}
			}
			match &self.server_is_closing {
				Some(closing) => {
					if closing.load(ATOMIC_ORDERING) {
						return;
					}
				}
				None => {}
			}
			req.clear();
		}
		self.stream.flush().await.err();
	}
}