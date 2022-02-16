use std::convert::Infallible;
use warp;
use warp::hyper::StatusCode;

use crate::models::Business;
use crate::db::Db;

pub async fn list_businesses(db: Db) -> Result<impl warp::Reply, Infallible> {
  let businesses = db.lock().await;
  let businesses: Vec<Business> = businesses.clone();
  Ok(warp::reply::json(&businesses))
}

pub async fn create_business(new_business: Business, db: Db) -> Result<impl warp::Reply, Infallible> {
  let mut businesses = db.lock().await;
  
  for business in businesses.iter() {
    if business.id == new_business.id {
      return Ok(StatusCode::BAD_REQUEST);
    }
  }

  businesses.push(new_business);

  Ok(StatusCode::CREATED)
}

pub async fn get_business(id: i32, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
  let businesses = db.lock().await;
  let business = businesses.iter().find(|business| business.id == id);
  match business {
    Some(business) => Ok(Box::new(warp::reply::json(business))),
    None => Ok(Box::new(StatusCode::NOT_FOUND))
  }
}

pub async fn update_business(id: i32, new_business: Business, db: Db) -> Result<impl warp::Reply, Infallible> {
  let mut businesses = db.lock().await;
  let business = businesses.iter_mut().find(|business| business.id == id);
  match business {
    Some(business) => {
      *business = new_business;
      Ok(StatusCode::OK)
    },
    None => Ok(StatusCode::NOT_FOUND)
  }
}

pub async fn delete_business(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
  let mut businesses = db.lock().await;
  let business = businesses.iter_mut().find(|business| business.id == id);
  match business {
    Some(_) => {
      businesses.retain(|business| business.id != id);
      Ok(StatusCode::OK)
    },
    None => Ok(StatusCode::NOT_FOUND)
  }
}