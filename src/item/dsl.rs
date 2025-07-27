/// item_struct is used to build structures for items using the item syntax.
#[macro_export]
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
		pub struct $ident<'a> {
			$($member)*
		}
	};
	// This is the case responsible for handling fixed token values.
	($ident:ident { $($member:tt)* } $field:ident ($($_:literal)|+) $(as $binding:ident)?, $($entry:tt)*) => {
		item_struct!($ident { $($binding: <$field<'a> as Parseable<'a>>::Target,)?  $($member)* } $($entry)*);
	};
	// This is the case responsible for handling other items (and by extension tokens).
	($ident:ident { $($member:tt)* } $field:ident : $item:ident, $($entry:tt)*) => {
		item_struct!($ident { $($member)* } $field : $item<'a>, $($entry)*);
	};
	($ident:ident { $($member:tt)* } $field:ident : $item:ty, $($entry:tt)*) => {
		item_struct!($ident { $field: <$item as Parseable<'a>>::Target, $($member)* } $($entry)*);
	};
}

/// item_parse is used to build parsers for items using the item syntax.
#[macro_export]
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
	// This is the case responsible for handling other items (and by extension tokens).
	($stream:ident => $ident:ident { $($member:tt)* } $field:ident : $item:ident, $($entry:tt)*) => {
		item_parse!($stream => $ident { $($member)* } $field : $item<'a>, $($entry)*)
	};
	($stream:ident => $ident:ident { $($member:tt)* } $field:ident : $item:ty, $($entry:tt)*) => {
		<$item>::parse($stream).and_then(|result| {
			item_parse!($stream => $ident { $field: result.into(), $($member)* } $($entry)*)
		})
	};
}

/// item builds items, both their structures and their parsers.
#[macro_export]
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

				#[allow(unused_variables)]
				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					item_parse!(stream => $ident {} $($entry)*)
				}
			}
		)*
	};
}

/// item_choice_parse builds the option chain used to parse multiple possible
/// items.
#[macro_export]
macro_rules! item_choice_parse {
	($stream:ident =>) => {
		None
	};
	($stream:ident => $field:ident $(,)? $($rest:ident),*) => {
		$field::parse($stream).map(::std::convert::Into::into).or_else(|| {
			item_choice_parse!($stream => $($rest),*)
		})
	};
}

/// item_choice is used to build items that act as containers for a choice of
/// select items.
#[macro_export]
macro_rules! item_choice {
	($(
		$ident:ident {
			$($field:ident),* $(,)?
		}
	)*) => {
		$(
			#[derive(Clone, Eq, PartialEq)]
			pub enum $ident<'a> {
				$($field(<$field<'a> as Parseable<'a>>::Target)),*
			}

			impl<'a> Parseable<'a> for $ident<'a> {
				type Target = Self;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					item_choice_parse!(stream => $($field),*)
				}
			}

			#[cfg_attr(coverage, coverage(off))]
			impl ::std::fmt::Debug for $ident<'_> {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
					match self {
						$($ident::$field(x) => x.fmt(f)),*
					}
				}
			}

			$(
				impl<'a> From<<$field<'a> as Parseable<'a>>::Target> for $ident<'a> {
					fn from(value: <$field<'a> as Parseable<'a>>::Target) -> Self {
						Self::$field(value)
					}
				}
			)*
		)*
	};
}

/// The item_dsl macro is a combination of item and item_choice in one
/// convenient syntax.
#[macro_export]
macro_rules! item_dsl {
	($ident:ident {}) => {
		item!($ident {});
	};
	($ident:ident { $($field:ident)|* }) => {
		item_choice!($ident { $($field,)* });
	};
	($ident:ident { $($entry:tt)* }) => {
		item!($ident { $($entry)* });
	};
	($(
		$ident:ident {
			$($tt:tt)*
		}
	)*) => {
		$(
			item_dsl!($ident { $($tt)* });
		)*
	};
}
