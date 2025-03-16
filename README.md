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

- [ ] Given a sequence of notes (such as `A4E5D4C3F4`), generate a sequence of the same length that satisfies the harmonic fitness requirement.
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
	- [x] Test `breed()`: make sure mutation happens about half the time
	- [ ] Generate population of sequences

- [ ] Get parameters with `clap`
- [ ] Formalize the contrapuntal part of the fitness function
- [ ] Given a sequence of notes with durations, generate a sequence of the same duration that satisfies both the harmonic and contrapuntal fitness requirements.
- [ ] Output the generated sequence as midi.
- [ ] Read an input sequence from midi.
- [ ] Detect a repeated phrase (allowing for some degree of imprecision) from midi input and treat that as a target sequence.
- [ ] Tidy up
	- [ ] Disallow dead code

-----

Maybe later:


- [ ] Convert `Note` to a struct
- [ ] Implement `Display` trait for `NoteVec`; see `game_of_life` class notes

## Resources

- Look into https://en.wikipedia.org/wiki/Contrapuntal_motion; think about a fitness function for contrapuntal motion.
- Evolving solutions to the 8-queens problem: file:~/Sync/cs541-artificial_intelligence/assignments/programming2/README.md
- Bending midi signals to just intonation: file:~/Sync/cs510-music_sound_computation/intemperate_bend-course_project/README.md
- Wiki pages:
	- Scientific pitch notation: https://en.wikipedia.org/wiki/Scientific_pitch_notation
	- https://en.wikipedia.org/wiki/MIDI
	- https://en.wikipedia.org/wiki/MIDI_tuning_standard
		- See section on converting between MIDI notes and frequencies.
- Dominique Vandenneucker's MIDI tutorial: https://www.cs.cmu.edu/~music/cmsip/readings/MIDI%20tutorial%20for%20programmers.html
- Joe Monzo; "A gentle introduction to the MIDI tuning specification"; 2001; http://tonalsoft.com/monzo/miditune/miditune.aspx.  Examples of MIDI pitch bend calculations.

## Sequence representation

Suppose I were to represent each point in time with a `u32`, and that I sampled at a rate of 44.1kHz.
Then I would have 44,100 `u32`s a second with 4 bytes per `u32` and 1 kibibyte per 1024 bytes.
Multiplying: `(44100 * 4) / 1024 => 172.265625`, so that would take about 172 kibibytes per second.
Not terrible, but that's a kinda unwieldly representation for evolving accompaniments---is there a simpler way?

Per the [midi tuning standard](https://midi.org/midi-tuning-updated-specification), midi frequencies are represended by 3 bytes, with the top of each byte reserved.
The first byte represents one of the 128 possible midi notes.
The next two represent one of `100/2^14` microtonal increments to the base pitch, which, although below the threshold of human hearing, the specification recommends keeping so as to make it easier to communicate with instruments that strictly follow the specification.
That is, the situation is like this:

	0-------|0-------|0-------
	^        ^        ^
	|        |        |
	|        |        still the offset
	|        offset
	note

That being the case, I am going to represent note midi frequencies with `(u8, u16)` tuples.

## Fitness function

I'd like evolved patters to vary harmonically and rhythmically.

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
