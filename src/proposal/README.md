---
title: CS523---WriggleTune proposal
author: JO Brickley
date: \today
---

problems
	chromosomes
	fitness

I propose writing a small program that, given a target sequence of notes, evolves a phrase of musical accompaniment to that target sequence.
While I will use midi to represent all sequences of notes handled by the proposed program, I am not yet sure as to the precise form these representations will take since each sequence must encode both tones and durations.
Specifically, I think it may be necessary to employ more than one "chromosome"---one for notes and the other for durations---I do not intend to consider dynamics.
The genetic algorithm I will use to evolve the musical accompaniments will, in the broadest of strokes, do the following:

Randomly generate an initial population of sequences, where each population member is a sequence of notes with the same duration as the target sequence;
Lottery selection from the population, weighted by sequence fitness;
Breed selected pairs using a randomly selected crossover point for splicing the sequences, and producing either two offspring per pair or selecting the fittest of the two;

Roadmap:

- Pick representation for candidates
- Evolve a string of midi notes

randomly generated population
lottery selection in population
	pick members with higher probability based on fitness
	if a picked member exceeds the fitness threshold, return that member
breed pairs
      crossover on randomly selected point
      either pick fitter of the two children
      or add both to the new population
      continue until the new population is the same size as the old one.
