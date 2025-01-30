//Bencode ("BitTorrent Encode")

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum BencodeValue {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<BencodeValue>),
    Dictionary(HashMap<Vec<u8>, BencodeValue>),
}

pub struct BencodeParser {
    data: Vec<u8>,
    position: usize,
}

impl BencodeParser {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    // Parse a single valur from the current position
    pub fn parse_value(&mut self) -> Result<BencodeValue, String> {
        if self.position >= self.data.len() {
            return Err("Unexpected end of data".to_string());
        }

        match self.data[self.position] {
            b'i' => self.parse_integer(),
            b'l' => self.parse_list(),
            b'd' => self.parse_dictionary(),
            b'0'..=b'9' => self.parse_string(),
            _ => Err(format!("Invalid value type at position {}", self.position)),
        }
    }

    // string parser
    fn parse_string(&mut self) -> Result<BencodeValue, String> {
        // Find the colon seperator
        let colon_pos = self.data[self.position..]
            .iter()
            .position(|&b| b == b':')
            .ok_or("No colon found in string")?;

        // parse the length
        let length_str = std::str::from_utf8(&self.data[self.position..self.position + colon_pos])
            .map_err(|e| e.to_string())?;
        let length: usize = length_str.parse().map_err(|e| e.to_string())?;

        // Extract the string
        let start = self.position + colon_pos + 1;
        let end = start + length;

        if end > self.data.len() {
            return Err("String extends beyond end of data".to_string());
        }

        let string_data = self.data[start..end].to_vec();
        self.position = end;

        Ok(BencodeValue::String(string_data))
    }

    // integer parser
    fn parse_integer(&mut self) -> Result<BencodeValue, String> {
        // Skip the 'i' at the start
        self.position += 1;

        // Find the position of e Format: i<number>e , currently are pointer is at i
        let e_pos = self.data[self.position..]
            .iter()
            .position(|&b| b == b'e')
            .ok_or("No e found in string")?;

        // Extract the number string (everything between 'i' and 'e')
        let num_str = std::str::from_utf8(&self.data[self.position..self.position + e_pos])
            .map_err(|_| "Invalid UTF-8 in integer")?;

        // parse the string as an i64
        let num = num_str
            .parse::<i64>()
            .map_err(|_| "Invalid integer format")?;

        // Update position to after the 'e'
        self.position += e_pos + 1;

        Ok(BencodeValue::Integer(num))
    }

    // list parser
    fn parse_list(&mut self) -> Result<BencodeValue, String> {
        //1. Skip the 'l' at the start
        self.position += 1;

        // 2. Create a vector to store our parsed values
        let mut values = Vec::new();

        //3. Keep parsing util we find the end marker 'e'
        while self.position < self.data.len() && self.data[self.position] != b'e' {
            // Parse next valur and add it to our list
            let value = self.parse_value()?;
            values.push(value);
        }
        //4. Check if we actually found the end marker
        if self.position >= self.data.len() {
            return Err("Undetermined list".to_string());
        }

        //5. Skip the 'e' suffix
        self.position += 1;

        Ok(BencodeValue::List(values))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let data = b"4:spam".to_vec();
        let mut parser = BencodeParser::new(data);

        match parser.parse_value() {
            Ok(BencodeValue::String(bytes)) => {
                assert_eq![bytes, b"spam"];
            }
            _ => panic!("Expected string value"),
        }
    }

    #[test]
    fn test_parse_integer() {
        let test_cases = vec![
            ("i42e", Ok(42)),
            ("i-42e", Ok(-42)),
            ("i0e", Ok(0)),
            ("i123456789e", Ok(123456789)),
            ("ie", Err("Invalid integer format")),
            ("i12", Err("No 'e' found in integer")),
            ("iabc", Err("Invalid integer format")),
        ];

        for (input, expected) in test_cases {
            let mut parser = BencodeParser::new(input.as_bytes().to_vec());
            match (parser.parse_value(), expected) {
                (Ok(BencodeValue::Integer(n)), Ok(expected_n)) => {
                    assert_eq!(n, expected_n);
                }
                (Err(_), Ok(_)) => panic!("Expected success, got error"),
                (Ok(_), Err(_)) => panic!("Expected error, got success"),
                (Err(_), Err(_)) => {} // Both failed as expected
            }
        }
    }

    #[test]
    fn test_parse_list() {
        // Test case 1: Empty list
        let mut parser = BencodeParser::new(b"le".to_vec());
        match parser.parse_value() {
            Ok(BencodeValue::List(v)) => assert_eq!(v.len(), 0),
            _ => panic!("Expected empty list"),
        }

        // Test case 2: List with string and integer
        let mut parser = BencodeParser::new(b"l4:spami42ee".to_vec());
        match parser.parse_value() {
            Ok(BencodeValue::List(v)) => {
                assert_eq!(v.len(), 2);
                match &v[0] {
                    BencodeValue::String(s) => assert_eq!(s, b"spam"),
                    _ => panic!("Expected string"),
                }
                match v[1] {
                    BencodeValue::Integer(n) => assert_eq!(n, 42),
                    _ => panic!("Expected integer"),
                }
            }
            _ => panic!("Expected list"),
        }
    }
}
