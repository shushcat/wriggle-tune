# wriggle-tune

This is a small, work-in-progress program to evolve (questionably) musical accompaniments for short phrases.

## The algorithm, broadly

1. Randomly generate an initial population of sequences, each having the same duration as the target sequence;
2. Evolve a new population of the same size as the initial population by
  - Carrying out fitness-weighted lottery selection of pairs from the initial population
    - (and, if a chosen sequence meets a predetermined fitness threshold, returning that sequence),
  - Splice each pair using a random crossover point to generate two new sequences, and
  - Adding one or both of the new sequences to the new population; and
3. Use the new population as the initial population in step 2.

## Roadmap & log

- [x] Given a sequence of notes (such as `A4E5D4C3F4`), generate a sequence of the same length that satisfies the harmonic fitness requirement.
	- [x] Pick representation for this stage: vector of `(u8, u16)`s?
	- [x] Design fitness function
		- [x] Formalize harmonic part
	- [x] Mock up fitness function
	- [x] Implement fitness function that handles harmonic fitness
		- [x] Make sure that a pattern's fitness increases as it gets closer to each target value, and that its fitness decreases at the same rate on either side of the target value.
	- [x] Make it work with a mocked sequence.
	- [x] Parameterize and pass `p_steps` and `p_notes`
	- [x] Parameterize the target sequence
	- [x] Test the fitness function
	- [x] Test mutation
		- [x] Fix ownership; need mutable borrow, not steal
	- [x] Implement `breed()`
		- [x] Splice `self` and `other`
		- [x] Randomly mutate both `child1` and `child2` with some probability.
	- [x] Test `mutate()`: make sure mutation happens about half the time
	- [x] Test `breed()`: ensure there returned children don't differ by more than they should given mutation and crossover
	- [x] Generate population of sequences
		- [x] Initialize population values to random sequences
		- [x] Test to make sure it gets setup
		- [x] It would be better if `target` were part of the `Population` struct, so let's do that
	- [x] Draft `weighted_selection()`
	- [x] Test & revise `weighted_selection()`
	- [x] Draft `evolve()`
	- [x] Fix infinite loop in `weighted_selection()`
		- Thresholds are being set that are higher than any items in the population.
		- [x] Set the threshold modulus to the population as a hot-fix, and
		- [x] Set the threshold modulus to the population mean plus one standard deviation
			- See the _Rust cookbook_'s section on this at https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
			- ~~Calculate a running standard deviation using Welford's method~~  Just use the _Rust cookbook_ since I only need to calculate the standard deviation once per population; in future, perhaps see
				- https://stackoverflow.com/questions/1174984/how-to-efficiently-calculate-a-running-standard-deviation
				- https://rosettacode.org/wiki/Cumulative_standard_deviation#Rust
				- https://www.johndcook.com/blog/standard_deviation/
				- https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford%27s_online_algorithm
			1. threshold::mean
			2. Randomly pick an item, then pick it with a probability determined by its fitness.
			3. Actually do the lottery selection thing as intended.
	- [x] Test `standard_deviation()`
	- [x] Test `weighted_selection()`
	- [x] Cache population fitness standard deviation and mean
	- [x] Test & revise `evolve()`
		- [x] Make sure population fitness increases over time.
		- [x] Use parameters stored in `Population` rather than passing
- [x] Rename `target_seq` to `src_seq`
- [x] Get parameters with `clap`
	- For now, just get `target_notes`, `target_steps`, and `src_seq`
- [x] Output the midi sequence
	- [x] Pick the midi crate; use `midir`: https://crates.io/crates/midir
	- [x] Play sequence to `timidity` or `fluidsynth`?  To Renoise?
- [x] Output the generated sequence as midi.
- [x] Tidy up
	- [x] Disallow dead code
	- [ ] Cargo check, format, and so on

-----

Wishlist:

- [ ] Formalize the contrapuntal part of the fitness function
- [ ] Convert `Note` to a struct
- [ ] Implement `Display` trait for `NoteVec`; see `game_of_life` class notes
- [ ] Replace `StdRng` with `rand_chacha` for portability
- [ ] Cache `NoteVec` fitnesses
- [ ] Add a `movement` parameter to affect octave-jumps
- [ ] Add flags for command line invocation
- [ ] Allow for setting different numbers of notes by anchoring to currently playing notes
- [ ] Handle keypresses without blocking, maybe using `crossterm`
- [ ] Setup basic terminal interface.
- [ ] Input target sequence with midi
- [ ] Given a sequence of notes with durations, generate a sequence of the same duration that satisfies both the harmonic and contrapuntal fitness requirements.
- [ ] Detect a repeated phrase (allowing for some degree of imprecision) from midi input and treat that as a target sequence.

## Resources

- Look into https://en.wikipedia.org/wiki/Contrapuntal_motion; think about a fitness function for contrapuntal motion.
- Evolving solutions to the 8-queens problem: file:~/Sync/cs541-artificial_intelligence/assignments/programming2/README.md
- Bending midi signals to just intonation: file:~/Sync/cs510-music_sound_computation/intemperate_bend-course_project/README.md
- Useful crates
	- https://crates.io/crates/jack real-time audio and midi with Jack
	- https://crates.io/crates/midir real-time midi-processing library
		- Used and updated a lot; used in Bart's `synthkit`
		- See https://www.ntietz.com/blog/parsing-midi-rust/.
	- https://crates.io/crates/midly fast processing of midi files and signals
- Wiki pages:
	- Scientific pitch notation: https://en.wikipedia.org/wiki/Scientific_pitch_notation
	- https://en.wikipedia.org/wiki/MIDI
	- https://en.wikipedia.org/wiki/MIDI_tuning_standard
		- See section on converting between MIDI notes and frequencies.
- Dominique Vandenneucker's MIDI tutorial: https://www.cs.cmu.edu/~music/cmsip/readings/MIDI%20tutorial%20for%20programmers.html
- Joe Monzo; "A gentle introduction to the MIDI tuning specification"; 2001; http://tonalsoft.com/monzo/miditune/miditune.aspx.  Examples of MIDI pitch bend calculations.

## Working notes

### Sequence representation

~~Suppose I were to represent each point in time with a `u32`, and that I sampled at a rate of 44.1kHz.~~
~~Then I would have 44,100 `u32`s a second with 4 bytes per `u32` and 1 kibibyte per 1024 bytes.~~
~~Multiplying: `(44100 * 4) / 1024 => 172.265625`, so that would take about 172 kibibytes per second.~~
~~Not terrible, but that's a kinda unwieldly representation for evolving accompaniments---is there a simpler way?~~
[No, that's nonsense and I should sleep more---I need to focus on midi, not sample rates.]

Per the [midi tuning standard](https://midi.org/midi-tuning-updated-specification), midi frequencies are represended by 3 bytes, with the top of each byte reserved.
The first byte represents one of the 128 possible midi notes.
The next two represent one of `100/2^14` microtonal increments to the base pitch, increments which, although below the threshold of human hearing, the specification recommends keeping so as to make it easier to communicate with instruments that strictly follow the specification.
That is, the situation is like this:

	0-------|0-------|0-------
	^        ^        ^
	|        |        |
	|        |        still the offset
	|        offset
	note

That being the case, I am going to represent note midi frequencies with ~~`(u8, u16)` tuples~~.
Actually, I should be using `(i8, i16)` tuples since that way I need to handle any changes in sign as they occur when dealing with the note.
It's still somewhat unclear what will need to happen when dealing with microtonal adjustments; it might be a good idea to split the `i16` into `i8`s, but there will be some bit-level mussing around in either case.

### Fitness function

I'd like evolved patterns to vary harmonically and rhythmically.

I need to quantize the number of steps since if they weren't quantized, a four note pattern would have among its accompaniments a pattern with a single note that is offset from the source sequence by four steps and a pattern with four notes that are each offset by one step, and the two would be identically fit.

Initially, I'll need to place a single note.

Likely parameters:

- number of steps
- chunk size
- resolution
- counterpoint (logarithmic?)

Leave aside resolution and counterpoint for now---just try to get a single note placed with the required number of steps.
This will take a little bit of planning to make sure I don't paint myself into a corner here.

Determine how many notes to place as the ratio of steps to chunk size.
So, for instance, if the number of steps was 4 (a third) and the chunk size was 4, then only one note would be placed, and all of the following would be equally fit.

	C#	F	G#	source sequence

	F			target sequences...
	A#
		A
		C#
			C
			E

Experiment with initial `C#3F3G#3` sequence; test the fitness of compared sequences.
