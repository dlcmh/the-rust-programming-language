fn largest(nums: &[i32]) -> i32 {
    let mut largest = nums[0];

    for &num in nums {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn main() {
    // duplicate #1
    let nums1 = vec![30, 1, 200, 3];

    let mut largest1 = nums1[0];

    for num1 in nums1 {
        if num1 > largest1 {
            largest1 = num1;
        }
    }

    println!("largest of nums1 is {}", largest1);
    // largest of nums1 is 200

    // duplicate #2
    let nums2 = vec![111, 700, 10, 99];

    let mut largest2 = nums2[0];

    for num2 in nums2 {
        if num2 > largest2 {
            largest2 = num2;
        }
    }

    println!("largest of nums2 is {}", largest2);
    // largest of nums2 is 700

    // use extracted function
    let nums3 = vec![18, 55, 88, 990];
    println!("largest of nums3 is {}", largest(&nums3));
    // largest of nums3 is 990
}
