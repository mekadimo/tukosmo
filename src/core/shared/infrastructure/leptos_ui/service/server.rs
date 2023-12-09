use leptos::ServerFnError;

use tukosmo_domain::core::shared::model::ServerResponse;
use tukosmo_domain::core::shared::model::ServerResponseError;

pub fn manage_response<T>(
    server_response: Result<ServerResponse<T>, ServerFnError>,
    success_callback: impl Fn(T),
    error_callback: impl Fn(ServerResponseError)
) {
    // TODO: Avoid using unwrap() and map ServerFnError to many DomainErrors
    match server_response.unwrap() {
        ServerResponse::Response(response) => {
            success_callback(response);
        }
        ServerResponse::Error(server_error) => {
            error_callback(server_error);
        }
    }
}
