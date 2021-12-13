use anyhow::Result;
use env_logger::Env;
use log::info;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CaveSize {
    Big,
    Small,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Cave {
    size: CaveSize,
    name: String,
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
struct Network {
    edges: HashMap<Cave, HashSet<Cave>>,
}

#[derive(Clone, Default)]
struct Path {
    nodes: Vec<Cave>,
    used_double: bool
}

impl Network {
    fn from_stdin() -> Result<Self> {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let lines = handle.lines();

        Ok(Self::from_vec_str(
            lines.into_iter().map(|x| x.unwrap()).collect(),
        ))
    }

    fn from_vec_str(lines: Vec<String>) -> Self {
        let mut edges = HashMap::new();
        for line in lines {
            let caves: Vec<Cave> = line
                .split("-")
                .map(|cave_literal| Cave {
                    name: cave_literal.to_string(),
                    size: match cave_literal
                        .clone()
                        .chars()
                        .next()
                        .unwrap()
                        .is_ascii_uppercase()
                    {
                        true => CaveSize::Big,
                        false => CaveSize::Small,
                    },
                })
                .collect();

            for cave_s in &caves {
                for cave_e in &caves {
                    if !(cave_s == cave_e) {
                        edges
                            .entry(cave_s.clone())
                            .or_insert(HashSet::new())
                            .insert(cave_e.clone());

                        edges
                            .entry(cave_e.clone())
                            .or_insert(HashSet::new())
                            .insert(cave_s.clone());
                    }
                }
            }
        }
        Self { edges }
    }

    fn start() -> Cave {
        Cave {
            name: "start".to_string(),
            size: CaveSize::Small,
        }
    }
    fn end() -> Cave {
        Cave {
            name: "end".to_string(),
            size: CaveSize::Small,
        }
    }

    fn explore(&self, starting_point: Cave, path: &Path) -> Vec<Path> {
        let mut paths = Vec::new();
        let nexts = self
            .edges
            .get(&starting_point)
            .expect("All should have entry");
        for next in nexts {
            if *next == Network::start() {
                continue;
            }
            if *next == Network::end() {
                let mut ending_path = path.clone();
                ending_path.nodes.push(Network::end());
                paths.push(ending_path);
            } else if next.size == CaveSize::Big || !path.nodes.contains(&next) {
                let mut next_path = path.clone();
                next_path.nodes.push(next.clone());
                for path in self.explore(next.clone(), &next_path) {
                    let mut joined = next_path.clone();
                    joined.nodes.append(&mut path.nodes.clone());
		    if path.used_double {
			joined.used_double = true;
		    }
                    paths.push(joined);
                }
            } else if !path.used_double && path.nodes.contains(&next) {
                let mut next_path = path.clone();
		next_path.used_double = true;
                next_path.nodes.push(next.clone());
                for path in self.explore(next.clone(), &next_path) {
                    let mut joined = next_path.clone();
                    joined.nodes.append(&mut path.nodes.clone());
                    paths.push(joined);
                }
	    }
        }
        paths
    }
}

fn count_paths(network: &Network) -> usize {
    let empty_path = Path::default();
    let paths = network.explore(Network::start(), &empty_path);
    paths.len()
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let test_input = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    let test_network = Network::from_vec_str(test_input.iter().map(|x| x.to_string()).collect());
    assert_eq!(count_paths(&test_network), 36);
    info!("Completed test");

    let network = Network::from_stdin()?;
    println!("Paths: {}", count_paths(&network));
    
    Ok(())
}
