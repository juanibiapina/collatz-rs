fn main() {
    println!("longest sequence: {}", longest_sequence(1000000));
}

pub fn next_hailstone(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        (3 * n) + 1
    }
}

pub fn heilstone_size(mut n: u64) -> usize {
    let mut total_size = 1;

    while n > 1 {
        n = next_hailstone(n);
        total_size += 1;
    }

    return total_size;
}

pub fn longest_sequence(last: u64) -> u64 {
    let mut max_size = 0;
    let mut max_starter = 0;

    for i in 1..last + 1 {
        let size = heilstone_size(i);
        if size > max_size {
            max_size = size;
            max_starter = i;
        }
    }

    return max_starter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heilstone_size() {
        assert_eq!(1, heilstone_size(1)); // 1
        assert_eq!(2, heilstone_size(2)); // 2 1
        assert_eq!(8, heilstone_size(3)); // 3 10 5 16 8 4 2 1
    }
}
