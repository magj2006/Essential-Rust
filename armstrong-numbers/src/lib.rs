use std::convert::TryInto;

pub fn is_armstrong_number(num: u32) -> bool {
    let n_str = num.to_string();

    let len: u32 = match n_str.len().try_into() {
        Ok(n) => n,
        _ => 0
    };

    let sum = n_str.chars().map(|x| x.to_digit(10).expect("input is not integer").pow(len)).sum();

    num == sum


    /* from other person

    let exponent = ((num as f32).log10().floor() + 1_f32) as u32;
    let sum_of_powers = (0..exponent)
        .map(|i| (num / 10_u32.pow(i)) % 10_u32)
        .map(|base| base.pow(exponent))
        .sum::<u32>();
    sum_of_powers == num

    **/
}
