use std::{env, fs};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

#[derive(Debug)]
enum Expression {
    Number(u64),
    Array(Vec<Expression>),
    Error,
}

fn parser<'a>() -> impl Parser<'a, &'a str, Expression, extra::Err<Rich<'a, char>>> {
    let number = any()
        .filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .collect::<String>()
        .map(|s| {
            if let Ok(n) = s.parse::<u64>() {
                return Expression::Number(n);
            }
            return Expression::Error;
        });

    just('[')
        .ignore_then(
            number
                .recover_with(via_parser(none_of([']']).map(|_| Expression::Error)))
                .separated_by(just(',').recover_with(via_parser(none_of([']']).map(|_| ','))))
                .collect::<Vec<Expression>>(),
        )
        .then_ignore(just(']').recover_with(via_parser(any().or_not().map(|_| ']'))))
        .map(move |arr| Expression::Array(arr))
        .or(number)
}

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let (json, errs) = parser().parse(src.trim()).into_output_errors();
    println!("{:#?}", json);
    errs.into_iter().for_each(|e| {
        println!("{}", e);
    });
}
