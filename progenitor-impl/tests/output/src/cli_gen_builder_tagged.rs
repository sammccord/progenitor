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

    impl UnoBody {
        pub fn builder() -> builder::UnoBody {
            Default::default()
        }
    }

    #[doc = r" Types for composing complex structures."]
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct UnoBody {
            gateway: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            required: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        }

        impl ::std::default::Default for UnoBody {
            fn default() -> Self {
                Self {
                    gateway: Ok(Default::default()),
                    required: Err("no value supplied for required".to_string()),
                }
            }
        }

        impl UnoBody {
            pub fn gateway<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.gateway = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gateway: {}", e));
                self
            }
            pub fn required<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.required = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for required: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UnoBody> for super::UnoBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UnoBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    gateway: value.gateway?,
                    required: value.required?,
                })
            }
        }

        impl ::std::convert::From<super::UnoBody> for UnoBody {
            fn from(value: super::UnoBody) -> Self {
                Self {
                    gateway: Ok(value.gateway),
                    required: Ok(value.required),
                }
            }
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
impl Client {
    #[doc = "Sends a `GET` request to `/uno`\n\n```ignore\nlet response = client.uno()\n    .gateway(gateway)\n    .body(body)\n    .send()\n    .await;\n```"]
    pub fn uno(&self) -> builder::Uno<'_> {
        builder::Uno::new(self)
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
    #[doc = "Builder for [`Client::uno`]\n\n[`Client::uno`]: super::Client::uno"]
    #[derive(Debug, Clone)]
    pub struct Uno<'a> {
        client: &'a super::Client,
        gateway: Result<::std::string::String, String>,
        body: Result<types::builder::UnoBody, String>,
    }

    impl<'a> Uno<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                gateway: Err("gateway was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn gateway<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.gateway = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for gateway failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UnoBody>,
            <V as std::convert::TryInto<types::UnoBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `UnoBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::UnoBody) -> types::builder::UnoBody,
        {
            self.body = self.body.map(f);
            self
        }

        #[doc = "Sends a `GET` request to `/uno`"]
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self {
                client,
                gateway,
                body,
            } = self;
            let gateway = gateway.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::UnoBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/uno", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .json(&body)
                .query(&progenitor_client::QueryParam::new("gateway", &gateway))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "uno",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client and"]
#[doc = r" extension traits."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
