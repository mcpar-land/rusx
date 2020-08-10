/// `rusx` is an ergonomic synatx for creating a parent-child tree of nodes, inspired by JSX.
///
/// ```rust
/// use rusx::prelude::*;
///
/// #[derive(Default)]
/// struct Foo {
/// 	bar: u32,
/// 	orb: bool
/// }
///
/// let tree = rusx! {
/// 	<Foo bar=10, orb=true {
/// 		<orb=false, bar=230>,
/// 		<bar=100>
/// 	}
/// }
///
/// println!("{:?}", tree.children);
/// ```
/// By using a JSX-esque syntax, you can quickly construct nested trees of data.
///
/// ```
/// # use rusx::prelude::*;
/// #
/// # #[derive(Default)]
/// # struct Foo {
/// # 	bar: u32,
/// # 	orb: bool
/// # }
/// #
/// let tree = rusx! {
/// 	<Foo bar=10, orb=false> {
/// 		<orb=true> {
/// 			<bar=1>,
/// 			<bar=2300>,
/// 			<bar=88>,
/// 			<bar=19234>,
/// 			<bar=3, orb=true>,
/// 			<> { <>, <bar=2> },
/// 			<bar=1>,
/// 			<orb=true>
/// 		},
/// 		<bar=2203, orb=true> {
/// 			<orb=false, bar=23>,
/// 			<orb=true, bar=393>,
/// 			<orb=false, bar=3393>,
/// 		}
/// 	}
/// }
/// ```
///
/// Generic types can be inferred.
/// ```
/// #[derive(Default)]
/// struct Store<T> {
/// 	first: T,
/// 	second: bool
/// }
///
/// let tree = rusx! {
/// 	<Store first=(Some(10), "hello there"), second=false> {
/// 		<first=(None, "hi"), second=true>
/// 	}
/// }
///
/// let other_tree = rusx! {
/// 	<Story first=63.2, second=false> {
/// 		<first=220.0>,
/// 		<first=-1.0>
/// 	}
/// }
/// ```
///
/// Props can be built from code blocks.
/// ```
/// #[derive(Default)]
/// struct Point {
/// 	x: f32,
/// 	y: f32
/// }
///
/// const base_x: f32 = 100.0;
/// const base_y: f32 = 32.0;
///
/// let tree = rusx! {
/// 	<Point x={base_x + 323.0} y={base_y-10.0}> {
/// 		<y={base_y * -3.0}>
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! rusx {
	(<$type:ident $($key:ident=$val:tt),*> $({$($children:tt)*})?) => {{
		#[allow(unused_mut)]
		let mut n = RusxNode {
			props: $type {
				$(
					$key: $val,
				)*
				..$type::default()
			},
			children: Vec::new()
		};
		$(rusx!(@rec &mut n, $type $($children)*);)?
		n
	}};

	(@rec $parent:expr, $type:ident $( <$($key:ident=$val:tt),*> $({$($children:tt)*})?),* $(,)*) => {{
	$(
			#[allow(unused_mut)]
			let mut t = rusx!(<$type $($key=$val),*> {});
			$(rusx!(@rec &mut t, $type $($children)*);)?
			$parent.children.push(t);
		)*
	}};
}
