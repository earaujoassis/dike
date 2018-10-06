use std::vec::Vec;
use router::Router;
use slog::Logger;
use iron::BeforeMiddleware;

use controllers::power_dns as PowerDNSController;
use controllers::services as ServicesController;

use utils::pool::DieselPool;
use middlewares::utils::SelectiveMiddleWare;
#[allow(unused_imports)]
use middlewares::GetSaltMiddleware;
use middlewares::DieselMiddleware;
use middlewares::LoggerMiddleware;

macro_rules! with_middleware {
    ($handler:expr, before=[$($y:expr),+] ) => {
        {
            let mut before: Vec<Box<BeforeMiddleware>> = Vec::new();
            $(
                before.push(Box::new($y) as Box<BeforeMiddleware>);
            )*
            SelectiveMiddleWare::new($handler, before)
        }
    };
    ($handler:expr, [$($y:expr),+] ) => {
        with_middleware!($handler, before=[$( $y  ),*])
    }
}

pub fn general_endpoints(logger: &Logger, pool: &DieselPool) -> Router {
    let router = router!(
        index:             get "/ping"                                 => with_middleware!(ServicesController::ping, [LoggerMiddleware::new(&logger), DieselMiddleware::new(&logger, &pool)]),
        clients:           get "/clients"                              => with_middleware!(ServicesController::clients, [LoggerMiddleware::new(&logger), DieselMiddleware::new(&logger, &pool)]),
        dns_adddomainkey:  get "/dns/adddomainkey/:domain/:domain_id"  => with_middleware!(PowerDNSController::dns_add_domain_key, [DieselMiddleware::new(&logger, &pool)]),
        dns_getdomainkeys: get "/dns/getdomainkeys/:domain/:domain_id" => with_middleware!(PowerDNSController::dns_get_domain_keys, [DieselMiddleware::new(&logger, &pool)]),
        dns_lookup:        get "/dns/lookup/:domain/:qtype"            => with_middleware!(PowerDNSController::dns_lookup, [DieselMiddleware::new(&logger, &pool)]),
        dns_list:          get "/dns/list/:domain_id/:domain"          => with_middleware!(PowerDNSController::dns_list, [DieselMiddleware::new(&logger, &pool)])
    );

    router
}
