// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    // split vector into two halves
    // let (v1, v2) = v.split_at(v.len()/2);
    // let v1 = v1.to_vec();
    // let v2 = v2.to_vec();
    let midpoint = v.len() / 2;
    let mut sum1 = 0;
    let mut sum2 = 0;

    // compute the sum of each half in a separate thread
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let first = &v[..midpoint];
            let first = first.to_vec();
            sum1 = sum(first);
        });
        scope.spawn(|| {
            let second = &v[midpoint..];
            let second = second.to_vec();
            sum2 = sum(second);
        });
    });

    // add them together once computation complete
    sum1 + sum2

}

// pub fn sum(v: Vec<i32>) -> i32 {
//     let mut l = 0;
//     let mut r = 0;
//     let m = v.len()/2;
//     std::thread::scope(|scope| {

//         scope.spawn(|| {
//             for i in 0..m {
//                 l += v[i];
//             }
//         });
//         scope.spawn(|| {
//             for i in m..v.len() {
//                 r += v[i];
//             }
//         });
//     });
//     return l + r;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
