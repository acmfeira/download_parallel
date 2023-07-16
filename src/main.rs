mod mods;
use std::process::exit;

use fltk::app::App;
use mods::{download::{Download}, ui_message::Message, ui_main::UI};

pub enum Status {
    OnPercent(f64),
    OnStop(&'static str),
    OnFinished(&'static str),
    OnError(&'static str),
}
fn main() {

    let (tx,rx) = fltk::app::channel::<Status>();

    let app = App::default();

        let mut ui = UI::new(tx);
        ui.load();

    while app.wait() {
        match rx.recv() {

            Some(Status::OnPercent(pct)) => {

                ui.set_progress(pct);

            },
            Some(Status::OnFinished(f_name)) => {

                ui.invisible();

                let msg = format!("Download Finished\n{f_name}");
                let mut msg = Message::show(&msg);

                ui.set_progress(0.0);
                ui.enable_up();
                ui.set_button_label("Download");

                let mut ui_ = ui.clone();

                msg.on_close(move||{

                    ui_.visible();

                });

            },
            Some(Status::OnError(err)) => {

                let msg = format!("Error Download\n{err}");
                let mut msg = Message::show(&msg);

                ui.set_progress(0.0);
                ui.enable_up();
                ui.set_button_label("Download");

                let mut ui_ = ui.clone();
                
                msg.on_close(move||{

                    ui_.visible();

                });

                ui.invisible();

            },
            Some(Status::OnStop(msg)) => {

                ui.set_progress(0.0);

            },
            _=>{ }
        }
    }
    
    //test_me();
}

pub fn test_me(){


    //ser with a lot of ISOS: https://crustywindo.ws/collection/Windows%207/
    //let url = "https://www.mirrorservice.org/sites/quakeunity.com/movies/2v2barnak&bacon.avi";
    //let url = "https://doc.downloadha.com/h/Documentaries/December%202016/Planet.Earth.II.S01E01.1080p.BluRay.x264-ROVERS%20%28www.Downloadha.com%29.mkv";
    //let url = "https://doc.downloadha.com/h/Documentaries/April2021/The.93rd.Annual.Academy.Awards.After.Dark.2021.720p.WEB.h264-BAE_www.Downloadha.com_.mkv";
    //let url = "http://www.tapir.caltech.edu/~phopkins/movies/hubble_sim_mix.mp4";
    //let url = "http://73.66.228.201:9800/Movies/A/A%20Dark%20Truth%20(2012).mkv";
    //let url = "http://releases.ubuntu.com/focal/ubuntu-20.04.6-live-server-amd64.iso";
    //let url = "https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-11.6.0-amd64-standard.iso"; //302
    //let url = "https://crustywindo.ws/collection/Windows%207/Seven_VietNam_X86.iso";//400mb
    //let url = "https://crustywindo.ws/collection/Windows%203.1/winlgtpremstable1.zip";//9.3mb
    
}
