#[allow(unused_imports)]
use progenitor_middleware_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_middleware_client::{ByteStream, ClientInfo, Error, ResponseValue};
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
}

#[derive(Clone, Debug)]
#[doc = "Client for Parameter override test\n\nMinimal API for testing parameter overrides\n\nVersion: v1"]
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
        "v1"
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
impl Client {
    #[doc = "Gets a key\n\nSends a `GET` request to `/key`\n\nArguments:\n- `key`: The same key parameter that overlaps with the path level parameter\n- `unique_key`: A key parameter that will not be overridden by the path spec\n```ignore\nlet response = client.key_get()\n    .key(key)\n    .unique_key(unique_key)\n    .send()\n    .await;\n```"]
    pub fn key_get(&self) -> builder::KeyGet<'_> {
        builder::KeyGet::new(self)
    }
}

#[doc = r" Types for composing operation parameters."]
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue,
    };
    #[doc = "Builder for [`Client::key_get`]\n\n[`Client::key_get`]: super::Client::key_get"]
    #[derive(Debug, Clone)]
    pub struct KeyGet<'a> {
        client: &'a super::Client,
        key: Result<Option<bool>, String>,
        unique_key: Result<Option<::std::string::String>, String>,
    }

    impl<'a> KeyGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                key: Ok(None),
                unique_key: Ok(None),
            }
        }

        pub fn key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.key = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for key failed".to_string());
            self
        }

        pub fn unique_key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.unique_key = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for unique_key failed".to_string()
            });
            self
        }

        #[doc = "Sends a `GET` request to `/key`"]
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                key,
                unique_key,
            } = self;
            let key = key.map_err(Error::InvalidRequest)?;
            let unique_key = unique_key.map_err(Error::InvalidRequest)?;
            let url = format!("{}/key", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .query(&progenitor_middleware_client::QueryParam::new("key", &key))
                .query(&progenitor_middleware_client::QueryParam::new(
                    "uniqueKey",
                    &unique_key,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "key_get",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    pub use self::super::Client;
}
