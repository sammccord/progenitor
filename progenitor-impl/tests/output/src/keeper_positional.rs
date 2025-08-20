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

    #[doc = "`EnrolBody`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"EnrolBody\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"host\","]
    #[doc = "    \"key\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"host\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EnrolBody {
        pub host: ::std::string::String,
        pub key: ::std::string::String,
    }

    impl ::std::convert::From<&EnrolBody> for EnrolBody {
        fn from(value: &EnrolBody) -> Self {
            value.clone()
        }
    }

    #[doc = "`GlobalJobsResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GlobalJobsResult\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"summary\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"summary\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ReportSummary\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalJobsResult {
        pub summary: ::std::vec::Vec<ReportSummary>,
    }

    impl ::std::convert::From<&GlobalJobsResult> for GlobalJobsResult {
        fn from(value: &GlobalJobsResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`OutputRecord`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"msg\","]
    #[doc = "    \"stream\","]
    #[doc = "    \"time\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"msg\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"stream\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"time\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OutputRecord {
        pub msg: ::std::string::String,
        pub stream: ::std::string::String,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&OutputRecord> for OutputRecord {
        fn from(value: &OutputRecord) -> Self {
            value.clone()
        }
    }

    #[doc = "`PingResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PingResult\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"host\","]
    #[doc = "    \"ok\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"host\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"ok\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PingResult {
        pub host: ::std::string::String,
        pub ok: bool,
    }

    impl ::std::convert::From<&PingResult> for PingResult {
        fn from(value: &PingResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`ReportFinishBody`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ReportFinishBody\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"duration_millis\","]
    #[doc = "    \"end_time\","]
    #[doc = "    \"exit_status\","]
    #[doc = "    \"id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"duration_millis\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"end_time\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"exit_status\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ReportId\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportFinishBody {
        pub duration_millis: i32,
        pub end_time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub exit_status: i32,
        pub id: ReportId,
    }

    impl ::std::convert::From<&ReportFinishBody> for ReportFinishBody {
        fn from(value: &ReportFinishBody) -> Self {
            value.clone()
        }
    }

    #[doc = "`ReportId`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"host\","]
    #[doc = "    \"job\","]
    #[doc = "    \"pid\","]
    #[doc = "    \"time\","]
    #[doc = "    \"uuid\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"host\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"job\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"pid\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"time\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"uuid\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportId {
        pub host: ::std::string::String,
        pub job: ::std::string::String,
        pub pid: u64,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub uuid: ::std::string::String,
    }

    impl ::std::convert::From<&ReportId> for ReportId {
        fn from(value: &ReportId) -> Self {
            value.clone()
        }
    }

    #[doc = "`ReportOutputBody`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ReportOutputBody\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"record\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ReportId\""]
    #[doc = "    },"]
    #[doc = "    \"record\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/OutputRecord\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportOutputBody {
        pub id: ReportId,
        pub record: OutputRecord,
    }

    impl ::std::convert::From<&ReportOutputBody> for ReportOutputBody {
        fn from(value: &ReportOutputBody) -> Self {
            value.clone()
        }
    }

    #[doc = "`ReportResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ReportResult\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"existed_already\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"existed_already\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportResult {
        pub existed_already: bool,
    }

    impl ::std::convert::From<&ReportResult> for ReportResult {
        fn from(value: &ReportResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`ReportStartBody`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ReportStartBody\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"script\","]
    #[doc = "    \"start_time\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ReportId\""]
    #[doc = "    },"]
    #[doc = "    \"script\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"start_time\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportStartBody {
        pub id: ReportId,
        pub script: ::std::string::String,
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&ReportStartBody> for ReportStartBody {
        fn from(value: &ReportStartBody) -> Self {
            value.clone()
        }
    }

    #[doc = "`ReportSummary`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"age_seconds\","]
    #[doc = "    \"duration_seconds\","]
    #[doc = "    \"host\","]
    #[doc = "    \"job\","]
    #[doc = "    \"status\","]
    #[doc = "    \"when\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"age_seconds\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"duration_seconds\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"host\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"job\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"status\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"when\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportSummary {
        pub age_seconds: i32,
        pub duration_seconds: i32,
        pub host: ::std::string::String,
        pub job: ::std::string::String,
        pub status: i32,
        pub when: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&ReportSummary> for ReportSummary {
        fn from(value: &ReportSummary) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
#[doc = "Client for Keeper API\n\nreport execution of cron jobs through a mechanism other than mail\n\nVersion: 1.0"]
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
        "1.0"
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
    #[doc = "Sends a `POST` request to `/enrol`\n\nArguments:\n- `authorization`: Authorization header (bearer token)\n- `body`\n"]
    pub async fn enrol<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::EnrolBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/enrol", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "enrol",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/global/jobs`\n\nArguments:\n- `authorization`: Authorization header (bearer token)\n"]
    pub async fn global_jobs<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::GlobalJobsResult>, Error<()>> {
        let url = format!("{}/global/jobs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "global_jobs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/ping`\n\nArguments:\n- `authorization`: Authorization header (bearer token)\n"]
    pub async fn ping<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::PingResult>, Error<()>> {
        let url = format!("{}/ping", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/report/finish`\n\nArguments:\n- `authorization`: Authorization header (bearer token)\n- `body`\n"]
    pub async fn report_finish<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ReportFinishBody,
    ) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
        let url = format!("{}/report/finish", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "report_finish",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/report/output`\n\nArguments:\n- `authorization`: Authorization header (bearer token)\n- `body`\n"]
    pub async fn report_output<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ReportOutputBody,
    ) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
        let url = format!("{}/report/output", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "report_output",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/report/start`\n\nArguments:\n- `authorization`: Authorization header (bearer token)\n- `body`\n"]
    pub async fn report_start<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ReportStartBody,
    ) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
        let url = format!("{}/report/start", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "report_start",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
