use crate::Solution;

impl Solution {
    fn byte_order(byte: u8) -> usize {
        byte as usize - 'A' as usize
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut chars = [0usize; 'z' as usize - 'A' as usize + 1];

        magazine.as_bytes().iter().for_each(|value| {
            chars[Solution::byte_order(*value)] += 1;
        });

        ransom_note.as_bytes().iter().all(|value| {
            let byte_order = Solution::byte_order(*value);

            if chars[byte_order] == 0 {
                false
            } else {
                chars[byte_order] -= 1;
                true
            }
        })
    }
}

#[test]
fn test_code_383() {
    assert_eq!(Solution::can_construct("aa".to_owned(), "b".to_owned()), false);
    assert_eq!(Solution::can_construct("aa".to_owned(), "ab".to_owned()), false);
    assert_eq!(Solution::can_construct("aa".to_owned(), "aab".to_owned()), true);
    
}