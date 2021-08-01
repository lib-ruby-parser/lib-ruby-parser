use super::Loc;

#[test]
fn test_loc_new() {
    let loc = Loc::new(1, 2);
    assert_eq!(loc.begin(), 1);
    assert_eq!(loc.end(), 2);
}
