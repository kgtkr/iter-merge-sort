pub trait MergeSortImpl<T>
where
    T: Iterator + Clone,
    T::Item: Clone + Ord,
{
    fn merge_sort(self, vec: Vec<T>, desc: bool) -> MergeSort<T>;
}

impl<T> MergeSortImpl<T> for T
where
    T: Iterator + Clone,
    T::Item: Clone + Ord,
{
    fn merge_sort(self, vec: Vec<T>, desc: bool) -> MergeSort<T> {
        MergeSort {
            iters: vec.iter()
                .map(|iter| (iter.clone(), None))
                .collect::<Vec<_>>(),
            desc: desc,
        }
    }
}

pub struct MergeSort<T>
where
    T: Iterator + Clone,
    T::Item: Clone + Ord,
{
    iters: Vec<(T, Option<T::Item>)>,
    desc: bool,
}

impl<T> Iterator for MergeSort<T>
where
    T: Iterator + Clone,
    T::Item: Clone + Ord,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> {
        //None埋め
        for i in 0..self.iters.len() {
            match self.iters[i].1 {
                Option::None => self.iters[i].1 = self.iters[i].0.next(),
                _ => {}
            }
        }
        if let Some((i, item)) = {
            let it =
                self.iters.clone().into_iter().enumerate().filter_map(
                    |(i, (_, item))| match item {
                        Option::Some(item) => Some((i, item)),
                        Option::None => None,
                    },
                );
            if self.desc {
                it.max_by_key(|x| x.clone().1)
            } else {
                it.min_by_key(|x| x.clone().1)
            }
        } {
            self.iters[i].1 = None;
            Some(item)
        } else {
            None
        }
    }
}
