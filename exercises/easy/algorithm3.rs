/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: PartialOrd>(array: &mut [T]) {
	let count = array.len();
    // bubble sorting
    // for i in 0..count - 1 {
    //     for j in i + 1..count {
    //         if array[i] > array[j] {
    //             array.swap(i, j);
    //         }
    //     }
    // }

    //select sorting
    for i in 0..count - 1 {
        let mut min_idx = i;
        for j in i + 1..count {
            if array[j] < array[min_idx] {
                min_idx = j;
            }
        }
        array.swap(i, min_idx);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}