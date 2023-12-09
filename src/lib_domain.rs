pub mod core {
    pub mod language {
        mod domain;
        pub use domain::*;
    }
    pub mod shared {
        mod domain;
        pub use domain::*;
    }
    pub mod tag {
        mod domain;
        pub use domain::*;
    }
    pub mod user {
        mod domain;
        pub use domain::*;
    }
}
