// Function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Function to find the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to find the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

// Function to check if a number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to find the median of a sorted array
fn median(arr: &[i32]) -> f64 {
    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[mid] as f64
    }
}

// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first = strings[0].as_bytes();
    for (i, &byte) in first.iter().enumerate() {
        for string in &strings[1..] {
            if i >= string.len() || byte != string.as_bytes()[i] {
                return String::from_utf8(first[..i].to_vec()).unwrap();
            }
        }
    }
    String::from_utf8(first.to_vec()).unwrap()
}

// Function to find the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to calculate the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// Function to reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Function to check if a number is prime
fn is_prime_rust(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    if i < arr1.len() {
        merged.extend_from_slice(&arr1[i..]);
    }
    if j < arr2.len() {
        merged.extend_from_slice(&arr2[j..]);
    }

    merged
}

// Function to find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    
    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }
    
    max_sum
}

fn main() {
    // Test cases
    println!("Is 'racecar' a palindrome? {}", is_palindrome("racecar"));
    println!("Index of first occurrence of 5: {:?}", first_occurrence(&[1, 3, 5, 5, 7], 5));
    println!("Shortest word in 'the quick brown fox': {:?}", shortest_word("the quick brown fox"));
    println!("Is 17 prime? {}", is_prime(17));
    println!("Median of [1, 3, 5, 7, 9]: {}", median(&[1, 3, 5, 7, 9]));
    println!(
        "Longest common prefix of ['flower', 'flow', 'flight']: {}",
        longest_common_prefix(&["flower", "flow", "flight"])
    );

    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    println!("3rd smallest element in {:?}: {:?}", arr, kth_smallest(&arr, 3));

    let tree = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("Maximum depth of the binary tree: {}", max_depth(tree));

    let s = "Hello, World!";
    println!("Reversed string: {}", reverse_string(s));

    println!("Is 17 prime? {}", is_prime_rust(17));

    let arr1 = [1, 3, 5, 7];
    let arr2 = [2, 4, 6, 8];
    println!("Merged sorted arrays: {:?}", merge_sorted_arrays(&arr1, &arr2));

    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&arr));
}
