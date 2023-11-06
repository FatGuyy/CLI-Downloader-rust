// clap crate is used to make the command line app here
use clap::{Arg, Command};
use std::env; // std::env to get the package name and few more details from cargo.toml

// import the other files
mod download;
mod errors;

// Few static varibles to be used in code
static DEBUG: bool = false;
static PREFIX: &'static str = "[cli-downloader]";

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
        // mode
        // Define the types of arguments and their names.
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .help("set which mode to download with (default: auto, other: audio)"),
        )
        // universal
        .arg(
            Arg::new("apiurl")
                .short('a')
                .long("apiurl")
                .help("set api url, don't include https (default: co.wuk.sh)"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("path to save files to (default: ~/Downloads/)"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("url to download from"),
        )
        // video
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .help("set video quality (default: 1080p, other: 4320p+, 2160p, 720p, 480p, 360p)"),
        )
        .arg(
            Arg::new("codec")
                .short('c')
                .long("codec")
                .help("set youtube codec (default: h264, other: av1, vp9)"),
        )
        .arg(
            Arg::new("ttwatermark")
                .short('w')
                .long("ttwatermark")
                .num_args(0)
                .help("disable tiktok watermark (default: false)"),
        )
        // audio
        .arg(
            Arg::new("audioformat")
                .short('f')
                .long("audioformat")
                .help("set audio format (default: mp3, other: best, ogg, wav, opus)"),
        )
        .arg(
            Arg::new("dublang")
                .short('d')
                .long("dublang")
                .num_args(0)
                .help("dub language (default: false)"),
        )
        .arg(
            Arg::new("fullaudio")
                .short('k')
                .long("fullaudio")
                .num_args(0)
                .help("get tiktok full audio (default: false)"),
        )
        .arg(
            Arg::new("mute")
                .short('j')
                .long("mute")
                .num_args(0)
                .help("mute audio when possible (default: false)"),
        )
        .get_matches();

    let homedirpathbuf = dirs::home_dir(); // string representation of the user's home directory
    let homedirexpect = homedirpathbuf.expect("method not found in `Option<PathBuf>`"); // Expect homedirpathbuf if not found
    let homedir = homedirexpect.display(); // Gets a human readalbe string of home directory

    let mut mode = "unspecified".to_string(); // Initialize variable mode
    if matches.get_one::<String>("mode").is_none() { // Check if the mode was provided in commandline args
        errors::create_end("you didn't specify a mode"); // If not, throw error
    } else {
        mode = matches.get_one::<String>("mode").unwrap().to_string(); // If yes, unwrap and convert to string
    }

    // "co.wuk.sh" is the service
    let d_apiurl = "co.wuk.sh".to_string(); // Set default value of d_apiurl (api_url of service used)
    let apiurl: &String = matches.get_one::<String>("apiurl").unwrap_or(&d_apiurl); // Use this one if specified

    let d_path = format!("{homedir}/Downloads").to_string(); // Defalut path to store the downloads
    let path: &String = matches.get_one::<String>("path").unwrap_or(&d_path); // Change if different on is provided

    let mut url = "unspecified".to_string(); // Set default value of url (url of service used)
    if matches.get_one::<String>("url").is_none() { // Checks to see if any is provided or not
        errors::create_end("you didn't specify a video url"); // If not, throw an error
    } else {
        url = matches.get_one::<String>("url").unwrap().to_string(); // If provided, store it in url
    }

    let d_quality = "1080p".to_string(); // Set default quality of video
    let quality: &String = matches.get_one::<String>("quality").unwrap_or(&d_quality); // If provided, use the given one

    let d_codec = "h264".to_string(); // Set default compression and decompression techqnique
    let codec: &String = matches.get_one::<String>("codec").unwrap_or(&d_codec); // If other is provided, use it

    let mut ttwatermark = false; // Default watermark turn off for tiktok
    if matches.get_flag("ttwatermark") { // If the watermark flag is given read and store it
        ttwatermark = true;
    } else {
        ttwatermark = false;
    }

    let d_audioformat = "mp3".to_string(); // set default audio format
    let audioformat: &String = matches.get_one::<String>("audioformat").unwrap_or(&d_audioformat); // If other is given, use it.

    let mut dublang = false; // Set dublang to false as default
    if matches.get_flag("dublang") { // If the dublang flag is given read and store it
        dublang = true;
    } else {
        dublang = false;
    }

    let mut fullaudio = false; // Default fullaudio falg to false
    if matches.get_flag("fullaudio") { // If the dublang fullaudio is given read and store it
        fullaudio = true;
    } else {
        fullaudio = false;
    }

    let mut mute = false; // Default mute flag
    if matches.get_flag("mute") { // If the mute flag is given, read and store it
        mute = true;
    } else {
        mute = false;
    }

    // If the mode is debug, Display all the selected options on the screen
    if DEBUG {
        println!(" ");
        println!("{PREFIX} {}", "=  ===[ debug ]====");
        println!("{PREFIX} **mode**: {mode};");
        println!("{PREFIX} apiurl: {apiurl}; path: {path}; url: {url};");
        println!("{PREFIX} quality: {quality}; codec: {codec};");
        println!(
            "{PREFIX} ttwatermark: {ttwatermark}; audioformat: {audioformat}; dublang: {dublang};"
        );
        println!("{PREFIX} fullaudio: {fullaudio}; mute: {mute};");
        println!("{PREFIX} {}", "====[ debug ]====");
        println!(" ");
    }

    // This makes the function call based on the type of the mode
    if mode == "auto" { // If auto, this is called
        download::auto(PREFIX, DEBUG, &apiurl, &path, &url, &quality, &codec,  ttwatermark, &audioformat, dublang, fullaudio, mute)
    } else if mode == "audio" { // If audio, this is called
        download::audio(PREFIX, DEBUG, &apiurl, &path, &url, &quality, &codec, ttwatermark, &audioformat, dublang, fullaudio, mute)
    } else { // else part handles all the if any mode other than these 2 is given
        errors::create_end("invalid mode. options: auto, audio");
    }
}
