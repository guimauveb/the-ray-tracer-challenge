#[cfg(test)]
use crate::approx_eq::ApproxEq;

#[test]
fn assert_two_floats_are_equal_if_their_absolute_difference_is_less_than_epsilon() {
    let a = 1.1234560;
    let b = 1.1234567;
    assert_eq!(a.approx_eq(b), true);

    let c = 4.76543210;
    let d = 4.76543228;
    assert_eq!(c.approx_eq(d), true);
}

#[test]
fn assert_two_floats_are_not_equal_if_their_absolute_difference_is_greater_than_or_equal_to_epsilon(
) {
    let a = 1.12346;
    let b = 1.12344;
    assert_eq!(a.approx_eq(b), false);

    let c = 4.76543;
    let d = 4.76542;
    assert_eq!(c.approx_eq(d), false);
}
