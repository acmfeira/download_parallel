use std::{sync::{Arc, Mutex}, process::exit, io::Write, thread};

use super::{reqwest::Reqwest, structs::ReqError, functions::{get_temp_dir, create_file}};

pub struct Download{

}

enum DownStatus {
    Percentage(f32),
    Finished,
    Error(&'static str)
}

#[derive(Debug, PartialEq,Eq, PartialOrd)]
///# Examples
///<h3> Auto - automatic 1 - 30<br>
/// Normal: 1<br>
/// Fast0: 5<br>
/// Fast1: 10<br>
/// Fast2: 15<br>
/// Super: 30<br>
/// Ultra: 60
pub enum SpeedOption {
    ///1-30
    Auto,
    ///1
    Normal,
    ///5
    Fast0,
    ///10
    Fast1,
    ///15
    Fast2,
    ///30
    Super,
    ///60
    Ultra,
}
impl Download {

    pub fn download(url: &'static str, speed: SpeedOption){

        let new_file_name = url.split("/").last().unwrap();

        let full_size = Arc::new(Mutex::new(0));
        let (tx, rx) = std::sync::mpsc::channel::<DownStatus>();

        let now = std::time::Instant::now();

        let temp_dir = get_temp_dir();

        //create temp dir
        std::fs::create_dir_all(temp_dir).unwrap();        

        match Reqwest::new(url){
            Ok(reqw) => {

                let full_size_ = full_size.clone();
                let tx_ = tx.clone();
                
                thread::scope(move |t|{

                    let file_size = reqw.get_filesize();
                    
                    let def_size = 10000000;

                    let quants = match speed {
                        SpeedOption::Auto => {
                            
                            if file_size <= def_size {1} // 10mb
                            else {30} 

                        },
                        SpeedOption::Normal => {
                            1
                        },
                        SpeedOption::Fast0 => {
                            5
                        },
                        SpeedOption::Fast1 => {
                            10
                        },
                        SpeedOption::Fast2 => {
                            15
                        },
                        SpeedOption::Super => {
                            30
                        },
                        SpeedOption::Ultra => {
                            60
                        }
                    };
                    
                    let chunks = Self::create_slices(file_size, quants);

                    for (idx, (start, end)) in chunks.into_iter().enumerate() {

                        let file_size  = file_size.clone();
                        let reqw = reqw.clone();
                        let full_size = full_size.clone();
                        let tx = tx_.clone();

                        t.spawn(move ||{

                            thread::sleep(std::time::Duration::from_millis(500));

                            //it force the smaller chunk first.
                        
                            match reqw.bytes(start,end,|size_of|{
    
                                *full_size.lock().unwrap() += size_of;
            
                                let fs = *full_size.lock().unwrap() as f32;
                                let pct = fs * 100.0 / file_size as f32;

                                let pct = format!("{:.1}", pct);

                                print!("Downloading: {}%\r", pct);
                                std::io::stdout().flush().unwrap();

                            }) {
                                //finished download chunk
                                Ok(data) => {
                                    
                                    let temp_f_name = format!("{}/{:02}_temp_file.temp", temp_dir, idx);
                                    //println!("{}", temp_f_name);   

                                    std::fs::write(temp_f_name, &data).unwrap();
                                },
                                Err(err) => {
                                    match err {
                                        ReqError::ErrorFound(err) => {
                                            tx.send(DownStatus::Error(err)).unwrap();
                                        },
                                        _=> {}
                                    }
                                }
                            }
        
                        });

                    }
    
                });

                if *full_size_.lock().unwrap() > 0 {

                    *&tx.send(DownStatus::Finished).unwrap();

                } else {
                    *&tx.send(DownStatus::Error("Error download!!!!!")).unwrap();
                }
            },
            Err(err) => {
                match err {
                    ReqError::ErrorFound(err) => {
                        tx.send(DownStatus::Error(err)).unwrap();
                    }
                    
                }
            }
        }

        loop {
            match rx.recv() {
                Ok(DownStatus::Percentage(pct)) => {

                },
                Ok(DownStatus::Finished) => {

                    println!("100.0%");
                    println!("\nFinished Download in {} seconds", now.elapsed().as_secs());

                    println!("\nCreating and saving local file...");
                    if let Some(full_name_file) = create_file(temp_dir, new_file_name){

                        println!("\nSaved as sucess: {}", full_name_file);
                    };
        
                    break;

                },
                Ok(DownStatus::Error(err)) => {

                    println!("Error:\n{err}\nABOARTED!!!!");exit(0);
                }
                _=> {}
            }
        }
    
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