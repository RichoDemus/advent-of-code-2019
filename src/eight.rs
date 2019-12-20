use itertools::{Itertools, Chunk};
use std::str::Chars;

pub fn eight() {}

fn partition(s: &str, length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars();
    loop {
        let chunk_str: String = chars.by_ref().take(length).collect();
        if chunk_str.is_empty() {
            break;
        }
        result.push(chunk_str);
    }
    result
}

fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

fn to_digits(str: &str) -> Vec<i32> {
    str.chars().into_iter().map(|c|c.to_digit(10))
    panic!()
}

fn data_to_layers(image_data: &str, width: usize, height: usize) -> Vec<Vec<Vec<i32>>> {

    let layers = partition(image_data, width * height);

    let layers: Vec<Vec<String>> = layers.into_iter()
        .map(|layer|partition(layer.as_str(), width))
        .collect();

    println!("{:?}", layers);

    layers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let image_data = "123456789012";
        let result = data_to_layers(image_data, 3, 2);
        assert_eq!(result,
                   vec![
                       vec![
                           vec![1, 2, 3],
                           vec![4, 5, 6],
                       ],
                       vec![
                           vec![7, 8, 9],
                           vec![0, 1, 2],
                       ]
                   ]
        );
    }
}