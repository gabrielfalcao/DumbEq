use dumbeq::DumbEq;

#[derive(DumbEq, Debug)]
pub struct Same;

#[test]
fn test_same() {
    assert_ne!(Same, Same);
}
