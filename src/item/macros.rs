macro_rules! item {
	(@expand {$name:tt}) => { $crate::Spanned<'a, item!(@expand $name)> };
	(@expand ($name:tt)) => { Box<item!(@expand $name)> };
	(@expand [$name:tt]) => { Vec<item!(@expand $name)> };
	(@expand $name:ident) => { $name<'a> };
	(@item $name:ident {}) => {
		#[derive(Default, Clone, Eq, PartialEq)]
		pub struct $name<'a> {
			_marker: ::std::marker::PhantomData<&'a ()>
		}

		#[cfg_attr(coverage, coverage(off))]
		impl ::std::fmt::Debug for $name<'_> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
				write!(f, stringify!($name))
			}
		}
	};
	(@item $name:ident { $($field:ident : $tt:tt),* $(,)? }) => {
		#[derive(Debug, Clone, Eq, PartialEq)]
		pub struct $name<'a> {
			$(pub $field: item!(@expand $tt)),*
		}
	};
	($(
		$name:ident {
			$($field:ident : $tt:tt),* $(,)?
		}
	)*) => {
		$(item!(@item $name { $($field : $tt),* });)*
	};
}

macro_rules! parse {
	(@expand $obj:ident { $($field:ident),* $(,)? } $stream:ident ($(,)?) => $body:block) => {
		Some({ $body })
	};
	(@expand $obj:ident { $(,)? } $stream:ident ($(,)?)) => {
		Some($obj::default())
	};
	(@expand $obj:ident { $($field:ident),+ $(,)? } $stream:ident ($(,)?)) => {
		Some($obj {
			$($field: $field.into()),+
		})
	};
	(@expand $obj:ident { $($field:ident),* $(,)? } $stream:ident ($ident:ident ($($value:literal)|+) $(as $name:ident)?, $($item:tt)*) $(=> $body:block)?) => {
		match $stream.next() {
			#[allow(unused_parens)]
			$($name @)? ($(Some($crate::Spanned { value: Token::$ident($ident($value)), .. }))|+) => {
				$(
					let $name = match $name {
						Some($crate::Spanned { span, value: Token::$ident(value) }) => span.wrap(value),
						_ => unreachable!(),
					};
				)?

				parse!(@expand $obj { $($field,)* $($name)? } $stream ($($item)*) $(=> $body)?)
			}
			_ => None
		}
	};
	(@expand $obj:ident { $($field:ident),* $(,)? } $stream:ident ($t:ty as $name:ident, $($item:tt)*) $(=> $body:block)?) => {
		if let Some($name) = <$t>::parse($stream) {
			parse!(@expand $obj { $($field,)* $name } $stream ($($item)*) $(=> $body)?)
		} else {
			None
		}
	};
	($($ident:ident { $($item:tt)+ } $(=> $body:block)? ;)+) => {
		$(
			impl<'a> Parseable<'a> for $ident<'a> {
				type Target = Self;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					stream.with(|stream| {
						parse!(@expand $ident {} stream ($($item)+,) $(=> $body)?)
					})
				}
			}
		)+
	};
}

macro_rules! generic {
	(@or $ident:ident $stream:ident) => {
		None
	};
	(@or $ident:ident $stream:ident $item:ident, $($rest:ident,)*) => {
		$item::parse($stream).map($ident::$item).or_else(|| {
			generic!(@or $ident $stream $($rest,)*)
		})
	};
	($(
		$ident:ident {
			$($name:ident),+ $(,)?
		}
	)+) => {
		$(
			impl<'a> Parseable<'a> for $ident<'a> {
				type Target = Self;

				fn parse(stream: &mut TokenStream<'a>) -> Option<Self::Target> {
					generic!(@or $ident stream $($name,)+)
				}
			}
		)+

		$(
			#[derive(Clone, Eq, PartialEq)]
			pub enum $ident<'a> {
				$($name(<$name<'a> as Parseable<'a>>::Target)),*
			}

			#[cfg_attr(coverage, coverage(off))]
			impl ::std::fmt::Debug for $ident<'_> {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
					match self {
						$($ident::$name(x) => x.fmt(f)),+
					}
				}
			}

			$(
				impl<'a> From<<$name<'a> as Parseable<'a>>::Target> for $ident<'a> {
					fn from(value: <$name<'a> as Parseable<'a>>::Target) -> Self {
						Self::$name(value)
					}
				}
			)*
		)+
	};
}

pub(super) use generic;
pub(super) use item;
pub(super) use parse;
