// *************************************************************************
//
// Copyright (c) 2025 Andrei Gramakov. All rights reserved.
//
// This file is licensed under the terms of the MIT license.
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://agramakov.me
// e-mail:  mail@agramakov.me
//
// *************************************************************************
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

pub type Shared<T> = Arc<Mutex<T>>;

/// Convenience trait to add `into_shared()` to any type
pub trait IntoShared<T> {
    fn into_shared(self) -> Shared<T>;
}

impl<T> IntoShared<T> for T {
    fn into_shared(self) -> Shared<T> {
        Arc::new(Mutex::new(self))
    }
}

pub trait WithSharedInner<T> {
    /// Waits for the lock to become available and then calls the closure
    fn with_inner<F, R>(&self, func: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R;

    /// Immediately locks the mutex and calls the closure
    fn try_with_inner<F, R>(&self, func: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R;
}

impl<T> WithSharedInner<T> for Shared<T> {
    fn with_inner<F, R>(&self, func: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.lock().ok().map(|mut guard| func(&mut *guard))
    }

    fn try_with_inner<F, R>(&self, func: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.try_lock().ok().map(|mut guard| func(&mut *guard))
    }
}
