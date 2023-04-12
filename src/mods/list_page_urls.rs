use std::{io::{Write, Read}, process::exit};

use super::functions::LocalFunctions;

pub struct SiteUrls;

impl SiteUrls {
    pub fn get_urls(site_url: &'static str) -> Option<(&'static str,Vec<&'static str>)>{

        if let Some(dirs) = Self::filter_links(site_url, "Season"){

            for page in dirs {

                let url = format!("{site_url}{page}");                
                let url = Box::leak(url.into_boxed_str());

                println!("\n>> {} <<", page);
                if let Some(files)=Self::filter_links(url, "S"){

                    for file in files {

                        println!("{}", file)
                    }
                }

            }
        };
        exit(0);
        todo!()
    }

    fn filter_links(url: &'static str, target: &'static str) -> Option<Vec<&'static str>>{

        let (header, source) = Self::get_souce(url).unwrap();

        let lst = source.split("href").filter_map(|i|{

            match i.contains(target) {
                true => {

                    let dr = i.get_slice("\"","\"").unwrap().0;
                    Some(dr)
                },
                false => None
            }
        }).collect::<Vec<_>>();

        if lst.len()> 0 {
            Some(lst)
        } else {
            None
        }


    }
    
    fn get_souce<'a>(url: &'a str) -> Result<(&'a str, &'a str), &'a str>{

        let (dns, path) = url.get_slice("//", "/").unwrap();
        if let Ok(mut stream) = std::net::TcpStream::connect(format!("{dns}:80")) {

            let mut header = String::new();
            header.push_str(&format!("GET /{path} HTTP/1.0\r\n"));
            header.push_str(&format!("Host: {dns}\r\n"));
            header.push_str("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36\r\n\r\n");
        
            stream.write_all(header.as_bytes()).unwrap();

            let mut data = String::new();

            if let Ok(_) = stream.read_to_string(&mut data) {

                let data = Box::leak(data.into_boxed_str());

                let (header, source) =  data.split_once("\r\n\r\n").unwrap();

                let status = header.lines().next().unwrap();

                if status.contains("200") {

                    Ok((header, source))
                } else {
                    Err(status)
                }
                
            } else {
                Err("Error found!!!!!")
            }   
        } else {
            Err("Error found!!!")
        }


    }

}