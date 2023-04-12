mod mods;
use mods::{download::{Download, SpeedOption}};

fn main() {
    //ser with a lot of ISOS: https://crustywindo.ws/collection/Windows%207/
    //let url = "https://www.mirrorservice.org/sites/quakeunity.com/movies/2v2barnak&bacon.avi";
    //let url = "https://doc.downloadha.com/h/Documentaries/December%202016/Planet.Earth.II.S01E01.1080p.BluRay.x264-ROVERS%20%28www.Downloadha.com%29.mkv";
    //let url = "https://doc.downloadha.com/h/Documentaries/April2021/The.93rd.Annual.Academy.Awards.After.Dark.2021.720p.WEB.h264-BAE_www.Downloadha.com_.mkv";
    //let url = "http://www.tapir.caltech.edu/~phopkins/movies/hubble_sim_mix.mp4";
    //let url = "http://73.66.228.201:9800/Movies/A/A%20Dark%20Truth%20(2012).mkv";
    //let url = "http://releases.ubuntu.com/focal/ubuntu-20.04.6-live-server-amd64.iso";
    let url = "https://cdimage.debian.org/debian-cd/current-live/amd64/iso-hybrid/debian-live-11.6.0-amd64-standard.iso"; //302
    //let url = "https://crustywindo.ws/collection/Windows%207/Seven_VietNam_X86.iso";//400mb
    //let url = "https://crustywindo.ws/collection/Windows%203.1/winlgtpremstable1.zip";//9.3mb

    //test!!!    
    //it doesn't work so better!!! (site is slow)
    //SiteUrls::get_urls("http://movie.basnetbd.com/Data/TV%20Series/The%20Big%20Bang%20Theory/");
    
    Download::download(url,SpeedOption::Ultra);
    
    //test_me();
}

pub fn test_me(){

    //implement convert secons to Hour format 00:00:00
    //std::fs::remove_dir_all("/tmp/temp_folder").unwrap();
    
}
