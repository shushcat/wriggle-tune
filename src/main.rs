#![allow(dead_code)] // TODO Get rid of this nonsense.

#[cfg(test)]
mod tests;

use rand::{rngs::StdRng, Rng, SeedableRng};

// This might be better as a struct.
type Note = (i8, i16);

type NoteVec = Vec<Note>;

trait Chromosome {
    fn breed(&self, other: &Self) -> [Self; 2]
    where
        Self: Sized;
    fn display(&self);
    fn fitness(&self, target: &NoteVec, p_notes: &i8, p_steps: &i8) -> f32;
    fn mutate(&mut self);
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
        child2.extend_from_slice(&other[crossover_index..]);
        child2.extend_from_slice(&self[..crossover_index]);



	// Mutate!

        [child1, child2]
    }

    // If I change `Note` to a struct, this should be removed and I
    // should derive `Display`.
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
    /// value around the edges, to avoid infinities near zero and out
    /// of an abundance of caution on the other end, also seemed
    /// prudent.
    fn fitness(&self, target: &NoteVec, notes_param: &i8, steps_param: &i8) -> f32 {
        let mut notes: isize = 0;
        let mut steps: isize = 0;

        // Get the total number of notes along with the sum of the
        // number of steps by which each note in this sequence is
        // offset from the note at the same index in the target
        // sequence.
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

        // The `0.1429` in the following expressions is just an
        // approximation of 1/7th, which makes the curve described by
        // `x.log2()` in the 1 by 1 square immediately above the origin
        // look flatter toward the right and steeper toward the left,
        // both in precisely the way I think this particular reward
        // function should be.
        notes_deviation = -(notes_deviation.log2() * 0.1429);
        steps_deviation = -(steps_deviation.log2() * 0.1429);

        let result = (notes_deviation + steps_deviation) / 2.0;
        if result > 0.99 {
            return 1.0;
        } else {
            return result;
        }
    }

    /// Randomly change a Note in a NoteVec.  For now, this only
    /// affects notes proper---microtunings and contrapoint come
    /// later.  This is called probabilistically from `breed()`.
    fn mutate(&mut self) {
        let mut seed_rng = StdRng::from_os_rng();
        let flip: u8 = (seed_rng.random::<u8>()) % 255;
	if flip > 127 {
	    seed_rng = StdRng::from_os_rng();
	    let mutation_index: usize = ((seed_rng.random::<u32>()) % (self.len() as u32)) as usize;
	    let random_note: i8 = seed_rng.random::<i8>() % 127;
	    self[mutation_index].0 = random_note;
	}
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
    let test_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    test_seq.display();
    println!("Hello, primordial ooze!");
}
