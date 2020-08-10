//! `rusx` is an ergonomic synatx for creating a parent-child tree of nodes, inspired by JSX.
//!
//! ```rust
//! use rusx::prelude::*;
//!
//! #[derive(Default)]
//! struct Foo {
//! 	bar: u32,
//! 	orb: bool
//! }
//!
//! let tree = rusx! {
//! 	<Foo bar=10, orb=true {
//! 		<orb=false, bar=230>,
//! 		<bar=100>
//! 	}
//! }
//!
//! println!("{:?}", tree.children);
//! ```
//! By using a JSX-esque syntax, you can quickly construct nested trees of data.
//!
//! ```
//! # use rusx::prelude::*;
//! #
//! # #[derive(Default)]
//! # struct Foo {
//! # 	bar: u32,
//! # 	orb: bool
//! # }
//! #
//! let tree = rusx! {
//! 	<Foo bar=10, orb=false> {
//! 		<orb=true> {
//! 			<bar=1>,
//! 			<bar=2300>,
//! 			<bar=88>,
//! 			<bar=19234>,
//! 			<bar=3, orb=true>,
//! 			<> { <>, <bar=2> },
//! 			<bar=1>,
//! 			<orb=true>
//! 		},
//! 		<bar=2203, orb=true> {
//! 			<orb=false, bar=23>,
//! 			<orb=true, bar=393>,
//! 			<orb=false, bar=3393>,
//! 		}
//! 	}
//! }
//! ```
//!
//! Generic types can be inferred.
//! ```
//! #[derive(Default)]
//! struct Store<T> {
//! 	first: T,
//! 	second: bool
//! }
//!
//! let tree = rusx! {
//! 	<Store first=(Some(10), "hello there"), second=false> {
//! 		<first=(None, "hi"), second=true>
//! 	}
//! }
//!
//! let other_tree = rusx! {
//! 	<Story first=63.2, second=false> {
//! 		<first=220.0>,
//! 		<first=-1.0>
//! 	}
//! }
//! ```
//!
//! Props can be built from code blocks.
//! ```
//! #[derive(Default)]
//! struct Point {
//! 	x: f32,
//! 	y: f32
//! }
//!
//! const base_x: f32 = 100.0;
//! const base_y: f32 = 32.0;
//!
//! let tree = rusx! {
//! 	<Point x={base_x + 323.0} y={base_y-10.0}> {
//! 		<y={base_y * -3.0}>
//! 	}
//! }
//! ```

#[macro_use]
#[warn(missing_docs)]
mod macros;
#[warn(missing_docs)]
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

		let n: RusxNode<TestProps<_>> = rusx! {
			<TestProps> {
				<foo=1> {
					<bar=20> {
						<bar=33>,
						<bar={
							let mut x = 0;
							x += 2 * 5;
							x
						}>
					}
				},
				<foo=100>,
			}
		};

		#[allow(unused_variables)]
		let g = rusx! {
			<TestProps foo="no", bar="sure"> {
				<foo="awful", bar="terrible">,
				<> {
					<>,
					<>
				},
			}
		};

		let h = rusx! {
			<TestProps foo=(Some(1.0), Some(2.0))> {
				<foo=(Some(1.0), Some(2.0))>,
				<> {
					<>,
					<>
				},
			}
		};

		println!("{:#?}", n);
		println!("{:#?}", h);
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
			s.props.foo += 1;
			println!("{:?} - foo is {}", i, s.props.foo);
		});
	}
}
