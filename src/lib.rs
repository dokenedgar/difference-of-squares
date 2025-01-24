pub fn square_of_sum(n: u32) -> u32 {
    let mut counter = 1;
    let mut total: u32 = 0;
    loop {
        if counter <= n {
            total += counter;
            counter += 1;
        } else {
            break;
        }
    }
    total = total.pow(2);
    total
}

pub fn sum_of_squares(n: u32) -> u32 {
    todo!("sum of squares of 1...{n}")
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    let diff_result = square_of_sum(n);
    diff_result
}
