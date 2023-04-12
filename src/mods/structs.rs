#[derive(Debug, Clone, Copy)]
pub struct UrlItems{

    pub dns: &'static str,
    pub path: &'static str,
    pub port: &'static str,
    pub is_secure: bool
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReqError {

    ErrorFound(&'static str),

} 
