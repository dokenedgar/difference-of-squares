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
    let mut counter = 1;
    let mut total: u32 = 0;
    loop {
        if counter <= n {
            total += counter.pow(2);
            counter += 1;
        } else {
            break;
        }
    }
    total
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
