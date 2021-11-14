use oauth2_client::re_exports::{
    http::header::ACCEPT, serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request,
    Response, SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://graph.instagram.com/v12.0/me";

//
#[derive(Debug, Clone)]
pub struct GraphMeEndpoint {
    access_token: String,
}
impl GraphMeEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for GraphMeEndpoint {
    type RenderRequestError = GraphMeEndpointError;

    type ParseResponseOutput = User;
    type ParseResponseError = GraphMeEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(
                format!(
                    "{}?fields=account_type,id,username&access_token={}",
                    URL, self.access_token
                )
                .as_str(),
            )
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(GraphMeEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<User>(response.body())
            .map_err(GraphMeEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub account_type: String,
    pub id: String,
    pub username: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GraphMeEndpointError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_user() {
        match serde_json::from_str::<User>(include_str!(
            "../../tests/response_body_json_files/graph_me.json"
        )) {
            Ok(user) => {
                assert_eq!(user.id, "4242566562536072");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
