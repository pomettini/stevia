#[test]
fn test_undefined() {
    let output = "A;";
    assert_eq!("", output);
}

#[test]
fn test_print() {
    let output = "P;Hello There";
    assert_eq!("", output);
}

#[test]
fn test_question() {
    let output = "Q;Yes, I like it!;00120;No, I do not like it;00136";
    assert_eq!("", output);
}

#[test]
fn test_jump() {
    let output = "J;00001";
    assert_eq!("", output);
}

#[test]
fn test_end() {
    let output = "E;";
    assert_eq!("", output);
}
