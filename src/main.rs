#[cfg(test)]
mod tests;

use rand::{rngs::StdRng, Rng, SeedableRng};

use clap::Parser;

// See the Clap docs at https://docs.rs/clap/latest/clap/.  Some of
// this is cargo-culty, if'n youknowutimean.
#[derive(Debug, Parser)]
#[command(author, version, about = "Wriggle me up some midi", long_about = None)]
struct Args {

    /// Space-delimited midi note numbers.
    #[arg(value_delimiter = ' ', num_args = 1..)]
    src_notes: Vec<i8>,

    /// The number of notes in maximally-fit target sequences.
    #[arg()]
    target_notes: i8,

    /// The number of steps in maximally-fit target sequences.
    #[arg()]
    target_steps: i8,
}


// Courtesy of _Programming Rust_ by Blandy, Orendorff, and Tindall.
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = std::result::Result<T, GenericError>;

// This might be better as a struct.
type Note = (i8, i16);

type NoteVec = Vec<Note>;

trait Chromosome {
    fn breed(&self, other: &Self) -> [Self; 2]
    where
        Self: Sized;
    fn display(&self);
    fn fitness(&self, target_seq: &NoteVec, target_notes: &i8, target_steps: &i8) -> f32;
    fn mutate(&mut self) -> bool;
    fn randomize(&mut self, length: usize);
}

impl Chromosome for NoteVec {
    fn breed(&self, other: &Self) -> [Self; 2] {
        let mut seed_rng = StdRng::from_os_rng();
        let crossover_index: usize = ((seed_rng.random::<u32>()) % (self.len() as u32)) as usize;
        let mut child1 = NoteVec::new();
        let mut child2 = NoteVec::new();
        // The following can be compressed with `take()` and
        // `collect()` (see the StackExchange discussion at
        // https://stackoverflow.com/questions/40154150/how-do-i-concatenate-two-slices-in-rust),
        // but I think the following is clearer in this case.
        child1.extend_from_slice(&self[..crossover_index]);
        child1.extend_from_slice(&other[crossover_index..]);
        child2.extend_from_slice(&other[..crossover_index]);
        child2.extend_from_slice(&self[crossover_index..]);

        child1.mutate();
        child2.mutate();

        [child1, child2]
    }

    /// Display the contents of a `NoteVec`.  I would prefer
    /// implementing `Display` for `NoteVec`, but since `NoteVec` is
    /// an aliased type, the orphan rule won't allow it.
    fn display(&self) {
        for i in 0..(self.len() - 1) {
            print!("({}, {}), ", self[i].0, self[i].1);
        }
        println!("({}, {})", self.last().unwrap().0, self.last().unwrap().1);
    }

    /// The fitness of a given `NoteVec` is calculated by counting the
    /// number of notes it contains, summing the stepwise distances
    /// between each note and an associated note in the target
    /// sequence, converting both values to percentages of 2^7, then
    /// using an inverted logarithmic function to represent the decay
    /// in fitness as either the note number or the number of steps
    /// get farther away from the desired values.  Some clamping of
    /// values around the edges, to avoid infinities near zero and out
    /// of an abundance of caution on the other end, also seemed
    /// prudent.
    fn fitness(&self, target_seq: &NoteVec, notes_param: &i8, steps_param: &i8) -> f32 {
        let mut notes: isize = 0;
        let mut steps: isize = 0;

        // Get the total number of notes along with the sum of the
        // number of steps by which each note in this sequence is
        // offset from the note at the same index in the target
        // sequence.
        for i in 0..self.len() {
            notes = notes + 1;
            steps = steps + (target_seq[i].0 - self[i].0).abs() as isize;
        }

        let mut notes_deviation: f32 = (notes as f32 - *notes_param as f32).abs() / 128.0;
        let mut steps_deviation: f32 = (steps as f32 - *steps_param as f32).abs() / 128.0;

        // Clamp note and step deviations.
        if notes_deviation < 0.0079 {
            notes_deviation = 0.0079;
        } else if notes_deviation > 1.0 {
            notes_deviation = 1.0;
        }
        if steps_deviation < 0.0079 {
            steps_deviation = 0.0079;
        } else if steps_deviation > 1.0 {
            steps_deviation = 1.0;
        }

        // The `0.1429` in the following expressions is just an
        // approximation of 1/7th, which makes the curve described by
        // `x.log2()` in the 1 by 1 square immediately above the origin
        // look flatter toward the right and steeper toward the left,
        // both in precisely the way I think this particular reward
        // function should be.
        notes_deviation = -(notes_deviation.log2() * 0.1429);
        steps_deviation = -(steps_deviation.log2() * 0.1429);

        let result = (notes_deviation + steps_deviation) / 2.0;

        // Clamp the result.
        if result > 0.99 {
            return 1.0;
        } else {
            return result;
        }
    }

    /// Randomly change a Note in a NoteVec with 50% probability.  For
    /// now, this only affects notes proper---microtunings and
    /// counterpoint come later.  Returns `true` when mutation
    /// happens, `false` when it doesn't, which I think is an okay
    /// usage here (as opposed to returning a `Result`) since the
    /// caller is modified.  Nonetheless, this isn't as idiomatic as
    /// it could be.
    fn mutate(&mut self) -> bool {
        let mut seed_rng = StdRng::from_os_rng();
        let flip: u8 = (seed_rng.random::<u8>()) % 255;
        let mut mutated: bool = false;
        if flip > 127 {
            let mutation_index: usize = ((seed_rng.random::<u32>()) % (self.len() as u32)) as usize;
            let random_note: i8 = (seed_rng.random::<i8>() % 127).abs();
            self[mutation_index].0 = random_note;
            mutated = true;
        }
        mutated
    }

    /// Populate the `NoteVec` with some nice new notes.
    fn randomize(&mut self, length: usize) {
        self.clear();
        let mut seed_rng = StdRng::from_os_rng();
        for _ in 0..length {
            let random_note: Note = ((seed_rng.random::<i8>() % 127).abs(), 0);
            self.push(random_note);
        }
    }
}

struct Population {
    oldsters: [NoteVec; 1_000],
    younguns: [NoteVec; 1_000], // Only used while `evolve()`-ing.
    fitness_sum: f32,           // Normalized in `fitness()`.
    mean: f32,
    standard_dev: f32,
    target_notes: i8,
    target_steps: i8,
    target_seq: NoteVec,
}

impl Population {
    /// This would be less of a pain to initialize if I implemented
    /// the `Copy` trait on `NoteVec`, but I think leaving it this
    /// way is preferable since I only ever want to transfer
    /// ownership of `younguns` to `oldsters`, not to copy.
    fn new() -> Self {
        let oldsters = [0; 1_000].map(|_| NoteVec::new());
        let younguns = [0; 1_000].map(|_| NoteVec::new());
        let target_seq = NoteVec::new();
        Population {
            oldsters,
            younguns,
            fitness_sum: 0.0,
            mean: 0.0,
            standard_dev: 0.0,
            target_notes: 0,
            target_steps: 0,
            target_seq,
        }
    }

    /// Generate a new population of `NoteVec`s, with the length of
    /// each `NoteVec` determined by the target sequence.  This
    /// function should only be called to jumpstart the whole process;
    /// to evolve an existing population, call `evolve()`.
    fn generate_spontaneously(
        &mut self,
        target_seq: NoteVec,
        target_notes: &i8,
        target_steps: &i8,
    ) {
        self.target_notes = *target_notes;
        self.target_steps = *target_steps;
        self.target_seq = target_seq;
        for i in 0..self.oldsters.len() {
            // Take ownership of the target sequence.
            // Populate the `NoteVec` with nice new notes.
            self.oldsters[i].randomize(self.target_seq.len());
            // Call `update_stats()` here once it's updated to
            // generate running statistics.
        }
        self.update_stats();
    }

    /// This function updates the statistics carried by a `Population`
    /// struct.  For the sake of efficiency, it should be updated to
    /// generate running statistics.
    fn update_stats(&mut self) {
        self.fitness_sum = 0.0;
        for i in 0..self.oldsters.len() {
            self.fitness_sum = self.fitness_sum
                + self.oldsters[i].fitness(
                    &self.target_seq,
                    &self.target_notes,
                    &self.target_steps,
                );
        }
        self.set_mean();
        self.set_standard_dev();
    }

    /// This function chooses a population member, weighted according
    /// to members' fitnesses.  The `fitness_threshold` is used as a
    /// modulus to make sure that the threshold for picking a
    /// population member isn't set too high.
    fn weighted_selection(&self) -> Option<&NoteVec> {
        let mut seed_rng = StdRng::from_os_rng();
        let mut selected: Option<&NoteVec> = None;
        let flip_modulus: f32 = self.mean + (3.0 * self.standard_dev);
        let flip: f32 = if flip_modulus < 0.1 {
            0.1
        } else {
            (seed_rng.random::<f32>()) % flip_modulus
        };
        let mut population_index: usize;
        while selected == None {
            population_index = ((seed_rng.random::<i32>()) % 1000).abs() as usize;
            if self.oldsters[population_index].fitness(
                &self.target_seq,
                &self.target_notes,
                &self.target_steps,
            ) >= flip
            {
                selected = Some(&self.oldsters[population_index]);
            }
        }

        selected
    }

    /// Calculate the population fitness.  The denominator here should
    /// be a parameter.  The `fitness_sum` value should only be
    /// changed when preparing a new population with
    /// `generate_spontaneously()` and during calls to `evolve()`.
    /// This function is only used during testing.
    fn _fitness(&self) -> f32 {
        self.fitness_sum / self.oldsters.len() as f32
    }

    // Create a new population, then become that population, just like
    // in real life.
    fn evolve(&mut self) -> GenericResult<bool> {
        let mut child1: NoteVec;
        let mut child2: NoteVec;
        let youngeruns = [0; 1_000].map(|_| NoteVec::new());

        for i in 0..self.younguns.len() {
            // See
            // https://stackoverflow.com/questions/28572101/what-is-a-clean-way-to-convert-a-result-into-an-option#28572170
            // for info on converting `Option`s to `Result`s with
            // `ok()` and pals.
            let parent1 = self.weighted_selection().ok_or("Lottery malfunction")?;
            let parent2 = self.weighted_selection().ok_or("Lottery malfunction")?;

            [child1, child2] = parent1.breed(parent2);

            // Pick the fitter of the two children at each step.  I am
            // told there is precedent for this.
            if child1.fitness(&self.target_seq, &self.target_notes, &self.target_steps)
                > child2.fitness(&self.target_seq, &self.target_notes, &self.target_steps)
            {
                self.younguns[i] = child1;
            } else {
                self.younguns[i] = child2;
            }
        }

        assert!(self.oldsters.len() == self.younguns.len());

        // See https://doc.rust-lang.org/std/mem/fn.replace.html and
        // _Programming Rust_ chapter 4.  The `std::mem::swap()`
        // function would also do the trick, but using it would take
        // another line.  `take()` would work if I'd implemented
        // the `Default` trait.
        self.oldsters = std::mem::replace(&mut self.younguns, youngeruns);
        // TODO update all the population stats here.
        self.update_stats();
        print!("oldsters:");
        self.oldsters.first().unwrap().display();

        Ok(true)
    }

    fn set_mean(&mut self) {
        self.mean = self.fitness_sum / self.oldsters.len() as f32;
    }

    // Very adapted from
    // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html,
    // with some help from an LLM for the syntax in the closure.
    fn set_standard_dev(&mut self) {
        let variance = self
            .oldsters
            .iter()
            .map(|n_vec| {
                let fitness =
                    n_vec.fitness(&self.target_seq, &self.target_notes, &self.target_steps);
                let diff = self.mean - fitness;
                diff * diff
            })
            .sum::<f32>()
            / self.oldsters.len() as f32;
        self.standard_dev = variance.sqrt();
    }
}

fn main() -> GenericResult<()> {
    let args = Args::parse();

    if args.src_notes.iter().any(|&src_note| src_note < 0) {
	return Err("Midi notes must be at least 0.".into());
    }

    let src_seq: NoteVec = args.src_notes.into_iter().map(|note| (note, 0)).collect();

    let mut pop = Population::new();
    pop.generate_spontaneously(src_seq, &args.target_notes, &args.target_steps);

    for _ in 0..5 {
        pop.evolve()?;
    }

    Ok(())

}
