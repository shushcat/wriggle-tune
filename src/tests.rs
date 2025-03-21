use crate::*;

#[test]
fn chromosome_fitness() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let test_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    assert_eq!(0.85625196, test_seq.fitness(&target_seq, &3, &4));
    let target_seq = vec![(49, 0), (53, 0), (56, 0)];
    let test_seq = vec![(52, 0), (53, 0), (57, 0)];
    assert_eq!(1.0, test_seq.fitness(&target_seq, &3, &4));
    let test_seq = vec![(49, 0), (53, 0), (56, 0)];
    assert_eq!(0.92770195, test_seq.fitness(&test_seq, &1, &1));
    let test_seq = vec![(49, 0), (53, 0), (56, 0)];
    assert_eq!(0.88590634, test_seq.fitness(&test_seq, &0, &0));
    let test_seq = vec![(49, 0), (53, 0), (56, 0)];
    assert_eq!(1.0, test_seq.fitness(&test_seq, &3, &0));
}

#[test]
fn chromosome_mutate() {
    let mut test_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut mutation_count: f32 = 0.0;
    let runs = 1_000;
    for _i in 0..runs {
        if test_seq.mutate() {
            mutation_count = mutation_count + 1.0;
        }
    }
    mutation_count = mutation_count / runs as f32;

    assert!(0.40 < mutation_count);
    assert!(0.60 > mutation_count);
}

#[test]
fn chromosome_breed() {
    let mut seq1 = NoteVec::new();
    let mut seq2 = NoteVec::new();
    let seq3: NoteVec;
    let seq4: NoteVec;
    seq1.randomize(6);
    seq2.randomize(6);
    seq1.display();
    seq2.display();
    [seq3, seq4] = seq1.breed(&seq2);
    seq3.display();
    seq4.display();
    // Used an LLM to find the `any()` method instead of combing
    // through everything available on iterators.  This check could be
    // done much more quickly by just indexing the vectors, but I'm
    // keeping it for reference since this is just a test.
    let s1_3count: i8 = seq1.iter().filter(|&x| seq3.iter().any(|y| x == y)).count() as i8;
    let s2_4count: i8 = seq2.iter().filter(|&x| seq4.iter().any(|y| x == y)).count() as i8;
    println!("{}, {}", s1_3count, s2_4count);
    assert!((s1_3count - s2_4count).abs() >= 0);
    assert!((s1_3count - s2_4count).abs() <= 1);
}

#[test]
fn population_generate_spontaneously() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut pop = Population::new();
    pop.generate_spontaneously(target_seq, &3, &5);
    assert!(pop.oldsters[0].len() == 3);
    assert!(pop.oldsters[499].len() == 3);
    assert!(pop.oldsters[999].len() == 3);
}

#[test]
fn population_fitness() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut pop = Population::new();
    pop.generate_spontaneously(target_seq, &3, &5);
    let fit = pop._fitness();
    assert!(0.0 <= fit);
    assert!(1.0 >= fit);
}

#[test]
fn population_standard_deviation() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut pop = Population::new();
    pop.generate_spontaneously(target_seq, &3, &5);
    println!("{}", pop.standard_dev);
    let stdev = pop.standard_dev;
    assert_eq!(stdev, pop.standard_dev);
    assert!(0.6 > pop.standard_dev);
    assert!(0.01 < pop.standard_dev);
}

#[test]
fn population_lottery_selection() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut pop = Population::new();
    pop.generate_spontaneously(target_seq, &3, &5);
    assert!(pop.weighted_selection() != None);
}

#[test]
fn population_evolve() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut pop = Population::new();
    pop.generate_spontaneously(target_seq, &3, &5);
    let fit1 = pop._fitness();
    let _ = pop.evolve();
    let fit2 = pop._fitness();
    assert!(fit1 < fit2);
    _ = pop.evolve();
    let fit1 = pop._fitness();
    assert!(fit1 > fit2);
}
