use std::error;
use std::fmt;

use iron::{Handler, IronResult, BeforeMiddleware, Request, Response, Chain};

#[derive(Debug)]
pub enum MiddlewareErrorTypes {
    AuthorizationError
}

impl fmt::Display for MiddlewareErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MiddlewareErrorTypes::AuthorizationError => write!(f, "Authorization failed"),
        }
    }
}

impl error::Error for MiddlewareErrorTypes {
    fn description(&self) -> &str {
        match *self {
            MiddlewareErrorTypes::AuthorizationError => "Authorization failed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MiddlewareErrorTypes::AuthorizationError => None
        }
    }
}

pub struct SelectiveMiddleWare {
    chain: Chain,
}

impl SelectiveMiddleWare {
    pub fn new<H: Handler, M: BeforeMiddleware>(handler: H, m: Vec<M>) -> Self {
        let mut chain = Chain::new(handler);
        for item in m.into_iter() {
            chain.link_before(item);
        }

        SelectiveMiddleWare {
            chain: chain
        }
    }
}

impl Handler for SelectiveMiddleWare {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.chain.handle(req)
    }
}
