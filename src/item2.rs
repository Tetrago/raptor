use crate::item::Expression;
use crate::lexer::Literal;
use crate::lexer::Parseable;
use crate::lexer::Token;
use crate::lexer::TokenStream;

/// item_struct is used to build structures for items using the item syntax.
macro_rules! item_struct {
	($ident:ident {}) => {
		#[derive(Default, Clone, Eq, PartialEq)]
		struct $ident<'a> {
			_marker: ::std::marker::PhantomData<&'a ()>
		}

		#[cfg_attr(coverage, coverage(off))]
		impl ::std::fmt::Debug for $ident<'_> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
				write!(f, stringify!($name))
			}
		}
	};
	($ident:ident { $($member:tt)* }) => {
		#[derive(Debug, Clone, Eq, PartialEq)]
		struct $ident<'a> {
			$($member)*
		}
	};
	// This is the case responsible for handling fixed token values.
	($ident:ident { $($member:tt)* } $field:ident ($($_:literal)|+) $(as $binding:ident)?, $($entry:tt)*) => {
		item_struct!($ident { $($binding: <$field<'a> as Parseable<'a>>::Target,)?  $($member)* } $($entry)*);
	};
	// These @field cases are used to deal with special cases of item types.
	(@field $item:ident) => {
		<$item<'a> as Parseable<'a>>::Target
	};
	// Such as boxes, which are used to prevent enum recursion.
	(@field [$item:tt]) => {
		Box::<item_struct!(@field $item)>
	};
	// This is the case responsible for handling other items (and by extension tokens).
	($ident:ident { $($member:tt)* } $field:ident : $item:tt, $($entry:tt)*) => {
		item_struct!($ident { $field: item_struct!(@field $item), $($member)* } $($entry)*);
	};
}

/// item_parse is used to build parsers for items using the item syntax.
macro_rules! item_parse {
	($stream:ident => $ident:ident {}) => {
		Some($ident::default())
	};
	($stream:ident => $ident:ident { $($member:tt)* }) => {
		Some($ident {
			$($member)*
		})
	};
	// This is the case responsible for handling fixed token values.
	($stream:ident => $ident:ident { $($member:tt)* } $func:ident ($($value:literal)|+) $(as $binding:ident)?, $($entry:tt)*) => {
		#[allow(unused_parens)]
		match $stream.next() {
			$($binding @)? ($(Some($crate::Spanned { value: Token::$func($func($value)), .. }))|+) => {
				$(
					let $binding = match $binding {
						Some($crate::Spanned { span, value: Token::$func(value) }) => span.wrap(value),
						_ => unreachable!(),
					};
				)?

				item_parse!($stream => $ident { $($binding,)? $($member)* } $($entry)*)
			}
			_ => None
		}
	};
	// These @field cases are used to deal with special cases of item types.
	(@field $item:ident) => {
		$item<'a>
	};
	// In this macro, they usually just unwrap the type.
	(@field [$item:tt]) => {
		item_parse!(@field $item)
	};
	// This is the case responsible for handling other items (and by extension tokens).
	($stream:ident => $ident:ident { $($member:tt)* } $field:ident : $item:tt, $($entry:tt)*) => {
		if let Some(result) = <item_parse!(@field $item)>::parse($stream) {
			item_parse!($stream => $ident { $field: result.into(), $($member)* } $($entry)*)
		} else {
			None
		}
	};
}

/// Builds items, both their structures and their parsers.
macro_rules! item {
	($(
		$ident:ident {
			$($entry:tt)*
		}
	)*) => {
		$(
			item_struct!($ident {} $($entry)*);

			impl<'a> Parseable<'a> for $ident<'a> {
				type Target = Self;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					item_parse!(stream => $ident {} $($entry)*)
				}
			}
		)*
	};
}

item! {
	Group {
		Literal("("),
		expr: [Expression],
		Literal(")"),
	}
}
