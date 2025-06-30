//i'm going to be doing a lot of disecting here since the book repackages tokio and some base crates such as futures
//which i think is not pedagogical as i gain no insight on the inner workings of rust
//except things to get messy and potentially stay messy


pub mod asyncs{
//use std::{future::Future, pin::pin};
    
    pub struct Examples{
    }

    impl Examples{

        //stealing code
        //https://github.com/rust-lang/book/blob/8a6d44e45b7b564eeb6bae30507e1fbac439d72d/packages/trpl/src/lib.rs#L102C1-L104C2
        // pub async fn get(url: &str) -> Response {
        //     //https://docs.rs/http/latest/http/response/struct.Response.html
        //     Response(reqwest::get(url).await.unwrap())//not request?
        // }

        //SIGH, WELCOME TO ANOTHER RUST ADVENTURE
    }

    //trying to rebuild a barebones http client with std packages only

    use std::{collections::HashMap, future::{self}, io::{Read, Write}, net::TcpStream, task::Poll};


    //just one crate...
    use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};

    #[derive(Debug)]
    pub struct MyClient{
        client: SslStream<TcpStream>,
        pub absolute_url: String
    }

    impl MyClient{
        //https://doc.rust-lang.org/std/net/struct.TcpStream.html
        pub fn new(url: String) -> Self{
            let stream = TcpStream::connect(format!("{}", url))
            .expect("to connect");
            let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
            builder.set_verify(SslVerifyMode::NONE);
            let stream = builder.build().connect(&url, stream).unwrap();
            
            //let builder = connector.connect(&url, stream).unwrap();
            println!("Connection established to {}", url);
                
            MyClient{
                client: stream,
                absolute_url: url
            }

        }
        pub fn async_get(&mut self, headers: HttpHeader<'_>) -> String{
            /*
            C# with Task Parralell Libray
            public Task<String> Get(...)
            {
                //Task effectively spawns a thread whose inner work is implemented as a delegate
                //()=> {...} goes by many names, lambda expression, anonymous functions, arrow functions
                //or even specific Types such as Action, Funcs, maybe even EventCallBack(s)
                //s in essence they syntactic sugar, for delgate(Identifier(*args)) where identifier is a method
                //defined elsewhere
                return Task.Run(() => {
                    ...
                });
            }

            usage:
            public async method(...)
            {
                await Get();
            }
             */
            //java like future?
                    println!("request made with headers:\n{}", headers.as_str());
            let _ = async{

                let t = future::poll_fn(move |_| -> Poll<String>{
                    let _ = &mut self.client.write_all(headers.as_str().as_bytes());
                    println!("request made with headers:\n{}", headers.as_str());
                    let mut encoded_response = Vec::new();
                    let _size = &mut self.client.read_to_end(&mut encoded_response)
                    .expect("to read");

                    let response =  String::from_utf8_lossy(&encoded_response);
                    let mut data = response.split("\r\n\r\n").enumerate();

                    let _response_headers: &str = data.next().unwrap().1; 
                    let body = data.next().unwrap().1; 
                
                    Poll::Ready(body.to_owned())
                }).await;
                        println!("{t}");
            };
            "asdf".to_owned()
        }

        
        pub fn get(&mut self, headers: HttpHeader<'_>) -> String{
            let _ = &mut self.client.write_all(headers.as_str().as_bytes());
            println!("request made with headers:\n{}", headers.as_str());
            let mut encoded_response = Vec::new();
            let _size = &mut self.client.read_to_end(&mut encoded_response)
            .expect("to read");

            let response =  String::from_utf8_lossy(&encoded_response);
            let mut data = response.split("\r\n\r\n").enumerate();

            let _response_headers: &str = data.next().unwrap().1; 
            let body = data.next().unwrap().1; 
            
            
            body.to_owned()
        }
    }
    
    pub enum HttpMethods{
        GET,
        CONNECT,
    }

    impl HttpMethods{
        pub fn as_str(&self) -> String{
            match self {
                HttpMethods::GET => "GET".to_string(),
                HttpMethods::CONNECT => "CONNECT".to_string()
            }
        }
    }

    //the things we take for granted...
    //https://datatracker.ietf.org/doc/html/rfc2616#section-5
    pub struct HttpHeader<'a>{
        pub method: String,
        pub http_version: &'a str,
        pub uri: &'a str,
        pub absolute_uri: &'a str,
        //pub port: &'a str,
        pub headers: HashMap<&'a str, &'a str>
    }
    impl HttpHeader<'_>{
        pub fn as_str(&self) -> String{
            format!("{method} /{uri} HTTP/{http_version}\r\n\
            Host: {absolute_uri}\r\n\
            Connection: close\r\n\
            Accept: image/jpg\r\n\
            \r\n{body}",
            method=self.method, uri=self.uri,http_version = self.http_version,
            absolute_uri = self.absolute_uri,// port=self.port,
            //needs a crate so im avoiding this for now, ill implement my own later...maybe
            //headers = self.headers.iter().map(|(k,v)| format!("{}: {}", k,v)).join("\n"),
            body="")
        }
    }
}