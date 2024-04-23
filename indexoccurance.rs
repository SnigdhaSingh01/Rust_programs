//2.Given a sorted array of integers, implement a function that returns the index of the //first occurrence of a given number.

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 4;
    
    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} does not exist in the array.", target);
    }
}


