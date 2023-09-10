#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFilePath,
    InvalidMethod,
    InvalidUrl,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    method: RequestMethod,
    url: String,
}

pub mod httop {
    use nom::{
        bytes::complete::{tag, take_until},
        IResult,
    };

    use super::{ParseError, Request, RequestMethod};
    use std::{env, fs};

    pub fn get_current_working_dir() -> Result<String, ParseError> {
        let current_dir = env::current_dir();

        match current_dir {
            Ok(path) => {
                let current_dir_str = path.to_str().unwrap_or("Invalid UTF-8 path");
                Ok(String::from(current_dir_str))
            }
            Err(_) => Err(ParseError::InvalidFilePath),
        }
    }

    fn read_from_file(path: &str) -> Result<String, ParseError> {
        let contents = fs::read_to_string(path);

        match contents {
            Ok(val) => Ok(val),
            Err(_) => Err(ParseError::InvalidFilePath),
        }
    }

    fn clean_up_comments(input: &str) -> String {
        // Split the input_text into lines and filter out lines that start with '#'
        let filtered_lines: Vec<&str> = input
            .lines()
            .filter(|line| !line.trim_start().starts_with("#"))
            .collect();

        // Join the filtered lines back into a single string
        filtered_lines.join("\n")
    }

    fn parse_key<'a>(input: &'a str, key: &'a str) -> IResult<&'a str, &'a str> {
        let (input, data) = take_until(key)(input)?;

        if data.is_empty() {
            let (input, _) = tag(key)(input)?;
            let (input, value) = take_until("--")(input.trim())?;
            Ok((input, value.trim()))
        } else {
            let (input, _) = tag(key)(input.trim())?;
            Ok((input, input.trim()))
        }
    }

    pub fn parse_file(path: &str) -> Result<Request, ParseError> {
        let file_contents = read_from_file(path).unwrap();
        let cleaned_file_contents = clean_up_comments(&file_contents);
        let parse_method_result = parse_key(&cleaned_file_contents, "--method");
        let method: Result<RequestMethod, ParseError> = match parse_method_result {
            Ok((_, val)) => match val.to_uppercase() {
                x if x == "GET" => Ok(RequestMethod::GET),
                x if x == "POST" => Ok(RequestMethod::POST),
                x if x == "PUT" => Ok(RequestMethod::PUT),
                x if x == "PATCH" => Ok(RequestMethod::PATCH),
                x if x == "DELETE" => Ok(RequestMethod::DELETE),
                x if x == "HEAD" => Ok(RequestMethod::HEAD),
                x if x == "OPTIONS" => Ok(RequestMethod::OPTIONS),
                _ => Err(ParseError::InvalidMethod),
            },
            Err(_) => Err(ParseError::InvalidMethod),
        };

        let parse_url_result = parse_key(&cleaned_file_contents, "--url");
        let url: Result<String, ParseError> = match parse_url_result {
            Ok((_, val)) => Ok(String::from(val)),
            Err(_) => Err(ParseError::InvalidUrl),
        };

        Ok(Request {
            method: method.unwrap(),
            url: url.unwrap(),
        })
    }
}

fn main() {}

#[test]
fn test_parse_file() {
    use crate::httop::parse_file;

    assert_eq!(
        parse_file(
            format!(
                "{}/tests/get.httop",
                httop::get_current_working_dir().unwrap()
            )
            .as_str()
        ),
        Ok(Request {
            method: RequestMethod::GET,
            url: String::from("https://hisamafahri-v1.web.val.run")
        })
    );
}
