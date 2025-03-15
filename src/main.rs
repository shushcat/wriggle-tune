#![allow(dead_code)] // TODO Get rid of this nonsense.

#[cfg(test)]
mod tests;

use rand::{rngs::StdRng, Rng, SeedableRng};
// use std::f32::log2;

type Note = (i8, i16);
type NoteVec = Vec<Note>;

trait Chromosome {
    fn breed(&self, other: Self) -> Self;
    fn display(&self);
    fn fitness(&self, target: &NoteVec, p_notes: &i8, p_steps: &i8) -> f32;
    fn mutate(self);
}

impl Chromosome for NoteVec {
    fn breed(&self, _other: Self) -> Self {
        todo!();
    }

    fn display(&self) {
	for i in 0..self.len() {
	    print!("({}, {})", self[i].0, self[i].1);
	}
    }

    // Parameters are passed by reference in case I need to re-use
    // them elsewhere.
    fn fitness(&self, target: &NoteVec, p_notes: &i8, p_steps: &i8) -> f32 {

        let mut notes: isize = 0;
        let mut steps: isize = 0;

	// let mut note_denom: isize = *p_notes as isize;
	// let mut steps_denom: isize = *p_steps as isize;

        self[0].0;
        for i in 0..self.len() {
            notes = notes + 1;
            steps = steps + (target[i].0 - self[i].0).abs() as isize;
        }

	// if note_denom == 0 {
	//     note_denom = notes;
	// }
	// if steps_denom == 0 {
	//     steps_denom = steps;
	// }

	// Maybe worth smoothing out the gradient towards fitness by
	// using a logarithm, but doing things this way would require
	// lots of type-reshuffling, and handling of infinities when
	// either parameter is 0.
	// notes = -(((notes - *p_notes as f32).abs() * 10.0).log2());
	// steps = -(((steps - *p_steps as f32).abs() * 10.0).log2());

        // These expressions make sure that the fitness of a pattern
        // increases as it approaches the parameters, then decreases at
        // the same rate it increased at as it grows beyond the target
        // values.  I know they are ugly, and I am sorry.
        notes = ((*p_notes as isize)
            - (((*p_notes as isize) * (notes / (*p_notes as isize) - 1).abs()
                + (notes % (*p_notes as isize)))))
            .abs();
        steps = ((*p_steps as isize)
            - (((*p_steps as isize) * (steps / (*p_steps as isize) - 1).abs()
                + (steps % (*p_steps as isize)))))
            .abs();
	println!{"{}, {}", notes, steps};

        ((steps + notes) as f32) / ((p_steps + p_notes) as f32)
    }

    /// Randomly change a Note in a NoteVec.
    fn mutate(mut self) {
	let mut seed_rng = StdRng::from_os_rng();
	let mutation_index: usize = (seed_rng.random::<i32>() % (self.len() as i32)) as usize;
	let random_note: i8 = seed_rng.random::<i8>() % 127;
	self[mutation_index].0 = random_note;
    }
}

struct Population {
    note_vecs: Vec<NoteVec>,
    fitness: usize,
}

impl Population {
    fn generate_spontaneously(self) {
        todo!();
    }

    fn lottery_selection(&self) -> NoteVec {
        todo!();
    }

    /// Calculate the population's `fitness` parameter.
    fn calc_fitness(self) {
        // Rather than calculating this, maybe have a `fitness_numer`
        // and `fitness_denom` in the `Population` struct.  Then there
        // could be `fitness()` function for `Population` that would
        // just do the division when called.  The values would need to
        // be updated each time a population member was polled.
        todo!();
    }

    fn evolve(self) {
        todo!();
    }
}

fn main() {
    println!("Hello, primordial ooze!");
}
