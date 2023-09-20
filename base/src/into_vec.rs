pub trait IntoVec {
    type Item;
    fn into_vec(self) -> Vec<Self::Item>;
}

impl<T, I, U> IntoVec for T where T: IntoIterator<Item=I>, I: IntoIterator<Item=U> {
    type Item = Vec<U>;
    fn into_vec(self) -> Vec<Vec<U>> {
        self.into_iter().map(|value| {
            value.into_iter().collect()
        }).collect()
    }
}

// impl<T, I> IntoVec for T where T: IntoIterator<Item=I> {
//     type Item = I;
//     fn into_vec(self) -> Vec<I> {
//         self.into_iter().collect()
//     }
// }

