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
    let test_orig = test_seq.clone();
    test_seq.mutate();
    let mut same_notes = 0;
    for i in 0..test_seq.len() {
        if test_seq[i] == test_orig[i] {
            same_notes = same_notes + 1;
        }
    }
    assert_eq!(2, same_notes);
}

#[test]
fn breed_test() {
}
