

pub fn euclidean_algorithim(a : i128, b : i128) -> i128 {
    let mut a = a;
    let mut b = b;
    // if a < b {
    //     let tmp = a;
    //     let a = b;
    //     let b = tmp;
    // }
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
    // if a > b {
    //     euclidean_recursive_helper(a, b)
    // }
    // else {
    //     euclidean_recursive_helper(b, a)
    // }
}

// fn euclidean_recursive_helper(larger : i32, smaller: i32) -> i32 {
//     if larger == smaller {
//         return smaller;
//     }
//     euclidean_recursive_helper(smaller, larger%smaller)
// }
//
pub fn extended_euclidean_algorithim(a : i128, b : i128) -> (i128,i128,i128) {
    let mut card_a = (a, 1, 0);
    let mut card_b = (b, 0, 1);
    // let mut a = a;
    // let mut b = b;
    // let mut copies_a = 0;
    // let mut old_copies_a = 1;
    // let mut copies_b = 1;
    // let mut old_copies_b = 0;
    while card_b.0 != 0 {
        let number_of_copies = card_a.0 / card_b.0;
        // println!("{:?} {:?} {:?}", card_a, card_b, number_of_copies);
        (card_a, card_b) = (card_b, (card_a.0 - number_of_copies * card_b.0, card_a.1 - number_of_copies * card_b.1, card_a.2 - number_of_copies * card_b.2));
    }
    card_a
}

#[cfg(test)]
mod tests {
    use crate::euclidean::*;
    #[test]
    fn euclidean_works() {
        assert_eq!(euclidean_algorithim(4,4), 4);
        assert_eq!(euclidean_algorithim(30,21), 3);
        assert_eq!(euclidean_algorithim(21,30), 3);
    }
    #[test]
    fn extented_works() {
        assert_eq!(extended_euclidean_algorithim(4,4), (4,0,1));
        assert_eq!(extended_euclidean_algorithim(30,21), (3, -2, 3));
        assert_eq!(extended_euclidean_algorithim(21,30), (3, 3, -2));
        assert_eq!(extended_euclidean_algorithim(240,46), (2, -9, 47));
    }
}
