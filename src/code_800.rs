use crate::Solution;

impl Solution {
    pub fn similar_rgb(color: String) -> String {
        let r = &color[1..=2];
        let g = &color[3..=4];
        let b = &color[5..=6];

        let mut result = String::from('#');
        result.push_str(&Solution::find_similar_color(r));
        result.push_str(&Solution::find_similar_color(g));
        result.push_str(&Solution::find_similar_color(b));
        result
    }

    fn find_similar_color(color: impl AsRef<str>) -> String {
        let color = color.as_ref().chars().collect::<Vec<char>>();
        let first = color[0];
        let second = color[1];

        let char = match second {
            '8'|'9'|'a'|'b'|'c'|'d'|'e'|'f' => {
                match first {
                    '0'..='9'|'a'..='e' => {
                        ((first as u8) + 1) as char
                    }
                    _=> {
                        'f'
                    }
                }
            }
            _ => {
                first
            }
        };

        let mut result = String::from(char);
        result.push(char);
        result
    }
}

#[test]
fn test_code_800() {
    println!("{}", Solution::similar_rgb("#ef2177".into()))
}