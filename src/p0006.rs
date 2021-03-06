
struct Solution;

impl Solution {
    pub fn convert(mut s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let hop = 2 * num_rows - 2;
        let chars = s.as_bytes();
        let mut val = String::with_capacity(s.len());
        for r in 0..num_rows {
            let slant_offset = hop - 2 * r;
            let mut i = r;
            while i < chars.len() {
                val.push(char::from(chars[i]));
                if slant_offset != 0 && slant_offset != hop {
                    let slant_i = i + slant_offset;
                    if slant_i < chars.len() {
                        val.push(char::from(chars[slant_i]));
                    }
                }
                i += hop;
            }
        }
        val
    }
}