/*
 * here i will make the communecator.
 * [JOB]
 * - define the structs and enums that i will need in my buster.
 * - define function that will do the most of world, as i know:
 *  1. bring_status() # return the status code 
 *
 * [WHAT I DO KNOW]
 * for now i will make the MyStatusCode enum to know what kind of status codes it is, and a function called bring_status()
 * that takes url and path then check for status code and return a MyStatusCode value
 *
 * [WHY?]
 * the tool will try a list by using this function and show result for the hacker with the MyStatusCode help
*/

// use reqwest::header::STRICT_TRANSPORT_SECURITY;

use reqwest::*;

pub struct StatusQ {
    pub code: String,
    pub state: String
}

pub enum ResStatus {
    Found(StatusQ),
    NotFound(StatusQ)
}
impl ResStatus {
    pub fn new (code: u16, state: &str, found: bool) -> ResStatus {
        if found {
            return ResStatus::Found(StatusQ {
                code: code.to_string(),
                state: state.to_string()
            });
        }

        ResStatus::NotFound(StatusQ {
            code: code.to_string(),
            state: state.to_string()
        })
    }
}

pub async fn bring_status(url: &str, path: &str) -> ResStatus {
    // prepare the query
    let query = format!("{}{}", url, path);

    // send the query
    let client = reqwest::Client::new();
    let req = client.get(query).send().await;
    // extract the response
    let res = match req {
        Ok(response) => response.status(),
        Err(response) =>
        {
            let status = response.status();
            match status {
                Some(x) => x,
                _ => StatusCode::from_u16(400).expect("IDK")
            }
        }
    };
    
    // declare the status code
    let status_code = res.as_u16();

    // if statement to get the final ResStatus
    if status_code < 300 {
        ResStatus::new(status_code, "found(OK)", true)
    } else if status_code < 400 {
        ResStatus::new(status_code, "found(REDIRECT)", true)
    } else if status_code < 500 {
        ResStatus::new(status_code, "not found(CLEINT-ERROR)", false)
    } else {
        ResStatus::new(status_code, "not found(SERVER-ERROR)", false)
    }
}

//=====[Old Code]====//

//pub enum MyStatusCode {
//    Ok,
//    Redirect,
//    CleintError,
//    ServerError
//}
//impl MyStatusCode {
//    pub fn readable(&self) -> &str {
//        match self {
//            MyStatusCode::Ok => "OK[found]",
//            MyStatusCode::Redirect => "REDIRECT[found]",
//            MyStatusCode::CleintError => "CLEINT-ERROR[not found!]",
//            MyStatusCode::ServerError => "SERVER-ERROR[not found!]"
//        }
//    }
//
//    pub fn kind(&self) -> StatusKind {
//        match self {
//            MyStatusCode::Ok => StatusKind::Found,
//            MyStatusCode::Redirect => StatusKind::Found,
//            MyStatusCode::CleintError => StatusKind::NotFound,
//            MyStatusCode::ServerError => StatusKind::NotFound
//        }
//    }
//}
//
//pub enum StatusKind {
//    Found,
//    NotFound
//}
//
//pub async fn bring_status(url: String, path: String) -> MyStatusCode {
//    let query = format!("{}{}", url, path);
//    let req = reqwest::get(query).await;
//    let res = match req {
//        Ok(x) => x,
//        Err(x) => panic!("Failed to get response: {}", x)
//    };
//    let status = res.status().as_u16();
//    if status <= 200 {
//        MyStatusCode::Ok
//    } else if status <= 300 {
//        MyStatusCode::Redirect
//    } else if status <= 400 {
//        MyStatusCode::CleintError
//    } else {
//        MyStatusCode::ServerError
//    }
//}
