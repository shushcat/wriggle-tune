#![allow(dead_code)] // TODO Get rid of this nonsense.

#[cfg(test)]
mod tests;

// TODO Replace StdRng with `rand_chacha` for portability.
use rand::{rngs::StdRng, Rng, SeedableRng};

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
    fn fitness(&self, target_seq: &NoteVec, p_notes: &i8, p_steps: &i8) -> f32;
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
            // seed_rng = StdRng::from_os_rng();
            let mutation_index: usize = ((seed_rng.random::<u32>()) % (self.len() as u32)) as usize;
            let random_note: i8 = (seed_rng.random::<i8>() % 127).abs();
            self[mutation_index].0 = random_note;
            mutated = true;
        }
        mutated
    }

    fn randomize(&mut self, length: usize) {
        self.clear();
        let mut seed_rng = StdRng::from_os_rng();
        // Populate the `NoteVec` with nice new notes.
        for _ in 0..length {
            let random_note: Note = ((seed_rng.random::<i8>() % 127).abs(), 0);
            self.push(random_note);
        }
    }
}

struct Population {
    oldsters: [NoteVec; 1_000],
    younguns: [NoteVec; 1_000], // Only used while `evolve()`-ing.
    size: usize,                // Unnecessary if vector lengths hardcoded.
    fitness_sum: f32,           // Normalized in `fitness()`.
    target_notes: i8,
    target_steps: i8,
    target_seq: NoteVec,
}

impl Population {
    fn new() -> Self {
        // This would be less of a pain to initialize if I implemented
        // the `Copy` trait on `NoteVec`, but I think doing that would
        // make evolving the next generation more expensive since
        // ownership of each value in `younguns` wouldn't simply be
        // transferred to `oldsters`.
        let oldsters = [0; 1_000].map(|_| NoteVec::new());
        let younguns = [0; 1_000].map(|_| NoteVec::new());
        let target_seq = NoteVec::new();
        Population {
            oldsters,
            younguns,
            size: 0,
            fitness_sum: 0.0,
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
            self.size = self.size + 1;
            self.fitness_sum = self.fitness_sum
                + self.oldsters[i].fitness(
                    &self.target_seq,
                    &self.target_notes,
                    &self.target_steps,
                );
        }
    }

    /// This function chooses a population member, weighted according
    /// to members' fitnesses.  The `fitness_threshold` is used as a
    /// modulus to make sure that the threshold for picking a
    /// population member isn't set too high.
    fn weighted_selection(&self) -> Option<&NoteVec> {
        let mut seed_rng = StdRng::from_os_rng();
        let mut selected: Option<&NoteVec> = None;
        let flip_modulus: f32 = self.mean() + (3.0 * self.standard_deviation());
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
    fn fitness(&self) -> f32 {
        self.fitness_sum / 1000.0
    }

    // Create a new population, then become that population, just like
    // in real life.
    fn evolve(&mut self) -> GenericResult<bool> {
        let p_notes: i8 = 4; // TODO parameterize
        let p_steps: i8 = 3; // TODO parameterize
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
            if child1.fitness(&self.target_seq, &p_notes, &p_steps)
                > child2.fitness(&self.target_seq, &p_notes, &p_steps)
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

        Ok(true)
    }

    fn mean(&self) -> f32 {
        self.fitness_sum / self.oldsters.len() as f32
    }

    // Very adapted from
    // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html,
    // with some help from an LLM for the closure syntax.
    fn standard_deviation(&self) -> f32 {
        let mean = self.mean();
        let variance = self
            .oldsters
            .iter()
            .map(|n_vec| {
                // let first_note = n_vec.first().map_or(0.0, |&(n, _)| n as f32);
                let fitness =
                    n_vec.fitness(&self.target_seq, &self.target_notes, &self.target_steps);
                let diff = mean - fitness;
                diff * diff
            })
            .sum::<f32>()
            / self.oldsters.len() as f32;
        variance.sqrt()
    }
}

fn main() {
    let test_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    test_seq.display();
    println!("Hello, primordial ooze!");
}
