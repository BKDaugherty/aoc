use anyhow::{anyhow, Result};
use env_logger::Env;
use log::info;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
enum Reflection {
    X(isize),
    Y(isize),
}

#[derive(Debug)]
struct Graph {
    /// Stack of reflections
    reflections: Vec<Reflection>,
    /// Points in the graph
    points: HashSet<Point>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_x = self
            .points
            .iter()
            .max_by(|point, point2| point.x.cmp(&point2.x))
            .unwrap()
            .x;
        let max_y = self
            .points
            .iter()
            .max_by(|point, point2| point.y.cmp(&point2.y))
            .unwrap()
            .y;

        Ok(for y in 0..max_y + 1 {
            write!(f, "\n");
            for x in 0..max_x + 1 {
                if self.points.contains(&Point { x, y }) {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }
            }
        })
    }
}

impl Graph {
    fn from_stdin() -> Result<Self> {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let lines = handle.lines();

        Ok(Self::from_vec_str(
            lines.into_iter().map(|x| x.unwrap()).collect(),
        ))
    }

    fn from_vec_str(data: Vec<String>) -> Self {
        let mut points = HashSet::new();
        let mut reflections = Vec::new();
        let mut i = 0;
        while i < data.len() {
            let entry = &data[i];
            i += 1;
            if entry == "" {
                break;
            }

            let values = entry.split(",").collect::<Vec<&str>>();
            points.insert(Point {
                x: values[0].parse::<isize>().expect("x"),
                y: values[1].parse::<isize>().expect("y"),
            });
        }

        while i < data.len() {
            let reflection = &data[i];
            let mut values = reflection.split("=");
            let axis = values
                .next()
                .expect("Axis")
                .split_whitespace()
                .collect::<Vec<&str>>()[2];
            let integral = values
                .next()
                .expect("have value")
                .parse::<isize>()
                .expect("Should be integral");

            reflections.push(match axis {
                "x" => Reflection::X(integral),
                "y" => Reflection::Y(integral),
                _ => panic!("unknown reflection found"),
            });
            i += 1;
        }
        let reflections = reflections.into_iter().rev().collect::<Vec<Reflection>>();
        Self {
            reflections,
            points,
        }
    }

    fn transform(reflection: Reflection, point: Point) -> Point {
        match reflection {
            Reflection::X(integral) => {
                if point.x > integral {
                    Point {
                        x: integral - (point.x - integral),
                        y: point.y,
                    }
                } else {
                    point
                }
            }
            Reflection::Y(integral) => {
                if point.y > integral {
                    Point {
                        x: point.x,
                        y: integral - (point.y - integral),
                    }
                } else {
                    point
                }
            }
        }
    }

    /// Pop one of the reflections off of the graph and update the points involved
    fn fold(&mut self) -> Option<String> {
        let reflection = self.reflections.pop();
        let reflection = match reflection {
            Some(r) => r,
            None => return None,
        };

        let points = self
            .points
            .iter()
            .cloned()
            .map(|point| Graph::transform(reflection, point))
            .collect::<HashSet<Point>>();
        self.points = points;
        Some(format!("Completed fold {:?}", reflection))
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let testing_list = vec!["1,1", "1,5", "", "fold along y=3"];
    let mut testing_graph = Graph::from_vec_str(
        testing_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    testing_graph.fold();

    assert!(testing_graph.points.contains(&Point { x: 1, y: 1 }));
    assert_eq!(testing_graph.points.len(), 1);

    let mut graph = Graph::from_stdin()?;
    graph.fold();
    info!("Part 1: {}", graph.points.len());
    while let Some(_) = graph.fold() {}
    println!("{}", graph);
    Ok(())
}
