use crate::method::{Method, MethodError};
use crate::query_string::*;
use std::str; // Assuming Method is defined in the same crate
use std::str::Utf8Error; // Assuming Method is defined in the same crate
use std::{convert::TryFrom, error::Error,fmt::{Debug, Display, Formatter, Result as fmtResult}};


#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}


impl<'buf>  Request<'buf> {
    pub fn path(&self) ->&str{
        &self.path
    }
    pub fn method(&self)->&Method{
        &self.method
    }
    
    pub fn query_string(&self)->Option<&QueryString>{
        self.query_string.as_ref()

    }
}



impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

 let request = str::from_utf8(buf)?;
 let (method,request) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
 let ( mut path,request) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
 let (protocol,_) =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;

 if protocol != "HTTP/1.1" {
     return Err(ParseError::InvalidProtocol)
 }


let method:Method = method.parse()?;
let mut query_string = None;
if let Some(i) = path.find('?'){
    query_string = Some(QueryString::from(&path[i +1..]));
    path = &path[..i];

}
Ok(Self{
    path:path,
    query_string: query_string,
    method:method,

}
)

    
}

}


fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
  None
}
    

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidMethod,
    InvalidProtocol,
    
}

impl ParseError{
    fn message(&self)->&str {
        match self  {
    Self::InvalidEncoding=> "Invalid Encoding",
    Self::InvalidRequest=> "Invalid Request",
    Self::InvalidMethod=> "Invalid Method",
    Self::InvalidProtocol=> "Invalid Protocol",


            
        }
        
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
Self::InvalidMethod
    }
}



impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult{
        write!(f,"{}",self.message())
    }
    
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult{
        write!(f,"{}",self.message())
    }
    
}

impl Error for ParseError {
  
}
