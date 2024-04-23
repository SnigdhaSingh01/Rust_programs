//4.Implement a function that checks whether a given number is prime or not.

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    if num <= 3 {
        return true; // 2 and 3 are prime
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false; // Numbers divisible by 2 or 3 are not prime
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false; // Numbers divisible by other factors are not prime
        }
        i += 6;
    }

    true // If no factors were found, the number is prime
}

fn main() {
    let num = 17;
    
    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}

