use std::{process::exit, iter::Flatten, io::{Read, Write}};

use super::structs::UrlItems;


pub trait LocalFunctions {

    fn get_slice<'a>(&'a self, start: &'a str, end: &'a str) -> Option<(&'a str, &'a str)>;
    fn split_url(&'static self) -> Option<UrlItems>;
}

impl LocalFunctions for str {
    
    fn get_slice<'a>(&'a self, start: &'a str, end: &'a str) -> Option<(&'a str, &'a str)> {
        
        if let Some((_, next)) = self.split_once(start) {
            return next.split_once(end);
        }
        None
    }

    fn split_url(&'static self) -> Option<UrlItems> {

        let protocol = ["https", "http", "ftp"].into_iter().filter(|i| 
            self.to_lowercase().contains(i)
        ).next().unwrap();

        match protocol {
            "https" => {
                //https://nothing.com:443/...
                let (dns, path) = self.get_slice("//","/").unwrap();
                let port = "443";
                Some(UrlItems {dns: dns, path: path, port: port, is_secure: true})
            },
            "http" => {
                let (dns, path) = self.get_slice("//","/").unwrap();

                match dns.contains(":") {
                    true => {
                        //http://nothing.com:3587/......
                        let (dns, port) = dns.split_once(":").unwrap();
                        Some(UrlItems {dns: dns, path: path, port: port, is_secure: false})                 
                    },
                    false => {
                        //http://nothing.com/.....
                        let (dns, path) = self.get_slice("//","/").unwrap();
                        let port = "80";
                        Some(UrlItems {dns: dns, path: path, port: port, is_secure: false})
                    }
                }
            }
            "ftp" => {
                //ftp://192.199.257/folder/....
                let (dns, path) = self.get_slice("//","/").unwrap();
                let port = "21";
                Some(UrlItems {dns: dns, path: path, port: port, is_secure: false})
            }
            _=> {None}
        }

    }

}

///get path from Download HOME folder<br>
/// # Return Example
///<h3>/dir/home_folder/Downloads
pub fn get_download_folder() -> &'static str {

    std::env::vars()
        .filter_map(|(env, vl)|{
            match env.to_lowercase().starts_with("home") {
                true => {
                    let path = Box::leak(format!("{vl}/Downloads").into_boxed_str());
                    Some(path)
                },
                false => None
            }
            
        }).next().unwrap()

}

///Gets <b>tmp</b> SO folder and add "/temp_folder"
///# Example of return:
/// <h3>/.../tmp/temp_folder
pub fn get_temp_dir() -> &'static str{

    let tmp = std::env::temp_dir();
    let tmp = format!("{}/temp_folder", tmp.display());
    Box::leak(tmp.into_boxed_str())

}

pub fn create_file(dir_saved: &'static str, new_name_file: &'static str) -> Option<String>{

    let save_at = &format!("{}/{new_name_file}", get_download_folder());

    let mut new_file = std::fs::File::create(save_at).unwrap();

    let mut files = std::fs::read_dir(dir_saved)
        .unwrap()
        .map(|i| {

            let file_name = i.unwrap().file_name();
            file_name.to_string_lossy().to_string()
        })
        .collect::<Vec<String>>();
        
    files.sort();

    let mut saved = Some(save_at.clone());

    for f_name in files {

        let full_f_name = format!("{dir_saved}/{f_name}");

        if let Ok(mut file) = std::fs::File::open(full_f_name){

            let mut buff = vec![];

            if let Ok(file_size) = file.read_to_end(&mut buff) {

                if let Ok(file_size) = new_file.write(&buff) {

                    //one chunk save as sucess !!!

                } else {

                    //have any error!!!! 
                    //set Option to none
                    saved = None;

                    break;
                }
            }
        }        
    }

    //remove temp_dir
    std::fs::remove_dir_all(dir_saved).unwrap();

    saved
}
