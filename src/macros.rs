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
			let mut t = rusx!(<$type $($key=$val)*> {});
			$(rusx!(@rec &mut t, $type $($children)*);)?
			$parent.children.push(t);
		)*
	}};
}
