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

    #[doc = "`GetThingOrThingsId`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetThingOrThingsId {
        Variant0(::std::string::String),
        Variant1(::std::vec::Vec<::std::string::String>),
    }

    impl ::std::convert::From<&Self> for GetThingOrThingsId {
        fn from(value: &GetThingOrThingsId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for GetThingOrThingsId {
        fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
            Self::Variant1(value)
        }
    }

    #[doc = "`HeaderArgAcceptLanguage`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"default\": \"en\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"de\","]
    #[doc = "    \"en\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum HeaderArgAcceptLanguage {
        #[serde(rename = "de")]
        De,
        #[serde(rename = "en")]
        En,
    }

    impl ::std::convert::From<&Self> for HeaderArgAcceptLanguage {
        fn from(value: &HeaderArgAcceptLanguage) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for HeaderArgAcceptLanguage {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::De => f.write_str("de"),
                Self::En => f.write_str("en"),
            }
        }
    }

    impl ::std::str::FromStr for HeaderArgAcceptLanguage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "de" => Ok(Self::De),
                "en" => Ok(Self::En),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for HeaderArgAcceptLanguage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for HeaderArgAcceptLanguage {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for HeaderArgAcceptLanguage {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for HeaderArgAcceptLanguage {
        fn default() -> Self {
            HeaderArgAcceptLanguage::En
        }
    }

    #[doc = "`ObjWithOptionArray`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"stranger-things\","]
    #[doc = "    \"things\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"stranger-things\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"oneOf\": ["]
    #[doc = "          {"]
    #[doc = "            \"type\": \"null\""]
    #[doc = "          },"]
    #[doc = "          {"]
    #[doc = "            \"allOf\": ["]
    #[doc = "              {"]
    #[doc = "                \"$ref\": \"#/components/schemas/Task\""]
    #[doc = "              }"]
    #[doc = "            ],"]
    #[doc = "            \"oneOf\": ["]
    #[doc = "              {}"]
    #[doc = "            ]"]
    #[doc = "          }"]
    #[doc = "        ]"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"things\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"oneOf\": ["]
    #[doc = "          {"]
    #[doc = "            \"type\": \"null\""]
    #[doc = "          },"]
    #[doc = "          {"]
    #[doc = "            \"allOf\": ["]
    #[doc = "              {"]
    #[doc = "                \"$ref\": \"#/components/schemas/Task\""]
    #[doc = "              }"]
    #[doc = "            ]"]
    #[doc = "          }"]
    #[doc = "        ]"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ObjWithOptionArray {
        #[serde(rename = "stranger-things")]
        pub stranger_things: ::std::vec::Vec<::std::option::Option<Task>>,
        pub things: ::std::vec::Vec<::std::option::Option<Task>>,
    }

    impl ::std::convert::From<&ObjWithOptionArray> for ObjWithOptionArray {
        fn from(value: &ObjWithOptionArray) -> Self {
            value.clone()
        }
    }

    #[doc = "`Task`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"output_rules\","]
    #[doc = "    \"script\","]
    #[doc = "    \"state\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"output_rules\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"script\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Task {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub output_rules: ::std::vec::Vec<::std::string::String>,
        pub script: ::std::string::String,
        pub state: ::std::string::String,
    }

    impl ::std::convert::From<&Task> for Task {
        fn from(value: &Task) -> Self {
            value.clone()
        }
    }

    #[doc = "`TaskEvent`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"payload\","]
    #[doc = "    \"seq\","]
    #[doc = "    \"stream\","]
    #[doc = "    \"time\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"payload\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"seq\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint\","]
    #[doc = "      \"minimum\": 0.0"]
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
    pub struct TaskEvent {
        pub payload: ::std::string::String,
        pub seq: u32,
        pub stream: ::std::string::String,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&TaskEvent> for TaskEvent {
        fn from(value: &TaskEvent) -> Self {
            value.clone()
        }
    }

    #[doc = "`TaskOutput`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"path\","]
    #[doc = "    \"size\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"path\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskOutput {
        pub id: ::std::string::String,
        pub path: ::std::string::String,
        pub size: u64,
    }

    impl ::std::convert::From<&TaskOutput> for TaskOutput {
        fn from(value: &TaskOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "`TaskSubmit`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"default\","]
    #[doc = "    \"name\","]
    #[doc = "    \"script\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"default\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"output_rules\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"script\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskSubmit {
        pub default: bool,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub output_rules: ::std::vec::Vec<::std::string::String>,
        pub script: ::std::string::String,
    }

    impl ::std::convert::From<&TaskSubmit> for TaskSubmit {
        fn from(value: &TaskSubmit) -> Self {
            value.clone()
        }
    }

    #[doc = "`TaskSubmitResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskSubmitResult {
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&TaskSubmitResult> for TaskSubmitResult {
        fn from(value: &TaskSubmitResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`UploadedChunk`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UploadedChunk {
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&UploadedChunk> for UploadedChunk {
        fn from(value: &UploadedChunk) -> Self {
            value.clone()
        }
    }

    #[doc = "`UserCreate`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserCreate {
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&UserCreate> for UserCreate {
        fn from(value: &UserCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "`UserCreateResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"token\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"token\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserCreateResult {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub token: ::std::string::String,
    }

    impl ::std::convert::From<&UserCreateResult> for UserCreateResult {
        fn from(value: &UserCreateResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`WhoamiResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WhoamiResult {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&WhoamiResult> for WhoamiResult {
        fn from(value: &WhoamiResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`Worker`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"deleted\","]
    #[doc = "    \"id\","]
    #[doc = "    \"recycle\","]
    #[doc = "    \"tasks\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"deleted\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"instance_id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"lastping\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"recycle\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"tasks\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/WorkerTask\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Worker {
        pub deleted: bool,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastping: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub recycle: bool,
        pub tasks: ::std::vec::Vec<WorkerTask>,
    }

    impl ::std::convert::From<&Worker> for Worker {
        fn from(value: &Worker) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerAddOutput`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"chunks\","]
    #[doc = "    \"path\","]
    #[doc = "    \"size\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"chunks\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"path\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int64\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerAddOutput {
        pub chunks: ::std::vec::Vec<::std::string::String>,
        pub path: ::std::string::String,
        pub size: i64,
    }

    impl ::std::convert::From<&WorkerAddOutput> for WorkerAddOutput {
        fn from(value: &WorkerAddOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerAppendTask`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"payload\","]
    #[doc = "    \"stream\","]
    #[doc = "    \"time\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"payload\": {"]
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
    pub struct WorkerAppendTask {
        pub payload: ::std::string::String,
        pub stream: ::std::string::String,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&WorkerAppendTask> for WorkerAppendTask {
        fn from(value: &WorkerAppendTask) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerBootstrap`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"bootstrap\","]
    #[doc = "    \"token\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"bootstrap\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"token\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerBootstrap {
        pub bootstrap: ::std::string::String,
        pub token: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerBootstrap> for WorkerBootstrap {
        fn from(value: &WorkerBootstrap) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerBootstrapResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerBootstrapResult {
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerBootstrapResult> for WorkerBootstrapResult {
        fn from(value: &WorkerBootstrapResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerCompleteTask`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"failed\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"failed\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerCompleteTask {
        pub failed: bool,
    }

    impl ::std::convert::From<&WorkerCompleteTask> for WorkerCompleteTask {
        fn from(value: &WorkerCompleteTask) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerPingResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"poweroff\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"poweroff\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"task\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/WorkerPingTask\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerPingResult {
        pub poweroff: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub task: ::std::option::Option<WorkerPingTask>,
    }

    impl ::std::convert::From<&WorkerPingResult> for WorkerPingResult {
        fn from(value: &WorkerPingResult) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerPingTask`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"output_rules\","]
    #[doc = "    \"script\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"output_rules\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"script\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerPingTask {
        pub id: ::std::string::String,
        pub output_rules: ::std::vec::Vec<::std::string::String>,
        pub script: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerPingTask> for WorkerPingTask {
        fn from(value: &WorkerPingTask) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkerTask`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"owner\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"owner\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerTask {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub owner: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerTask> for WorkerTask {
        fn from(value: &WorkerTask) -> Self {
            value.clone()
        }
    }

    #[doc = "`WorkersResult`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"workers\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"workers\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Worker\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkersResult {
        pub workers: ::std::vec::Vec<Worker>,
    }

    impl ::std::convert::From<&WorkersResult> for WorkersResult {
        fn from(value: &WorkersResult) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
#[doc = "Client for Buildomat\n\nVersion: 1.0"]
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
    #[doc = "Sends a `POST` request to `/v1/control/hold`\n\n"]
    pub async fn control_hold<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/control/hold", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "control_hold",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/v1/control/resume`\n\n"]
    pub async fn control_resume<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/control/resume", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "control_resume",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/v1/task/{Task}`\n\n"]
    pub async fn task_get<'a>(
        &'a self,
        task: &'a str,
    ) -> Result<ResponseValue<types::Task>, Error<()>> {
        let url = format!(
            "{}/v1/task/{}",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "task_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/v1/tasks`\n\n"]
    pub async fn tasks_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Task>>, Error<()>> {
        let url = format!("{}/v1/tasks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "tasks_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/v1/tasks`\n\n"]
    pub async fn task_submit<'a>(
        &'a self,
        body: &'a types::TaskSubmit,
    ) -> Result<ResponseValue<types::TaskSubmitResult>, Error<()>> {
        let url = format!("{}/v1/tasks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "task_submit",
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

    #[doc = "Sends a `GET` request to `/v1/tasks/{task}/events`\n\n"]
    pub async fn task_events_get<'a>(
        &'a self,
        task: &'a str,
        minseq: Option<u32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::TaskEvent>>, Error<()>> {
        let url = format!(
            "{}/v1/tasks/{}/events",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("minseq", &minseq))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "task_events_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/v1/tasks/{task}/outputs`\n\n"]
    pub async fn task_outputs_get<'a>(
        &'a self,
        task: &'a str,
    ) -> Result<ResponseValue<::std::vec::Vec<types::TaskOutput>>, Error<()>> {
        let url = format!(
            "{}/v1/tasks/{}/outputs",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "task_outputs_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/v1/tasks/{task}/outputs/{output}`\n\n"]
    pub async fn task_output_download<'a>(
        &'a self,
        task: &'a str,
        output: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/v1/tasks/{}/outputs/{}",
            self.baseurl,
            encode_path(&task.to_string()),
            encode_path(&output.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "task_output_download",
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

    #[doc = "Sends a `POST` request to `/v1/users`\n\n"]
    pub async fn user_create<'a>(
        &'a self,
        body: &'a types::UserCreate,
    ) -> Result<ResponseValue<types::UserCreateResult>, Error<()>> {
        let url = format!("{}/v1/users", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "user_create",
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

    #[doc = "Sends a `GET` request to `/v1/whoami`\n\n"]
    pub async fn whoami<'a>(&'a self) -> Result<ResponseValue<types::WhoamiResult>, Error<()>> {
        let url = format!("{}/v1/whoami", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "whoami",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `PUT` request to `/v1/whoami/name`\n\n"]
    pub async fn whoami_put_name<'a>(
        &'a self,
        body: String,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/whoami/name", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("text/plain"),
            )
            .body(body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "whoami_put_name",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/v1/worker/bootstrap`\n\n"]
    pub async fn worker_bootstrap<'a>(
        &'a self,
        body: &'a types::WorkerBootstrap,
    ) -> Result<ResponseValue<types::WorkerBootstrapResult>, Error<()>> {
        let url = format!("{}/v1/worker/bootstrap", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "worker_bootstrap",
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

    #[doc = "Sends a `GET` request to `/v1/worker/ping`\n\n"]
    pub async fn worker_ping<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::WorkerPingResult>, Error<()>> {
        let url = format!("{}/v1/worker/ping", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "worker_ping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/v1/worker/task/{task}/append`\n\n"]
    pub async fn worker_task_append<'a>(
        &'a self,
        task: &'a str,
        body: &'a types::WorkerAppendTask,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/append",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_append",
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

    #[doc = "Sends a `POST` request to `/v1/worker/task/{task}/chunk`\n\n"]
    pub async fn worker_task_upload_chunk<'a, B: Into<reqwest::Body>>(
        &'a self,
        task: &'a str,
        body: B,
    ) -> Result<ResponseValue<types::UploadedChunk>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/chunk",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .header(
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("application/octet-stream"),
            )
            .body(body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_upload_chunk",
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

    #[doc = "Sends a `POST` request to `/v1/worker/task/{task}/complete`\n\n"]
    pub async fn worker_task_complete<'a>(
        &'a self,
        task: &'a str,
        body: &'a types::WorkerCompleteTask,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/complete",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_complete",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/v1/worker/task/{task}/output`\n\n"]
    pub async fn worker_task_add_output<'a>(
        &'a self,
        task: &'a str,
        body: &'a types::WorkerAddOutput,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/output",
            self.baseurl,
            encode_path(&task.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_add_output",
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

    #[doc = "Sends a `GET` request to `/v1/workers`\n\n"]
    pub async fn workers_list<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::WorkersResult>, Error<()>> {
        let url = format!("{}/v1/workers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
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
            operation_id: "workers_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/v1/workers/recycle`\n\n"]
    pub async fn workers_recycle<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/workers/recycle", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "workers_recycle",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/v1/things`\n\n"]
    pub async fn get_thing_or_things<'a>(
        &'a self,
        id: Option<&'a types::GetThingOrThingsId>,
    ) -> Result<ResponseValue<::std::string::String>, Error<()>> {
        let url = format!("{}/v1/things", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_thing_or_things",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/v1/header-arg`\n\n"]
    pub async fn header_arg<'a>(
        &'a self,
        accept_language: Option<types::HeaderArgAcceptLanguage>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/header-arg", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        if let Some(value) = accept_language {
            header_map.append("accept-language", value.to_string().try_into()?);
        }

        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "header_arg",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::ErrorResponse(ResponseValue::empty(response))),
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
