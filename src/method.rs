use std::str::FromStr;

#[derive(Debug)]
pub enum  Method {
    GET,
    POST,
    DELETE,
    PUT,
    
}

impl FromStr for Method {
    type Err=MethodError ;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" =>Ok(Self::GET),
            "POST" =>Ok(Self::POST),
            "DELETE" =>Ok(Self::DELETE),
            "PUT" =>Ok(Self::PUT),
            _ => Err(MethodError),
        }

       
    }
}


pub struct MethodError;