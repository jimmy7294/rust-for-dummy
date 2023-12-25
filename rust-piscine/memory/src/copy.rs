/*
Create 3 functions with the following requirements:
- nbr_function: takes an i32 as an argument and returns a tuple
    . with the original value
    . the exponential function of the value (e^x) -> f64, n.exp()
    . the natural logarithm of the absolute value (ln(|x|)) -> f64, n.abs().ln()
- str_function: takes a String as an argument and returns a tuple
    . with the original value
    . and the exponential function of each value as a string (e^x)
- vec_function: takes a Vec<i32> as an argument and returns a tuple
    . with the original value
    . and the natural logarithm of each absolute value (ln(|x|))
*/

pub fn nbr_function(n: i32) -> (i32, f64, f64) {
    (n, (n as f64).exp(), (n as f64).abs().ln())
}

// note: str_function will be divided into 2 functions, the first function takes a String as an argument and parses it into numeric values for calculation.
// the second function takes the result of the first function and returns a tuple with the original value and the exponential function of each value as a String
pub fn str_function(s: String) -> (String, String) {
    let parsed = parse_string(s);
    let mut original = String::new();
    for e in parsed[0].clone() {
        original.push_str(&e.to_string());
        original.push(' ');
    }
    let mut exp = String::new();
    for e in parsed[1].clone() {
        exp.push_str(&e.to_string());
        exp.push(' ');
    }

    (original.trim().to_string(), exp.trim().to_string())
}
/*
parse_string i.e.:
"1 2 3 4 5"
result:
[ {1, 2, 3, 4, 5}, {2.718281828459045, 7.38905609893065, 20.085536923187668, 54.598150033144236, 148.4131591025766}]
*/
fn parse_string(s: String) -> Vec<Vec<f64>> {
    let mut s_to_vec = Vec::new();
    let s = s.split_whitespace();
    for number in s {
        s_to_vec.push(number.parse::<f64>().unwrap());
    }

    let mut exp = Vec::new();
    for number in s_to_vec.clone() {
        exp.push(number.exp());
    }

    let mut res = Vec::new();
    res.push(s_to_vec);
    res.push(exp);
    res
}

pub fn vec_function(v: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ori = v.clone();
    let exp = v.iter().map(|n| (*n as f64).abs().ln()).collect();
    (ori, exp)
}
