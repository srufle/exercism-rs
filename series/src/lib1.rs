pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return Vec::new();
    } else if len >= digits.len() {
        return vec![(&digits[..]).to_string()];
    } else {
        let mut ans: Vec<String> = Vec::new();
        let mut start = 0;
        let mut end = start + len;
        println!("5:5 {:?}", &digits[5..5]);
        while start < digits.len() + 1 {
            println!("from {:?}, to {:?}, {:?}", start, len, end);
            let s = &digits[start..end];
            println!("{:?}", &s);

            ans.push(s.to_string());
            start = start + 1;
            end = start + len;
            if end > digits.len() {
                break;
            }
        }

        return ans;
    }
}
