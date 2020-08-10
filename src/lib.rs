// #![recursion_limit = "256"]
#[macro_use]
mod macros;
mod node;
pub mod prelude;

pub use crate::macros::*;
pub use crate::node::*;

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[derive(Default, Debug)]
	struct TestProps<T> {
		foo: T,
		bar: T,
	}

	#[test]
	fn macro_test() {
		rusx! {
			<TestProps foo=10, bar=20> {
				<foo=1>,
				<foo=100>,
			}
		};

		let n = rusx! {
			<TestProps> {
				<foo=1> {
					<bar=2020> {
						<bar=333>,
						<bar=1>
					}
				},
				<foo=100>,
			}
		};

		let g: RusxNode<TestProps<u8>> = rusx! {
			<TestProps> {
				<>,
				<> {
					<>,
					<>
				},
			}
		};

		println!("{:#?}", n);
	}

	#[test]
	fn bubble_down() {
		let mut n = rusx! {
			<TestProps foo=10, bar=20> {
				<foo=1>,
				<foo=100> {
					<bar=222>
				},
			}
		};

		n.bubble_down(|i: Option<usize>, s: &mut RusxNode<TestProps<u8>>| {
			println!("{:?} - foo is {}", i, s.props.foo);
		});
	}

	#[test]
	fn bubble_up() {
		let mut n = rusx! {
			<TestProps foo=10, bar=20> {
				<foo=1>,
				<foo=100>,
			}
		};

		n.bubble_up(|i: Option<usize>, s: &mut RusxNode<TestProps<u8>>| {
			println!("{:?} - foo is {}", i, s.props.foo);
		});
	}
}
