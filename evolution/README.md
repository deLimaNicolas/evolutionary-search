# evolution

A generic evolutionary algorithm library for solving the 0/1 Knapsack problem, built as a Rust lib crate.


## How It Works

An evolutionary algorithm mimics natural selection. You start with a bunch of random solutions, score them, breed the best ones, introduce some chaos, and repeat until you converge on something good. It won't find the perfect answer (that's NP-hard), but it'll find a damn good one fast.

The loop runs for N generations. Each generation goes through 6 phases:


## The Pipeline

### 1. Initialize

Generate a random population of valid candidate solutions.

```
fn initialize(problem: &KnapsackProblem, population_size: usize, rng: &mut StdRng) -> Vec<Candidate>
```

Each candidate is a set of item indices whose total weight does not exceed capacity. Generation strategy: iterate through items in random order, greedily add items that fit. This guarantees every initial candidate is valid — no wasted generations fixing broken solutions.

A `Candidate` is:
```rust
struct Candidate {
    selected: Vec<bool>,  // selected[i] = true means item i is in the bag
    fitness: u64,         // total value (computed during evaluate)
}
```

Using `Vec<bool>` instead of `Vec<usize>` makes crossover and mutation simpler — just flip bits.


### 2. Evaluate

Score each candidate by total value and sort descending (best first).

```
fn evaluate(candidates: &mut Vec<Candidate>, problem: &KnapsackProblem)
```

Fitness = total value of selected items. If a candidate is overweight (shouldn't happen if initialize and mutate are correct, but safety first), fitness = 0. Sort the population so index 0 is always the best.


### 3. Select

Pick the top percentage of candidates to serve as parents.

```
fn select(candidates: &[Candidate], selection_rate: f64) -> Vec<&Candidate>
```

`selection_rate` is a float between 0.0 and 1.0. A rate of 0.4 means the top 40% become parents. The rest are discarded. This is "truncation selection" — simple, aggressive, effective.

Returns references to the selected candidates (no cloning yet).


### 4. Crossover

Combine pairs of parents to produce children that fill the next generation.

```
fn crossover(
    parents: &[&Candidate],
    num_children: usize,  // = population_size - elite_count
    rng: &mut StdRng,
) -> Vec<Candidate>
```

Strategy: **uniform crossover**. For each child, pick two random parents. For each item index, randomly inherit from parent A or parent B (50/50 coin flip per gene).

After crossover, **repair** the child if it's overweight: remove random selected items until weight <= capacity.

Produce exactly `num_children` children to fill the population back up (minus the elite slots).


### 5. Mutate

Randomly tweak some children to introduce diversity.

```
fn mutate(
    children: &mut Vec<Candidate>,
    problem: &KnapsackProblem,
    mutation_rate: f64,
    rng: &mut StdRng,
)
```

For each child, for each gene (item), there's a `mutation_rate` probability of flipping it (in → out, out → in). Typical rate: 0.01-0.05 (1-5% per gene).

After mutation, **repair** if overweight: remove random selected items until valid.


### 6. Replace (Elitism + Merge)

Combine the elite survivors with the new children to form the next generation.

```
fn replace(
    candidates: &[Candidate],  // current gen, sorted by fitness
    children: Vec<Candidate>,
    elite_count: usize,         // fixed small number: 1-3
) -> Vec<Candidate>
```

Take the top `elite_count` candidates from the current generation (unchanged, no mutation). Append all children. This is the new population. Elites guarantee the best solution is never lost.


## The Main Loop

```
fn evolve(
    problem: &KnapsackProblem,
    config: &EvolutionConfig,
) -> KnapsackSolution
```

Where `EvolutionConfig` is:
```rust
struct EvolutionConfig {
    population_size: usize,   // e.g. 100
    generations: usize,        // e.g. 500
    elite_count: usize,        // e.g. 2 (fixed, small)
    selection_rate: f64,       // e.g. 0.4 (top 40% become parents)
    mutation_rate: f64,        // e.g. 0.02 (2% per gene)
    seed: u64,                 // deterministic RNG seed
}
```

Pseudocode:
```
candidates = initialize(problem, population_size)

for gen in 0..generations:
    evaluate(&mut candidates, problem)
    parents  = select(&candidates, selection_rate)
    children = crossover(&parents, population_size - elite_count)
    mutate(&mut children, problem, mutation_rate)
    candidates = replace(&candidates, children, elite_count)

evaluate(&mut candidates, problem)
return candidates[0] as KnapsackSolution
```


## Repair Function

Several steps (initialize, crossover, mutate) can produce overweight candidates. A shared repair function handles this:

```
fn repair(candidate: &mut Candidate, problem: &KnapsackProblem, rng: &mut StdRng)
```

If total weight > capacity, randomly remove selected items one at a time until weight <= capacity. Removing by worst density (highest weight / lowest value) instead of randomly is a smarter variant you can try later.


## Suggested Starting Parameters

| Parameter | Small (n < 1K) | Medium (n < 100K) | Large (n > 100K) |
|---|---|---|---|
| population_size | 50 | 100 | 200 |
| generations | 200 | 500 | 1000 |
| elite_count | 1 | 2 | 2 |
| selection_rate | 0.4 | 0.4 | 0.3 |
| mutation_rate | 0.05 | 0.02 | 0.01 |

These are starting points. Tune them.


## Crate Structure

```
evo-knapsack/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs       # all public types and functions
```

The crate exposes:
- `Candidate` — a solution representation
- `EvolutionConfig` — algorithm parameters
- `evolve()` — the main entry point
- Individual phase functions if you want to compose your own loop


## Usage

```rust
use evo_knapsack::{evolve, EvolutionConfig};

let config = EvolutionConfig {
    population_size: 100,
    generations: 500,
    elite_count: 2,
    selection_rate: 0.4,
    mutation_rate: 0.02,
    seed: 42,
};

let solution = evolve(&problem, &config);
```

Then plug `solution` into the knapsack benchmark harness as your `solve()` implementation.
