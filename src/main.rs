use serde::Deserialize;
use serde_json::Error;
use std::io;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ServerLog {
    user_id: u32,
    username: String,
}

pub fn parse(r: impl io::Read) -> Result<Vec<ServerLog>, Error> {
    let stream = serde_json::Deserializer::from_reader(r).into_iter();
    let mut logs = Vec::new();
    for entry in stream {
        logs.push(entry?);
    }
    Ok(logs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_succeeds_when_passed_valid_serverlog_serialization() {
        let want = ServerLog {
            user_id: 42,
            username: String::from("User McUserton"),
        };
        let json_data = r#"
        {
            "user_id": 42,
            "username": "User McUserton"
        }"#;
        let input = io::Cursor::new(json_data);
        let got = parse(input).unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0], want)
    }
    #[test]
    fn test_parse_fails_when_passed_syntactically_invalid_server_log() {
        let json_data = r#"{ "user_id": 42, "username": "Username"}}"#;
        let input = io::Cursor::new(json_data);
        let got = parse(input);
        assert!(got.is_err());
    }
    #[test]
    fn test_parse_fails_when_passed_non_serverlog_entry() {
        let json_data = r#"{ "something_unrelated" = true }"#;
        let input = io::Cursor::new(json_data);
        let got = parse(input);
        assert!(got.is_err());
    }

    #[test]
    fn test_parse_succeds_when_passed_multiple_entries_in_json_line_format() {
        let json_data = r#" 
            {
                "user_id": 42,
                "username": "User McUserton"
            }
            {
                "user_id": 43,
                "username": "Senor Seconduser"
            }
        "#;
        let input = io::Cursor::new(json_data);
        let got = parse(input).unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[1].user_id, 43);
    }
}
