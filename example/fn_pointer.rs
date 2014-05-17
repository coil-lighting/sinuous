// How to use function pointers inside of structs, how to get them out and
// call the functions, and how to do it with generic <T> typed functions, too.

fn double(x: i64) -> i64 {
    x * 2
}

fn noop<T>(x: T) -> T {
    x
}

#[test]
fn test_double() {
    assert!(double(0) == 0);
    assert!(double(3) == 6);
}

struct Ab {
    function_a: fn(i64) -> i64,
    function_b: fn(i64) -> i64,
}

#[test]
fn test_astruct() {
    let ab = Ab {
        function_a: double,

        // don't put "noop<i64>" here because function_b was already defined as
        // a fn(i64) -> i64. Restating the <i64> is just a syntax error.
        function_b: noop,
    };
    let fa = ab.function_a;
    assert!(fa(0) == 0);
    assert!(fa(3) == 6);

    let fb = ab.function_b;
    assert!(fb(0) == 0);
    assert!(fb(3) == 3);
}
