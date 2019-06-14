use super::*;

#[test]
fn args_given_no_arguments_succeeds() {
    // setup
    let app = "test";
    let args = vec![app];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should be OK
    assert_eq!(result.is_ok(), true);
}

#[test]
fn args_given_an_argument_fails() {
    // setup
    let app = "test";
    let arg = "hello";
    let args = vec![app, arg];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(result.is_err(), true);
}
