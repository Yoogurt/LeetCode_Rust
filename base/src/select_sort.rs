pub trait SelectSort {
    fn select_sort(&mut self);
}

impl<T> SelectSort for Vec<T>
where
    T: PartialOrd,
{
    fn select_sort(&mut self) {
        for index in 0..self.len() {
            let mut partial_min_index = index;

            for scan in (index + 1)..self.len() {
                if self[partial_min_index] > self[scan] {
                    partial_min_index = scan;
                }
            }

            self.swap(partial_min_index, index);
        }
    }
}
