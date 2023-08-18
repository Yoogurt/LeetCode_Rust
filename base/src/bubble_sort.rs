pub trait BubbleSort {
    fn bubble_sort(&mut self);
}

impl<T> BubbleSort for Vec<T> where T: PartialOrd {
    fn bubble_sort(&mut self) {
        for index in 0..self.len() {
            for index_2 in 0..(self.len() - index - 1) {
                if self[index_2] > self[index_2 + 1] {
                    self.swap(index_2, index_2 + 1);
                }
            }
        }
    }
}