pub fn encode(n: u64) -> String {
    let less_twenty = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let tens = vec![
        "one", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let more_than_two_digit = vec![
        " hundred",
        " thousand",
        " million",
        " billion",
        " trillion",
        " quadrillion",
        " quintillion",
    ];

    let mut res = String::new();

    let mut parse_num = |num: u64, base: usize| unsafe {
        let h = num.wrapping_div(100) as usize;
        let t = num.wrapping_rem(100).wrapping_div(10) as usize;
        let u = num.wrapping_rem(100).wrapping_rem(10) as usize;

        if h > 0 {
            res.push_str(less_twenty.get_unchecked(h));
            res.push_str(more_than_two_digit.get_unchecked(0));
            if !res.is_empty() {
                res.push(' ')
            }
        }

        match (t, u) {
            (0, 0) if !res.is_empty() => {
                res.remove(res.len() - 1);
            } // 100, 1000

            (b, c) if b == 0 || b == 1 => {
                res.push_str(less_twenty.get_unchecked(b.wrapping_mul(10).wrapping_add(c)))
            } // 0-19

            (b, 0) => res.push_str(tens.get_unchecked(b)), // 20, 30

            (b, c) => {
                res.push_str(tens.get_unchecked(b));
                res.push('-');
                res.push_str(less_twenty.get_unchecked(c))
            } // 21, 22
        }

        if base > 0 {
            res.push_str(more_than_two_digit.get_unchecked(base));
            if !res.is_empty() {
                res.push(' ')
            }
        }
    };

    let mut check_num = |len: usize| {
        let mut base = len.wrapping_div(3);
        let mut rem = n;
        while base > 0 {
            let div = rem.wrapping_div(10u64.pow(base.wrapping_mul(3) as u32));
            rem = rem.wrapping_rem(10u64.pow(base.wrapping_mul(3) as u32));

            if div > 0 {
                parse_num(div, base) // if number is 143_237 then process 143 firstly
            }

            base -= 1;
        }

        parse_num(rem, 0) // if number is 143_237 then process 237
    };

    let mut s = String::new();
    itoa::fmt(&mut s, n).unwrap();
    let len = s.len();

    check_num(len);

    res
}
