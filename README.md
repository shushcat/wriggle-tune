# wriggle-tune

This is a placeholder for a project that will evolve short bits of musical accompaniment.

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
	- What should the actual representation be?  See 8-queens with genetic algorithms and midi signal bending programs.
- [ ] Given a sequence of notes with durations, generate a sequence of the same duration that satisfies both the harmonic and contrapuntal fitness requirements.
- [ ] Output the generated sequence as midi.
- [ ] Read an input sequence from midi.
- [ ] Detect a repeated phrase (allowing for some degree of imprecision) from midi input and treat that as a target sequence.

## Resources

- Look into https://en.wikipedia.org/wiki/Contrapuntal_motion; think about a fitness function for contrapuntal motion.
- Evolving solutions to the 8-queens problem: file:~/Sync/cs541-artificial_intelligence/assignments/programming2/README.md
- Bending midi signals to just intonation: file:~/Sync/cs510-music_sound_computation/intemperate_bend-course_project/README.md

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
