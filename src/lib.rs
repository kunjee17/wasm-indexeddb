mod utils;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use validator::{Validate, ValidationErrors};
use wasm_bindgen::prelude::*;
use anyhow::Result;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-tsify!");
}

#[derive(Tsify, Serialize, Deserialize, Validate, Debug, Clone, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point {
    #[validate(range(min = 1, max = 10))]
    x: i32,
    #[validate(range(min = 1, max = 10))]
    y: i32,
}

#[wasm_bindgen]
pub fn into_js(p : i32) -> Point {
    Point { x: p, y: p }
}

pub fn collect_errors(error: ValidationErrors) -> Vec<String> {
    error
        .field_errors()
        .into_iter()
        .map(|error| {
            let default_error = format!("{} is required", error.0);
            error.1[0]
                .message
                .as_ref()
                .unwrap_or(&std::borrow::Cow::Owned(default_error))
                .to_string()
        })
        .collect()
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct ErrorMessages {
    message: Vec<String>,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PointsRes {
    points: Vec<Point>,
}

pub async fn insert_all_points () {
    todo!();
}

#[wasm_bindgen]
pub async fn get_all_points () -> Result<PointsRes, String> {

    let res = reqwest::get("http://localhost:5000/points")
                .await.map_err(|_| "API call error".to_string())?.json()
                .await.map_err(|_| "JSON error".to_string())?;

    Ok(PointsRes {
        points: res
    })
}

//Faking it for now
#[wasm_bindgen]
pub fn from_js(point: Point) -> Result<(), ErrorMessages> {
    //Here validate the point
    println!("x: {}, y: {}", point.x, point.y);
     point.validate().map_err(|e| {
        ErrorMessages {
            message: collect_errors(e),
        }
     } )
}


//Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_js() {
        let p = into_js(10);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn test_from_js() {
        let p = Point { x: 10, y: 10 };
        let r = from_js(p);
        assert_eq!(r.is_ok(), true);
    }
}