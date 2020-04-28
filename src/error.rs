pub use error_chain::bail;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        ParseFloatError(::std::num::ParseFloatError);
        ParseIntError(::std::num::ParseIntError);
    }

    links {
        // Invidious(invidious::Error, invidious::ErrorKind);
    }

    errors {
        // CefError(return_value: ::std::os::raw::c_int) {
        //     description("cef error")
        //     display("cef error {}", return_value)
        // }
    }
}
