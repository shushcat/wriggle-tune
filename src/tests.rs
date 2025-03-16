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
}
