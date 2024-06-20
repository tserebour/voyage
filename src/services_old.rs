// use actix::fut::ok;
// use actix_web::{get, 
//     // guard::Options, 
//     post, web::{Data,Json}, HttpResponse, Responder
// };




// use crate::AppState;
// mod models;
// mod voyage_user_sign_up_model;
// mod driver_location_update_model;
// mod update_earn_type_model;

// mod helper_functions;
// mod voyage_driver_sign_up_model;
// mod my_websocket_model;


// // use voyage_user_sign_up_model::VoyageUser;
// use models::models::voyage_models;
// use models::models::bra_fie_models;
// use helper_functions::helpers_functions::{hash_password,verify_password};
// use actix_web_actors::ws;

// use chrono::NaiveDateTime;









































// pub async fn websocket_handler(req: actix_web::HttpRequest, stream: actix_web::web::Payload, data: actix_web::web::Data<Addr<Server>>) -> actix_web::Result<actix_web::HttpResponse> {
//     let server = data.get_ref().clone();
//     let resp = ws::start(my_websocket_model::MyWebSocket::new(server), &req, stream);
//     resp
// }

