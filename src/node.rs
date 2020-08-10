#[derive(Debug)]
pub struct RusxNode<T: Default> {
	pub props: T,
	pub children: Vec<RusxNode<T>>,
}

impl<T: Default + std::fmt::Debug> RusxNode<T> {
	pub fn new(props: T) -> Self {
		Self {
			props: props,
			children: vec![],
		}
	}
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
