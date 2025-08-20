#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[doc = r" Types used as operation parameters and responses."]
#[allow(clippy::all)]
pub mod types {
    #[doc = r" Error types."]
    pub mod error {
        #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    #[doc = "`UnoBody`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"required\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"gateway\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UnoBody {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gateway: ::std::option::Option<::std::string::String>,
        pub required: ::serde_json::Value,
    }

    impl ::std::convert::From<&UnoBody> for UnoBody {
        fn from(value: &UnoBody) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
#[doc = "Client for CLI gen test\n\nTest case to exercise CLI generation\n\nVersion: 9000"]
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    #[doc = r" Create a new client."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            let reqwest_client = reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
                .build()
                .unwrap();
            reqwest_middleware::ClientBuilder::new(reqwest_client).build()
        };
        #[cfg(target_arch = "wasm32")]
        let client = {
            let reqwest_client = reqwest::ClientBuilder::new().build().unwrap();
            reqwest_middleware::ClientBuilder::new(reqwest_client).build()
        };
        Self::new_with_client(baseurl, client)
    }

    #[doc = r" Construct a new client with an existing `reqwest_middleware::ClientWithMiddleware`,"]
    #[doc = r" allowing more control over its configuration."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest_middleware::ClientWithMiddleware`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new_with_client(
        baseurl: &str,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "9000"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest_middleware::ClientWithMiddleware {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    #[doc = "Sends a `GET` request to `/uno`\n\n"]
    pub async fn uno<'a>(
        &'a self,
        gateway: &'a str,
        body: &'a types::UnoBody,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/uno", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .json(&body)
            .query(&progenitor_client::QueryParam::new("gateway", &gateway))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "uno",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
