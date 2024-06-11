pub trait LoginAndRegister {

    fn login<T>() -> Result<HttpResponse, actix_web::Error>;
    
}