pub mod core {
    pub mod language {
        mod application;
        pub use application::*;
    }
    pub mod shared {
        mod application;
        pub use application::*;
    }
    pub mod tag {
        mod application;
        pub use application::*;
    }
    pub mod user {
        mod application;
        pub use application::*;
    }
}
