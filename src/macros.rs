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

#[macro_export]
macro_rules! newtype {
	($(
		$(#[$meta:meta])*
		$vis:vis type $ident:ident $(<$($gen:tt),*>)? = $t:ty $(where $($where:tt)*)?;
	)+) => {
		$(
			$(#[$meta])*
			$vis struct $ident $(<$($gen),*>)? ($t);

			#[cfg_attr(coverage, coverage(off))]
			impl $(<$($gen),*>)? ::std::ops::Deref for $ident $(<$($gen),*>)?
			$(where $($where)*)? {
				type Target = $t;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			#[cfg_attr(coverage, coverage(off))]
			impl $(<$($gen),*>)? ::std::ops::DerefMut for $ident $(<$($gen),*>)?
			$(where $($where)*)? {
				fn deref_mut(&mut self) -> &mut Self::Target {
					&mut self.0
				}
			}

			#[cfg_attr(coverage, coverage(off))]
			impl $(<$($gen),*>)? From<$t> for $ident $(<$($gen),*>)?
			$(where $($where)*)? {
				fn from(value: $t) -> Self {
					Self(value)
				}
			}

			#[cfg_attr(coverage, coverage(off))]
			impl $(<$($gen),*>)? From<$ident $(<$($gen),*>)?> for $t
			$(where $($where)*)? {
				fn from(value: $ident $(<$($gen),*>)?) -> Self {
					value.0
				}
			}
		)+
	};
}
