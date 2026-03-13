use capacity::KnapsackProblem;

pub struct EvolutionConfig {
    population_size: usize,
    generations: usize,
    elite_count: usize,
    selection_rate: f64,
    mutation_rate: f64,
    seed: usize,
}

fn initialize(problem: &KnapsackProblem, config: &EvolutionConfig) -> Vec<Vec<usize>> {}

fn evaluate(candidates: Vec<Vec<usize>>) -> Vec<(Vec<usize>, u64)> {}

fn select(candidates: Vec<Vec<usize>>, config: &EvolutionConfig) -> Vec<Vec<usize>> {}

// Crossover will also elitize
fn crossover(candidates: Vec<Vec<usize>>, config: &EvolutionConfig) -> Vec<Vec<usize>> {}

fn mutate(candidates: &Vec<Vec<usize>>, config: &EvolutionConfig) -> Vec<Vec<usize>> {}

fn evolve(config: &EvolutionConfig) -> Vec<Vec<usize>> {}
