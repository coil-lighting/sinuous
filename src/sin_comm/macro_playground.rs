#![feature(macro_rules)]

fn foo_alpha(a: int) -> int {
	a+1
}

fn foo_beta(a: int) -> int {
	a-1
}

macro_rules! call_foo(
	($var: ident, $val: expr) => (
		concat_idents!(foo_,$var)($val)
	);
)

struct Bar {
	val: int
}

impl Bar {
	fn bar_alpha(&self, a: int) -> int {
		self.val + a
	}
}

enum Puke {
	A(int),
	B(int),
	C(int)
}

macro_rules! puke_list(
	() => (
		[A,B,C]
	);
)


macro_rules! test_puke(
	( $to_match: expr, $($name: ident),* ) => (
		match $to_match {
			$(
				$name(val) => $name(val + 1)
			)*
			_ => fail!()
		}
	);
)


// sadly this doesn't work due to limitations in concat_idents!
/*
macro_rules! call_bar(
	($var: ident, $val: expr, $a_bar:ident) => (
		$a_bar.concat_idents!(bar_,$var)($val)
	);
)
*/

fn main() {
	let a = call_foo!(alpha,0);
	let b = call_foo!(beta,10);

	println!("{:?}",a);
	println!("{:?}",b);

	//let abar = Bar{val: 123};
	//println!("{:?}",c);

	//println!("{:?}", test_puke!(A(0), puke_list!() ));
	println!("{:?}", puke_list!());
}