//Write a function which takes a list of `i32`s and returns the biggest element
pub fn question_1(items: &[i32]) -> i32 {
    let mut max = i32::MIN;
    for &x in items {
        if x > max {
            max = x;
        }
    }
    max
}

//Write a function which takes a list of `i32`s and returns the smallest element
pub fn question_2(items: &[i32]) -> i32 {
    let mut min = i32::MAX;
    for &x in items {
        if x < min {
            min = x;
        }
    }
    min
}

//Write a function which takes a list of `i32`s and calculates the sum of the two largest elements
pub fn question_3(items: &[i32]) -> i32 {
    let mut largest = i32::MIN;
    let mut second_largest = i32::MIN;
    for &x in items {
        if x > largest {
            second_largest = largest;
            largest = x
        } else if x > second_largest {
            second_largest = x
        }
    }
    largest + second_largest
}

//Write a function which takes a list of `i32`s and calculates the difference between the largest element and the smallest element.
//Once you've written this, try writing it by using the answers to previous questions
pub fn question_4(items: &[i32]) -> i32 {
    let max = question_1(items);
    let min = question_2(items);
    max - min
}

// Write a function which takes two lists of `i32`s and calculates the "dot product"
// - the dot product of two lists is calculated by:
//   - calculating the product of elements in the same position
//   - adding up each product
// - for example, the dot product of `[1, 2, 3]` and `[4, 5, 6]` is `(1 * 4) + (2 * 5) + (3 * 6) = 4 + 10 + 18 = 32` 
// - if the two lists have a different length, the dot product is "undefined" (i.e. there is no dot product, similar to how there is "no answer" to `1 / 0`).
pub fn question_5(a: &[i32], b: &[i32]) -> i32 {
    let mut sum = 0;
    let len_a = a.len();
    let len_b = b.len();
    for index in 0..len_a {
        if len_a != len_b {
            panic!("different length")
        } else {
            let item_a = a[index];
            let item_b = b[index];
            sum += item_a * item_b
        }
    }
    sum
}

// Write a function which takes a list of `i32`s, and another `i32` called `target`. The function should return the index of `target` in the list:
// - for example, if the list is `[1, 6, 5, 7, 86576]`, and `target`is `4`, the answer should be `2` (since the number `4` is in position `2`)
// - remember, we start counting at `0`
pub fn question_6(list: &[i32], target: i32) -> usize {
    let len = list.len();
    for index in 0..len {
        let number = list[index];
        if number == target {
            return index;
        }
    }
   panic!("no match");
}


fn question_6_alternative(list: &[i32], index: usize) -> i32 {
    list[index]
}

