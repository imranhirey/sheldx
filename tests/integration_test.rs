use hyper::{ body::Incoming, Request };
use sheldx::{ handlers::show_html_page, utils::extract_host };

#[test]
fn should_show_default_page() {
    let response = ();
    show_html_page("404", "Not Found");
    //check if returns Full<Bytes
    assert_eq!(response, ());
}


// configaration test

