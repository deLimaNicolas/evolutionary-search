use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::time::{Duration, Instant};

// ============================================================================
//  KNAPSACK BENCHMARK HARNESS
//
//  Levels 1-10:  Original gauntlet (exact DP feasible).
//  Levels 11-20: The real deal. Exact solutions are impossible.
//                You MUST approximate.
//
//  Each level is seeded deterministically so results are reproducible.
// ============================================================================

/// A single knapsack item.
#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u64,
    pub value: u64,
}

/// A problem instance.
#[derive(Debug, Clone)]
pub struct KnapsackProblem {
    pub items: Vec<Item>,
    pub capacity: u64,
}

/// Result from a solver.
#[derive(Debug, Clone)]
pub struct KnapsackSolution {
    /// Indices of items selected (0-based).
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

        // Check for out-of-bounds or duplicate indices
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

// ============================================================================
//  LEVEL DEFINITIONS
// ============================================================================

#[derive(Debug)]
pub struct Level {
    pub number: u8,
    pub name: &'static str,
    pub description: &'static str,
    pub num_items: usize,
    pub capacity_factor: f64, // capacity = factor * sum_of_all_weights
    pub max_weight: u64,
    pub max_value: u64,
    pub time_limit: Duration,
    pub is_exact_feasible: bool, // can a mortal machine solve this exactly?
}

pub fn get_levels() -> Vec<Level> {
    vec![
        // ====================================================================
        //  ORIGINAL LEVELS 1-10
        // ====================================================================
        Level {
            number: 1,
            name: "Warm-Up",
            description: "10 items. Your cat could solve this.",
            num_items: 10,
            capacity_factor: 0.5,
            max_weight: 50,
            max_value: 100,
            time_limit: Duration::from_secs(1),
            is_exact_feasible: true,
        },
        Level {
            number: 2,
            name: "Easy",
            description: "20 items. Still trivial.",
            num_items: 20,
            capacity_factor: 0.45,
            max_weight: 100,
            max_value: 200,
            time_limit: Duration::from_secs(2),
            is_exact_feasible: true,
        },
        Level {
            number: 3,
            name: "Getting There",
            description: "50 items. Brute force starts sweating.",
            num_items: 50,
            capacity_factor: 0.4,
            max_weight: 200,
            max_value: 500,
            time_limit: Duration::from_secs(5),
            is_exact_feasible: true,
        },
        Level {
            number: 4,
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
            number: 5,
            name: "Serious",
            description: "500 items with moderate weights. DP tables getting big.",
            num_items: 500,
            capacity_factor: 0.3,
            max_weight: 1_000,
            max_value: 2_000,
            time_limit: Duration::from_secs(15),
            is_exact_feasible: true,
        },
        Level {
            number: 6,
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
            number: 7,
            name: "Pain",
            description: "5,000 items with large weights. DP table is enormous.",
            num_items: 5_000,
            capacity_factor: 0.2,
            max_weight: 100_000,
            max_value: 100_000,
            time_limit: Duration::from_secs(60),
            is_exact_feasible: true,
        },
        Level {
            number: 8,
            name: "Suffering",
            description: "10,000 items, huge weights. Exact DP needs heroics.",
            num_items: 10_000,
            capacity_factor: 0.15,
            max_weight: 1_000_000,
            max_value: 1_000_000,
            time_limit: Duration::from_secs(120),
            is_exact_feasible: false,
        },
        Level {
            number: 9,
            name: "Existential Crisis",
            description: "50,000 items. Welcome to approximation territory.",
            num_items: 50_000,
            capacity_factor: 0.1,
            max_weight: 10_000_000,
            max_value: 10_000_000,
            time_limit: Duration::from_secs(120),
            is_exact_feasible: false,
        },
        Level {
            number: 10,
            name: "Heat Death",
            description: "200,000 items. The universe ends before brute force finishes.",
            num_items: 200_000,
            capacity_factor: 0.08,
            max_weight: 100_000_000,
            max_value: 100_000_000,
            time_limit: Duration::from_secs(180),
            is_exact_feasible: false,
        },
        Level {
            number: 11,
            name: "No More Tables",
            description: "5K items, massive weights. DP is physically impossible.",
            num_items: 5_000,
            capacity_factor: 0.3,
            max_weight: 1_000_000_000,
            max_value: 1_000_000,
            time_limit: Duration::from_secs(10),
            is_exact_feasible: false,
        },
        Level {
            number: 12,
            name: "Correlated Chaos",
            description: "10K items where value ~ weight. Density sorting is useless.",
            num_items: 10_000,
            capacity_factor: 0.2,
            max_weight: 500_000_000,
            max_value: 500_000_000,
            time_limit: Duration::from_secs(15),
            is_exact_feasible: false,
        },
        Level {
            number: 13,
            name: "Needle in Haystack",
            description: "50K items, most are junk. A few gems hidden in the noise.",
            num_items: 50_000,
            capacity_factor: 0.05,
            max_weight: 100_000_000,
            max_value: 100_000,
            time_limit: Duration::from_secs(15),
            is_exact_feasible: false,
        },
        Level {
            number: 14,
            name: "Tight Squeeze",
            description: "20K items, capacity is only 1% of total weight.",
            num_items: 20_000,
            capacity_factor: 0.01,
            max_weight: 1_000_000_000,
            max_value: 10_000_000,
            time_limit: Duration::from_secs(10),
            is_exact_feasible: false,
        },
        Level {
            number: 15,
            name: "Wide Open",
            description: "100K items, capacity is 60%. Hard to know what to EXCLUDE.",
            num_items: 100_000,
            capacity_factor: 0.6,
            max_weight: 500_000_000,
            max_value: 500_000_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: false,
        },
        Level {
            number: 16,
            name: "Speed Demon",
            description: "200K items, 5 second limit. Be fast or be nothing.",
            num_items: 200_000,
            capacity_factor: 0.15,
            max_weight: 1_000_000_000,
            max_value: 1_000_000_000,
            time_limit: Duration::from_secs(5),
            is_exact_feasible: false,
        },
        Level {
            number: 17,
            name: "Giant's Knapsack",
            description: "500K items. Half a million decisions.",
            num_items: 500_000,
            capacity_factor: 0.1,
            max_weight: 1_000_000_000,
            max_value: 1_000_000_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: false,
        },
        Level {
            number: 18,
            name: "Precision Matters",
            description: "100K items, values nearly identical. Tiny margins decide all.",
            num_items: 100_000,
            capacity_factor: 0.25,
            max_weight: 1_000_000_000,
            max_value: 1_000,
            time_limit: Duration::from_secs(20),
            is_exact_feasible: false,
        },
        Level {
            number: 19,
            name: "The Abyss",
            description: "1M items. One million. Good luck.",
            num_items: 1_000_000,
            capacity_factor: 0.08,
            max_weight: 1_000_000_000,
            max_value: 1_000_000_000,
            time_limit: Duration::from_secs(60),
            is_exact_feasible: false,
        },
        Level {
            number: 20,
            name: "God Mode",
            description: "2M items, 30 seconds. Beat greedy here and you're hired.",
            num_items: 2_000_000,
            capacity_factor: 0.05,
            max_weight: 1_000_000_000,
            max_value: 1_000_000_000,
            time_limit: Duration::from_secs(30),
            is_exact_feasible: false,
        },
    ]
}

// ============================================================================
//  PROBLEM GENERATION (deterministic via seed)
// ============================================================================

pub fn generate_problem(level: &Level, seed: u64) -> KnapsackProblem {
    let mut rng = StdRng::seed_from_u64(seed + level.number as u64 * 31337);

    let items: Vec<Item> = if level.number == 12 {
        // Special: correlated instances (value ≈ weight + noise)
        // This breaks greedy-by-density since all densities are ~1.0
        (0..level.num_items)
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
            .collect()
    } else if level.number == 13 {
        // Special: mostly junk items with a few hidden gems
        (0..level.num_items)
            .map(|_| {
                let is_gem = rng.gen_ratio(1, 500); // 0.2% chance of being a gem
                if is_gem {
                    Item {
                        weight: rng.gen_range(1..=level.max_weight / 100),
                        value: rng.gen_range(level.max_value / 2..=level.max_value * 100),
                    }
                } else {
                    Item {
                        weight: rng.gen_range(1..=level.max_weight),
                        value: rng.gen_range(1..=level.max_value / 10),
                    }
                }
            })
            .collect()
    } else if level.number == 18 {
        // Special: all values in a very tight range — hard to differentiate
        (0..level.num_items)
            .map(|_| Item {
                weight: rng.gen_range(1..=level.max_weight),
                value: rng.gen_range(500..=level.max_value), // values between 500-1000
            })
            .collect()
    } else {
        // Standard uniform generation
        (0..level.num_items)
            .map(|_| Item {
                weight: rng.gen_range(1..=level.max_weight),
                value: rng.gen_range(1..=level.max_value),
            })
            .collect()
    };

    let total_weight: u64 = items.iter().map(|i| i.weight).sum();
    let capacity = (total_weight as f64 * level.capacity_factor) as u64;

    KnapsackProblem { items, capacity }
}

// ============================================================================
//  REFERENCE SOLVER (greedy by value density — used as baseline)
// ============================================================================

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

// ============================================================================
//  ╔══════════════════════════════════════════════════════════════════════╗
//  ║  YOUR SOLVER GOES HERE                                             ║
//  ║                                                                    ║
//  ║  Replace the body of this function with your implementation.       ║
//  ║  Return a KnapsackSolution with the indices of selected items.     ║
//  ║                                                                    ║
//  ║  The problem gives you:                                            ║
//  ║    - problem.items  : Vec<Item> with .weight and .value            ║
//  ║    - problem.capacity : u64                                        ║
//  ║                                                                    ║
//  ║  Currently returns an empty solution (score: 0). Beat the greedy!  ║
//  ╚══════════════════════════════════════════════════════════════════════╝
// ============================================================================

// Simple Recursion No Memo
//pub fn solve(problem: &KnapsackProblem) -> KnapsackSolution {
//    fn recurse(items: &[Item], idx: usize, remaining: u64) -> (u64, Vec<usize>) {
//        if idx == items.len() || remaining == 0 {
//            return (0, vec![]);
//        }
//
//        // Skip this item
//        let (skip_val, skip_sel) = recurse(items, idx + 1, remaining);
//
//        // Take this item (if it fits)
//        if items[idx].weight <= remaining {
//            let (take_val, mut take_sel) = recurse(items, idx + 1, remaining - items[idx].weight);
//            let take_val = take_val + items[idx].value;
//
//            if take_val > skip_val {
//                take_sel.push(idx);
//                return (take_val, take_sel);
//            }
//        }
//
//        (skip_val, skip_sel)
//    }
//
//    let (_, selected) = recurse(&problem.items, 0, problem.capacity);
//    KnapsackSolution { selected }
//}

// Simple recursion + Memo
use std::collections::HashMap;

pub fn solve(problem: &KnapsackProblem) -> KnapsackSolution {
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

// Tabular DP
//pub fn solve(problem: &KnapsackProblem) -> KnapsackSolution {
//    let n = problem.items.len();
//    let cap = problem.capacity as usize;
//
//    // DP with a "keep" table to track decisions
//    let mut dp = vec![0u64; cap + 1];
//    let mut keep = vec![vec![false; cap + 1]; n];
//
//    for i in 0..n {
//        let w = problem.items[i].weight as usize;
//        let v = problem.items[i].value;
//        for c in (w..=cap).rev() {
//            if dp[c - w] + v > dp[c] {
//                dp[c] = dp[c - w] + v;
//                keep[i][c] = true;
//            }
//        }
//    }
//
//    // Backtrack using the keep table
//    let mut selected = Vec::new();
//    let mut remaining = cap;
//
//    for i in (0..n).rev() {
//        if keep[i][remaining] {
//            selected.push(i);
//            remaining -= problem.items[i].weight as usize;
//        }
//    }
//
//    KnapsackSolution { selected }
//}

// ============================================================================
//  BENCHMARK RUNNER
// ============================================================================

#[derive(Debug)]
struct BenchResult {
    level: u8,
    level_name: &'static str,
    num_items: usize,
    capacity: u64,
    your_value: u64,
    greedy_value: u64,
    ratio_vs_greedy: f64,
    your_weight: u64,
    elapsed: Duration,
    time_limit: Duration,
    valid: bool,
    within_time: bool,
    is_exact_feasible: bool,
}

fn run_level(level: &Level, seed: u64) -> BenchResult {
    let problem = generate_problem(level, seed);

    // Run greedy baseline
    let greedy_sol = greedy_baseline(&problem);
    let greedy_value = greedy_sol.total_value(&problem);

    // Run your solver
    let start = Instant::now();
    let your_sol = solve(&problem);
    let elapsed = start.elapsed();

    let valid = your_sol.is_valid(&problem);
    let your_value = if valid {
        your_sol.total_value(&problem)
    } else {
        0
    };
    let your_weight = if valid {
        your_sol.total_weight(&problem)
    } else {
        0
    };

    let ratio = if greedy_value > 0 {
        your_value as f64 / greedy_value as f64
    } else {
        0.0
    };

    BenchResult {
        level: level.number,
        level_name: level.name,
        num_items: level.num_items,
        capacity: problem.capacity,
        your_value,
        greedy_value,
        ratio_vs_greedy: ratio,
        your_weight,
        elapsed,
        time_limit: level.time_limit,
        valid,
        within_time: elapsed <= level.time_limit,
        is_exact_feasible: level.is_exact_feasible,
    }
}

fn print_separator() {
    println!("{}", "═".repeat(108));
}

fn print_header() {
    print_separator();
    println!(
        "  {:<3} {:<22} {:>8} {:>16} {:>16} {:>8} {:>10} {:>7} {:>7}",
        "Lv", "Name", "Items", "Your Value", "Greedy Value", "Ratio", "Time", "Limit", "Status"
    );
    print_separator();
}

fn print_result(r: &BenchResult) {
    let status = match (r.valid, r.within_time, r.your_value > 0) {
        (false, _, _) => "INVALID",
        (_, _, false) => "EMPTY",
        (true, false, _) => "SLOW",
        (true, true, true) => {
            if r.ratio_vs_greedy >= 1.0 {
                "PASS"
            } else {
                "WEAK"
            }
        }
        _ => "???",
    };

    let time_str = if r.elapsed.as_secs() >= 1 {
        format!("{:.2}s", r.elapsed.as_secs_f64())
    } else {
        format!("{:.1}ms", r.elapsed.as_secs_f64() * 1000.0)
    };

    let limit_str = format!("{}s", r.time_limit.as_secs());

    println!(
        "  {:<3} {:<22} {:>8} {:>16} {:>16} {:>7.1}% {:>10} {:>7} {:>7}",
        r.level,
        r.level_name,
        r.num_items,
        r.your_value,
        r.greedy_value,
        r.ratio_vs_greedy * 100.0,
        time_str,
        limit_str,
        status,
    );
}

fn main() {
    let seed: u64 = 42; // Deterministic. Change for different instances.
    let levels = get_levels();

    println!();
    println!("  KNAPSACK BENCHMARK — 0/1 Knapsack, 20 Levels of Pain");
    println!("  Seed: {seed}");
    println!();

    // Print level descriptions
    for lvl in &levels {
        let feasibility = if lvl.is_exact_feasible {
            "exact"
        } else {
            "approx only"
        };
        println!(
            "  Lv {:>2}: {:<22} — {} [{}]",
            lvl.number, lvl.name, lvl.description, feasibility
        );
    }

    println!();
    print_header();

    let total_levels = levels.len() as f64;
    let mut total_score: f64 = 0.0;
    let mut levels_beaten = 0;

    for level in &levels {
        let result = run_level(level, seed);

        print_result(&result);

        if result.valid && result.within_time && result.your_value > 0 {
            total_score += result.ratio_vs_greedy;
            if result.ratio_vs_greedy >= 1.0 {
                levels_beaten += 1;
            }
        }
    }

    print_separator();
    println!();
    println!("  SUMMARY");
    println!("  ───────────────────────────────────");
    println!(
        "  Levels beaten (>= greedy): {}/{}",
        levels_beaten,
        levels.len()
    );
    println!(
        "  Cumulative score:          {:.2} / {:.2}",
        total_score, total_levels
    );
    println!(
        "  Average vs greedy:         {:.1}%",
        if total_score > 0.0 {
            total_score / total_levels * 100.0
        } else {
            0.0
        }
    );
    println!();

    if levels_beaten == 0 {
        println!("  ... You returned empty solutions. The placeholder is still in there.");
        println!("  Go implement `solve()` in src/main.rs. I believe in you. Barely.");
    } else if levels_beaten <= 5 {
        println!("  You solved the baby levels. Congrats, you can tie your shoes.");
    } else if levels_beaten <= 10 {
        println!("  Not bad. You've got a functioning brain. Use it more.");
    } else if levels_beaten <= 15 {
        println!("  Impressive. You're actually making the greedy look stupid.");
    } else if levels_beaten <= 18 {
        println!("  Okay, you're genuinely good at this. Don't let it go to your head.");
    } else if levels_beaten <= 19 {
        println!("  One level away from perfection. So close it hurts.");
    } else {
        println!("  Perfect score across 20 levels. You've either written something brilliant");
        println!("  or you've broken the simulation. Either way, I respect it.");
    }
    println!();
}
