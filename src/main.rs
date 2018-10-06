#![feature(plugin)]
#![plugin(rocket_codegen)]

#[allow(unused)]
extern crate knock;
#[allow(unused)]
extern crate rocket;
#[allow(unused)]
extern crate diesel;
extern crate dotenv;

//use dotenv::dotenv;
//use std::env;

fn main() {
    //dotenv().ok();
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .mount("/", routes![knock::web::index, knock::web::knock])
        .mount("/dns",
            routes![
                knock::power_dns::dns_lookup,
                knock::power_dns::dns_list,
                knock::power_dns::dns_add_domain_key,
                knock::power_dns::dns_get_domain_keys
            ])
        .launch();
}
