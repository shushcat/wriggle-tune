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

## Roadmap

- [ ] Given a sequence of notes (such as `A4E5D4C3F4`), generate a sequence of the same length that satisfies the harmonic fitness requirement.
- [ ] Given a sequence of notes with durations, generate a sequence of the same duration that satisfies both the harmonic and contrapuntal fitness requirements.
- [ ] Output the generated sequence as midi.
- [ ] Read an input sequence from midi.
- [ ] Detect a repeated phrase (allowing for some degree of imprecision) from midi input and treat that as a target sequence.

## Resources

- Look into https://en.wikipedia.org/wiki/Contrapuntal_motion; think about a fitness function for contrapuntal motion.
