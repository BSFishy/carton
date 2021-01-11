//! TODO: document this

use std::any;
use std::borrow;
use std::fmt;
use std::hash;
use std::ops;

use crate::observer::Observer;

/// TODO: document this
pub struct Observable<T> {
    value: T,
    observers: Vec<Observer<T>>,
}

impl <T> Observable<T> {
    /// TODO: document this
    pub fn new(value: T) -> Self {
        Self {
            value,
            observers: Vec::new(),
        }
    }

    /// TODO: document this
    pub fn with_observers(value: T, observers: Vec<Observer<T>>) -> Self {
        Self {
            value,
            observers,
        }
    }
}

impl <T> Observable<T> {
    /// TODO: document this
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// TODO: document this
    pub fn set_value(&mut self, value: T) {
        // TODO: call will set on observers
        self.value = value;
        // TODO: call did set on observers
    }

    /// TODO: document this
    pub fn add_observer(&mut self, observer: Observer<T>) {
        self.observers.push(observer)
    }

    /// TODO: document this
    pub fn remove_observer(&mut self, needle: &Observer<T>) where Observer<T>: PartialEq {
        self.observers.retain(|observer| observer != needle);
    }
}

// impl <T> AsMut<T> for Observable<T> {
//     fn as_mut(&mut self) -> &mut T {
//         &mut self.value
//     }
// }

impl <T> AsRef<T> for Observable<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl <T> borrow::Borrow<T> for Observable<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

// impl <T> borrow::BorrowMut<T> for Observable<T> {
//     fn borrow_mut(&mut self) -> &mut T {
//         &mut self.value
//     }
// }

impl <T> Clone for Observable<T> where T: Clone {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            observers: self.observers.clone(),
        }
    }
}

impl <T> fmt::Debug for Observable<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&*format!("Observable<{}>", any::type_name::<T>()))
            .field("value", &self.value)
            .field("observers", &self.observers)
            .finish()
    }
}

impl <T> Default for Observable<T> where T: Default {
    fn default() -> Self {
        Self {
            value: T::default(),
            observers: Vec::new(),
        }
    }
}

impl <T> ops::Deref for Observable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// impl <T> ops::DerefMut for Observable<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.value
//     }
// }

impl <T> fmt::Display for Observable<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl <T> From<T> for Observable<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl <T> hash::Hash for Observable<T> where T: hash::Hash {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.observers.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let o = Observable::new(0i32);
        println!("num: {}, dbg: {:?}", o, o);

        assert_eq!(2 + 2, 4);
    }
}
