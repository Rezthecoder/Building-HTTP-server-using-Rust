
use crate::{request::{ParseError, Request},response::Response,status_code::StatusCode};
use std::{io::Read, net::TcpListener};

//______________________:]@[-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^0990900]
pub trait Handler {
    fn handle_request(&mut self,request: &Request)->Response;
    fn handle_bad_request(&mut self,e:&ParseError)->Response{
        println!("failed to parse request {}",e);
        Response::new(StatusCode::BadRequest, None)
    }
    
}


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self,mut handler: impl Handler) {
        println!("Server is listening on port {}", self.addr);
        
        let listener = TcpListener::bind(&self.addr).unwrap();
      loop {
          match listener.accept() {
            Ok((mut stream,_))=>{
                let mut buf = [0; 1024];
             match stream.read(&mut buf){
                Ok(_)=>{
                    println!("Received the request from:  {}", String::from_utf8_lossy(&buf));
             let response = match Request::try_from(&buf[..]){
                    Ok(request)=>{
                    handler.handle_request(&request)

                    }
                    Err(e)=>{
                      handler.handle_bad_request(&e)
                 }
                };
                if let Err(e) =response.send(&mut stream){
                    print!("failed to send response ,{}",e);
                }
            }
            Err(e)=>{
                println!("failed to read from connection : {}",e);
            }     
        }
    }
    Err(e) => print!("error occured {}",e),
}
          }
        }
    }