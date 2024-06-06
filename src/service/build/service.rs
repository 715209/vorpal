use crate::api::package_service_server::PackageService;
use crate::api::{BuildRequest, BuildResponse, PrepareRequest, PrepareResponse};
use crate::service::build::{build, prepare};
use anyhow::Result;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct Package {}

#[tonic::async_trait]
impl PackageService for Package {
    async fn prepare(
        &self,
        request: Request<PrepareRequest>,
    ) -> Result<Response<PrepareResponse>, Status> {
        prepare::run(request).await
    }

    async fn build(
        &self,
        request: Request<BuildRequest>,
    ) -> Result<Response<BuildResponse>, Status> {
        build::run(request).await
    }
}