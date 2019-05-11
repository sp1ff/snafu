// This test asserts that errors can be used across threads.
extern crate snafu;

use std::thread;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum InnerError {
    Boom,
}

#[derive(Debug, Snafu)]
enum Error {
    Leaf {
        name: String,
    },

    Wrapper {
        source: InnerError,
    },

    BoxedWrapper {
        source: Box<InnerError>,
    },

    BoxedTraitObjectSend {
        source: Box<dyn std::error::Error + Send + 'static>,
    },

    BoxedTraitObjectSendSync {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

fn example() -> Result<(), Error> {
    Boom.fail().eager_context(Wrapper)
}

#[test]
fn implements_thread_safe_error() {
    fn check_error<E: std::error::Error>() {}
    check_error::<InnerError>();
    check_error::<Error>();

    fn check_send<E: Send>() {}
    check_send::<InnerError>();
    check_send::<Error>();

    let t = thread::spawn(move || example());

    let v = t.join().expect("Thread panicked");
    v.unwrap_err();
}