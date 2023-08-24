pub trait MergeSort {
    fn merge_sort(&self) -> Self;
}

fn merge_sort_internal<T: PartialOrd + Clone>(
    data: &Vec<T>,
    start_index: usize,
    end_index: usize,
) -> Vec<T> {
    if start_index == end_index {
        return vec![data[start_index].clone()];
    } else if start_index >= end_index {
        return vec![];
    }

    let middle = (start_index + end_index) >> 1;

    let mut result = Vec::<T>::new();

    let left = merge_sort_internal(data, start_index, middle);
    let right = merge_sort_internal(data, middle + 1, end_index);

    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        result.push(if left[left_index] < right[right_index] {
            left_index += 1;
            left[left_index - 1].clone()
        } else {
            right_index += 1;
            right[right_index - 1].clone()
        })
    }

    while left_index < left.len() {
        result.push(left[left_index].clone());
        left_index += 1;
    }

    while right_index < right.len() {
        result.push(right[right_index].clone());
        right_index += 1;
    }
    result
}

impl<T> MergeSort for Vec<T>
where
    T: PartialOrd + Clone,
{
    fn merge_sort(&self) -> Self {
        merge_sort_internal(self, 0, self.len() - 1)
    }
}
