use iron::prelude::*;
use iron::{Handler, IronResult, BeforeMiddleware, Request, Response, Chain};

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
