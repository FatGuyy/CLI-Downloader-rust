use reqwest; // For making HTTP requests
use std::collections::HashMap; // For hash maps
use serde_json; // For serializing and deserializing the JSON data
use serde_json::Value; // generic enum representing different JSON values
use std::error::Error; // Value is an generic enum representing different JSON values
use std::io::Cursor; // Cursor is a virtual pointer used to navigate through a sequence of bytes, in a file
use crate::errors; // imports the 'errors' file

// This defines a custom Result type using a type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// This function takes in all the arguments passed in from the terminal or their default values as parameters 
pub fn auto(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);

    let mut getstream_body = HashMap::new(); // Declare a new hashmap
        // Push all the information in the hashmap using the insert method
        getstream_body.insert("url", url);
        getstream_body.insert("vCodec", codec);
        getstream_body.insert("vQuality", quality);
        getstream_body.insert("aFormat", audioformat);
        // Convert all the boolean values to strings, to insert them into the hashmap
        let inttwm = &ttwatermark.to_string();
        let ifa = &fullaudio.to_string();
        let iam = &mute.to_string();
        let idl = &dublang.to_string();
        // This section, inserts the values in the hashmap if the values are true
        if ttwatermark {
            getstream_body.insert("isNoTTWatermark", inttwm);
        }
        if fullaudio {
            getstream_body.insert("isTTFullAudio", ifa);
        }
        if mute {
            getstream_body.insert("isAudioMuted", iam);
        }
        if dublang {
            getstream_body.insert("dubLang", idl);
        }

    // declares the getstream_url, by formatting the apiurl variable
    let getstream_url = &format!("https://{apiurl}/api/json");
    // This part is for the debug functionality, if the debug is true, it prints the following
    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url);
        println!("{prefix} get stream url request body:");
        println!("{prefix} {}", serde_json::to_string(&getstream_body).unwrap());
        println!("{prefix} {}", "====[ debug ]====");
        println!(" ");
    }

    // Call the getstream function
    getstream(prefix, &getstream_url, getstream_body, path);
}

// This function takes in all the arguments passed in from the terminal or their default values as parameters 
pub fn audio(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{prefix} getting stream URL for {}...", url);

    let mut getstream_body = HashMap::new(); // Declare the hashmap
        // Push all the information in the hashmap using the insert method
        getstream_body.insert("isAudioOnly", "true");
        getstream_body.insert("url", url);
        getstream_body.insert("vCodec", codec);
        getstream_body.insert("vQuality", quality);
        getstream_body.insert("aFormat", audioformat);
        // Convert all the boolean values to strings, to insert them into the hashmap
        let inttwm = &ttwatermark.to_string();
        let ifa = &fullaudio.to_string();
        let iam = &mute.to_string();
        let idl = &dublang.to_string();
        // This section, inserts the values in the hashmap if the values are true
        if ttwatermark {
            getstream_body.insert("isNoTTWatermark", inttwm);
        }
        if fullaudio {
            getstream_body.insert("isTTFullAudio", ifa);
        }
        if mute {
            getstream_body.insert("isAudioMuted", iam);
        }
        if dublang {
            getstream_body.insert("dubLang", idl);
        }

    // declares the getstream_url, by formatting the apiurl variable
    let getstream_url = &format!("https://{apiurl}/api/json");
    // This part is for the debug functionality, if the debug is true, it prints the following
    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====");
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url);
        println!("{prefix} get stream url request body:");
        println!("{prefix} {}", serde_json::to_string(&getstream_body).unwrap());
        println!("{prefix} {}", "====[ debug ]====");
        println!(" ");
    }

    // Call the getstream function
    getstream(prefix, &getstream_url, getstream_body, path);
}

#[tokio::main] // Tokio main runtime for asynchronous functionality.
// takes in 
// prefix - for debug printing, url - for making post request
// Hashmap that has key-value pair holding all info for request body
// path - for storing the file locally
async fn getstream(prefix: &str, url: &str, body: HashMap<&str, &str>, path: &str) {
    let client = reqwest::Client::new(); // creates an instance of the 'reqwest::Client' for making HTTP requests
    let response = client.post(url) // sends post request to the url, with given info in hashmap 
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .json(&body) // Passing given info in hashmap in the post request
        .send()
        .await;
    // Convert the response to tect fromat and unwrap it
    let formatted_response = response.expect("method not found in `Result<Response, Error>`").text().await.unwrap();
    
    // converting the string response into 'serde_json::Value', using the 'from_str' function
    let fmtd_res2: Value = serde_json::from_str(&formatted_response).unwrap();

    // If the 'status' from the response is 'stream', it means the url stream is available.
    if fmtd_res2.get("status").unwrap() == "stream" {
        let streamurl = fmtd_res2.get("url").unwrap().to_string(); // change url to string
        
        let streamurl: &str = &streamurl[1..streamurl.len() - 1]; // remove the double quotes from the streamurl

        // Call the download from the stream function using the streamurl as string, prefix and path 
        let res: std::result::Result<(), Box<dyn Error + Send + Sync>> = downloadfromstream(prefix, &streamurl.to_string(), path).await;
        println!("{:?}", res); // prints the result
    } else { // Throw error if the stream is not available
        errors::create_end(&format!("{} failed to get stream url. {}", prefix, fmtd_res2.get("text").unwrap()).as_str());
    }
}

// This function is responsible for using the streamUrl and store it in the given path 
async fn downloadfromstream(prefix: &str, url: &str, path: &str) -> Result<()> {
    println!("{} got stream url. starting download...", prefix); // prints into telling that download is started 
    let response = reqwest::get(url.to_string()).await?; // Send HTTP GET request, to given url
    let filename1 = response.headers().get("Content-Disposition").unwrap().to_str().ok();
    let filename2 = filename1.unwrap().strip_prefix("attachment; filename=\"");
    let filename3 = filename2.unwrap().strip_suffix("\"").unwrap();
    let full_path = format!("{}/{}", path, filename3); // Declare full path for storing the file, by combing path and filename

    // Here, "?" is used to propagate errors up the call stack, it is part of rust's error handling mechanism, often used with functions that return a 'Result' type.
    let mut file = std::fs::File::create(format!("{path}/{filename3}"))?; // Creates the file at given location 
    let mut content =  Cursor::new(response.bytes().await?); // This creates a Cursor that wraps the bytes from the response
    std::io::copy(&mut content, &mut file)?; // This copies from the 'content' variable to 'file'
    println!("{} completed download. saved as {}", prefix, full_path); // Final log in the file
    Ok(()) // return an empty ok result
}