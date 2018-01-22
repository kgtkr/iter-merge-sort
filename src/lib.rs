pub trait MergeSortImpl<T>
where
    T: Iterator,
    T::Item: Ord + Clone,
{
    fn merge_sort(self, desc: bool) -> MergeSort<T>;
}

impl<T> MergeSortImpl<T> for Vec<T>
where
    T: Iterator,
    T::Item: Ord + Clone,
{
    fn merge_sort(self, desc: bool) -> MergeSort<T> {
        MergeSort {
            iters: self.into_iter()
                .map(|iter| (iter, None))
                .collect::<Vec<_>>(),
            desc: desc,
        }
    }
}

pub struct MergeSort<T>
where
    T: Iterator,
    T::Item: Ord + Clone,
{
    iters: Vec<(T, Option<T::Item>)>,
    desc: bool,
}

impl<T> Iterator for MergeSort<T>
where
    T: Iterator,
    T::Item: Ord + Clone,
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
            let it = self.iters
                .iter()
                .enumerate()
                .filter_map(|(i, x)| match x.1.clone() {
                    Option::Some(item) => Some((i, item)),
                    Option::None => None,
                });
            if self.desc {
                it.max_by_key(|x| x.1.clone())
            } else {
                it.min_by_key(|x| x.1.clone())
            }
        } {
            self.iters[i].1 = None;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emepy() {
        let vec1: Vec<i32> = Vec::new();
        let vec2: Vec<std::vec::IntoIter<i32>> = Vec::new();

        assert_eq!(vec1, vec2.merge_sort(true).collect::<Vec<_>>());
    }

    #[test]
    fn one_desc() {
        assert_eq!(
            vec![3, 2, 1],
            vec![vec![3, 2, 1].into_iter()]
                .merge_sort(true)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn one_not_desc() {
        assert_eq!(
            vec![1, 2, 3],
            vec![vec![1, 2, 3].into_iter()]
                .merge_sort(false)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn desc() {
        assert_eq!(
            vec![5, 3, 2, 1],
            vec![
                vec![5, 1].into_iter(),
                vec![].into_iter(),
                vec![3, 2].into_iter(),
                vec![].into_iter(),
            ].merge_sort(true)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn not_desc() {
        assert_eq!(
            vec![1, 2, 2, 3, 4, 4],
            vec![
                vec![2, 4].into_iter(),
                vec![1, 3].into_iter(),
                vec![2, 4].into_iter(),
            ].merge_sort(false)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn infinite_not_desc() {
        assert_eq!(
            vec![1, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7],
            vec![(3..), (4..), (1..)]
                .merge_sort(false)
                .take(15)
                .collect::<Vec<_>>()
        );
    }
}
