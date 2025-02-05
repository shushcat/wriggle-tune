---
title: CS523---WriggleTune proposal
author: JO Brickley
date: \today
---

I propose writing a small program, one **WriggleTune**, that, evolves a phrase of musical accompaniment to a given target sequence.
While I will use midi to represent all note sequences, I am not yet sure what form these representations will take since each sequence must encode both tones and durations.
Tentatively, I will use two "chromosome"---one for notes and the other for durations---to represent each sequence of notes.
The genetic algorithm I will use to evolve the musical accompaniments will broadly do the following:

1. Randomly generate an initial population of sequences, each having the same duration as the target sequence;
2. Evolve a new population of the same size as the initial population by
  - Carrying out fitness-weighted lottery selection of pairs from the initial population
    - (and, if a chosen sequence meets a predetermined fitness threshold, returning that sequence),
  - Splice each pair using a random crossover point to generate two new sequences, and
  - Adding one or both of the new sequences to the new population; and
3. Use the new population as the initial population in step 2.

Two problems I forsee are, one (as mentioned above), the specific representation of the sequences and, two, how to measure sequence fitness.
Of the two problems, I suspect sequence fitness will be the more difficult to solve.
If unqualified harmonization were the target, then the population would tend to evolve to be similar to the target sequence, which wouldn't be very interesting.
My initial plan, assuming the sequence representations will have note and duration chromosomes, is to use parameterized harmony and counterpoint values when calculating sequence fitness.
For instance, when working on the first step of the roadmap below, I will calculate a harmonization score based only on note names, with octaves equal to one, and each successive harmonic worth some value (perhaps 0.1) less than the preceeding harmonic.

Some milestones, which I will decompose further in the main `README` for this project, follow.

- Given a sequence of notes (such as `A4E5D4C3F4`), generate a sequence of the same length that satisfies the harmonic fitness requirement.
- Given a sequence of notes with durations, generate a sequence of the same duration that satisfies both the harmonic and contrapuntal fitness requirements.
- Output the generated sequence as midi.
- Read an input sequence from midi.
- Detect a repeated phrase (allowing for some degree of imprecision) from midi input and treat that as a target sequence.
