#[macro_export]
macro_rules! group {
	($(
		$(#[$attr:meta])*
		$vis:vis enum $parent:ident {
			$($name:ident),* $(,)?
		}
	)*) => {
		$(
			$(#[$attr])*
			$vis enum $parent<'a> {
				$($name($name<'a>)),*
			}

			#[cfg_attr(coverage, coverage(off))]
			impl ::std::fmt::Debug for $parent<'_> {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
					match self {
						$($parent::$name(x) => x.fmt(f)),+
					}
				}
			}

			$(
				impl<'a> From<$name<'a>> for $parent<'a> {
					fn from(value: $name<'a>) -> Self {
						Self::$name(value)
					}
				}
			)*
		)*
	};
}
