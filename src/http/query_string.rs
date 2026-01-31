//! Query string parsing for HTTP requests
//! 
//! Parses URL query parameters (e.g., ?name=value&key=value)

use std::collections::HashMap;

/// Represents parsed query string parameters
#[derive(Debug)]
pub struct QueryString<'buf>{
    data:HashMap<&'buf str, Value <'buf>>
}

/// Query parameter value (can be single or multiple)
#[derive(Debug)]
pub enum Value<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf>{
    /// Retrieve a query parameter value by key
    pub fn get(&self, key:&str)-> Option<&Value<'_>>{
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
    /// Parse query string from URL format (key=value&key2=value2)
    fn from(s: &'buf str) -> Self{
        let mut data = HashMap::new();

        // Split on '&' to get individual key=value pairs
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val="";
            
            // Split on '=' to separate key and value
            if let Some(i)= sub_str.find('='){
                key =&sub_str[..i];
                val = &sub_str[i+1..];
            }
            
            // Handle duplicate keys by converting to Multiple variant
            data.entry(key).and_modify(|existing|match existing{
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(vec)=>vec.push(val)
            }).or_insert(Value::Single(val));
        }
        QueryString{data}
    }
}