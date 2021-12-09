use anyhow::{anyhow, Result};
use maplit::{hashmap, hashset};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct Sequence {
    pub signals: HashSet<Signal>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone)]
struct SignalMapping {
    pub state: HashMap<Signal, Pos>,
}

impl SignalMapping {
    fn store(&mut self, signal: Signal, pos: Pos) {
        self.state.insert(signal, pos);
    }

    fn from_inputs(inputs: Vec<Sequence>) -> Self {
        let mut mapping = SignalMapping {
            state: HashMap::new(),
        };
        // 1, 4, 7, 8 are unique
        // Find the 1, then subtract from 7, which gives you top.
        let digit_1 = inputs
            .iter()
            .filter(|x| x.signals.len() == 2)
            .next()
            .expect("Found 1");
        let digit_4 = inputs
            .iter()
            .filter(|x| x.signals.len() == 4)
            .next()
            .expect("Found 4");
        let digit_7 = inputs
            .iter()
            .filter(|x| x.signals.len() == 3)
            .next()
            .expect("Found 7");
        let digit_8 = inputs
            .iter()
            .filter(|x| x.signals.len() == 7)
            .next()
            .expect("Found 8");
        let digits_length_6 = inputs.iter().filter(|x| x.signals.len() == 6);

        // 7 - 1 == Top
        mapping.store(
            *digit_7
                .signals
                .difference(&digit_1.signals)
                .next()
                .expect("storing top"),
            Pos::Top,
        );
        // 8 - (9, 6, 0(len 6)) == (BL, M, TR) (Membership in 1 == TR, which also gives BR)
        let mut unknown_6_diff_with_8 = HashSet::new();
        for digit in digits_length_6 {
            unknown_6_diff_with_8 = unknown_6_diff_with_8
                .union(
                    &digit_8
                        .signals
                        .difference(&digit.signals)
                        .cloned()
                        .collect::<HashSet<Signal>>(),
                )
                .cloned()
                .collect::<HashSet<Signal>>();
        }
        let only_tr = unknown_6_diff_with_8
            .intersection(&digit_1.signals)
            .cloned()
            .collect::<HashSet<Signal>>();

        mapping.store(
            *digit_1
                .signals
                .difference(&only_tr)
                .next()
                .expect("storing br"),
            Pos::BottomRight,
        );
        mapping.store(*only_tr.iter().next().expect("storing tr"), Pos::TopRight);

        // Found (T, TR, BR)
        // 4 - 1 => (TL, M)
        // (4 - 1) Intersect (8 - (9, 6, 0)) => (M) and by association (TL)
        let tl_and_m = digit_4
            .signals
            .difference(&digit_1.signals)
            .cloned()
            .collect::<HashSet<Signal>>();
        let only_m = tl_and_m
            .intersection(&unknown_6_diff_with_8)
            .cloned()
            .collect::<HashSet<Signal>>();
        let tl = tl_and_m
            .difference(&only_m)
            .cloned()
            .collect::<HashSet<Signal>>();

        mapping.store(*only_m.iter().next().expect("Storing M"), Pos::Mid);
        mapping.store(*tl.iter().next().expect("Storing TL"), Pos::TopLeft);

        // Found (T, TR, TL, M, BR)
        // (Int(5s) - 7) - 4 = B, last one is BL
        let digits_length_5 = inputs.iter().filter(|x| x.signals.len() == 5);
        let mut intersection_of_5s: Option<HashSet<Signal>> = None;
        for digit in digits_length_5 {
            intersection_of_5s = match intersection_of_5s {
                None => Some(digit.signals.clone()),
                Some(signals) => Some(
                    signals
                        .intersection(&digit.signals)
                        .cloned()
                        .collect::<HashSet<Signal>>(),
                ),
            }
        }
        mapping.store(
            *intersection_of_5s
                .expect("finding 5s")
                .difference(&digit_7.signals)
                .cloned()
                .collect::<HashSet<Signal>>()
                .difference(&digit_4.signals)
                .next()
                .expect("Storing B"),
            Pos::Bottom,
        );
        // B is present in all 5s.
        for signal in SIGNAL_VARIANTS {
            if let None = mapping.state.get(signal) {
                mapping.store(*signal, Pos::BottomLeft);
            }
        }
        mapping
    }
}

static SIGNAL_VARIANTS: &[Signal] = &[
    Signal::A,
    Signal::B,
    Signal::C,
    Signal::D,
    Signal::E,
    Signal::F,
    Signal::G,
];

impl Signal {
    fn from_literal(literal: String) -> HashSet<Signal> {
        let mut signals = HashSet::new();
        for c in literal.chars() {
            signals.insert(match c {
                'a' => Signal::A,
                'b' => Signal::B,
                'c' => Signal::C,
                'd' => Signal::D,
                'e' => Signal::E,
                'f' => Signal::F,
                'g' => Signal::G,
                x => panic!("Unknown signal"),
            });
        }
        signals
    }
}

fn get_digit_map() -> HashMap<u8, HashSet<Pos>> {
    hashmap!(
    0 => hashset!(Pos::Top, Pos::TopRight, Pos::TopLeft, Pos::BottomLeft, Pos::Bottom, Pos::BottomRight),
    1 => hashset!(Pos::TopRight, Pos::BottomRight),
    2 => hashset!(Pos::Top, Pos::TopRight, Pos::Mid, Pos::BottomLeft, Pos::Bottom),
    3 => hashset!(Pos::Top, Pos::TopRight, Pos::Mid, Pos::BottomRight, Pos::Bottom),
    4 => hashset!(Pos::TopLeft, Pos::TopRight, Pos::Mid, Pos::BottomRight),
    5 => hashset!(Pos::Top, Pos::TopLeft, Pos::Mid, Pos::BottomRight, Pos::Bottom),
    6 => hashset!(Pos::Top, Pos::TopLeft, Pos::Mid, Pos::BottomRight, Pos::BottomLeft, Pos::Bottom),
    7 => hashset!(Pos::Top, Pos::TopRight, Pos::BottomRight),
    8 => hashset!(Pos::Top, Pos::TopLeft, Pos::TopRight, Pos::Mid, Pos::BottomRight, Pos::BottomLeft, Pos::Bottom),
    9 => hashset!(Pos::Top, Pos::TopLeft, Pos::TopRight, Pos::Mid, Pos::BottomRight, Pos::Bottom))
}

static POS_VARIANTS: &[Pos] = &[
    Pos::Top,
    Pos::TopLeft,
    Pos::TopRight,
    Pos::Mid,
    Pos::BottomLeft,
    Pos::BottomRight,
    Pos::Bottom,
];

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
enum Pos {
    Top,
    TopLeft,
    TopRight,
    Mid,
    BottomLeft,
    BottomRight,
    Bottom,
}

fn translate(input: HashSet<Signal>, mapping: &SignalMapping) -> Result<HashSet<Pos>> {
    let mut output = HashSet::new();
    for i in input {
        output.insert(
            *mapping
                .state
                .get(&i)
                .ok_or(anyhow!("Couldnt find input {:?}", i))?,
        );
    }
    Ok(output)
}

impl Sequence {
    fn from_string(literal: String) -> Self {
        Self {
            signals: Signal::from_literal(literal),
        }
    }
}

fn read_input() -> Result<Vec<(Vec<Sequence>, Vec<Sequence>)>> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    Ok(lines
        .map(|line| {
            let line = line.expect("Line should exist");
            let mut pairs = line.split("|").map(|entry| {
                entry
                    .split_whitespace()
                    .map(|literal| Sequence::from_string(literal.to_string()))
                    .collect::<Vec<Sequence>>()
            });
            (pairs.next().unwrap(), pairs.next().unwrap())
        })
        .collect::<Vec<(Vec<Sequence>, Vec<Sequence>)>>())
}

fn slow_reverse_lookup(inputs: HashSet<Pos>) -> Result<u8> {
    let mapping = get_digit_map();
    for (idx, set) in mapping {
        if set == inputs {
            return Ok(idx);
        }
    }
    Err(anyhow!("set not found {:?}", inputs))
}

fn main() -> Result<()> {
    let entry_pairs = read_input()?;
    // Part 1
    // let mut count = 0;
    // for (signals, outputs) in entry_pairs {
    //     for output in outputs {
    //         match output.signals.len() {
    //             2 | 4 | 3 | 7 => {
    //                 count += 1;
    //             }
    //             _ => {}
    //         }
    //     }
    // }
    // println!("Count: {}", count);

    // Part 2
    let mut total = 0;
    for (signals, outputs) in entry_pairs {
        let mapping = SignalMapping::from_inputs(signals);
        let exp: Vec<usize> = vec![1000, 100, 10, 1];
        for (idx, output) in outputs.iter().enumerate() {
            let digit = slow_reverse_lookup(translate(output.clone().signals, &mapping)?)?;
            total += exp[idx] as usize * digit as usize
        }
    }
    println!("Total: {}", total);

    Ok(())
}
