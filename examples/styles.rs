#[macro_use]
extern crate rusx;

fn main() {
	use rusx::prelude::*;
	#[derive(Debug)]
	struct Styled<'a> {
		bt: BlockType,
		body: &'a str,
		font_family: &'a str,
		font_size: f32,
		font_weight: f32,
		color: &'a str,
		italic: bool,
		underline: bool,
		margin: Spacing,
		padding: Spacing,
	}

	#[derive(Debug, Default)]
	struct Spacing(f32, f32, f32, f32);

	#[derive(Debug)]
	enum BlockType {
		Header1,
		Header2,
		Header3,
		Header4,
		Header5,
		Header6,
		Body,
		Quote,
		Code,
		Numbered,
		Bullet,
		ListItem,
	}
	use BlockType::*;

	impl Default for Styled<'_> {
		fn default() -> Self {
			Self {
				bt: Body,
				body: "",
				font_family: "serif",
				font_size: 12.0,
				font_weight: 400.0,
				color: "black",
				italic: false,
				underline: false,
				margin: Spacing::default(),
				padding: Spacing::default(),
			}
		}
	}

	let styles = rusx! {
		<Styled> {
			<bt=Header1, body="This is my header!", color="pink">,
			<body="This is a body section of text", italic=true> {
				<bt=Quote, body="This is a quote inside the body.">
			},
			<body="This is another body section of text.">,
			<bt=Header2, body="About Me", color="blue">,
			<body="Here are some things about me:", margin={Spacing(10.0, 0.0, 0.0, 0.0)}>,
			<bt=Numbered> {
				<bt=ListItem, body="I am the first list item.">,
				<bt=ListItem, body="I am the second list item.">,
				<bt=ListItem, body="I am the third list item.">
			}
		}
	};
	println!("{:#?}", styles);
}
