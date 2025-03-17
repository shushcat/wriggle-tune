use crate::*;

#[test]
fn exists() {
    assert!(1 == 1);
}

#[test]
fn fitness_test() {
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
fn mutate_test() {
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
fn breed_test() {
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
    // through everything available on iterators.
    let s1_3count: i8 = seq1.iter().filter( |&x| seq3.iter().any( |y| x == y)).count() as i8;
    let s2_4count: i8 = seq2.iter().filter( |&x| seq4.iter().any( |y| x == y)).count() as i8;
    println!("{}, {}", s1_3count, s2_4count);
    assert!((s1_3count - s2_4count).abs() >= 0);
    assert!((s1_3count - s2_4count).abs() <= 1);
}

#[test]
fn test_generate_spontaneously() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let mut pop = Population::new();
    pop.generate_spontaneously(&target_seq, &3, &5);
    pop.oldsters[0].display();
    pop.oldsters[1].display();
    assert!(pop.oldsters[0].len() == 3);
    assert!(pop.oldsters[999].len() == 3);
    for i 0..1000 {
	println!("{}", pop.oldsters[i]);
    }
    assert!(0==1);
}
