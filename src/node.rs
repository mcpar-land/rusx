/// The base node struct that contains props, and a Vec of child nodes.
#[derive(Debug)]
pub struct RusxNode<T: Default> {
	/// The node's props. The only required traits to implement are `Debug` and `Default`
	pub props: T,
	/// A vector of child nodes. Can be empty.
	pub children: Vec<RusxNode<T>>,
}

impl<T: Default + std::fmt::Debug> RusxNode<T> {
	///
	pub fn new(props: T) -> Self {
		Self {
			props: props,
			children: vec![],
		}
	}
	/// Run a function on a node, and on all child nodes. Runs on itself first, *then* on child nodes, meaning the function will run on the topmost node first.
	///
	/// ```
	/// #[derive(Default)]
	/// struct Foo {
	/// 	bar: u8
	/// }
	///
	/// let mut tree = rusx! {
	/// 	<Foo bar=10> {
	/// 		<bar=20>,
	/// 		<bar=30>,
	/// 	}
	/// }
	///
	/// tree.bubble_down(|i: Option<usize>, s: &mut<RusxNode<Foo>| {
	/// 	s.props.bar += 1;
	/// 	println!("{}", s.props.bar);
	/// })
	///
	/// // 11
	/// // 21
	/// // 31
	/// ```
	pub fn bubble_down<F>(&mut self, func: F)
	where
		F: Fn(Option<usize>, &mut Self),
	{
		(func)(None, self);
		for (i, child) in &mut self.children.iter_mut().enumerate() {
			child._bubble_down(i, &func);
		}
	}
	fn _bubble_down<F>(&mut self, i: usize, func: &F)
	where
		F: Fn(Option<usize>, &mut Self),
	{
		(func)(Some(i), self);
		for (i, child) in &mut self.children.iter_mut().enumerate() {
			child._bubble_down(i, func);
		}
	}

	/// Run a function on a node, and on all child nodes. Runs on child nodes first, *then* on itself, meaning the function will run on the topmost node last.
	///
	/// ```
	/// #[derive(Default)]
	/// struct Foo {
	/// 	bar: u8
	/// }
	///
	/// let mut tree = rusx! {
	/// 	<Foo bar=10> {
	/// 		<bar=20>,
	/// 		<bar=30>,
	/// 	}
	/// }
	///
	/// tree.bubble_up(|i: Option<usize>, s: &mut<RusxNode<Foo>| {
	/// 	s.props.bar += 1;
	/// 	println!("{}", s.props.bar);
	/// })
	///
	/// // 21
	/// // 31
	/// // 11
	/// ```
	pub fn bubble_up<F>(&mut self, func: F)
	where
		F: Fn(Option<usize>, &mut Self),
	{
		for (i, child) in &mut self.children.iter_mut().enumerate() {
			child._bubble_up(i, &func);
		}
		(func)(None, self);
	}
	fn _bubble_up<F>(&mut self, i: usize, func: &F)
	where
		F: Fn(Option<usize>, &mut Self),
	{
		for (i, child) in &mut self.children.iter_mut().enumerate() {
			child._bubble_up(i, func);
		}
		(func)(Some(i), self);
	}
}
