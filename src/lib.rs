pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

// A type that implements FromRequest trait is called an Extractor
// An extractor can be used as a parameter of a handler function
// actix_web will invoke the from_request method for each extractor BEFORE executing the handler logic
// If an extractor fails, then the corresponding error is returned to the caller, keep in mind that an actix_web::Error can be converted into a HttpResponse
