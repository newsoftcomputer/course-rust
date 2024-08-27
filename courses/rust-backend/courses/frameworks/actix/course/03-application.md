
# Writing an Application

  -actix-web provides various primitives to build web servers and applications with Rust. It provides routing, middleware, pre-processing of requests, post-processing of responses, etc.

- All actix-web servers are built around the App instance. It is used for registering routes for resources and middleware. It also stores application state shared across all handlers within the same scope.

- An application's scope acts as a namespace for all routes, i.e. all routes for a specific application scope have the same url path prefix. The application prefix always contains a leading "/" slash. If a supplied prefix does not contain leading slash, it is automatically inserted. The prefix should consist of value path segments.

- For an application with scope /app, any request with the paths /app, /app/, or /app/test would match; however, the path /application would not match.

      use actix_web::{web, App, HttpServer, Responder};

      async fn index() -> impl Responder {
          "Hello world!"
      }

      #[actix_web::main]
      async fn main() -> std::io::Result<()> {
          HttpServer::new(|| {
              App::new().service(
                  // prefixes all resources and routes attached to it...
                  web::scope("/app")
                      // ...so this handles requests for `GET /app/index.html`
                      .route("/index.html", web::get().to(index)),
              )
          })
          .bind(("127.0.0.1", 8080))?
          .run()
          .await
      }

  - In this example, an application with the /app prefix and an index.html resource is created. This resource is available through the /app/index.html url.

# State

- Application state is shared with all routes and resources within the same scope. State can be accessed with the web::Data<T> extractor where T is the type of the state. State is also accessible for middleware.

- Let's write a simple application and store the application name in the state:
