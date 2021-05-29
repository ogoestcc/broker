use actix_web::middleware::Logger;

#[cfg(not(debug_assertions))]
pub fn logger() -> Logger {
    Logger::new("%s %r %Dms")
}

#[cfg(debug_assertions)]
pub fn logger() -> Logger {
    Logger::default()
} 