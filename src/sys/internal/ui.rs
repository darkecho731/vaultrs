pub mod mounts {
    use crate::{
        api::{
            self,
            sys::internal::ui::{requests, responses},
        },
        client::Client,
        error::ClientError,
    };

    /// Lists all mounted secret engines
    ///
    /// See [ListMountsRequest]
    #[instrument(skip(client), err)]
    pub async fn list(client: &impl Client) -> Result<responses::ListMountsResponse, ClientError> {
        let endpoint = requests::ListMountsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}
