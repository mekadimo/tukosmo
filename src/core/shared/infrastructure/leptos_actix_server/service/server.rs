use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::SessionMiddleware;
use actix_session::config::CookieContentSecurity;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::cookie;
use actix_web::dev::ServerHandle;
use actix_web::dev::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::dev::Transform;
use actix_web::dev::forward_ready;
use actix_web::http;
use actix_web::web;
use chrono::Utc;
use core::future::Future;
use futures::FutureExt;
use futures::future::Either;
use futures::future::Ready;
use futures::future::ok;
use leptos::get_configuration;
use leptos::view;
use leptos_actix::LeptosRoutes;
use leptos_actix::generate_route_list;
use rand::Rng;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tokio;
use tukosmo_domain::core::shared::model::DomainError;
use tukosmo_domain::core::shared::repository::DataRepository;

use crate::core::shared::leptos_actix_server::repository::FsDataRepository;
use crate::core::shared::leptos_actix_server::service::tls;
use crate::core::shared::leptos_ui::App;

/*
 * TODO: Manage errors with DomainError instead of .unwrap(), .except(), etc.
 */

pub struct AcmeChallengeServer {
    pub thread: tokio::task::JoinHandle<Result<(), std::io::Error>>,
    pub handle: ServerHandle,
}

#[derive(Clone)]
pub struct Handle(pub Arc<Mutex<Option<ServerHandle>>>);

#[derive(Default, Clone)]
pub struct RedirectHTTPS {
    replacements: Vec<(String, String)>,
}

pub struct RedirectHTTPSService<S> {
    service: S,
    replacements: Vec<(String, String)>,
}

pub struct TukosmoServer {
    pub thread: tokio::task::JoinHandle<Result<(), std::io::Error>>,
    pub child_threads: Vec<tokio::task::JoinHandle<()>>,
    pub handle: Handle,
}

const COOKIE_DURATION_IN_DAYS: i64 = 7;

impl Handle {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    pub fn replace(&self, handle: ServerHandle) -> Option<ServerHandle> {
        self.0.lock().unwrap().replace(handle)
    }

    pub async fn stop(&self, graceful: bool) -> impl Future<Output = ()> {
        let o = self.0.lock().unwrap().take();
        match o {
            Some(server_handle) => { server_handle.stop(graceful) }
            None => {
                panic!("None in Handle::stop()");
            }
        }
    }
}

impl RedirectHTTPS {
    pub fn with_replacements(replacements: &[(String, String)]) -> Self {
        RedirectHTTPS {
            replacements: replacements.to_vec(),
        }
    }
}

impl<S> Service<ServiceRequest>
    for RedirectHTTPSService<S>
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = actix_web::Error
        >,
        S::Future: 'static
{
    type Response = ServiceResponse;
    type Error = actix_web::Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.connection_info().scheme() == "https" {
            Either::Left(self.service.call(req))
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();

            let mut url = format!("https://{}{}", host, uri);
            for (s1, s2) in self.replacements.iter() {
                url = url.replace(s1, s2);
            }

            Either::Right(
                ok(
                    req
                        .into_response(
                            HttpResponse::MovedPermanently()
                                .append_header((http::header::LOCATION, url))
                                .finish()
                        )
                        .map_into_boxed_body()
                )
            )
        }
    }
}

pub fn start_acme_challenge_server(
    domain: &str
) -> Result<AcmeChallengeServer, DomainError> {
    let fs_data_repository = FsDataRepository::init()?;
    let acme_challenge_dir_path = fs_data_repository.acme_challenge_dir_path;

    // For HTTP, the challenge is a text file that needs to be accessible
    // over the web for the domain we are trying to get a certificate for:
    // http://mydomain.io/.well-known/acme-challenge/<token>
    // (the token is the filename and the proof is the content of the file)
    let server = HttpServer::new(move || {
        actix_web::App
            ::new()
            .service(
                Files::new(
                    "/.well-known/acme-challenge",
                    &acme_challenge_dir_path
                ).show_files_listing()
            )
    })
        .bind((domain, 80))
        .unwrap()
        .shutdown_timeout(0)
        .run();

    let handle = server.handle();
    let server_thread = tokio::spawn(server);

    return Ok(AcmeChallengeServer {
        thread: server_thread,
        handle,
    });
}

pub async fn start_server() -> Result<(), DomainError> {
    // TODO: Add option in TAP to reset cookies private key.
    // TODO: Add option in TAP to reset all sessions stored in PostgreSQL.
    let cookie_pkey: actix_web::cookie::Key = actix_web::cookie::Key::derive_from(
        &rand::thread_rng().gen::<[u8; 32]>()
    );

    loop {
        let server = start_server_thread(cookie_pkey.clone()).await?;
        println!("SERVER ON");

        match server.thread.await {
            Ok(_) => {
                for child_thread in server.child_threads {
                    child_thread.abort();
                }

                if let None = server.handle.0.lock().unwrap().take() {
                    println!("Restarting server...");
                    continue;
                } else {
                    break;
                }
            }
            Err(e) => {
                panic!("SERVER ERROR: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn start_server_thread(
    cookie_pkey: cookie::Key
) -> Result<TukosmoServer, DomainError> {
    let handle = Handle::new();

    let startup_code = format!("{:x}", Utc::now().timestamp());

    let server_config = FsDataRepository::init()?.get_server_config()?;

    let production_mode_is_enabled = server_config.has_production_mode();

    let rustls_config = tls::load_rustls_config().await?;

    let handle_server = web::Data::new(handle.clone());

    let conf_file = get_configuration(None).await.unwrap();
    let http_socket_addr = conf_file.leptos_options.site_addr;
    let https_socket_addr = SocketAddr::new(
        http_socket_addr.ip(),
        server_config.https_port
    );
    let routes = generate_route_list(|| view! { <App/> });

    let server = HttpServer::new(move || {
        let leptos_options = &conf_file.leptos_options;
        let site_root = &leptos_options.site_root;

        actix_web::App
            ::new()
            .app_data(web::Data::new(startup_code.clone()))
            .app_data(web::Data::new(server_config.clone()))
            .app_data(web::Data::clone(&handle_server))
            // wrap() and wrap_fn() execution order goes from last to first
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    cookie_pkey.clone()
                )
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(
                            cookie::time::Duration::days(
                                COOKIE_DURATION_IN_DAYS
                            )
                        )
                    )
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_secure(true)
                    .build()
            )
            .wrap_fn(|service_request, service| {
                service.call(service_request).map(|result| {
                    let mut service_response = result.unwrap();
                    service_response
                        .headers_mut()
                        .insert(
                            http::header::STRICT_TRANSPORT_SECURITY,
                            http::header::HeaderValue::from_static(
                                tls::HSTS_HEADER_VALUE
                            )
                        );
                    Ok(service_response)
                })
            })
            .wrap(
                // TODO: Use official Actix solution when they have it.
                RedirectHTTPS::with_replacements(
                    &[
                        (
                            server_config.http_port
                                .clone()
                                .to_string()
                                .to_owned(),
                            server_config.https_port
                                .clone()
                                .to_string()
                                .to_owned(),
                        ),
                    ]
                )
            )
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .service(favicon)
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/> }
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
        .bind(&http_socket_addr)
        .unwrap()
        .bind_rustls(&https_socket_addr, rustls_config)
        .unwrap()
        .run();

    handle.replace(server.handle());

    let server_thread = tokio::spawn(server);

    let mut child_threads: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    if production_mode_is_enabled {
        let renewal_thread = tls::spawn_renewal_thread(handle.clone())?;
        child_threads.push(renewal_thread);
    }

    Ok(TukosmoServer {
        thread: server_thread,
        child_threads,
        handle,
    })
}

impl<S> Transform<S, ServiceRequest>
    for RedirectHTTPS
    where
        S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = actix_web::Error
        >,
        S::Future: 'static
{
    type Response = ServiceResponse;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = RedirectHTTPSService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectHTTPSService {
            service,
            replacements: self.replacements.clone(),
        })
    }
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}
