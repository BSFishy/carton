//! TODO: document this

/// TODO: document this
#[macro_export]
macro_rules! config_use {
    ($visibility:vis $config:ident $(as $name:ident)? $(, $value:expr => $path:tt)*) => {
        $(
            #[cfg($config = $value)]
            $visibility use $path $(as $name)?;
        )*
    }
}

/// TODO: document this
#[macro_export]
macro_rules! config_mod {
    ($visibility:vis $config:ident $(, $value:expr => $path:tt)*) => {
        $(
            #[cfg($config = $value)]
            $visibility mod $path;
        )*
    }
}

/// TODO: document this
#[macro_export]
macro_rules! config_type {
    ($visibility:vis $config:ident for $name:ident $(, $value:expr => $type:ty)*) => {
        $(
            #[cfg($config = $value)]
            $visibility type $name = $type;
        )*

        #[cfg(not(any($($config = $value),*)))]
        $visibility type $name = ();
    };
    (#[$meta:meta] $visibility:vis $config:ident for $name:ident $(, $value:expr => $type:ty)*) => {
        $(
            #[$meta]
            #[cfg($config = $value)]
            $visibility type $name = $type;
        )*

        #[$meta]
        #[cfg(not(any($($config = $value),*)))]
        $visibility type $name = ();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
