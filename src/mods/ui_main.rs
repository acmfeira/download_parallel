use std::{ops::Deref, sync::{Arc, Mutex}, path::{PathBuf, Path}, str::FromStr};

use fltk::{
    window::Window, 
    prelude::{WidgetBase, GroupExt, WindowExt, WidgetExt, MenuExt, InputExt}, 
    group::Flex, frame::Frame, input::Input, misc::Progress, button::Button,
    menu::Choice, app::Sender, dialog::FileDialog
};

use crate::Status;

use super::download::Download;

#[derive(Clone)]
pub struct UI{
    tx: Sender<Status>,
    button: Button,
    input_url: Input,
    choice: Choice,
    win: Window,
    progress: Progress,
    flex_url: Flex,
    flex_speed: Flex,
    stop: Arc<Mutex<bool>>
}

impl UI {
    
    pub fn new(tx: Sender<Status>) -> Self{

        let mut win = Window::new(0, 0, 640, 130, "Download Parallel")
            .center_screen();

            let mut base = Flex::default_fill().column();

                let mut its0 = Flex::default_fill();

                    let label = Frame::default().with_label("Url:");
                    its0.set_size(&label, 28);

                    let input_url = Input::default();

                its0.set_pad(6);
                its0.end();
                base.set_size(&its0, 28);

                let mut its1 = Flex::default();

                    Frame::default();

                    let label1 = Frame::default().with_label("Speed:");

                    let opts = vec!["Normal", "Fast0", "Fast1", "Fast2", "Super", "Ultra"];

                    let mut choice = Choice::default();

                    for opt in opts {

                        choice.add_choice(opt);
                    }
                    choice.set_value(0);

                    its1.set_size(&label1,45);
                    its1.set_size(&choice, 100);

                    Frame::default();
                its1.set_pad(6);
                its1.end();
                base.set_size(&its1, 28);

                let progress = Progress::default();

                let mut bts = Flex::default();
                
                    Frame::default();

                    let mut button = Button::default().with_label("Download");
                    button.set_tooltip("Select destine and begin to download!!!");
                    bts.set_size(&button, 100);
                    Frame::default();
                bts.set_margin(2);
                bts.end();
                base.set_size(&bts, 32);

            base.set_margin(2);
            base.end();


        win.end();

        win.show();
        
        Self { tx: tx, button: button, input_url: input_url, choice: choice, win: win, progress: progress,
            flex_url: its0, flex_speed: its1,
            stop: Arc::new(Mutex::new(false))
         }

    }

    pub fn set_progress(&mut self, value: f64) {

        self.progress.set_value(value);
        let pct = format!("{:.1}%", value);
        self.progress.set_label(&pct);
    
    }

    pub fn start_download(&mut self) {

        if let Some(path) =  self.select_folder() {

            *self.stop.lock().unwrap() = false;//reset value to default

            let url = Box::leak(self.input_url.value().into_boxed_str()).trim();
            let ch = Box::leak(self.choice.choice().unwrap().into_boxed_str());
    
            let parallels = match ch.trim() {
                "Normal" => 1,
                "Fast0" => 5,
                "Fast1" => 10,
                "Fast2" => 15,
                "Super" => 30,
                _=>{60}//Ultra
            } ;
    
            let down = Download::new(self.tx.clone());
            
            let file_name = url.split("/").last().unwrap();
            let path = format!("{path}/{file_name}");
            let pb = PathBuf::from_str(&path).unwrap();
            
            down.download(pb, &url, parallels, self.stop.clone());
    
        };
    }

    pub fn select_folder(&self) -> Option<String> {

        let mut br = FileDialog::new(fltk::dialog::FileDialogType::BrowseDir);

        if let Ok(home) = std::env::var("HOME"){

            println!("{}",home);
            let down = Box::leak(format!("{home}/Downloads").into_boxed_str());

            br.set_directory(&Path::new(down)).unwrap();

        };
        br.show();

        let dr = br.filename();

        if dr.display().to_string().len() > 0 {
            Some(dr.display().to_string())
        } else { None}

    }

    pub fn stop(&mut self) {
        *self.stop.lock().unwrap() = true;

    }

    pub fn load(&self) {

        let mut bt = self.button.clone();
        let mut sf = self.clone();

        bt.set_callback(move|bt|{

            if bt.label() == "Download" {

                bt.set_tooltip("Stop download!!!");
                bt.set_label("STOP");  
                sf.disable_up();              
                sf.start_download();

            } else {

                bt.set_tooltip("Select destine and begin to download!!!");
                bt.set_label("Download");
                sf.enable_up();                
                sf.stop();

            };
        });
    }

    pub fn invisible(&mut self) {
        self.win.set_opacity(0.0);
    }

    pub fn visible(&mut self) {

        self.win.set_opacity(1.0);
    }

    fn disable_up(&mut self) {

        self.flex_speed.deactivate();
        self.flex_url.deactivate();
    }

    pub fn enable_up(&mut self) {

        self.flex_speed.activate();
        self.flex_url.activate();
    }

    pub fn set_button_label(&mut self, text: &'static str) {

        self.button.set_label(text);
    }

}
