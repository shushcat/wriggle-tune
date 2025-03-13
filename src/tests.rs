use crate::*;

#[test]
fn exists() {
    assert!(1 == 1);
}

#[test]
fn fitness_max() {
    let test_seq: NoteVec = vec![(49,0), (53,0), (56,0)];
    assert_eq!(4, test_seq.fitness());

}
