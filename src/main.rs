use regex::Regex;
use std::env;

/// expands the a-z patterns in charset
fn parse_charset(charset: &str) -> String {
    let re = Regex::new("(\\w-\\w)").unwrap();
    let res = re
        .captures_iter(charset)
        .into_iter()
        .map(|x| x.extract())
        .collect::<Vec<(&str, [&str; 1])>>()
        .into_iter()
        .map(|(matched_string, _)| {
            let first = matched_string.chars().nth(0).unwrap();
            let last = matched_string
                .chars()
                .nth(matched_string.len() - 1)
                .unwrap();
            (first..=last)
                .into_iter()
                .map(|c| format!("{}", c))
                .reduce(|acc, s| format!("{}{}", acc, s))
                .unwrap()
        })
        .reduce(|acc, s| format!("{}{}", acc, s))
        .unwrap();
    return res;
}
fn generate_line_combinations(prefix: String, charset: &String) -> Vec<String> {
    let res = charset
        .chars()
        .into_iter()
        .map(|c| format!("{}{}", c, prefix))
        .collect::<Vec<_>>();
    res
}

fn generate_line_combinations_with_length(
    charset: &String,
    acc: Vec<String>,
    counter: usize,
) -> Vec<String> {
    if counter == 0 {
        return acc;
    }
    let new_acc = acc
        .into_iter()
        .map(|prefix| generate_line_combinations(prefix, charset))
        .flatten()
        .collect::<Vec<String>>();

    generate_line_combinations_with_length(charset, new_acc, counter - 1)
}

fn generate_line_combinations_from_to(charset: &String, from: usize, to: usize) -> Vec<String> {
    (from..=to)
        .into_iter()
        .map(|i| generate_line_combinations_with_length(charset, vec!["".to_string()], i))
        .flatten()
        .collect::<Vec<String>>()
}

// fn generate_line_combinations_rec(
//     charset: &String,
//     acc: Vec<String>,
//     counter: usize,
// ) -> Vec<String> {
//     if counter == 0 {
//         return acc;
//     }
//     // let new_acc = acc
//     //     .clone()
//     //     .into_iter()
//     //     .chain(
//     //         acc.clone()
//     //             .into_iter()
//     //             .map(|prefix| generate_line_combinations(prefix, charset))
//     //             .flatten()
//     //             .collect::<Vec<String>>(),
//     //     )
//     //     .collect::<Vec<String>>();

//     let new_acc = acc
//         .into_iter()
//         .map(|prefix| generate_line_combinations(prefix, charset))
//         .flatten()
//         .collect::<Vec<String>>();

//     generate_line_combinations_rec(charset, new_acc, counter - 1)
// }

fn generate(minlength: usize, maxlength: usize, charset: String) -> Vec<String> {
    if minlength < 1 || maxlength < minlength {
        panic!("minlength should be > 1 and maxlength should be > minlength")
    }
    let res = generate_line_combinations_from_to(&charset, minlength, maxlength);
    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let charset = parse_charset("a-c");
    let res = generate(3, 4, charset);
    println!("res: {:?}", res);
    println!("res count = {:?}", res.len());
    println!("Hello, world!");
}
