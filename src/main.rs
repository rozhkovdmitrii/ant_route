use clap::{CommandFactory, Parser, Subcommand};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::str::FromStr;

trait DigitSum {
    fn digit_sum(&self) -> u32;
}

impl DigitSum for i32 {
    fn digit_sum(&self) -> u32 {
        let mut copy: u32 = self.clone().abs() as u32;
        let mut sum = 0u32;

        while copy != 0 {
            sum += copy.borrow() % 10;
            copy = copy / 10;
        }
        sum
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[cfg(test)]
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn inc(self: &mut Self, x_diff: i32, y_diff: i32) -> Self {
        let mut copy = self.clone();
        copy.x += x_diff;
        copy.y += y_diff;
        copy
    }
}

impl DigitSum for Point {
    fn digit_sum(&self) -> u32 {
        self.x.digit_sum() + self.y.digit_sum()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut p = Point { x: 0, y: 0 };
        let count = value
            .split(":")
            .enumerate()
            .map(|(index, num)| {
                if index == 0 {
                    p.x = i32::from_str(num).expect("Wrong i32 number: {number}");
                } else {
                    p.y = i32::from_str(num).expect("Wrong i32 number: {number}");
                }
            })
            .count();
        assert_eq!(count, 2, "Wrong point format: {value}");
        p
    }
}

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    Start {
        #[clap(long, value_parser, value_name = "X:Y")]
        from: Point,
        #[clap(long, value_parser, value_name = "NUM")]
        dig_sum_limit: u32,
    },
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn execute() {
        let parsed_cli = Self::parse();
        match &parsed_cli.command {
            Some(Command::Start {
                from,
                dig_sum_limit,
            }) => {
                calc_points(*from, *dig_sum_limit);
            }
            None => {
                Self::command().print_help().unwrap();
            }
        };
    }
}

fn calc_points(start_from: Point, dig_sum_limit: u32) {
    let count = calc_points_impl(start_from, dig_sum_limit);
    println!("Points are on ant's trace: {count}");
}

fn calc_points_impl(start_from: Point, dig_sum_limit: u32) -> usize {
    fn step_into(mut point: Point, dig_sum_limit: u32, visited: &mut HashSet<Point>) {
        if point.digit_sum() > dig_sum_limit || visited.contains(&point) {
            return;
        }

        visited.insert(point);
        step_into(point.inc(0, 1), dig_sum_limit, visited);
        step_into(point.inc(1, 0), dig_sum_limit, visited);
        step_into(point.inc(0, -1), dig_sum_limit, visited);
        step_into(point.inc(-1, 0), dig_sum_limit, visited);
    }
    let mut visited = HashSet::<Point>::new();
    step_into(start_from, dig_sum_limit, &mut visited);
    visited.len()
}

fn main() {
    Cli::execute();
}

#[test]
fn test_digit_sum_u32() {
    assert_eq!(40, 28282828.digit_sum());
    assert_eq!(0, 0.digit_sum());
    assert_eq!(40, (-28282828).digit_sum());
}

#[test]
fn test_digit_sum_point() {
    assert_eq!(40, Point::new(2828, 2828).digit_sum());
    assert_eq!(40, Point::new(2828, -2828).digit_sum());
    assert_eq!(20, Point::new(0, -2828).digit_sum());
}

#[test]
fn test_inc_point() {
    assert_eq!(Point::new(1, 1), Point::default().inc(1, 1));
    assert_eq!(Point::new(0, 0), Point::default().inc(0, 0));
    assert_eq!(Point::new(0, 5), Point::default().inc(0, 5));
    assert_eq!(Point::new(15, -100), Point::new(25, -105).inc(-10, 5));
}

#[test]
fn test_calc_points() {
    assert_eq!(25, calc_points_impl(Point::new(0, 0), 3));
}
