fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        // For odd length arrays
        return arr[len / 2] as f64;
    } else {
        // For even length arrays
        let mid1 = arr[len / 2 - 1];
        let mid2 = arr[len / 2];
        return (mid1 as f64 + mid2 as f64) / 2.0;
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    let median1 = find_median(&arr1);
    let median2 = find_median(&arr2);

    println!("Median of arr1: {}", median1);
    println!("Median of arr2: {}", median2);
}

