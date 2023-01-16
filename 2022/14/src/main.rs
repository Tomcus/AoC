use std::collections::HashSet;
use nom::{
    IResult,
    character::complete::{
        i64,
        multispace0,
        char
    },
    sequence::{
        delimited,
        pair,
        terminated
    },
    combinator::opt,
    multi::many1
};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i64,
    y: i64
}

fn min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn parse_point(line: &str) -> IResult<&str, Point> {
    let (rem, x) = i64(line)?;
    let (rem2, _) = char(',')(rem)?;
    let (rem3, y) = i64(rem2)?;
    Ok((rem3, Point{x, y}))
}

fn parse_point_separator(line: &str) -> IResult<&str, (char, char)> {
    delimited(multispace0, pair(char('-'), char('>')), multispace0)(line)
}

fn parse_line(line: &str) -> IResult<&str, HashSet<Point>> {
    let (rem, points) = many1(terminated(parse_point, opt(parse_point_separator)))(line)?;
    let mut res = HashSet::new();
    if points.len() == 1 {
        res.insert(points[0].clone());
    } else {
        for i in 0..(points.len() - 1) {
            let min_x = min(points[i].x, points[i + 1].x);
            let max_x = max(points[i].x, points[i + 1].x);
            for x in min_x..=max_x {
                let min_y = min(points[i].y, points[i + 1].y);
                let max_y = max(points[i].y, points[i + 1].y);
                for y in min_y..=max_y {
                    res.insert(Point {x, y});
                }
            }
        }
    }
    Ok((rem, res))
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut points = HashSet::new();
    for line_raw in BufReader::new(input).lines() {
        let line = line_raw.unwrap();
        points.extend(parse_line(&line).unwrap().1);
    }
    let original_cnt = points.len();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    loop {
        let mut point = Point {x: 500, y: 0};
        if points.contains(&point) {
            break;
        }
        loop {
            let point_under = Point{x: point.x, y: point.y + 1};
            if point_under.y == max_y + 2 {
                break;
            }
            if !points.contains(&point_under) {
                point = point_under;
                continue;
            }
            let point_left = Point{x: point.x - 1, y: point.y + 1};
            if !points.contains(&point_left) {
                point = point_left;
                continue;
            }
            let point_right = Point{x: point.x + 1, y: point.y + 1};
            if !points.contains(&point_right) {
                point = point_right;
                continue;
            }
            break;
        }
        points.insert(point);
    }
    println!("Res: {}", points.len() - original_cnt);
}
