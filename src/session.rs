use axum::AddExtensionLayer;
use futures::future::BoxFuture;
use hyper::{Request, Response};
use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::layer::util::{Identity, Stack};
use tower::{Layer, Service, ServiceBuilder};
use tracing::debug;
use uuid::Uuid;

pub struct Session {
    id: Uuid,
}

impl Default for Session {
    fn default() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

pub struct SessionLayer {
    store: HashMap<Uuid, Session>,
}

impl SessionLayer {
    pub fn build() -> Stack<AddExtensionLayer<Arc<Option<Session>>>, Stack<SessionLayer, Identity>>
    {
        ServiceBuilder::new()
            .layer(Self {
                store: Default::default(),
            })
            .layer(AddExtensionLayer::new(Arc::new(None::<Session>)))
            .into_inner()
    }
}

impl<S> Layer<S> for SessionLayer {
    type Service = SessionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionService {
            store: self.store.clone(),
            inner,
        }
    }
}

#[derive(Clone)]
pub struct SessionService<S> {
    store: HashMap<Uuid, Session>,
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SessionService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        debug!("`MyMiddleware` called!");

        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let res: Response<ResBody> = inner.call(req).await?;

            debug!("`MyMiddleware` received the response");

            Ok(res)
        })
    }
}
