fn main() {
	let number = 9;
	println!("fibonacci_1: {}", fibonacci_1(number));
	println!("fibonacci_2: {}", fibonacci_2(number));
	println!("fibonacci_3: {}", fibonacci_2(number));
}

fn fibonacci_1(n: u128) -> u128 {
	fn f(n: u128, current: u128, next: u128) -> u128 {
		match n {
			0 => current,
			_ => f(n - 1, next, current + next)
		}
	}
	f(n, 0, 1)
}

fn fibonacci_2(n: u128) -> u128 {
	let mut current: u128 = 0;
	let mut next: u128 = 1;
	for _ in 0..n {
		(current, next) = (next, current + next)
	};
	current
}

fn fibonacci_3(count: usize) -> Vec<u32> {
	let mut digits = vec![1,1];
	if count < 2 {
		digits.remove(1); } else {
			for i in 2..count {
			let next = digits[i-2] + digits[i-1];
			digits.push(next);
		}
	}
	return digits
}

#[test]
fn fib_1() {
    assert_eq!(34, fibonacci_1(9));
}
#[test]
fn fib_2() {
    assert_eq!(34, fibonacci_2(9));
}

#[test]
fn fib_3() {
	assert_eq!(fibonacci_3(5), &[1, 1, 2, 3, 5]);
	assert_eq!(fibonacci_3(1), &[1]);
	assert_eq!(fibonacci_3(8), &[1, 1, 2, 3, 5, 8, 13, 21]);
}
