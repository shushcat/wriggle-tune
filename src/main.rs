#![allow(dead_code)]  // TODO Get rid of this nonsense.

#[cfg(test)]
mod tests;

type Note = (i8, i16);
type NoteVec = Vec<Note>;

trait Chromosome {
    fn breed(&self, other: Self) -> Self;
    fn display(&self);
    fn fitness(&self) -> isize;
    fn mutate(self);
}

impl Chromosome for NoteVec {

    fn breed(&self, _other: Self) -> Self {
	todo!();
    }

    fn display(&self) {
	todo!();
    }

    fn fitness(&self) -> isize {

	// Passed &params; get with Clap
	let target: NoteVec = vec![(49,0), (53,0), (56,0)];
	let p_steps: i8 = 4;
	let p_notes: i8 = 3;

	let mut steps: isize = 0;
	let mut notes: isize = 0;
	self[0].0;
	for i in 0..self.len() {
	    steps = steps + (target[i].0 - self[i].0).abs() as isize;
	    notes = notes + 1;
	}

	// These expressions make sure that the fitness of a pattern
	// increases as it approaches the parameters, the decreases at
	// the same rate it increased as it grows beyond the target
	// values.  I know they are ugly, and I am sorry.
	steps = ((p_steps as isize) - (((p_steps as isize) * (steps / (p_steps as isize)) - 1).abs() + (steps % (p_steps as isize)))).abs();
	notes = ((p_notes as isize) - (((p_notes as isize) * (notes / (p_notes as isize)) - 1).abs() + (notes % (p_notes as isize)))).abs();

	steps + notes
    }

    fn mutate(self) {
	todo!();
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
