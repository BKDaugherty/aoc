use anyhow::{anyhow, Result};
use env_logger::Env;
use grid::{Grid, Index};
use log::info;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

/// Finds a path with the lowest risk level using Dijkstra's algorithm
fn dijkstra_search(grid: &Grid) -> Result<usize> {
    let mut unvisited = HashSet::new();

    for x in 0..grid.length as isize {
        for y in 0..grid.height as isize {
            unvisited.insert(Index { x, y });
        }
    }
    let mut distances = HashMap::new();

    let start = Index { x: 0, y: 0 };
    distances.insert(start, 0);

    let mut current_node = start;
    let finish = Index {
        x: (grid.length - 1) as isize,
        y: (grid.height - 1) as isize,
    };

    while unvisited.contains(&finish) {
        let current_distance = *distances
            .get(&current_node)
            .expect("Must have distance to explored node");

        for neighbor in grid.neighbors(&current_node) {
            if let Some(value) = grid.get(&neighbor) {
                let neighbor_distance = distances
                    .entry(neighbor)
                    .or_insert(current_distance + value);
                if *neighbor_distance > current_distance + value {
                    *neighbor_distance = current_distance + value;
                }
            }
        }
        unvisited.remove(&current_node);

        if unvisited.len() == 0 {
            break;
        }

        // Get the next current node
        current_node = *distances
            .iter()
            .filter(|(idx, _)| unvisited.contains(*idx))
            .min_by(|(_, v), (_, v1)| v.cmp(v1))
            .expect("should be a minimum")
            .0;
    }

    match distances.get(&finish) {
        Some(value) => Ok(*value),
        None => Err(anyhow!("Didnt' find finish value??")),
    }
}

fn big_grid(grid: Grid, dim_up: usize) -> Grid {
    let mut storage = Vec::new();
    let length = grid.length * dim_up;
    let height = grid.height * dim_up;

    for y in 0..height {
        for x in 0..length {
            let modx = x / grid.length;
            let mody = y / grid.height;

            let old_value = grid
                .access((x % grid.length) as isize, (y % grid.height) as isize)
                .expect("should be there");

            let new_value = match (old_value + modx + mody) {
                x if x > 9 => (x % 10) + 1,
                x => x,
            };

            storage.push(new_value);
        }
    }

    let mut big_grid = Grid {
        storage,
        length,
        height,
    };

    big_grid
}

// Hamming Distance
fn a_star_heuristic(current: &Index, finish: &Index) -> usize {
    (finish.x - current.x + finish.y - current.y) as usize
}

fn a_star_search(grid: &Grid) -> Result<usize> {
    let mut discovered = PriorityQueue::new();
    // Start to point
    let mut total_score = HashMap::new();

    let start = Index { x: 0, y: 0 };
    let goal = Index {
        x: (grid.length - 1) as isize,
        y: (grid.height - 1) as isize,
    };

    // Guess at point to goal
    let mut guess_score = HashMap::new();

    total_score.insert(start, 0);
    let guess = a_star_heuristic(&start, &goal);
    guess_score.insert(goal, guess);

    discovered.push(start, Reverse(guess));

    while let Some((current, score)) = discovered.pop() {
        // info!("Exploring {:?} with prio {}", current, score);
        if current == goal {
            return Ok(score.0);
        }

        let known_distance = *total_score
            .get(&current)
            .expect("Should know score of visited");

        for neighbor in grid.neighbors(&current) {
            let tentative_known = known_distance + grid.get(&neighbor).expect("should exist");

            let known_neighbor_dist = total_score.entry(neighbor).or_insert(tentative_known + 1);
            if tentative_known < *known_neighbor_dist {
                *known_neighbor_dist = tentative_known;
                let new_priority = tentative_known + a_star_heuristic(&neighbor, &goal);
                if discovered.get(&neighbor).is_some() {
                    discovered.change_priority(&neighbor, Reverse(new_priority));
                } else {
                    discovered.push(neighbor, Reverse(new_priority));
                }
            }
        }
    }
    Err(anyhow!("No path found :("))
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let test_grid = Grid::from_vec_str(
        3,
        3,
        vec!["133", "133", "111"]
            .iter()
            .map(|x| x.to_string())
            .collect(),
    )?;

    // TODO: Write test for big grid

    let test_result = dijkstra_search(&test_grid)?;
    assert_eq!(test_result, 4);

    let test_result = a_star_search(&test_grid)?;
    assert_eq!(test_result, 4);

    let tiny_grid = Grid::from_vec_str(
        2,
        2,
        vec!["12", "34"].iter().map(|x| x.to_string()).collect(),
    )?;

    let example_expand =
        Grid::from_vec_str(1, 1, vec!["8"].iter().map(|x| x.to_string()).collect())?;

    let test_expand = big_grid(example_expand, 5);
    assert_eq!(
        test_expand.storage,
        Grid::from_vec_str(
            5,
            5,
            vec!["89123", "91234", "12345", "23456", "34567"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )?
        .storage
    );

    let test_big_grid = big_grid(tiny_grid, 2);
    assert_eq!(
        test_big_grid.storage,
        Grid::from_vec_str(
            4,
            4,
            vec!["1223", "3445", "2334", "4556"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )?
        .storage
    );

    let big_example_expand = Grid::from_vec_str(
        10,
        10,
        vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect(),
    )?;
    let big_example_expand = big_grid(big_example_expand, 5);

    let big_example_expand_result = Grid::from_vec_str(
        50,
        50,
        vec![
            "11637517422274862853338597396444961841755517295286",
            "13813736722492484783351359589446246169155735727126",
            "21365113283247622439435873354154698446526571955763",
            "36949315694715142671582625378269373648937148475914",
            "74634171118574528222968563933317967414442817852555",
            "13191281372421239248353234135946434524615754563572",
            "13599124212461123532357223464346833457545794456865",
            "31254216394236532741534764385264587549637569865174",
            "12931385212314249632342535174345364628545647573965",
            "23119445813422155692453326671356443778246755488935",
            "22748628533385973964449618417555172952866628316397",
            "24924847833513595894462461691557357271266846838237",
            "32476224394358733541546984465265719557637682166874",
            "47151426715826253782693736489371484759148259586125",
            "85745282229685639333179674144428178525553928963666",
            "24212392483532341359464345246157545635726865674683",
            "24611235323572234643468334575457944568656815567976",
            "42365327415347643852645875496375698651748671976285",
            "23142496323425351743453646285456475739656758684176",
            "34221556924533266713564437782467554889357866599146",
            "33859739644496184175551729528666283163977739427418",
            "35135958944624616915573572712668468382377957949348",
            "43587335415469844652657195576376821668748793277985",
            "58262537826937364893714847591482595861259361697236",
            "96856393331796741444281785255539289636664139174777",
            "35323413594643452461575456357268656746837976785794",
            "35722346434683345754579445686568155679767926678187",
            "53476438526458754963756986517486719762859782187396",
            "34253517434536462854564757396567586841767869795287",
            "45332667135644377824675548893578665991468977611257",
            "44961841755517295286662831639777394274188841538529",
            "46246169155735727126684683823779579493488168151459",
            "54698446526571955763768216687487932779859814388196",
            "69373648937148475914825958612593616972361472718347",
            "17967414442817852555392896366641391747775241285888",
            "46434524615754563572686567468379767857948187896815",
            "46833457545794456865681556797679266781878137789298",
            "64587549637569865174867197628597821873961893298417",
            "45364628545647573965675868417678697952878971816398",
            "56443778246755488935786659914689776112579188722368",
            "55172952866628316397773942741888415385299952649631",
            "57357271266846838237795794934881681514599279262561",
            "65719557637682166874879327798598143881961925499217",
            "71484759148259586125936169723614727183472583829458",
            "28178525553928963666413917477752412858886352396999",
            "57545635726865674683797678579481878968159298917926",
            "57944568656815567976792667818781377892989248891319",
            "75698651748671976285978218739618932984172914319528",
            "56475739656758684176786979528789718163989182927419",
            "67554889357866599146897761125791887223681299833479",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect(),
    )?;

    assert_eq!(
        big_example_expand.storage,
        big_example_expand_result.storage
    );

    assert_eq!(a_star_search(&big_example_expand)?, 315);

    let grid = Grid::from_stdin(100, 100)?;
    // println!("distance: {}", dijkstra_search(&grid)?);
    assert_eq!(a_star_search(&grid)?, 702);

    let big_grid = big_grid(grid, 5);
    let result = a_star_search(&big_grid)?;

    println!("big distance: {}", result);

    Ok(())
}
