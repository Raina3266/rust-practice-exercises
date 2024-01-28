/// print all the numbers from 1 to 100 inclusive
pub fn question_1() {
    for x in 1..=100 {
        println!("{x}");
    }
}

//print all the even numbers from 1 to 100 inclusive
pub fn question_2() {
    for x in 1..=100 {
        if x % 2 == 0 {
            println!("{x}");
        }
    }
}

//print all the numbers that are a multiple of 3 or 5 from 1 to 100 inclusive
pub fn question_3() {
    for x in 1..=100 {
        if x % 3 == 0 || x % 5 == 0 {
            println!("{x}")
        }
    }
}

//print all the square numbers from 1 to 100 inclusive
//(a square number is a number that is equal to `n*n` where `n` is an integer)
pub fn question_4() {
    let max = 100;

    for x in 1.. {
        let y = x * x;
        if y > max {
            return;
        }
        println!("{y}")
    }
}

//print all the odd square numbers from 1 to 100 inclusive
pub fn question_5() {
    let max = 100;
    for x in 1.. {
        let y = x * x;

        if y > max {
            return;
        }

        if x % 2 == 1 {
            println!("{y}")
        }
    }
}

//print the sum of all the odd square numbers from 1 to 100 inclusive
pub fn question_6() {
    let max: i32 = 100;
    let mut sum = 0;
    for x in 1.. {
        let y = x * x;
        if y < max {
            if x % 2 == 1 {
                sum += y;
            }
        } else {
            break;
        }
    }
    println!("{sum}")
}

//print the product of all the odd square numbers from 1 to 100 inclusive
pub fn question_7() {
    let max = 100;
    let mut product = 1;
    for x in 1.. {
        let y = x * x;
        if y < max {
            if x % 2 == 1 {
                product *= y;
            }
        } else {
            break;
        }
    }
    println!("{product}")
}

//print all the numbers from 1 to 100 inclusive which have a digit sum greater than 10.
//The digit sum is the sum of the individual digits of a number
//(i.e. the digit sum of 123 is `1 + 2 + 3 = 6`)
pub fn question_8() {
    for x in 1..=100 {
        if digit_sum(x) > 10 {
            println!("{x}")
        }
    }
}

fn digit_sum(mut i: i32) -> i32 {
    let mut sum = 0;

    while i != 0 {
        let last_digit = i % 10;
        let without_last_digit = i / 10;
        sum += last_digit;
        i = without_last_digit;
    }
    sum
}

//print all the numbers from 1 to 100 inclusive which have a digit sum that is even
pub fn question_9() {
    for x in 1..=100 {
        if digit_sum(x) % 2 == 0 {
            println!("{x}")
        }
    }
}

//print numbers from 1 to 100 at random.
//It must give different results every time it's run, and should print roughly 50 numbers each time.
pub fn question_10() {
    for x in 1..=100 {
        let random = rand::random();
        if random {
            println!("{x}")
        }
    }
}

//write a function which returns the "Collatz Stopping Time" for a number.
//The collatz stopping time for a number is the number of steps it takes for a number to reach 1 when you repeatedly apply the following rules:
//- if the number is even, the next number is `n / 2`
//- if the numeber is odd, the next number is `(3 * n) + 1`
pub fn question_11() {
    let cst = collatz_stopping_time(2);
    println!("the collatz stopping time is {cst}")
}

fn collatz_stopping_time(mut n: u32) -> u32 {
    let mut count = 0;

    while n != 1 {
        if n % 2 == 1 {
            count += 1;
            n = (3 * n) + 1;
        } else {
            count += 1;
            n = n / 2;
        }
    }
    count
}

//print `1!` to `10!` inclusive (`n!` is `1 * 2 * 3 * ... * n`, pronounced "n factorial")
pub fn question_12() {
    for n in 1..=10 {
        let x = n_factorial(n);
        println!("{x}")
    }
}

fn n_factorial(n: i32) -> i32 {
    let mut x = 1;
    let mut count = 1;
    while count < n {
        count += 1;
        x = x * count;
    }
    x
} 

//write a function that plays fizzbuzz up to 100. Fizzbuzz is a game where you print numbers from 1 to 100 except
// - if the number is divisible by 3, you print "fizz" instead of the number
// - if the number is divisible by 5, you print "buzz" instead of the number
// - if the number is divisible by 3 and 5, you print "fizzbuzz" instead of the number
pub fn question_14() {
    for x in 1..=100 {
        if x % 15 == 0 {
            println!("fizzbuzz")
        } else if x % 5 == 0 {
            println!("buzz")
        } else if x % 3 == 0 {
            println!("fizz")
        } else {
            println!("{x}")
        }
    }
}

fn cams_version_of_fizzbuzz() {
    for x in 1..=100 {
        let divisible_by_3 = x % 3 == 0;
        let divisible_by_5 = x % 5 == 0;

        match (divisible_by_3, divisible_by_5) {
            (false, false) => println!("{x}"),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (true, true) => println!("fizzbuzz"),
        }
    }
}

//write a function that calculates the area of a triangle with the following properties:
// - there are 3 points, A, B and C
// - the distance between A and B is `x` (one of the inputs to the function)
// - the distance between A and C is `y` (one of the inputs to the function)
// - the angle between the lines AB and AC is 90 degrees
pub fn question_15() {
    let z = triangle_area(6, 8);
    println!("the area is {z}")
}

fn triangle_area(x: i32, y: i32) -> i32 {
    let square = x * y / 2;
    return square
}

//Write a function that finds the largest single digit in a number (i.e. for 251 it would be `5`)
pub fn question_16() {
    let largest = largest_digit(148723461);
    println!("{largest}")
}

fn largest_digit(mut i: i32) -> i32 {
    let mut max = 0;

    while i != 0 {
        let last_digit = i % 10;
        let without_last_digit = i / 10;
        if last_digit > max {
            max = last_digit
        }
        i = without_last_digit;
    }
    max
}