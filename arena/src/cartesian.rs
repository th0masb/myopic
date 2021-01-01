///
pub fn product<T: Clone>(lists: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = vec![];

    let mut list_iter = lists.iter();
    if let Some(first_list) = list_iter.next() {
        for i in first_list {
            res.push(vec![i.clone()]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el.clone());
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}

#[cfg(test)]
mod test {
    #[test]
    fn cartesian_product_1() {
        assert_eq!(
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]],
            super::product(&vec![vec![1, 2], vec![3, 4]])
        )
    }

    #[test]
    fn cartesian_product_2() {
        assert_eq!(
            vec![vec![1, 3, 2], vec![1, 4, 2]],
            super::product(&vec![vec![1], vec![3, 4], vec![2]])
        )
    }
}
