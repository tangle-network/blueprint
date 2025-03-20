//! Rejection response types.

use crate::__define_rejection as define_rejection;

define_rejection! {
    #[body = "Request body didn't contain valid UTF-8"]
    /// Rejection type used when buffering a [`JobCall`] into a [`String`] if the
    /// body doesn't contain valid UTF-8.
    ///
    /// [`JobCall`]: crate::JobCall
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    pub struct InvalidUtf8(Error);
}
