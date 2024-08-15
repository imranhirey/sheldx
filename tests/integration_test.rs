use std::fs;

use hyper::{ body::Incoming, Request };
use sheldx::{ handlers::show_html_page, utils::{create_default_config, extract_host} };
use tokio::fs::File;

#[test]
fn should_show_default_page() {
    let response = ();
    show_html_page("404", "Not Found");
    //check if returns Full<Bytes
    assert_eq!(response, ());
}


// configaration test

// showuld craete sheldx folder in etc
#[test]
fn should_create_sheldx_folder() {
    let response = create_default_config();
    //check if returns Full<Bytes
  // check if tehre is a folder called sheldx in etc
    match fs::read_dir("/etc") {
        Ok(dir) => {
            let mut found = false;
            for entry in dir {
                if entry.unwrap().file_name() == "sheldx" {
                    found = true;
                    break;
                }
            }
            assert_eq!(found, true);
        }
        Err(_) => {
            assert_eq!(true, false);
        }
    }
}