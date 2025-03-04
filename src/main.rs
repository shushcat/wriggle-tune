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
	todo!();
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
}
