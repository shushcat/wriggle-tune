#![allow(dead_code)]  // TODO Get rid of this nonsense.

#[cfg(test)]
mod tests;

type Note = (u8, u16);
type NoteVec = Vec<Note>;

trait Chromosome {
    fn breed(&self, other: Self) -> Self;
    fn display(&self);
    fn fitness(&self) -> usize;
    fn mutate(self);
}

impl Chromosome for NoteVec {

    fn breed(&self, other: Self) -> Self {
	todo!();
    }

    fn display(&self) {
	todo!();
    }

    fn fitness(&self) -> usize {
	let target: NoteVec = vec![(49,0), (53,0), (56,0)];
	let steps: u8 = 4; // Will need to pass these.
	let chunks: u8 = 1;
	self[0].0;
	for i in 0..self.len() {
	    // Sum the distances.
	    // n.0 - target...
	}
	0
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
	todo!();
    }

    fn evolve(self) {
	todo!();
    }

}

fn main() {
    println!("Hello, primordial ooze!");
    let test_seq: NoteVec = vec![(49,0), (53,0), (56,0)];
    println!("{}", test_seq.fitness());
}
