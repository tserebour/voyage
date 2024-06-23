use actix_web::web;
// use voyage_drivers_create::voyage_create_driver;

pub mod index;
pub mod voyage_users_create;
pub mod voyage_users_login;

pub mod ride_requests;
// pub mod brafie_users_login;
pub mod voyage_drivers_create;
pub mod voyage_drivers_login;
pub mod voyage_driver_earn_type_update;
pub mod voyage_drivers_location_update_create;
pub mod update_driver_online_status;




pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(voyage_users_login::voyage_user_sign_in)
       .service(voyage_users_create::voyage_create_user)
       .service(voyage_drivers_create::voyage_create_driver)
       .service(voyage_drivers_login::voyage_driver_sign_in)

       .service(ride_requests::create_ride_request)
       .service(voyage_drivers_location_update_create::add_driver_location_to_database)
       .service(voyage_driver_earn_type_update::update_earn_type_to_database)
        .service(update_driver_online_status::update_driver_status);

    
       

    
}