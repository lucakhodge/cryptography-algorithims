
// mod crate::euclidean;
use crate::euclidean::extended_euclidean_algorithim;

pub fn chinese_remainder_two ((a1, n1) : (i128, i128), (a2, n2) : (i128, i128)) -> (i128, i128){
    let (_, m1, m2) = extended_euclidean_algorithim(n1, n2);
    let mut a = a1*n2*m2 + a2*n1*m1;
    let n = n1 * n2;
    //make result lowest positive
    while a >= 0 {
        a -= n;
    }
    while a < 0 {
        a += n;
    }
    (a,n)
}

pub fn chinese_remainder_list (list : Vec<(i128, i128)>) -> (i128, i128) {
    if list.len() == 0 {
        return (0, 0);
    }
    let mut acc = list[0];
    for i in 1..list.len() {
        acc = chinese_remainder_two(acc, list[i]);
        // println!("{:?}", acc);
    }
    acc
}

#[cfg(test)]
mod tests {
    use crate::chinese::*;
    #[test]
    fn chinese_two_works() {
        assert_eq!(chinese_remainder_two((2,3),(3,5)), (8, 15));
    }
    #[test]
    fn chinese_list_works() {
        assert_eq!(chinese_remainder_list(vec![(2,3),(3,5)]), (8, 15));
        assert_eq!(chinese_remainder_list(vec![(2,3),(3,5),(2,7)]), (23, 105));
    }
}
