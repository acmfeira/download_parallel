use std::{sync::{Arc, Mutex}, process::exit, io::Write, thread, path::{Path, PathBuf}};

use fltk::app::Sender;

use crate::Status;

use super::{reqwest::Reqwest, structs::ReqError, functions::{get_temp_dir, create_file}};

pub struct Download{
    tx: Sender<Status>,
}


impl Download {

    pub fn new(tx: Sender<Status>) -> Self {

        Self {tx: tx }
    }

    pub fn download(self, path: PathBuf, url: &'static str, parallel: usize, stop: Arc<Mutex<bool>>){
        
        //let new_file_name = url.split("/").last().unwrap();

        let full_size = Arc::new(Mutex::new(0));

        let temp_dir = get_temp_dir();

        let tx = self.tx.clone();

        std::fs::create_dir_all(temp_dir).unwrap();        

        thread::spawn(move||{

            match Reqwest::new(url){
                Ok(reqw) => {
    
                    let full_size_ = full_size.clone();
                    let tx_ = tx.clone();
                    let tx = tx.clone();
                    
                    let stop = stop.clone();
                    let stop_ = stop.clone();
    
                    let error_message = Arc::new(Mutex::new("Error download!!!!!"));
                    let error_message_ = error_message.clone();
                                    
                    thread::scope(move |t|{
    
                        let file_size = reqw.get_filesize();
                        
                        let def_size = 10000000;
    
                        let chunks = Self::create_slices(file_size, parallel);
    
                        for (idx, (start, end)) in chunks.into_iter().enumerate() {
    
                            let file_size  = file_size.clone();
                            let reqw = reqw.clone();
                            let full_size = full_size.clone();
                            let tx = tx_.clone();
                            let stop = stop_.clone();
                            let stop_ = stop_.clone();
                            let err_msg = error_message_.clone();
    
                            t.spawn(move ||{
    
                                thread::sleep(std::time::Duration::from_millis(500));
    
                                //it force the smaller chunk first.
                                let st = stop_.clone();
    
                                match reqw.bytes(start,end, st, |size_of|{
        
                                    if !*stop_.lock().unwrap() {
    
                                        *full_size.lock().unwrap() += size_of;
                
                                        let fs = *full_size.lock().unwrap() as f64;
                                        let pct = fs * 100.0 / file_size as f64;
        
                                        tx.send(Status::OnPercent(pct));
        
                                    }
    
                                }) {
                                    //finished download chunk
                                    Ok(data) => {
                                        
                                        if !*stop_.lock().unwrap() {
    
                                            let temp_f_name = format!("{}/{:02}_temp_file.temp", temp_dir, idx);
    
                                            std::fs::write(temp_f_name, &data).unwrap();
    
                                        }
    
                                    },
                                    Err(err) => {
                                        match err {
                                            ReqError::ErrorFound(err) => {
                                                *err_msg.lock().unwrap() = err;
                                                *full_size.lock().unwrap() = 0;//set error!!!
                                            },
                                            _=> {}
                                        }
                                    }
                                }
            
                            });
    
                            if *stop.lock().unwrap() {
                                break;
                            }
                        }
        
                    });
    
                    if !*stop.lock().unwrap() {

                        if *full_size_.lock().unwrap() > 0 {
    
                            println!("Finished created file!!");
                            if let Some(full_name_file) = create_file(path){

                                tx.send(Status::OnFinished(full_name_file));

                            }else {
                                //error criate final file!!!!
                                println!("error criate final file!!!!");
                            };
        
        
                        } else {
                            println!("Null else");
                            //println!("Error: {}", stop.lock().unwrap());
                            //tx.send(Status::OnStop(*error_message.lock().unwrap()));
                            //tx.send(Status::OnError(*error_message.lock().unwrap()));
                        }
                    } else {
                        println!("Stoped");
                        tx.send(Status::OnStop(*error_message.lock().unwrap()));

                    }

                },
                Err(err) => {
                    match err {
                        ReqError::ErrorFound(err) => {
                            tx.send(Status::OnError(err));
                        }
                        
                    }
                }
            }
    
    

        });

    }

    fn create_slices(file_size: usize, quants: usize) -> Vec<(usize, usize)>{

        let ch_size = file_size / quants;
        let rest = file_size % quants;

        vec![ch_size; quants].into_iter()
            .enumerate()
            .map(|(idx, ch_size)|{
                let start = ch_size * idx;
                let end = if idx+1 == quants {

                    start + ch_size + rest
                } else {

                    start + ch_size -1
                };

                (start, end) 
                
            }).collect::<Vec<_>>()

    }

}