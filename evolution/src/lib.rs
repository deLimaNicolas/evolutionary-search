use capacity::KnapsackProblem;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

pub struct EvolutionConfig {
    pub population_size: usize,
    pub generations: usize,
    pub elite_count: usize,
    pub selection_rate: f64,
    pub mutation_rate: f64,
    pub seed: usize,
}

pub fn initialize(problem: &KnapsackProblem, config: &EvolutionConfig) -> Vec<Vec<bool>> {
    let mut candidates: Vec<Vec<bool>> = Vec::new();
    let mut rng = StdRng::seed_from_u64(config.seed as u64);

    while candidates.len() < config.population_size {
        let mut new_cand: Vec<bool> = vec![false; problem.items.len()];
        let mut cand_weight: u64 = 0;

        let mut order: Vec<usize> = (0..problem.items.len()).collect();
        order.shuffle(&mut rng);

        for idx in order {
            if rng.gen_bool(0.5) {
                let new_weight = cand_weight + problem.items[idx].weight;
                if new_weight <= problem.capacity {
                    new_cand[idx] = true;
                    cand_weight = new_weight;
                }
            }
        }

        candidates.push(new_cand);
    }

    candidates
}

pub fn evaluate(candidates: Vec<Vec<bool>>, problem: &KnapsackProblem) -> Vec<(Vec<bool>, u64)> {
    let mut scored: Vec<(Vec<bool>, u64)> = candidates
        .into_iter()
        .map(|cand| {
            let value: u64 = cand
                .iter()
                .enumerate()
                .filter(|(_, &selected)| selected)
                .map(|(i, _)| problem.items[i].value)
                .sum();
            (cand, value)
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));

    scored
}

pub fn select(candidates: Vec<(Vec<bool>, u64)>, config: &EvolutionConfig) -> Vec<Vec<bool>> {
    // TODO: implement — take top selection_rate % of candidates
    let count = (candidates.len() as f64 * config.selection_rate) as usize;
    candidates
        .into_iter()
        .take(count)
        .map(|(cand, _)| cand)
        .collect()
}

pub fn crossover(
    parents: Vec<Vec<bool>>,
    problem: &KnapsackProblem,
    config: &EvolutionConfig,
) -> Vec<Vec<bool>> {
    // TODO: implement — combine parents to produce children
    parents
}

pub fn mutate(children: &mut Vec<Vec<bool>>, problem: &KnapsackProblem, config: &EvolutionConfig) {
    // TODO: implement — randomly flip genes
}

pub fn evolve(problem: &KnapsackProblem, config: &EvolutionConfig) -> Vec<usize> {
    // TODO: implement — run the full loop
    vec![]
}
