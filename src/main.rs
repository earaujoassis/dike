#![feature(plugin)]
#![plugin(rocket_codegen)]

#[allow(unused)]
extern crate knock;
#[allow(unused)]
extern crate rocket;

fn main() {
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
