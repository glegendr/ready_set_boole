#[allow(dead_code)]
fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut clone = set.to_vec();
    match clone.pop() {
        Some(last) => {
            let mut ret = powerset(&clone[..]);
    
            for mut set in ret.clone() {
                set.push(last);
                ret.push(set);
            }
    
            ret

        },
        None => vec![Vec::new()]
    }
}

#[cfg(test)]
mod powerset {

    use super::powerset;
    #[test]
    fn powerset_test() {
        assert_eq!(powerset(&[1,2,3]), vec![
            vec![],
            vec![1],
            vec![2],
            vec![1,2],
            vec![3],
            vec![1,3],
            vec![2,3],
            vec![1,2,3],
        ]);
        assert_eq!(powerset(&[]), vec![
            vec![],
        ]);
        assert_eq!(powerset(&[1,2,3,4,5]), vec![
            vec![],
            vec![1],
            vec![2],
            vec![1,2],
            vec![3],
            vec![1,3],
            vec![2,3],
            vec![1,2,3],
            vec![4],
            vec![1,4],
            vec![2,4],
            vec![1,2,4],
            vec![3,4],
            vec![1,3,4],
            vec![2,3,4],
            vec![1,2,3,4],
            vec![5],
            vec![1,5],
            vec![2,5],
            vec![1,2,5],
            vec![3,5],
            vec![1,3,5],
            vec![2,3,5],
            vec![1,2,3,5],
            vec![4,5],
            vec![1,4,5],
            vec![2,4,5],
            vec![1,2,4,5],
            vec![3,4,5],
            vec![1,3,4,5],
            vec![2,3,4,5],
            vec![1,2,3,4,5],
        ]);
    }
}
