/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

// pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let mut intervals = intervals;
//     intervals.sort_by(|v1, v2| v1[0].cmp(&v2[0]));
//     let mut output: Vec<Vec<i32>> = Vec::new();
//     let mut stop;
//     let mut i = 0;
//     let mut j;

//     while i < intervals.len() {
//         let start = intervals[i][0];
//         stop = intervals[i][1];

//         j = i + 1;
//         while j < intervals.len() {
//             if intervals[j][0] > stop {
//                 break;
//             }

//             stop.max(intervals[j][1]);
//             j += 1;
//         }
//         output.push(vec![start, stop]);
//         i = j;
//     }

//     output
// }

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return intervals;
    }

    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    intervals.into_iter().fold(Vec::new(), |mut acc, interval| {
        if let Some(last) = acc.last_mut() {
            if interval[0] <= last[1] {
                last[1] = last[1].max(interval[1]);
                return acc;
            }
        }

        acc.push(interval);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
