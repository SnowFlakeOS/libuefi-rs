//! Some code was borrowed from [Tifflin](https://github.com/thepowersgang/rust_os)

macro_rules! efi_fcn {
	(fn $name:ident ( $($n:ident: $t:ty),* ) -> $rv:ty) => {
		extern "win64" fn $name( $($n: $t),* ) -> $rv
	};
	(fn ( $($n:ident: $t:ty),* ) -> $rv:ty) => {
		unsafe extern "win64" fn( $($n: $t),* ) -> $rv
	};
	(fn ( $($t:ty),* ) -> $rv:ty) => {
		unsafe extern "win64" fn( $($t),* ) -> $rv
	};
}
