use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Business;

pub fn business_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_business(db.clone())
        .or(update_business(db.clone()))
        .or(delete_business(db.clone()))
        .or(create_business(db.clone()))
        .or(businesses_list(db))
}

fn businesses_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("businesses")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_businesses)
}

/// POST /customers
fn create_business(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("businesses")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_business)
}

/// GET /customers/{guid}
fn get_business(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("businesses" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_business)
}

/// PUT /customers/{guid}
fn update_business(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("businesses" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_business)
}

/// DELETE /customers/{guid}
fn delete_business(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("businesses" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_business)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Business,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}