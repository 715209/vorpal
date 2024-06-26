use crate::api::package_service_server::PackageService;
use crate::api::{BuildRequest, BuildResponse, PrepareRequest, PrepareResponse};
use crate::service::build::{run_build, run_prepare};
use anyhow::Result;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct Package {}

#[tonic::async_trait]
impl PackageService for Package {
    async fn prepare(
        &self,
        request: Request<Streaming<PrepareRequest>>,
    ) -> Result<Response<PrepareResponse>, Status> {
        run_prepare::run(request.into_inner()).await
    }

    async fn build(
        &self,
        request: Request<BuildRequest>,
    ) -> Result<Response<BuildResponse>, Status> {
        run_build::run(request).await
    }
}
