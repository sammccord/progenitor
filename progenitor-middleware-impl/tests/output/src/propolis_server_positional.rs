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

    #[doc = "`CrucibleOpts`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"lossy\","]
    #[doc = "    \"read_only\","]
    #[doc = "    \"target\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"cert_pem\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"control\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"flush_timeout\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"uint32\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key_pem\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"lossy\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"read_only\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"root_cert_pem\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CrucibleOpts {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cert_pem: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub control: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub flush_timeout: ::std::option::Option<u32>,
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key_pem: ::std::option::Option<::std::string::String>,
        pub lossy: bool,
        pub read_only: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub root_cert_pem: ::std::option::Option<::std::string::String>,
        pub target: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&CrucibleOpts> for CrucibleOpts {
        fn from(value: &CrucibleOpts) -> Self {
            value.clone()
        }
    }

    #[doc = "`DiskAttachment`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"disk_id\","]
    #[doc = "    \"generation_id\","]
    #[doc = "    \"state\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"disk_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"generation_id\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/DiskAttachmentState\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskAttachment {
        pub disk_id: ::uuid::Uuid,
        pub generation_id: u64,
        pub state: DiskAttachmentState,
    }

    impl ::std::convert::From<&DiskAttachment> for DiskAttachment {
        fn from(value: &DiskAttachment) -> Self {
            value.clone()
        }
    }

    #[doc = "`DiskAttachmentState`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Detached\","]
    #[doc = "        \"Destroyed\","]
    #[doc = "        \"Faulted\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"Attached\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"Attached\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        }"]
    #[doc = "      },"]
    #[doc = "      \"additionalProperties\": false"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum DiskAttachmentState {
        Detached,
        Destroyed,
        Faulted,
        Attached(::uuid::Uuid),
    }

    impl ::std::convert::From<&Self> for DiskAttachmentState {
        fn from(value: &DiskAttachmentState) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for DiskAttachmentState {
        fn from(value: ::uuid::Uuid) -> Self {
            Self::Attached(value)
        }
    }

    #[doc = "`DiskRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"device\","]
    #[doc = "    \"gen\","]
    #[doc = "    \"name\","]
    #[doc = "    \"read_only\","]
    #[doc = "    \"slot\","]
    #[doc = "    \"volume_construction_request\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"device\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"gen\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"read_only\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"slot\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Slot\""]
    #[doc = "    },"]
    #[doc = "    \"volume_construction_request\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/VolumeConstructionRequest\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskRequest {
        pub device: ::std::string::String,
        pub gen: u64,
        pub name: ::std::string::String,
        pub read_only: bool,
        pub slot: Slot,
        pub volume_construction_request: VolumeConstructionRequest,
    }

    impl ::std::convert::From<&DiskRequest> for DiskRequest {
        fn from(value: &DiskRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "Error information from a response."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Error information from a response.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"message\","]
    #[doc = "    \"request_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"error_code\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"request_id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_code: ::std::option::Option<::std::string::String>,
        pub message: ::std::string::String,
        pub request_id: ::std::string::String,
    }

    impl ::std::convert::From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    #[doc = "`Instance`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"disks\","]
    #[doc = "    \"nics\","]
    #[doc = "    \"properties\","]
    #[doc = "    \"state\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"disks\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/DiskAttachment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"nics\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/NetworkInterface\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"properties\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/InstanceProperties\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/InstanceState\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Instance {
        pub disks: ::std::vec::Vec<DiskAttachment>,
        pub nics: ::std::vec::Vec<NetworkInterface>,
        pub properties: InstanceProperties,
        pub state: InstanceState,
    }

    impl ::std::convert::From<&Instance> for Instance {
        fn from(value: &Instance) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceEnsureRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"properties\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"cloud_init_bytes\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"disks\": {"]
    #[doc = "      \"default\": [],"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/DiskRequest\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"migrate\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/InstanceMigrateInitiateRequest\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"nics\": {"]
    #[doc = "      \"default\": [],"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/NetworkInterfaceRequest\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"properties\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/InstanceProperties\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceEnsureRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cloud_init_bytes: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub disks: ::std::vec::Vec<DiskRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub migrate: ::std::option::Option<InstanceMigrateInitiateRequest>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub nics: ::std::vec::Vec<NetworkInterfaceRequest>,
        pub properties: InstanceProperties,
    }

    impl ::std::convert::From<&InstanceEnsureRequest> for InstanceEnsureRequest {
        fn from(value: &InstanceEnsureRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceEnsureResponse`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"migrate\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/InstanceMigrateInitiateResponse\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceEnsureResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub migrate: ::std::option::Option<InstanceMigrateInitiateResponse>,
    }

    impl ::std::convert::From<&InstanceEnsureResponse> for InstanceEnsureResponse {
        fn from(value: &InstanceEnsureResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for InstanceEnsureResponse {
        fn default() -> Self {
            Self {
                migrate: Default::default(),
            }
        }
    }

    #[doc = "`InstanceGetResponse`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"instance\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"instance\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Instance\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceGetResponse {
        pub instance: Instance,
    }

    impl ::std::convert::From<&InstanceGetResponse> for InstanceGetResponse {
        fn from(value: &InstanceGetResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceMigrateInitiateRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"migration_id\","]
    #[doc = "    \"src_addr\","]
    #[doc = "    \"src_uuid\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"migration_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"src_addr\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"src_uuid\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateInitiateRequest {
        pub migration_id: ::uuid::Uuid,
        pub src_addr: ::std::string::String,
        pub src_uuid: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrateInitiateRequest> for InstanceMigrateInitiateRequest {
        fn from(value: &InstanceMigrateInitiateRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceMigrateInitiateResponse`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"migration_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"migration_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateInitiateResponse {
        pub migration_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrateInitiateResponse> for InstanceMigrateInitiateResponse {
        fn from(value: &InstanceMigrateInitiateResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceMigrateStatusRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"migration_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"migration_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateStatusRequest {
        pub migration_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrateStatusRequest> for InstanceMigrateStatusRequest {
        fn from(value: &InstanceMigrateStatusRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceMigrateStatusResponse`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"state\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/MigrationState\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateStatusResponse {
        pub state: MigrationState,
    }

    impl ::std::convert::From<&InstanceMigrateStatusResponse> for InstanceMigrateStatusResponse {
        fn from(value: &InstanceMigrateStatusResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceProperties`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"bootrom_id\","]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"image_id\","]
    #[doc = "    \"memory\","]
    #[doc = "    \"name\","]
    #[doc = "    \"vcpus\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"bootrom_id\": {"]
    #[doc = "      \"description\": \"ID of the bootrom used to initialize this Instance.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"Free-form text description of an Instance.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"Unique identifier for this Instance.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"image_id\": {"]
    #[doc = "      \"description\": \"ID of the image used to initialize this Instance.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"memory\": {"]
    #[doc = "      \"description\": \"Size of memory allocated to the Instance, in MiB.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"Human-readable name of the Instance.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"vcpus\": {"]
    #[doc = "      \"description\": \"Number of vCPUs to be allocated to the Instance.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint8\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceProperties {
        #[doc = "ID of the bootrom used to initialize this Instance."]
        pub bootrom_id: ::uuid::Uuid,
        #[doc = "Free-form text description of an Instance."]
        pub description: ::std::string::String,
        #[doc = "Unique identifier for this Instance."]
        pub id: ::uuid::Uuid,
        #[doc = "ID of the image used to initialize this Instance."]
        pub image_id: ::uuid::Uuid,
        #[doc = "Size of memory allocated to the Instance, in MiB."]
        pub memory: u64,
        #[doc = "Human-readable name of the Instance."]
        pub name: ::std::string::String,
        #[doc = "Number of vCPUs to be allocated to the Instance."]
        pub vcpus: u8,
    }

    impl ::std::convert::From<&InstanceProperties> for InstanceProperties {
        fn from(value: &InstanceProperties) -> Self {
            value.clone()
        }
    }

    #[doc = "Current state of an Instance."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Current state of an Instance.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Creating\","]
    #[doc = "    \"Starting\","]
    #[doc = "    \"Running\","]
    #[doc = "    \"Stopping\","]
    #[doc = "    \"Stopped\","]
    #[doc = "    \"Rebooting\","]
    #[doc = "    \"Migrating\","]
    #[doc = "    \"Repairing\","]
    #[doc = "    \"Failed\","]
    #[doc = "    \"Destroyed\""]
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
    pub enum InstanceState {
        Creating,
        Starting,
        Running,
        Stopping,
        Stopped,
        Rebooting,
        Migrating,
        Repairing,
        Failed,
        Destroyed,
    }

    impl ::std::convert::From<&Self> for InstanceState {
        fn from(value: &InstanceState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for InstanceState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => f.write_str("Creating"),
                Self::Starting => f.write_str("Starting"),
                Self::Running => f.write_str("Running"),
                Self::Stopping => f.write_str("Stopping"),
                Self::Stopped => f.write_str("Stopped"),
                Self::Rebooting => f.write_str("Rebooting"),
                Self::Migrating => f.write_str("Migrating"),
                Self::Repairing => f.write_str("Repairing"),
                Self::Failed => f.write_str("Failed"),
                Self::Destroyed => f.write_str("Destroyed"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Creating" => Ok(Self::Creating),
                "Starting" => Ok(Self::Starting),
                "Running" => Ok(Self::Running),
                "Stopping" => Ok(Self::Stopping),
                "Stopped" => Ok(Self::Stopped),
                "Rebooting" => Ok(Self::Rebooting),
                "Migrating" => Ok(Self::Migrating),
                "Repairing" => Ok(Self::Repairing),
                "Failed" => Ok(Self::Failed),
                "Destroyed" => Ok(Self::Destroyed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`InstanceStateMonitorRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"gen\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"gen\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceStateMonitorRequest {
        pub gen: u64,
    }

    impl ::std::convert::From<&InstanceStateMonitorRequest> for InstanceStateMonitorRequest {
        fn from(value: &InstanceStateMonitorRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceStateMonitorResponse`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"gen\","]
    #[doc = "    \"state\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"gen\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/InstanceState\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceStateMonitorResponse {
        pub gen: u64,
        pub state: InstanceState,
    }

    impl ::std::convert::From<&InstanceStateMonitorResponse> for InstanceStateMonitorResponse {
        fn from(value: &InstanceStateMonitorResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "`InstanceStateRequested`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Run\","]
    #[doc = "    \"Stop\","]
    #[doc = "    \"Reboot\","]
    #[doc = "    \"MigrateStart\""]
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
    pub enum InstanceStateRequested {
        Run,
        Stop,
        Reboot,
        MigrateStart,
    }

    impl ::std::convert::From<&Self> for InstanceStateRequested {
        fn from(value: &InstanceStateRequested) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for InstanceStateRequested {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Run => f.write_str("Run"),
                Self::Stop => f.write_str("Stop"),
                Self::Reboot => f.write_str("Reboot"),
                Self::MigrateStart => f.write_str("MigrateStart"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceStateRequested {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Run" => Ok(Self::Run),
                "Stop" => Ok(Self::Stop),
                "Reboot" => Ok(Self::Reboot),
                "MigrateStart" => Ok(Self::MigrateStart),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`MigrationState`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Sync\","]
    #[doc = "    \"RamPush\","]
    #[doc = "    \"Pause\","]
    #[doc = "    \"RamPushDirty\","]
    #[doc = "    \"Device\","]
    #[doc = "    \"Arch\","]
    #[doc = "    \"Resume\","]
    #[doc = "    \"RamPull\","]
    #[doc = "    \"Finish\","]
    #[doc = "    \"Error\""]
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
    pub enum MigrationState {
        Sync,
        RamPush,
        Pause,
        RamPushDirty,
        Device,
        Arch,
        Resume,
        RamPull,
        Finish,
        Error,
    }

    impl ::std::convert::From<&Self> for MigrationState {
        fn from(value: &MigrationState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for MigrationState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Sync => f.write_str("Sync"),
                Self::RamPush => f.write_str("RamPush"),
                Self::Pause => f.write_str("Pause"),
                Self::RamPushDirty => f.write_str("RamPushDirty"),
                Self::Device => f.write_str("Device"),
                Self::Arch => f.write_str("Arch"),
                Self::Resume => f.write_str("Resume"),
                Self::RamPull => f.write_str("RamPull"),
                Self::Finish => f.write_str("Finish"),
                Self::Error => f.write_str("Error"),
            }
        }
    }

    impl ::std::str::FromStr for MigrationState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Sync" => Ok(Self::Sync),
                "RamPush" => Ok(Self::RamPush),
                "Pause" => Ok(Self::Pause),
                "RamPushDirty" => Ok(Self::RamPushDirty),
                "Device" => Ok(Self::Device),
                "Arch" => Ok(Self::Arch),
                "Resume" => Ok(Self::Resume),
                "RamPull" => Ok(Self::RamPull),
                "Finish" => Ok(Self::Finish),
                "Error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`NetworkInterface`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"attachment\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"attachment\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/NetworkInterfaceAttachmentState\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterface {
        pub attachment: NetworkInterfaceAttachmentState,
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&NetworkInterface> for NetworkInterface {
        fn from(value: &NetworkInterface) -> Self {
            value.clone()
        }
    }

    #[doc = "`NetworkInterfaceAttachmentState`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Detached\","]
    #[doc = "        \"Faulted\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"Attached\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"Attached\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Slot\""]
    #[doc = "        }"]
    #[doc = "      },"]
    #[doc = "      \"additionalProperties\": false"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum NetworkInterfaceAttachmentState {
        Detached,
        Faulted,
        Attached(Slot),
    }

    impl ::std::convert::From<&Self> for NetworkInterfaceAttachmentState {
        fn from(value: &NetworkInterfaceAttachmentState) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<Slot> for NetworkInterfaceAttachmentState {
        fn from(value: Slot) -> Self {
            Self::Attached(value)
        }
    }

    #[doc = "`NetworkInterfaceRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"name\","]
    #[doc = "    \"slot\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"slot\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Slot\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceRequest {
        pub name: ::std::string::String,
        pub slot: Slot,
    }

    impl ::std::convert::From<&NetworkInterfaceRequest> for NetworkInterfaceRequest {
        fn from(value: &NetworkInterfaceRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "A stable index which is translated by Propolis into a PCI BDF, visible to the guest."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A stable index which is translated by Propolis into a PCI BDF, visible to the guest.\","]
    #[doc = "  \"type\": \"integer\","]
    #[doc = "  \"format\": \"uint8\","]
    #[doc = "  \"minimum\": 0.0"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Slot(pub u8);
    impl ::std::ops::Deref for Slot {
        type Target = u8;
        fn deref(&self) -> &u8 {
            &self.0
        }
    }

    impl ::std::convert::From<Slot> for u8 {
        fn from(value: Slot) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Slot> for Slot {
        fn from(value: &Slot) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<u8> for Slot {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for Slot {
        type Err = <u8 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for Slot {
        type Error = <u8 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Slot {
        type Error = <u8 as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Slot {
        type Error = <u8 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for Slot {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    #[doc = "`VolumeConstructionRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"block_size\","]
    #[doc = "        \"id\","]
    #[doc = "        \"sub_volumes\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"block_size\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"uint64\","]
    #[doc = "          \"minimum\": 0.0"]
    #[doc = "        },"]
    #[doc = "        \"id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"read_only_parent\": {"]
    #[doc = "          \"oneOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"type\": \"null\""]
    #[doc = "            },"]
    #[doc = "            {"]
    #[doc = "              \"allOf\": ["]
    #[doc = "                {"]
    #[doc = "                  \"$ref\": \"#/components/schemas/VolumeConstructionRequest\""]
    #[doc = "                }"]
    #[doc = "              ]"]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"sub_volumes\": {"]
    #[doc = "          \"type\": \"array\","]
    #[doc = "          \"items\": {"]
    #[doc = "            \"$ref\": \"#/components/schemas/VolumeConstructionRequest\""]
    #[doc = "          }"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"volume\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"block_size\","]
    #[doc = "        \"id\","]
    #[doc = "        \"type\","]
    #[doc = "        \"url\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"block_size\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"uint64\","]
    #[doc = "          \"minimum\": 0.0"]
    #[doc = "        },"]
    #[doc = "        \"id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"url\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"url\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"block_size\","]
    #[doc = "        \"gen\","]
    #[doc = "        \"opts\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"block_size\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"uint64\","]
    #[doc = "          \"minimum\": 0.0"]
    #[doc = "        },"]
    #[doc = "        \"gen\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"uint64\","]
    #[doc = "          \"minimum\": 0.0"]
    #[doc = "        },"]
    #[doc = "        \"opts\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/CrucibleOpts\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"region\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"block_size\","]
    #[doc = "        \"id\","]
    #[doc = "        \"path\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"block_size\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"uint64\","]
    #[doc = "          \"minimum\": 0.0"]
    #[doc = "        },"]
    #[doc = "        \"id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"path\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"file\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum VolumeConstructionRequest {
        #[serde(rename = "volume")]
        Volume {
            block_size: u64,
            id: ::uuid::Uuid,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            read_only_parent: ::std::option::Option<::std::boxed::Box<VolumeConstructionRequest>>,
            sub_volumes: ::std::vec::Vec<VolumeConstructionRequest>,
        },
        #[serde(rename = "url")]
        Url {
            block_size: u64,
            id: ::uuid::Uuid,
            url: ::std::string::String,
        },
        #[serde(rename = "region")]
        Region {
            block_size: u64,
            gen: u64,
            opts: CrucibleOpts,
        },
        #[serde(rename = "file")]
        File {
            block_size: u64,
            id: ::uuid::Uuid,
            path: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for VolumeConstructionRequest {
        fn from(value: &VolumeConstructionRequest) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
#[doc = "Client for Oxide Propolis Server API\n\nAPI for interacting with the Propolis hypervisor frontend.\n\nVersion: 0.0.1"]
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
        "0.0.1"
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
    #[doc = "Sends a `GET` request to `/instance`\n\n"]
    pub async fn instance_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::InstanceGetResponse>, Error<types::Error>> {
        let url = format!("{}/instance", self.baseurl,);
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
            operation_id: "instance_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `PUT` request to `/instance`\n\n"]
    pub async fn instance_ensure<'a>(
        &'a self,
        body: &'a types::InstanceEnsureRequest,
    ) -> Result<ResponseValue<types::InstanceEnsureResponse>, Error<types::Error>> {
        let url = format!("{}/instance", self.baseurl,);
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
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_ensure",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Issue a snapshot request to a crucible backend\n\nSends a `POST` request to `/instance/disk/{id}/snapshot/{snapshot_id}`\n\n"]
    pub async fn instance_issue_crucible_snapshot_request<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
        snapshot_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/instance/disk/{}/snapshot/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&snapshot_id.to_string()),
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_issue_crucible_snapshot_request",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/instance/migrate/status`\n\n"]
    pub async fn instance_migrate_status<'a>(
        &'a self,
        body: &'a types::InstanceMigrateStatusRequest,
    ) -> Result<ResponseValue<types::InstanceMigrateStatusResponse>, Error<types::Error>> {
        let url = format!("{}/instance/migrate/status", self.baseurl,);
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_migrate_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/instance/serial`\n\n"]
    pub async fn instance_serial<'a>(
        &'a self,
    ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
        let url = format!("{}/instance/serial", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .headers(header_map)
            .header(::reqwest::header::CONNECTION, "Upgrade")
            .header(::reqwest::header::UPGRADE, "websocket")
            .header(::reqwest::header::SEC_WEBSOCKET_VERSION, "13")
            .header(
                ::reqwest::header::SEC_WEBSOCKET_KEY,
                ::base64::Engine::encode(
                    &::base64::engine::general_purpose::STANDARD,
                    ::rand::random::<[u8; 16]>(),
                ),
            )
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_serial",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            101u16 => ResponseValue::upgrade(response).await,
            200..=299 => ResponseValue::upgrade(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `PUT` request to `/instance/state`\n\n"]
    pub async fn instance_state_put<'a>(
        &'a self,
        body: types::InstanceStateRequested,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/instance/state", self.baseurl,);
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
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_state_put",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `GET` request to `/instance/state-monitor`\n\n"]
    pub async fn instance_state_monitor<'a>(
        &'a self,
        body: &'a types::InstanceStateMonitorRequest,
    ) -> Result<ResponseValue<types::InstanceStateMonitorResponse>, Error<types::Error>> {
        let url = format!("{}/instance/state-monitor", self.baseurl,);
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_state_monitor",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
