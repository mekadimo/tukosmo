pub mod core {
    pub mod language {
        mod infrastructure;
        pub use infrastructure::*;
    }
    pub mod shared {
        mod infrastructure;
        pub use infrastructure::*;
    }
    pub mod tag {
        mod infrastructure;
        pub use infrastructure::*;
    }
    pub mod user {
        mod infrastructure;
        pub use infrastructure::*;
    }
}

#[cfg(feature = "ssr")]
use crate::core::shared::leptos_actix_server::service::server::start_server;

#[cfg(feature = "ssr")]
pub async fn run_server() {
    let end_result = start_server().await;
    match end_result {
        Ok(_) => { println!("Server thread finished.") }
        Err(e) => {
            panic!("SERVER ERROR: {:?}", e);
        }
    }
}
