pub trait MergeSortImpl<'a, T>
where
    T: Ord + Clone,
{
    fn merge_sort(self, desc: bool) -> MergeSort<'a, T>;
}

impl<'a, T> MergeSortImpl<'a, T> for Vec<&'a mut Iterator<Item = T>>
where
    T: Ord + Clone,
{
    fn merge_sort(self, desc: bool) -> MergeSort<'a, T> {
        MergeSort {
            iters: self.into_iter()
                .map(|iter| (iter, None))
                .collect::<Vec<_>>(),
            desc: desc,
        }
    }
}

pub struct MergeSort<'a, T>
where
    T: Ord,
    T: 'a,
{
    iters: Vec<(&'a mut Iterator<Item = T>, Option<T>)>,
    desc: bool,
}

impl<'a, T> Iterator for MergeSort<'a, T>
where
    T: Ord + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
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
        let vec2: Vec<&mut Iterator<Item = i32>> = Vec::new();

        assert_eq!(vec1, vec2.merge_sort(true).collect::<Vec<_>>());
    }

    #[test]
    fn one_desc() {
        assert_eq!(
            vec![3, 2, 1],
            vec![&mut vec![3, 2, 1].into_iter() as &mut Iterator<Item = i32>]
                .merge_sort(true)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn one_not_desc() {
        assert_eq!(
            vec![1, 2, 3],
            vec![&mut vec![1, 2, 3].into_iter() as &mut Iterator<Item = i32>]
                .merge_sort(false)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn desc() {
        assert_eq!(
            vec![5, 3, 2, 1],
            vec![
                &mut vec![5, 1].into_iter() as &mut Iterator<Item = i32>,
                &mut vec![].into_iter() as &mut Iterator<Item = i32>,
                &mut vec![3, 2].into_iter() as &mut Iterator<Item = i32>,
                &mut vec![].into_iter() as &mut Iterator<Item = i32>,
            ].merge_sort(true)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn not_desc() {
        assert_eq!(
            vec![1, 2, 2, 3, 4, 4],
            vec![
                &mut vec![2, 4].into_iter() as &mut Iterator<Item = i32>,
                &mut vec![1, 3].into_iter() as &mut Iterator<Item = i32>,
                &mut vec![2, 4].into_iter() as &mut Iterator<Item = i32>,
            ].merge_sort(false)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn infinite() {
        assert_eq!(
            vec![-2, -1, 0, 1, 1, 2, 2, 3, 3, 3, 4, 4, 5, 6, 7],
            vec![
                &mut (3..5) as &mut Iterator<Item = i32>,
                &mut (-2..4) as &mut Iterator<Item = i32>,
                &mut (1..) as &mut Iterator<Item = i32>,
            ].merge_sort(false)
                .take(15)
                .collect::<Vec<_>>()
        );
    }
}
