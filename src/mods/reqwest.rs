use std::{process::exit, net::{TcpStream, SocketAddr}, fmt::Error, io::{Write, Read}, time::Duration, sync::{Arc, Mutex}};

use native_tls::{TlsConnector, TlsStream};

use super::{structs::{UrlItems, ReqError}, functions::LocalFunctions};
#[derive(Debug, Clone)]
pub struct Reqwest{
    url: &'static str,
    url_items: UrlItems,
    file_size: usize
}

impl Reqwest {

    pub fn new(url: &'static str) -> Result<Self, ReqError> {
        
        if let Some(url_items) = url.split_url(){

            let mut rest = Self {url_items: url_items, file_size: 0, url: url};

            if let Ok(f_size) = rest.file_size(){

                rest.file_size = f_size;

                Ok(rest)
    
            } else {
                Err(ReqError::ErrorFound("File Size ERRor!!!!!!"))
            }

        } else {

            Err(ReqError::ErrorFound(">>>>>> Invalid Url struct <<<<<<"))
        }

    }

    pub fn bytes<F: FnOnce(usize) + Copy>(&self, start: usize, end: usize, stop: Arc<Mutex<bool>>, call_back: F) -> Result<Vec<u8>, ReqError>{

        let url_items = self.url_items;

        let mut buff = [0;2048*10];
        
        let mut acum = vec![];

        let mut cont = 0;

        match url_items.is_secure {
            true => {

                match self.reqw_443(url_items) {
                    Ok(ref mut stream) => {
        
                        let header = self.header(&format!("GET /{} HTTP/1.0\r\n",url_items.path));
                        let header = Self::header_add_range(header, start, end);
        
                        stream.write_all(header.as_bytes()).unwrap();
        
        
                        loop {
                            if let Ok(size_of_resp) = stream.read(&mut buff) {
        
                                if size_of_resp == 0 || *stop.lock().unwrap() {break;}
                                let data = &buff[..size_of_resp];
                                acum.extend_from_slice(data);
                                
                                if cont == 1 {
        
                                    call_back(data.len());
                                    cont = 0;
                                }

                                cont +=1;
        
                            }
                        }
        
                        let head_size= acum.windows(4).position(|i| i == b"\r\n\r\n").unwrap();
                        let acum = acum[head_size+4..].to_vec();
                        Ok(acum)

                    },
                    Err(err) => {
                        let err = Box::leak(err.into_boxed_str());
                        Err(ReqError::ErrorFound(err))
                    }
                }
        
            },
            false => {
                //not implemented yet!!!
                match self.reqw_unsecure(url_items) {
                    Ok(ref mut stream) => {

                        let header = self.header(&format!("GET /{} HTTP/1.0\r\n", url_items.path));
                        let header = Self::header_add_range(header, start,end);

                        stream.write_all(header.as_bytes()).unwrap();

                        loop {
                            if let Ok(res_size) = stream.read(&mut buff) {

                                if res_size == 0 {break;}

                                let data = &buff[..res_size];
                                acum.extend_from_slice(data);

                                if cont == 50 {

                                    call_back(data.len());
                                    cont = 0;
                                }

                                cont +=1;
                            }
                        }

                        let head_size = acum.windows(4).position(|i| i == b"\r\n\r\n").unwrap();
                        let data = acum[head_size+4..].to_vec();
                        Ok(data)
                    },
                    Err(err) => {
                        let err = Box::leak(err.into_boxed_str());
                        Err(ReqError::ErrorFound(err))
                    }
                }
            }
        }
    
    }
    
    pub fn get_filesize(&self) -> usize {
        self.file_size
    }
    fn file_size(&self) -> Result<usize, ReqError>{

        match self.url_items.is_secure {
            true => {

                if let Ok(ref mut stream) = self.reqw_443(self.url_items) {

                    if let Ok((_, size)) = self.get_size_status_s(stream){

                        return Ok(size);
                    }
                }
            },
            false => {
                if let Ok(ref mut stream) = self.reqw_unsecure(self.url_items) {
                    
                    if let Ok((_,file_size)) = self.get_size_status_u(stream) {

                        return Ok(file_size);
                    }
                }
            }
        }

        Err(ReqError::ErrorFound("Error file size"))
    }

    ///# Secure
    /// get header and file-size<br>
    ///return Ok: (header, file_size)<br>
    ///return Err: (header)
    fn get_size_status_s(&self, ref mut stream: &mut TlsStream<TcpStream>) -> Result<(String, usize), String>{

        let (path,dns) = (self.url_items.path, self.url_items.dns);

        let header = self.header(&format!("HEAD /{path} HTTP/1.0\r\n"));

        stream.write_all(header.as_bytes()).unwrap();

        let mut header = String::new();

        if let Ok(s_r) = stream.read_to_string(&mut header) {

            let mut lines = header.lines();
            let status = lines.next().unwrap();
            //println!("{}", header);

            if status.contains("302") {
                println!("302 - not implemented yet!!!!!");exit(0);
                //self.get_size_status_s(stream).unwrap();
            } else 
            if status.contains("200") {

                let file_size = lines.find(|i| i.to_lowercase().contains("content-length")).unwrap();
                let file_size = file_size.split(":").last().unwrap().trim();
                let file_size = file_size.parse::<usize>().unwrap();

                Ok((header, file_size))

            } else {

                Err(header)
            }
        } else {
            Err(header)
        }
        
    }

    ///# Unsecure
    /// get header and file-size<br>
    fn get_size_status_u(&self, ref mut stream: &mut TcpStream) -> Result<(String, usize), String> {

        let header = self.header(&format!("HEAD /{} HTTP/1.0\r\n", self.url_items.path));

        //begins handshake
        stream.write_all(header.as_bytes()).unwrap();

        let mut header = String::new();
        if let Ok(_) = stream.read_to_string(&mut header) {

            let f_size = header.lines()
                .find(|i| i.to_lowercase().contains("content-length"))
                .unwrap()
                .split(":")
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            Ok((header, f_size))

        } else {
            Err("Error file size".to_owned())
        }

    }

    ///create default request header<br> 
    /// <b>return a new header String</b>
    /// # Example
    /// <h3>header("GET /image/img.jpg HTTP/1.0")
    fn header(&self, protocol_path: &str) -> String {

        let dns = self.url_items.dns;

        let mut header = String::new();
        header.push_str(&format!("{}", protocol_path));
        header.push_str(&format!("Host: {}\r\n", dns));
        header.push_str(&format!("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36\r\n"));
        header.push_str("\r\n");

        header
    }
    
    fn header_add_range(target: String, start: usize, end: usize) -> String{

        //let start = 0;
        //let end = 267;
        let new_line = format!("Range: bytes={}-{}\r\n", start, end);

        let target = target.clone();
        let lines = target.lines();
        let pos = lines.clone().into_iter().position(|i| i.contains("User-Agent")).unwrap();

        let header = lines
            .enumerate().map(|(idx,i)|{

                match idx== pos {
                    true => {
                        format!("{i}\r\n{new_line}")
                    },
                    false => {
                        format!("{i}\r\n")
                    }
                }

            })            
            .reduce(|ac, i| ac + i.as_str());

        header.unwrap()

    }

    fn reqw_443(&self, url_i: UrlItems) -> Result<TlsStream<TcpStream>, String> {

        let url_i = self.verify_312(url_i).unwrap();

        match std::net::TcpStream::connect(format!("{}:{}", url_i.dns, url_i.port)) {
            Ok(conn) => {

                match native_tls::TlsConnector::new() {

                    Ok(tls) => {

                        let stream = tls.connect(url_i.dns, conn).unwrap();
                        Ok(stream)

                    },
                    Err(err) => {

                        Err(err.to_string())
                    }
                }
            },
            Err(err) => {
                
                Err(err.to_string())

            }
        }

    }

    fn reqw_unsecure(&self, url_i: UrlItems) -> Result<TcpStream, String>{

        let url_i = self.verify_312(url_i).unwrap();

        let addr = format!("{}:{}", url_i.dns, url_i.port);
        match std::net::TcpStream::connect(&addr) {
            Ok(stream) => {

                Ok(stream)
            },
            Err(err) => {
                Err(err.to_string())
            }
        }

    }

    fn verify_312(&self, url_items: UrlItems) -> Result<UrlItems, String>{

        let header = self.header(&format!("HEAD /{} HTTP/1.0\r\n", self.url_items.path));
        let mut buff = String::new();
        let dns = url_items.dns;
        let port = url_items.port;

        let verify = |buff: String|{

            let buff = Box::leak(buff.into_boxed_str());

            let mut lines = buff.lines();
            let status = lines.next().unwrap();

            if status.contains("302") {

                let new_url = lines.filter_map(|i| {
                    if i.to_lowercase().starts_with("location:") {
                        let url = i.split_once(":").unwrap().1.trim();
                        Some(url)
                    } else {
                        None
                    }
                }).next().unwrap();

                let url_items = new_url.split_url().unwrap();

                Ok(url_items)

            } else {
                let url_items = self.url_items;

                Ok(url_items)
            }

        };

        match self.url_items.is_secure {

            true => {

                match std::net::TcpStream::connect(format!("{dns}:443")) {
                    
                    Ok(conn) => {

                        if let Ok(tls) = native_tls::TlsConnector::new() {
                            if let Ok(mut stream) = tls.connect(dns, conn) {

                                stream.write_all(header.as_bytes()).unwrap();

                                if let Ok(_) = stream.read_to_string(&mut buff) {

                                    verify(buff)

                                } else {
                                    Err("Error Found on verify_321 A".to_owned())
                                }
                            } else {
                                Err("Error Found on verify_321 B".to_owned())
                            }
                        } else {
                            Err("Error Found on verify_321 C".to_owned())
                        }
                    },
                    Err(err) => {

                        Err("Error Found on verify_321 D".to_owned())
                    }
                }
                
            },
            false => {


                match std::net::TcpStream::connect(format!("{dns}:{port}")) {

                    Ok(mut stream) => {

                        stream.write_all(header.as_bytes()).unwrap();

                        if let Ok(_) = stream.read_to_string(&mut buff) {

                            verify(buff)
                        } else {
                            Err("Error found!!!!! E".to_owned())
                        }
                    },
                    Err(err) => {
                        Err("Error found!!!!! F".to_owned())
                    }
                }
            }
        }

    }

}
