use std::collections::HashMap;
use crate::{
    middleware::Middleware,
    request::{ HttpMethod, HttpRequest },
    response::{ HttpResponse, StatusCode },
    server::Handler,
};

pub mod methods;

type Controller = fn(&HttpRequest, &mut HttpResponse);

/// A router for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{ router::Router, response::StatusCode };
///
/// let mut main_router = Router::new();
/// let mut sub_router = Router::new();
/// let mut sub_sub_router = Router::new();
/// 
/// sub_sub_router
///   .get(|req, res| {
///     res.status(StatusCode::Ok);
///   })
///   .post(|req, res| {
///     res.status(StatusCode::Ok);
///   });
///
/// sub_router.use_router("suber", sub_sub_router);
/// main_router.use_router("sub", sub_router);
    /// ```
pub struct Router {
    endpoints: HashMap<HttpMethod, Controller>,
    subroutes: HashMap<String, Router>,
    request_middleware: Vec<Middleware>,
    response_middleware: Vec<Middleware>,
}

impl Router {
    /// Creates a new router
    ///
    /// # Example
    ///
    /// To create a `GET` method for `/`
    ///
    /// ```rust
    /// use krustie::{ router::Router, response::StatusCode };
    ///
    /// let mut main_router = Router::new();
    ///
    /// main_router.get(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    ///
    /// ```
    pub fn new() -> Router {
        Router {
            endpoints: HashMap::new(),
            subroutes: HashMap::new(),
            request_middleware: Vec::new(),
            response_middleware: Vec::new(),
        }
    }

    /// Adds a router endpoint to the router
    ///
    /// # Example
    ///
    /// Create a 'POST' method for `/sub/suber`
    ///
    /// ```rust
    /// use krustie::{ router::Router, response::StatusCode };
    ///
    /// let mut main_router = Router::new();
    /// let mut sub_router = Router::new();
    /// let mut sub_sub_router = Router::new();
    /// 
    /// sub_sub_router.post(|req, res| {
    ///  res.status(StatusCode::Ok);
    /// });
    ///
    /// sub_router.use_router("suber", sub_sub_router);
    /// main_router.use_router("sub", sub_router);
    /// ```
    pub fn use_router(&mut self, path: &str, router: Router) -> Result<(), String> {
        let path = if path.starts_with("/") { &path[1..] } else { path };

        if self.subroutes.contains_key(path) {
            return Err("Path already exists".to_string());
        }

        self.subroutes.insert(path.to_string(), router);
        return Ok(());
    }

    pub fn add_middleware(&mut self, middleware: MiddlewareType) {
        match middleware {
            MiddlewareType::Request(middleware) => {
                self.request_middleware.push(middleware);
            }
            MiddlewareType::Response(middleware) => {
                self.response_middleware.push(middleware);
            }
        }
    }

    pub fn handle_route(
        &self,
        request: &HttpRequest,
        response: &mut HttpResponse,
        path: &Vec<String>
    ) {
        for middleware in &self.request_middleware {
            middleware.handle(request, response);
        }

        if path.len() == 1 {
            if let Some(endpoint) = self.endpoints.get(&request.request.method) {
                endpoint(request, response);
            }
        } else {
            if let Ok(router) = self.get_route(&path[1]) {
                router.handle_route(request, response, &path[1..].to_vec());
            } else {
                response.status(StatusCode::NotFound);
            }
        }

        for middleware in &self.response_middleware {
            middleware.handle(request, response);
        }
    }

    fn get_route(&self, path: &str) -> Result<&Router, &str> {
        for (key, router) in &self.subroutes {
            if key == path {
                return Ok(router);
            }
        }

        return Err("Route not found");
    }
}

impl Handler for Router {
    /// Handles routing of requests to the appropriate endpoint
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;

        if request.request.path_array.len() > 0 {
            for (key, router) in &self.subroutes {
                if key == &path[0] {
                    router.handle_route(request, response, path);
                    return;
                }
            }
        }
    }
}

pub enum MiddlewareType {
    Request(Middleware),
    Response(Middleware),
}
