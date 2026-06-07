use crate::router::Route;

pub fn get_route_by_name(name: String) -> Route {
    match name.to_lowercase().as_str() {
        "digraph" => Route::Digraph,
        "matrix" => Route::Matrix,
        _ => Route::NotFound
    }
}