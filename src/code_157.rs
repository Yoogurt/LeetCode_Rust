use crate::Solution;

impl Solution {
    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut read_count = 0;

        for _ in 0..(n / 4 + 1) {
            let read_result = self.read4(buf);

            if read_result < 0 {
                return read_result;
            } else {
                read_count += read_result;
            }
        }

        0
    }

    fn read4(&self, buf4: &mut [char]) -> i32 {
        return 1;
    }
}

#[test]
fn test_code_157() {
}
