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
    fn fitness(&self, target: &NoteVec, notes_param: &i8, steps_param: &i8) -> f32 {

        let mut notes: isize = 0;
        let mut steps: isize = 0;

	// Get the total number of notes along with the sum of the
	// number of steps by which each note in this sequence is
	// offset from the note at the same index in the target
	// sequence.
        self[0].0;
        for i in 0..self.len() {
            notes = notes + 1;
            steps = steps + (target[i].0 - self[i].0).abs() as isize;
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

	notes_deviation = -(notes_deviation.log2() * 0.1429);
	steps_deviation = -(steps_deviation.log2() * 0.1429);

	let result = (notes_deviation + steps_deviation) / 2.0;
	if result > 0.99 {
	    return 1.0;
	} else {
	    return result;
	}
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
