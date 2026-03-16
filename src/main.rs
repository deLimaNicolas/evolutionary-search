mod evolution;

use evolution::{evolve, EvolutionConfig};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u64,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct KnapsackProblem {
    pub items: Vec<Item>,
    pub capacity: u64,
}

#[derive(Debug, Clone)]
pub struct KnapsackSolution {
    pub selected: Vec<usize>,
}

impl KnapsackSolution {
    pub fn total_value(&self, problem: &KnapsackProblem) -> u64 {
        self.selected.iter().map(|&i| problem.items[i].value).sum()
    }

    pub fn total_weight(&self, problem: &KnapsackProblem) -> u64 {
        self.selected.iter().map(|&i| problem.items[i].weight).sum()
    }

    pub fn is_valid(&self, problem: &KnapsackProblem) -> bool {
        let n = problem.items.len();
        let mut seen = vec![false; n];
        for &i in &self.selected {
            if i >= n || seen[i] {
                return false;
            }
            seen[i] = true;
        }
        self.total_weight(problem) <= problem.capacity
    }
}

#[derive(Debug)]
pub struct Level {
    pub number: u8,
    pub name: &'static str,
    pub description: &'static str,
    pub num_items: usize,
    pub capacity_factor: f64,
    pub max_weight: u64,
    pub max_value: u64,
    pub time_limit: Duration,
    pub is_exact_feasible: bool,
}

pub fn get_levels() -> Vec<Level> {
    vec![
        Level {
            number: 1,
            name: "Warm-Up",
            description: "10 items. Your cat could solve this.",
            num_items: 10,
            capacity_factor: 0.5,
            max_weight: 50,
            max_value: 100,
            time_limit: Duration::from_secs(2),
            is_exact_feasible: true,
        },
        Level {
            number: 2,
            name: "Easy",
            description: "50 items. Brute force starts sweating.",
            num_items: 50,
            capacity_factor: 0.4,
            max_weight: 200,
            max_value: 500,
            time_limit: Duration::from_secs(5),
            is_exact_feasible: true,
        },
        Level {
            number: 3,
            name: "Gem Trap S",
            description: "30 items. Heavy diamonds vs light pebbles.",
            num_items: 30,
            capacity_factor: 0.3,
            max_weight: 100,
            max_value: 100,
            time_limit: Duration::from_secs(5),
            is_exact_feasible: true,
        },
        Level {
            number: 4,
            name: "Weight Cliff S",
            description: "40 items. One heavy item worth more than all light ones combined.",
            num_items: 40,
            capacity_factor: 0.25,
            max_weight: 100,
            max_value: 100,
            time_limit: Duration::from_secs(5),
            is_exact_feasible: true,
        },
        Level {
            number: 5,
            name: "Inverse Density S",
            description: "50 items. Best combo has worst individual density.",
            num_items: 50,
            capacity_factor: 0.3,
            max_weight: 200,
            max_value: 500,
            time_limit: Duration::from_secs(5),
            is_exact_feasible: true,
        },
        Level {
            number: 6,
            name: "Respectable",
            description: "200 items. You need DP or you're cooked.",
            num_items: 200,
            capacity_factor: 0.35,
            max_weight: 500,
            max_value: 1_000,
            time_limit: Duration::from_secs(10),
            is_exact_feasible: true,
        },
        Level {
            number: 7,
            name: "Gem Trap M",
            description: "200 items. Greedy grabs pebbles, misses diamonds.",
            num_items: 200,
            capacity_factor: 0.2,
            max_weight: 500,
            max_value: 1_000,
            time_limit: Duration::from_secs(15),
            is_exact_feasible: true,
        },
        Level {
            number: 8,
            name: "Staircase",
            description: "300 items. Items form a staircase — greedy trips on the steps.",
            num_items: 300,
            capacity_factor: 0.25,
            max_weight: 1_000,
            max_value: 2_000,
            time_limit: Duration::from_secs(15),
            is_exact_feasible: true,
        },
        Level {
            number: 9,
            name: "Decoy Flood",
            description: "500 items. 90% are decoys with amazing density but tiny value.",
            num_items: 500,
            capacity_factor: 0.15,
            max_weight: 1_000,
            max_value: 1_000,
            time_limit: Duration::from_secs(15),
            is_exact_feasible: true,
        },
        Level {
            number: 10,
            name: "Anti-Density M",
            description: "500 items. Low density items pack perfectly together.",
            num_items: 500,
            capacity_factor: 0.2,
            max_weight: 1_000,
            max_value: 1_000,
            time_limit: Duration::from_secs(20),
            is_exact_feasible: true,
        },
        Level {
            number: 11,
            name: "Sweaty",
            description: "1,000 items. Memory-efficient DP or bust.",
            num_items: 1_000,
            capacity_factor: 0.25,
            max_weight: 5_000,
            max_value: 10_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: true,
        },
        Level {
            number: 12,
            name: "Gem Trap L",
            description: "1K items. Scaled up gem trap.",
            num_items: 1_000,
            capacity_factor: 0.15,
            max_weight: 5_000,
            max_value: 10_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: true,
        },
        Level {
            number: 13,
            name: "Quadratic Value",
            description: "2K items. Value = weight². Heavy items are gold.",
            num_items: 2_000,
            capacity_factor: 0.2,
            max_weight: 10_000,
            max_value: 100_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: false,
        },
        Level {
            number: 14,
            name: "Capacity Wasters",
            description: "2K items. Greedy leaves 30% capacity unused.",
            num_items: 2_000,
            capacity_factor: 0.2,
            max_weight: 10_000,
            max_value: 10_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: false,
        },
        Level {
            number: 15,
            name: "Gem Trap XL",
            description: "10K items. Massive gem trap.",
            num_items: 10_000,
            capacity_factor: 0.1,
            max_weight: 100_000,
            max_value: 100_000,
            time_limit: Duration::from_secs(60),
            is_exact_feasible: false,
        },
        Level {
            number: 16,
            name: "Correlated Chaos",
            description: "10K items where value ~ weight. Density is meaningless.",
            num_items: 10_000,
            capacity_factor: 0.2,
            max_weight: 500_000_000,
            max_value: 500_000_000,
            time_limit: Duration::from_secs(60),
            is_exact_feasible: false,
        },
        Level {
            number: 17,
            name: "Heavy Hitters",
            description: "10K items. Value scales with sqrt(weight).",
            num_items: 10_000,
            capacity_factor: 0.25,
            max_weight: 1_000_000,
            max_value: 1_000_000,
            time_limit: Duration::from_secs(60),
            is_exact_feasible: false,
        },
        Level {
            number: 18,
            name: "Greedy's Nightmare",
            description: "5K items. Every greedy heuristic fails here.",
            num_items: 5_000,
            capacity_factor: 0.2,
            max_weight: 100_000,
            max_value: 100_000,
            time_limit: Duration::from_secs(60),
            is_exact_feasible: false,
        },
        Level {
            number: 19,
            name: "Scale Test",
            description: "50K items. Pure scale.",
            num_items: 50_000,
            capacity_factor: 0.1,
            max_weight: 10_000_000,
            max_value: 10_000_000,
            time_limit: Duration::from_secs(120),
            is_exact_feasible: false,
        },
        Level {
            number: 20,
            name: "God Mode",
            description: "100K items. Beat greedy here and you're hired.",
            num_items: 100_000,
            capacity_factor: 0.08,
            max_weight: 100_000_000,
            max_value: 100_000_000,
            time_limit: Duration::from_secs(120),
            is_exact_feasible: false,
        },
    ]
}

pub fn generate_problem(level: &Level, seed: u64) -> KnapsackProblem {
    let mut rng = StdRng::seed_from_u64(seed + level.number as u64 * 31337);

    let items: Vec<Item> = match level.number {
        3 | 7 | 12 | 15 => {
            let mut items = Vec::new();
            let gem_count = level.num_items / 10;
            for _ in 0..gem_count {
                items.push(Item {
                    weight: rng.gen_range(level.max_weight * 7 / 10..=level.max_weight),
                    value: rng.gen_range(level.max_value * 8 / 10..=level.max_value),
                });
            }
            for _ in gem_count..level.num_items {
                items.push(Item {
                    weight: rng.gen_range(1..=level.max_weight / 10),
                    value: rng.gen_range(level.max_value / 3..=level.max_value / 2),
                });
            }
            items
        }
        4 => {
            let mut items = Vec::new();
            items.push(Item {
                weight: level.max_weight,
                value: level.max_value * 3,
            });
            for _ in 1..level.num_items {
                items.push(Item {
                    weight: rng.gen_range(1..=level.max_weight / 5),
                    value: rng.gen_range(1..=level.max_value / 4),
                });
            }
            items
        }
        5 | 10 => {
            let mut items = Vec::new();
            let combo_count = level.num_items / 5;
            for _ in 0..combo_count {
                let multiplier = rng.gen_range(3..=8);
                items.push(Item {
                    weight: multiplier * (level.max_weight / 10),
                    value: multiplier as u64 * (level.max_value / 5)
                        + rng.gen_range(0..=level.max_value / 10),
                });
            }
            for _ in combo_count..level.num_items {
                items.push(Item {
                    weight: rng.gen_range(1..=level.max_weight / 20),
                    value: rng.gen_range(level.max_value / 4..=level.max_value / 3),
                });
            }
            items
        }
        8 => (0..level.num_items)
            .map(|i| {
                let step = (i % 10) as u64 + 1;
                Item {
                    weight: step * (level.max_weight / 10),
                    value: step * step * (level.max_value / 100)
                        + rng.gen_range(0..=level.max_value / 50),
                }
            })
            .collect(),
        9 => {
            let mut items = Vec::new();
            let real_count = level.num_items / 10;
            for _ in 0..real_count {
                items.push(Item {
                    weight: rng.gen_range(level.max_weight / 5..=level.max_weight),
                    value: rng.gen_range(level.max_value / 2..=level.max_value),
                });
            }
            for _ in real_count..level.num_items {
                items.push(Item {
                    weight: rng.gen_range(1..=level.max_weight / 50),
                    value: rng.gen_range(1..=level.max_value / 20),
                });
            }
            items
        }
        13 | 17 => (0..level.num_items)
            .map(|_| {
                let w = rng.gen_range(1..=level.max_weight);
                let v = ((w as f64).sqrt() * (level.max_value as f64 / 100.0)) as u64
                    + rng.gen_range(0..=level.max_value / 100);
                Item {
                    weight: w,
                    value: v,
                }
            })
            .collect(),
        14 => {
            let mut items = Vec::new();
            for _ in 0..level.num_items {
                let base = rng.gen_range(1..=20) as u64;
                items.push(Item {
                    weight: base * (level.max_weight / 20),
                    value: rng.gen_range(1..=level.max_value),
                });
            }
            items
        }
        16 => (0..level.num_items)
            .map(|_| {
                let w = rng.gen_range(1..=level.max_weight);
                let noise = rng.gen_range(0..=(w / 10).max(1));
                let v = if rng.gen_bool(0.5) {
                    w.saturating_add(noise)
                } else {
                    w.saturating_sub(noise).max(1)
                };
                Item {
                    weight: w,
                    value: v,
                }
            })
            .collect(),
        18 => {
            let mut items = Vec::new();
            let chunk = level.num_items / 5;
            for _ in 0..chunk {
                items.push(Item {
                    weight: rng.gen_range(50_000..=100_000),
                    value: rng.gen_range(80_000..=100_000),
                });
            }
            for _ in 0..chunk {
                items.push(Item {
                    weight: rng.gen_range(100..=500),
                    value: rng.gen_range(500..=2_000),
                });
            }
            for _ in 0..chunk {
                let w = rng.gen_range(1_000..=50_000);
                items.push(Item {
                    weight: w,
                    value: w + rng.gen_range(0..=1_000),
                });
            }
            for _ in 0..chunk {
                let base = rng.gen_range(1..=100) as u64;
                items.push(Item {
                    weight: base * 500,
                    value: rng.gen_range(10_000..=50_000),
                });
            }
            for _ in 0..(level.num_items - 4 * chunk) {
                let w = rng.gen_range(1..=level.max_weight);
                let v = ((w as f64).sqrt() * 300.0) as u64;
                items.push(Item {
                    weight: w,
                    value: v,
                });
            }
            items
        }
        _ => (0..level.num_items)
            .map(|_| Item {
                weight: rng.gen_range(1..=level.max_weight),
                value: rng.gen_range(1..=level.max_value),
            })
            .collect(),
    };

    let total_weight: u64 = items.iter().map(|i| i.weight).sum();
    let capacity = (total_weight as f64 * level.capacity_factor) as u64;

    KnapsackProblem { items, capacity }
}

fn greedy_baseline(problem: &KnapsackProblem) -> KnapsackSolution {
    let mut indices: Vec<usize> = (0..problem.items.len()).collect();
    indices.sort_by(|&a, &b| {
        let density_a = problem.items[a].value as f64 / problem.items[a].weight as f64;
        let density_b = problem.items[b].value as f64 / problem.items[b].weight as f64;
        density_b.partial_cmp(&density_a).unwrap()
    });

    let mut remaining = problem.capacity;
    let mut selected = Vec::new();

    for i in indices {
        if problem.items[i].weight <= remaining {
            selected.push(i);
            remaining -= problem.items[i].weight;
        }
    }

    KnapsackSolution { selected }
}

fn solve_recursive(problem: &KnapsackProblem) -> KnapsackSolution {
    fn recurse(items: &[Item], idx: usize, remaining: u64) -> (u64, Vec<usize>) {
        if idx == items.len() || remaining == 0 {
            return (0, vec![]);
        }
        let (skip_val, skip_sel) = recurse(items, idx + 1, remaining);
        if items[idx].weight <= remaining {
            let (take_val, mut take_sel) = recurse(items, idx + 1, remaining - items[idx].weight);
            let take_val = take_val + items[idx].value;
            if take_val > skip_val {
                take_sel.push(idx);
                return (take_val, take_sel);
            }
        }
        (skip_val, skip_sel)
    }

    let (_, selected) = recurse(&problem.items, 0, problem.capacity);
    KnapsackSolution { selected }
}

fn solve_recursive_memo(problem: &KnapsackProblem) -> KnapsackSolution {
    let mut cache: HashMap<(usize, u64), (u64, Vec<usize>)> = HashMap::new();

    fn recurse(
        items: &[Item],
        idx: usize,
        remaining: u64,
        cache: &mut HashMap<(usize, u64), (u64, Vec<usize>)>,
    ) -> (u64, Vec<usize>) {
        if idx == items.len() || remaining == 0 {
            return (0, vec![]);
        }
        if let Some(cached) = cache.get(&(idx, remaining)) {
            return cached.clone();
        }
        let (skip_val, skip_sel) = recurse(items, idx + 1, remaining, cache);
        let best = if items[idx].weight <= remaining {
            let (take_val, mut take_sel) =
                recurse(items, idx + 1, remaining - items[idx].weight, cache);
            let take_val = take_val + items[idx].value;
            if take_val > skip_val {
                take_sel.push(idx);
                (take_val, take_sel)
            } else {
                (skip_val, skip_sel)
            }
        } else {
            (skip_val, skip_sel)
        };
        cache.insert((idx, remaining), best.clone());
        best
    }

    let (_, selected) = recurse(&problem.items, 0, problem.capacity, &mut cache);
    KnapsackSolution { selected }
}

fn solve_tabular_dp(problem: &KnapsackProblem) -> KnapsackSolution {
    let n = problem.items.len();
    let cap = problem.capacity as usize;

    let estimated_bytes = (n as u128) * (cap as u128) + (cap as u128) * 8;
    if estimated_bytes > 2_000_000_000 {
        return KnapsackSolution { selected: vec![] };
    }

    let mut dp = vec![0u64; cap + 1];
    let mut keep = vec![vec![false; cap + 1]; n];

    for i in 0..n {
        let w = problem.items[i].weight as usize;
        let v = problem.items[i].value;
        for c in (w..=cap).rev() {
            if dp[c - w] + v > dp[c] {
                dp[c] = dp[c - w] + v;
                keep[i][c] = true;
            }
        }
    }

    let mut selected = Vec::new();
    let mut remaining = cap;

    for i in (0..n).rev() {
        if keep[i][remaining] {
            selected.push(i);
            remaining -= problem.items[i].weight as usize;
        }
    }

    KnapsackSolution { selected }
}

fn solve_evolution(problem: &KnapsackProblem) -> KnapsackSolution {
    let n = problem.items.len();

    let config = if n <= 100 {
        EvolutionConfig {
            population_size: 50,
            generations: 150,
            elite_count: 2,
            selection_rate: 0.4,
            mutation_rate: 0.05,
            seed: 42,
        }
    } else if n <= 500 {
        EvolutionConfig {
            population_size: 40,
            generations: 100,
            elite_count: 2,
            selection_rate: 0.4,
            mutation_rate: 0.03,
            seed: 42,
        }
    } else if n <= 2_000 {
        EvolutionConfig {
            population_size: 30,
            generations: 60,
            elite_count: 2,
            selection_rate: 0.4,
            mutation_rate: 0.02,
            seed: 42,
        }
    } else if n <= 10_000 {
        EvolutionConfig {
            population_size: 20,
            generations: 40,
            elite_count: 2,
            selection_rate: 0.3,
            mutation_rate: 0.01,
            seed: 42,
        }
    } else if n <= 50_000 {
        EvolutionConfig {
            population_size: 15,
            generations: 20,
            elite_count: 1,
            selection_rate: 0.3,
            mutation_rate: 0.005,
            seed: 42,
        }
    } else {
        EvolutionConfig {
            population_size: 10,
            generations: 10,
            elite_count: 1,
            selection_rate: 0.3,
            mutation_rate: 0.002,
            seed: 42,
        }
    };

    let selected = evolve(problem, &config);
    KnapsackSolution { selected }
}

const SOLVER_TIMEOUT: Duration = Duration::from_secs(180); // 3 minutes

fn run_with_timeout(
    solver: fn(&KnapsackProblem) -> KnapsackSolution,
    problem: &KnapsackProblem,
) -> Option<(KnapsackSolution, Duration)> {
    let problem_clone = problem.clone();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let start = Instant::now();
        let solution = solver(&problem_clone);
        let elapsed = start.elapsed();
        let _ = tx.send((solution, elapsed));
    });

    match rx.recv_timeout(SOLVER_TIMEOUT) {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

#[derive(Debug)]
struct BenchResult {
    level: u8,
    level_name: &'static str,
    num_items: usize,
    your_value: u64,
    greedy_value: u64,
    diff: i64,
    elapsed: Duration,
    time_limit: Duration,
    valid: bool,
    within_time: bool,
    status: &'static str,
}

fn run_solver_on_level(
    _solver_name: &str,
    solver: fn(&KnapsackProblem) -> KnapsackSolution,
    level: &Level,
    problem: &KnapsackProblem,
    greedy_value: u64,
) -> BenchResult {
    match run_with_timeout(solver, problem) {
        Some((solution, elapsed)) => {
            let valid = solution.is_valid(problem);
            let your_value = if valid {
                solution.total_value(problem)
            } else {
                0
            };

            let diff = your_value as i64 - greedy_value as i64;
            let within_time = elapsed <= level.time_limit;

            let status = match (valid, within_time, your_value > 0) {
                (false, _, _) => "INVALID",
                (_, _, false) => "EMPTY",
                (true, false, _) => "SLOW",
                (true, true, true) if your_value >= greedy_value => "PASS",
                (true, true, true) => "WEAK",
                _ => "???",
            };

            BenchResult {
                level: level.number,
                level_name: level.name,
                num_items: level.num_items,
                your_value,
                greedy_value,
                diff,
                elapsed,
                time_limit: level.time_limit,
                valid,
                within_time,
                status,
            }
        }
        None => BenchResult {
            level: level.number,
            level_name: level.name,
            num_items: level.num_items,
            your_value: 0,
            greedy_value,
            diff: 0,
            elapsed: SOLVER_TIMEOUT,
            time_limit: level.time_limit,
            valid: false,
            within_time: false,
            status: "TIMEOUT",
        },
    }
}

fn print_separator() {
    println!("{}", "═".repeat(108));
}

fn print_solver_header(name: &str) {
    println!();
    println!();
    println!("  ┌─────────────────────────────────────────────────────┐");
    println!("  │  SOLVER: {:<43}│", name);
    println!("  └─────────────────────────────────────────────────────┘");
    println!();
    print_separator();
    println!(
        "  {:<3} {:<22} {:>8} {:>16} {:>16} {:>9} {:>10} {:>7} {:>7}",
        "Lv", "Name", "Items", "Your Value", "Greedy Value", "Diff", "Time", "Limit", "Status"
    );
    print_separator();
}

fn print_result(r: &BenchResult) {
    let time_str = if r.status == "TIMEOUT" {
        "TIMEOUT".to_string()
    } else if r.status == "SKIPPED" {
        "—".to_string()
    } else if r.elapsed.as_secs() >= 1 {
        format!("{:.2}s", r.elapsed.as_secs_f64())
    } else {
        format!("{:.1}ms", r.elapsed.as_secs_f64() * 1000.0)
    };

    let limit_str = format!("{}s", r.time_limit.as_secs());

    let diff_str = if r.your_value == 0 && r.status != "PASS" {
        format!("{:>9}", 0)
    } else if r.diff >= 0 {
        format!("{:>+9}", r.diff)
    } else {
        format!("{:>9}", r.diff)
    };

    println!(
        "  {:<3} {:<22} {:>8} {:>16} {:>16} {} {:>10} {:>7} {:>7}",
        r.level,
        r.level_name,
        r.num_items,
        r.your_value,
        r.greedy_value,
        diff_str,
        time_str,
        limit_str,
        r.status,
    );
}

fn print_summary(results: &[BenchResult], total_levels: usize) {
    let mut levels_beaten = 0;
    let mut timeouts = 0;
    let mut total_diff: i64 = 0;

    for r in results {
        if r.status == "TIMEOUT" {
            timeouts += 1;
        } else if r.valid && r.your_value > 0 {
            total_diff += r.diff;
            if r.your_value >= r.greedy_value && r.within_time {
                levels_beaten += 1;
            }
        }
    }

    print_separator();
    println!();
    println!(
        "  Levels beaten (>= greedy): {}/{}",
        levels_beaten, total_levels
    );
    println!("  Timeouts:                  {}", timeouts);
    println!("  Total value vs greedy:     {:+}", total_diff);
    println!();
}

fn main() {
    let seed: u64 = 42;
    let levels = get_levels();

    println!();
    println!(
        "  KNAPSACK BENCHMARK — 0/1 Knapsack, {} Levels",
        levels.len()
    );
    println!("  Seed: {seed}");
    println!("  Solver timeout: {}s", SOLVER_TIMEOUT.as_secs());
    println!();

    for lvl in &levels {
        let feasibility = if lvl.is_exact_feasible {
            "exact"
        } else {
            "approx"
        };
        println!(
            "  Lv {:>2}: {:<22} — {} [{}]",
            lvl.number, lvl.name, lvl.description, feasibility
        );
    }

    let problems: Vec<KnapsackProblem> = levels.iter().map(|l| generate_problem(l, seed)).collect();
    let greedy_values: Vec<u64> = problems
        .iter()
        .map(|p| greedy_baseline(p).total_value(p))
        .collect();

    let solvers: Vec<(&str, fn(&KnapsackProblem) -> KnapsackSolution)> = vec![
        ("Recursive (brute force)", solve_recursive),
        ("Recursive + Memoization", solve_recursive_memo),
        ("Tabular DP", solve_tabular_dp),
        ("Evolutionary", solve_evolution),
    ];

    for (solver_name, solver_fn) in &solvers {
        print_solver_header(solver_name);

        let mut results: Vec<BenchResult> = Vec::new();

        for (i, level) in levels.iter().enumerate() {
            let result = run_solver_on_level(
                solver_name,
                *solver_fn,
                level,
                &problems[i],
                greedy_values[i],
            );

            print_result(&result);

            let timed_out = result.status == "TIMEOUT";
            results.push(result);

            if timed_out {
                for j in (i + 1)..levels.len() {
                    let skip = BenchResult {
                        level: levels[j].number,
                        level_name: levels[j].name,
                        num_items: levels[j].num_items,
                        your_value: 0,
                        greedy_value: greedy_values[j],
                        diff: 0,
                        elapsed: Duration::ZERO,
                        time_limit: levels[j].time_limit,
                        valid: false,
                        within_time: false,
                        status: "SKIPPED",
                    };
                    print_result(&skip);
                    results.push(skip);
                }
                break;
            }
        }

        print_summary(&results, levels.len());
    }
}
