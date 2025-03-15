use crate::*;

#[test]
fn exists() {
    assert!(1 == 1);
}

#[test]
fn fitness_max() {
    let target_seq: NoteVec = vec![(49, 0), (53, 0), (56, 0)];
    let test_seq: NoteVec = vec![(49,0), (53,0), (56,0)];
    assert_eq!(0.42857143, test_seq.fitness(&target_seq, &3, &4));
    let target_seq = vec![(49, 0), (53, 0), (56, 0)];
    let test_seq = vec![(52,0), (53,0), (57,0)];
    assert_eq!(1.0, test_seq.fitness(&target_seq, &3, &4));

}
