use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{
    client::{
        create as create_client, delete as delete_client, get_all as get_all_clients,
        get_one as get_one_client, update as update_client,
    },
    collection::{
        create as create_collection, delete as delete_collection, get_all as get_all_collections,
        get_one as get_one_collection, update as update_collection,
    },
    file::{
        delete as delete_file, get_all_for_request, get_one as get_one_file, upload as upload_file,
    },
    firm::{
        create as create_firm, delete as delete_firm, get_all as get_all_firms,
        get_one as get_one_firm, update as update_firm,
    },
    request::{
        create as create_request, delete as delete_request, get_all as get_all_requests,
        get_one as get_one_request, update as update_request,
    },
    user::{
        create as create_user, delete as delete_user, get_all as get_all_users,
        get_one as get_one_user, login, update as update_user,
    },
};

use crate::app_state::AppState;
use crate::auth::auth_middleware;

pub fn router(app_state: AppState) -> Router {
    // Public routes for users (register, login)
    let public_users_router = Router::new()
        .route("/register", post(create_user)) // Register
        .route("/login", post(login)) // Login
        .with_state(app_state.clone());

    // Protected routes for users (get, update, delete)
    let protected_users_router = Router::new()
        .route("/", get(get_all_users))
        .route(
            "/:id",
            get(get_one_user).patch(update_user).delete(delete_user),
        )
        .with_state(app_state.clone());

    // All other routers (clients, firms, files, requests, collections) are assumed to be fully protected
    let clients_router = Router::new()
        .route("/", post(create_client).get(get_all_clients))
        .route(
            "/:id",
            get(get_one_client)
                .patch(update_client)
                .delete(delete_client),
        );

    let firms_router = Router::new()
        .route("/", post(create_firm).get(get_all_firms))
        .route(
            "/:id",
            get(get_one_firm).patch(update_firm).delete(delete_firm),
        )
        .with_state(app_state.clone());

    let files_router = Router::new()
        .route("/", post(upload_file)) // Upload is now on /files
        .route("/:id", get(get_one_file).delete(delete_file))
        .with_state(app_state.clone());

    let requests_router = Router::new()
        .route("/", post(create_request).get(get_all_requests))
        .route(
            "/:id",
            get(get_one_request)
                .patch(update_request)
                .delete(delete_request),
        )
        .route("/:request_id/files", get(get_all_for_request)) // Removed post(upload_file) as it's now on /files
        .with_state(app_state.clone());

    let collections_router = Router::new()
        .route("/", post(create_collection).get(get_all_collections))
        .route(
            "/:id",
            get(get_one_collection)
                .patch(update_collection)
                .delete(delete_collection),
        )
        .with_state(app_state.clone());

    // Group all protected routes and apply the middleware
    let protected_routes = Router::new()
        .nest("/users", protected_users_router) // Protected user routes
        .nest("/clients", clients_router)
        .nest("/firms", firms_router)
        .nest("/files", files_router)
        .nest("/requests", requests_router)
        .nest("/collections", collections_router)
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/", public_users_router) // Public user routes
        .merge(protected_routes) // Merge protected routes
        .with_state(app_state)
}

