pub trait QuickSort {
    fn quick_sort(&mut self);
}

fn quick_sort_internal<T: PartialOrd>(
    data: &mut Vec<T>,
    mut start_index: usize,
    mut end_index: usize,
) {
    let init_start = start_index;
    let init_end = end_index;

    if start_index >= end_index {
        return;
    }

    let mut key = &data[start_index];

    loop {
        while start_index < end_index {
            if key > &data[end_index] {
                break;
            } else {
                end_index -= 1;
            }
        }

        data.swap(start_index, end_index);
        key = &data[end_index];

        if start_index >= end_index {
            break;
        }

        while start_index < end_index {
            if &data[start_index] > key {
                break;
            } else {
                start_index += 1;
            }
        }

        data.swap(start_index, end_index);
        key = &data[start_index];
    }

    if start_index > 0 {
        quick_sort_internal(data, init_start, start_index - 1);
    }

    quick_sort_internal(data, start_index + 1, init_end);
}

impl<T> QuickSort for Vec<T>
where
    T: PartialOrd,
{
    fn quick_sort(&mut self) {
        quick_sort_internal(self, 0, self.len() - 1)
    }
}
