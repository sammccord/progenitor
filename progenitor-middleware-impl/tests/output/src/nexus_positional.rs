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

    #[doc = "Describes properties that should uniquely identify a Gimlet."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes properties that should uniquely identify a Gimlet.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"part\","]
    #[doc = "    \"revision\","]
    #[doc = "    \"serial\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"part\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"revision\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int64\""]
    #[doc = "    },"]
    #[doc = "    \"serial\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Baseboard {
        pub part: ::std::string::String,
        pub revision: i64,
        pub serial: ::std::string::String,
    }

    impl ::std::convert::From<&Baseboard> for Baseboard {
        fn from(value: &Baseboard) -> Self {
            value.clone()
        }
    }

    #[doc = "A type storing a range over `T`.\n\nThis type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom` types in the standard library. Those cover `(..end)`, `(start..end)`, and `(start..)` respectively."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A type storing a range over `T`.\\n\\nThis type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom` types in the standard library. Those cover `(..end)`, `(start..end)`, and `(start..)` respectively.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"A range unbounded below and exclusively above, `..end`.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"end\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"end\": {"]
    #[doc = "          \"type\": \"number\","]
    #[doc = "          \"format\": \"double\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"range_to\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"A range bounded inclusively below and exclusively above, `start..end`.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"end\","]
    #[doc = "        \"start\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"end\": {"]
    #[doc = "          \"type\": \"number\","]
    #[doc = "          \"format\": \"double\""]
    #[doc = "        },"]
    #[doc = "        \"start\": {"]
    #[doc = "          \"type\": \"number\","]
    #[doc = "          \"format\": \"double\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"range\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"A range bounded inclusively below and unbounded above, `start..`.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"start\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"start\": {"]
    #[doc = "          \"type\": \"number\","]
    #[doc = "          \"format\": \"double\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"range_from\""]
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
    pub enum BinRangedouble {
        #[doc = "A range unbounded below and exclusively above, `..end`."]
        #[serde(rename = "range_to")]
        RangeTo { end: f64 },
        #[doc = "A range bounded inclusively below and exclusively above, `start..end`."]
        #[serde(rename = "range")]
        Range { end: f64, start: f64 },
        #[doc = "A range bounded inclusively below and unbounded above, `start..`."]
        #[serde(rename = "range_from")]
        RangeFrom { start: f64 },
    }

    impl ::std::convert::From<&Self> for BinRangedouble {
        fn from(value: &BinRangedouble) -> Self {
            value.clone()
        }
    }

    #[doc = "A type storing a range over `T`.\n\nThis type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom` types in the standard library. Those cover `(..end)`, `(start..end)`, and `(start..)` respectively."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A type storing a range over `T`.\\n\\nThis type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom` types in the standard library. Those cover `(..end)`, `(start..end)`, and `(start..)` respectively.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"A range unbounded below and exclusively above, `..end`.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"end\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"end\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"int64\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"range_to\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"A range bounded inclusively below and exclusively above, `start..end`.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"end\","]
    #[doc = "        \"start\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"end\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"int64\""]
    #[doc = "        },"]
    #[doc = "        \"start\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"int64\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"range\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"A range bounded inclusively below and unbounded above, `start..`.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"start\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"start\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"int64\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"range_from\""]
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
    pub enum BinRangeint64 {
        #[doc = "A range unbounded below and exclusively above, `..end`."]
        #[serde(rename = "range_to")]
        RangeTo { end: i64 },
        #[doc = "A range bounded inclusively below and exclusively above, `start..end`."]
        #[serde(rename = "range")]
        Range { end: i64, start: i64 },
        #[doc = "A range bounded inclusively below and unbounded above, `start..`."]
        #[serde(rename = "range_from")]
        RangeFrom { start: i64 },
    }

    impl ::std::convert::From<&Self> for BinRangeint64 {
        fn from(value: &BinRangeint64) -> Self {
            value.clone()
        }
    }

    #[doc = "Type storing bin edges and a count of samples within it."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Type storing bin edges and a count of samples within it.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"count\","]
    #[doc = "    \"range\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"count\": {"]
    #[doc = "      \"description\": \"The total count of samples in this bin.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"range\": {"]
    #[doc = "      \"description\": \"The range of the support covered by this bin.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/BinRangedouble\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Bindouble {
        #[doc = "The total count of samples in this bin."]
        pub count: u64,
        #[doc = "The range of the support covered by this bin."]
        pub range: BinRangedouble,
    }

    impl ::std::convert::From<&Bindouble> for Bindouble {
        fn from(value: &Bindouble) -> Self {
            value.clone()
        }
    }

    #[doc = "Type storing bin edges and a count of samples within it."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Type storing bin edges and a count of samples within it.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"count\","]
    #[doc = "    \"range\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"count\": {"]
    #[doc = "      \"description\": \"The total count of samples in this bin.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"range\": {"]
    #[doc = "      \"description\": \"The range of the support covered by this bin.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/BinRangeint64\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Binint64 {
        #[doc = "The total count of samples in this bin."]
        pub count: u64,
        #[doc = "The range of the support covered by this bin."]
        pub range: BinRangeint64,
    }

    impl ::std::convert::From<&Binint64> for Binint64 {
        fn from(value: &Binint64) -> Self {
            value.clone()
        }
    }

    #[doc = "`BlockSize`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"disk block size in bytes\","]
    #[doc = "  \"type\": \"integer\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    512,"]
    #[doc = "    2048,"]
    #[doc = "    4096"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct BlockSize(i64);
    impl ::std::ops::Deref for BlockSize {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl ::std::convert::From<BlockSize> for i64 {
        fn from(value: BlockSize) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&BlockSize> for BlockSize {
        fn from(value: &BlockSize) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::TryFrom<i64> for BlockSize {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![512_i64, 2048_i64, 4096_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for BlockSize {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    #[doc = "A count of bytes, typically used either for memory or storage capacity\n\nThe maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A count of bytes, typically used either for memory or storage capacity\\n\\nThe maximum supported byte count is [`i64::MAX`].  This makes it somewhat inconvenient to define constructors: a u32 constructor can be infallible, but an i64 constructor can fail (if the value is negative) and a u64 constructor can fail (if the value is larger than i64::MAX).  We provide all of these for consumers' convenience.\","]
    #[doc = "  \"type\": \"integer\","]
    #[doc = "  \"format\": \"uint64\","]
    #[doc = "  \"minimum\": 0.0"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ByteCount(pub u64);
    impl ::std::ops::Deref for ByteCount {
        type Target = u64;
        fn deref(&self) -> &u64 {
            &self.0
        }
    }

    impl ::std::convert::From<ByteCount> for u64 {
        fn from(value: ByteCount) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&ByteCount> for ByteCount {
        fn from(value: &ByteCount) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<u64> for ByteCount {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for ByteCount {
        type Err = <u64 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for ByteCount {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for ByteCount {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for ByteCount {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for ByteCount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    #[doc = "Client view of a [`Certificate`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Certificate`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"service\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"service\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ServiceUsingCertificate\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Certificate {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        pub service: ServiceUsingCertificate,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Certificate> for Certificate {
        fn from(value: &Certificate) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`Certificate`](crate::external_api::views::Certificate)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`Certificate`](crate::external_api::views::Certificate)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"cert\","]
    #[doc = "    \"description\","]
    #[doc = "    \"key\","]
    #[doc = "    \"name\","]
    #[doc = "    \"service\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"cert\": {"]
    #[doc = "      \"description\": \"PEM file containing public certificate chain\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"integer\","]
    #[doc = "        \"format\": \"uint8\","]
    #[doc = "        \"minimum\": 0.0"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"description\": \"PEM file containing private key\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"integer\","]
    #[doc = "        \"format\": \"uint8\","]
    #[doc = "        \"minimum\": 0.0"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"service\": {"]
    #[doc = "      \"description\": \"The service using this certificate\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ServiceUsingCertificate\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CertificateCreate {
        #[doc = "PEM file containing public certificate chain"]
        pub cert: ::std::vec::Vec<u8>,
        pub description: ::std::string::String,
        #[doc = "PEM file containing private key"]
        pub key: ::std::vec::Vec<u8>,
        pub name: Name,
        #[doc = "The service using this certificate"]
        pub service: ServiceUsingCertificate,
    }

    impl ::std::convert::From<&CertificateCreate> for CertificateCreate {
        fn from(value: &CertificateCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Certificate\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CertificateResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Certificate>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&CertificateResultsPage> for CertificateResultsPage {
        fn from(value: &CertificateResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Identity-related metadata that's included in \"asset\" public API objects (which generally have no name or description)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Identity-related metadata that's included in \\\"asset\\\" public API objects (which generally have no name or description)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"component_type\","]
    #[doc = "    \"id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"component_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/UpdateableComponentType\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ComponentUpdate {
        pub component_type: UpdateableComponentType,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl ::std::convert::From<&ComponentUpdate> for ComponentUpdate {
        fn from(value: &ComponentUpdate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ComponentUpdate\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ComponentUpdateResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<ComponentUpdate>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&ComponentUpdateResultsPage> for ComponentUpdateResultsPage {
        fn from(value: &ComponentUpdateResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "A cumulative or counter data type."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A cumulative or counter data type.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"start_time\","]
    #[doc = "    \"value\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"start_time\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"value\": {"]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Cumulativedouble {
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub value: f64,
    }

    impl ::std::convert::From<&Cumulativedouble> for Cumulativedouble {
        fn from(value: &Cumulativedouble) -> Self {
            value.clone()
        }
    }

    #[doc = "A cumulative or counter data type."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A cumulative or counter data type.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"start_time\","]
    #[doc = "    \"value\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"start_time\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"value\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int64\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Cumulativeint64 {
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub value: i64,
    }

    impl ::std::convert::From<&Cumulativeint64> for Cumulativeint64 {
        fn from(value: &Cumulativeint64) -> Self {
            value.clone()
        }
    }

    #[doc = "A `Datum` is a single sampled data point from a metric."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A `Datum` is a single sampled data point from a metric.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"type\": \"boolean\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"bool\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"type\": \"integer\","]
    #[doc = "          \"format\": \"int64\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"i64\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"type\": \"number\","]
    #[doc = "          \"format\": \"double\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"f64\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"string\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"type\": \"array\","]
    #[doc = "          \"items\": {"]
    #[doc = "            \"type\": \"integer\","]
    #[doc = "            \"format\": \"uint8\","]
    #[doc = "            \"minimum\": 0.0"]
    #[doc = "          }"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"bytes\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Cumulativeint64\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"cumulative_i64\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Cumulativedouble\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"cumulative_f64\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Histogramint64\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"histogram_i64\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"datum\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"datum\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Histogramdouble\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"histogram_f64\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "datum")]
    pub enum Datum {
        #[serde(rename = "bool")]
        Bool(bool),
        #[serde(rename = "i64")]
        I64(i64),
        #[serde(rename = "f64")]
        F64(f64),
        #[serde(rename = "string")]
        String(::std::string::String),
        #[serde(rename = "bytes")]
        Bytes(::std::vec::Vec<u8>),
        #[serde(rename = "cumulative_i64")]
        CumulativeI64(Cumulativeint64),
        #[serde(rename = "cumulative_f64")]
        CumulativeF64(Cumulativedouble),
        #[serde(rename = "histogram_i64")]
        HistogramI64(Histogramint64),
        #[serde(rename = "histogram_f64")]
        HistogramF64(Histogramdouble),
    }

    impl ::std::convert::From<&Self> for Datum {
        fn from(value: &Datum) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<bool> for Datum {
        fn from(value: bool) -> Self {
            Self::Bool(value)
        }
    }

    impl ::std::convert::From<i64> for Datum {
        fn from(value: i64) -> Self {
            Self::I64(value)
        }
    }

    impl ::std::convert::From<f64> for Datum {
        fn from(value: f64) -> Self {
            Self::F64(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<u8>> for Datum {
        fn from(value: ::std::vec::Vec<u8>) -> Self {
            Self::Bytes(value)
        }
    }

    impl ::std::convert::From<Cumulativeint64> for Datum {
        fn from(value: Cumulativeint64) -> Self {
            Self::CumulativeI64(value)
        }
    }

    impl ::std::convert::From<Cumulativedouble> for Datum {
        fn from(value: Cumulativedouble) -> Self {
            Self::CumulativeF64(value)
        }
    }

    impl ::std::convert::From<Histogramint64> for Datum {
        fn from(value: Histogramint64) -> Self {
            Self::HistogramI64(value)
        }
    }

    impl ::std::convert::From<Histogramdouble> for Datum {
        fn from(value: Histogramdouble) -> Self {
            Self::HistogramF64(value)
        }
    }

    #[doc = "The type of an individual datum of a metric."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The type of an individual datum of a metric.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"bool\","]
    #[doc = "    \"i64\","]
    #[doc = "    \"f64\","]
    #[doc = "    \"string\","]
    #[doc = "    \"bytes\","]
    #[doc = "    \"cumulative_i64\","]
    #[doc = "    \"cumulative_f64\","]
    #[doc = "    \"histogram_i64\","]
    #[doc = "    \"histogram_f64\""]
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
    pub enum DatumType {
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "i64")]
        I64,
        #[serde(rename = "f64")]
        F64,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "bytes")]
        Bytes,
        #[serde(rename = "cumulative_i64")]
        CumulativeI64,
        #[serde(rename = "cumulative_f64")]
        CumulativeF64,
        #[serde(rename = "histogram_i64")]
        HistogramI64,
        #[serde(rename = "histogram_f64")]
        HistogramF64,
    }

    impl ::std::convert::From<&Self> for DatumType {
        fn from(value: &DatumType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for DatumType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bool => f.write_str("bool"),
                Self::I64 => f.write_str("i64"),
                Self::F64 => f.write_str("f64"),
                Self::String => f.write_str("string"),
                Self::Bytes => f.write_str("bytes"),
                Self::CumulativeI64 => f.write_str("cumulative_i64"),
                Self::CumulativeF64 => f.write_str("cumulative_f64"),
                Self::HistogramI64 => f.write_str("histogram_i64"),
                Self::HistogramF64 => f.write_str("histogram_f64"),
            }
        }
    }

    impl ::std::str::FromStr for DatumType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "bool" => Ok(Self::Bool),
                "i64" => Ok(Self::I64),
                "f64" => Ok(Self::F64),
                "string" => Ok(Self::String),
                "bytes" => Ok(Self::Bytes),
                "cumulative_i64" => Ok(Self::CumulativeI64),
                "cumulative_f64" => Ok(Self::CumulativeF64),
                "histogram_i64" => Ok(Self::HistogramI64),
                "histogram_f64" => Ok(Self::HistogramF64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for DatumType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DatumType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DatumType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`DerEncodedKeyPair`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"private_key\","]
    #[doc = "    \"public_cert\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"private_key\": {"]
    #[doc = "      \"description\": \"request signing private key (base64 encoded der file)\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"public_cert\": {"]
    #[doc = "      \"description\": \"request signing public certificate (base64 encoded der file)\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DerEncodedKeyPair {
        #[doc = "request signing private key (base64 encoded der file)"]
        pub private_key: ::std::string::String,
        #[doc = "request signing public certificate (base64 encoded der file)"]
        pub public_cert: ::std::string::String,
    }

    impl ::std::convert::From<&DerEncodedKeyPair> for DerEncodedKeyPair {
        fn from(value: &DerEncodedKeyPair) -> Self {
            value.clone()
        }
    }

    #[doc = "`DeviceAccessTokenRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"client_id\","]
    #[doc = "    \"device_code\","]
    #[doc = "    \"grant_type\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"client_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"device_code\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"grant_type\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeviceAccessTokenRequest {
        pub client_id: ::uuid::Uuid,
        pub device_code: ::std::string::String,
        pub grant_type: ::std::string::String,
    }

    impl ::std::convert::From<&DeviceAccessTokenRequest> for DeviceAccessTokenRequest {
        fn from(value: &DeviceAccessTokenRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "`DeviceAuthRequest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"client_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"client_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeviceAuthRequest {
        pub client_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&DeviceAuthRequest> for DeviceAuthRequest {
        fn from(value: &DeviceAuthRequest) -> Self {
            value.clone()
        }
    }

    #[doc = "`DeviceAuthVerify`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"user_code\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"user_code\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeviceAuthVerify {
        pub user_code: ::std::string::String,
    }

    impl ::std::convert::From<&DeviceAuthVerify> for DeviceAuthVerify {
        fn from(value: &DeviceAuthVerify) -> Self {
            value.clone()
        }
    }

    #[doc = "`Digest`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"sha256\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum Digest {
        #[serde(rename = "sha256")]
        Sha256(::std::string::String),
    }

    impl ::std::convert::From<&Self> for Digest {
        fn from(value: &Digest) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`Disk`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Disk`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"block_size\","]
    #[doc = "    \"description\","]
    #[doc = "    \"device_path\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"project_id\","]
    #[doc = "    \"size\","]
    #[doc = "    \"state\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"block_size\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"device_path\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"image_id\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"project_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "    },"]
    #[doc = "    \"snapshot_id\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/DiskState\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Disk {
        pub block_size: ByteCount,
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        pub device_path: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub image_id: ::std::option::Option<::uuid::Uuid>,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        pub project_id: ::uuid::Uuid,
        pub size: ByteCount,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub snapshot_id: ::std::option::Option<::uuid::Uuid>,
        pub state: DiskState,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Disk> for Disk {
        fn from(value: &Disk) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`Disk`](omicron_common::api::external::Disk)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"disk_source\","]
    #[doc = "    \"name\","]
    #[doc = "    \"size\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"disk_source\": {"]
    #[doc = "      \"description\": \"initial source for this disk\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/DiskSource\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"description\": \"total size of the Disk in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskCreate {
        pub description: ::std::string::String,
        #[doc = "initial source for this disk"]
        pub disk_source: DiskSource,
        pub name: Name,
        #[doc = "total size of the Disk in bytes"]
        pub size: ByteCount,
    }

    impl ::std::convert::From<&DiskCreate> for DiskCreate {
        fn from(value: &DiskCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "TODO-v1: Delete this Parameters for the [`Disk`](omicron_common::api::external::Disk) to be attached or detached to an instance"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"TODO-v1: Delete this Parameters for the [`Disk`](omicron_common::api::external::Disk) to be attached or detached to an instance\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskIdentifier {
        pub name: Name,
    }

    impl ::std::convert::From<&DiskIdentifier> for DiskIdentifier {
        fn from(value: &DiskIdentifier) -> Self {
            value.clone()
        }
    }

    #[doc = "`DiskMetricName`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"activated\","]
    #[doc = "    \"flush\","]
    #[doc = "    \"read\","]
    #[doc = "    \"read_bytes\","]
    #[doc = "    \"write\","]
    #[doc = "    \"write_bytes\""]
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
    pub enum DiskMetricName {
        #[serde(rename = "activated")]
        Activated,
        #[serde(rename = "flush")]
        Flush,
        #[serde(rename = "read")]
        Read,
        #[serde(rename = "read_bytes")]
        ReadBytes,
        #[serde(rename = "write")]
        Write,
        #[serde(rename = "write_bytes")]
        WriteBytes,
    }

    impl ::std::convert::From<&Self> for DiskMetricName {
        fn from(value: &DiskMetricName) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for DiskMetricName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Activated => f.write_str("activated"),
                Self::Flush => f.write_str("flush"),
                Self::Read => f.write_str("read"),
                Self::ReadBytes => f.write_str("read_bytes"),
                Self::Write => f.write_str("write"),
                Self::WriteBytes => f.write_str("write_bytes"),
            }
        }
    }

    impl ::std::str::FromStr for DiskMetricName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "activated" => Ok(Self::Activated),
                "flush" => Ok(Self::Flush),
                "read" => Ok(Self::Read),
                "read_bytes" => Ok(Self::ReadBytes),
                "write" => Ok(Self::Write),
                "write_bytes" => Ok(Self::WriteBytes),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for DiskMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for DiskMetricName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DiskMetricName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`DiskPath`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"disk\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"disk\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/NameOrId\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskPath {
        pub disk: NameOrId,
    }

    impl ::std::convert::From<&DiskPath> for DiskPath {
        fn from(value: &DiskPath) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Disk\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Disk>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&DiskResultsPage> for DiskResultsPage {
        fn from(value: &DiskResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Different sources for a disk"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Different sources for a disk\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Create a blank disk\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"block_size\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"block_size\": {"]
    #[doc = "          \"description\": \"size of blocks for this Disk. valid values are: 512, 2048, or 4096\","]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/BlockSize\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"blank\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Create a disk from a disk snapshot\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"snapshot_id\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"snapshot_id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"snapshot\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Create a disk from a project image\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"image_id\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"image_id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"image\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Create a disk from a global image\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"image_id\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"image_id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"global_image\""]
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
    pub enum DiskSource {
        #[doc = "Create a blank disk"]
        #[serde(rename = "blank")]
        Blank {
            #[doc = "size of blocks for this Disk. valid values are: 512, 2048, or 4096"]
            block_size: BlockSize,
        },
        #[doc = "Create a disk from a disk snapshot"]
        #[serde(rename = "snapshot")]
        Snapshot { snapshot_id: ::uuid::Uuid },
        #[doc = "Create a disk from a project image"]
        #[serde(rename = "image")]
        Image { image_id: ::uuid::Uuid },
        #[doc = "Create a disk from a global image"]
        #[serde(rename = "global_image")]
        GlobalImage { image_id: ::uuid::Uuid },
    }

    impl ::std::convert::From<&Self> for DiskSource {
        fn from(value: &DiskSource) -> Self {
            value.clone()
        }
    }

    #[doc = "State of a Disk (primarily: attached or not)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"State of a Disk (primarily: attached or not)\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk is being initialized\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"creating\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk is ready but detached from any Instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"detached\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk is being attached to the given Instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"instance\","]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"instance\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"attaching\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk is attached to the given Instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"instance\","]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"instance\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"attached\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk is being detached from the given Instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"instance\","]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"instance\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"detaching\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk has been destroyed\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"destroyed\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Disk is unavailable\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"faulted\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "state", content = "instance")]
    pub enum DiskState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "detached")]
        Detached,
        #[doc = "Disk is being attached to the given Instance"]
        #[serde(rename = "attaching")]
        Attaching(::uuid::Uuid),
        #[doc = "Disk is attached to the given Instance"]
        #[serde(rename = "attached")]
        Attached(::uuid::Uuid),
        #[doc = "Disk is being detached from the given Instance"]
        #[serde(rename = "detaching")]
        Detaching(::uuid::Uuid),
        #[serde(rename = "destroyed")]
        Destroyed,
        #[serde(rename = "faulted")]
        Faulted,
    }

    impl ::std::convert::From<&Self> for DiskState {
        fn from(value: &DiskState) -> Self {
            value.clone()
        }
    }

    #[doc = "OS image distribution"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"OS image distribution\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"name\","]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"The name of the distribution (e.g. \\\"alpine\\\" or \\\"ubuntu\\\")\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"description\": \"The version of the distribution (e.g. \\\"3.10\\\" or \\\"18.04\\\")\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Distribution {
        #[doc = "The name of the distribution (e.g. \"alpine\" or \"ubuntu\")"]
        pub name: Name,
        #[doc = "The version of the distribution (e.g. \"3.10\" or \"18.04\")"]
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&Distribution> for Distribution {
        fn from(value: &Distribution) -> Self {
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

    #[doc = "`ExternalIp`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"ip\","]
    #[doc = "    \"kind\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"ip\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"ip\""]
    #[doc = "    },"]
    #[doc = "    \"kind\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/IpKind\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExternalIp {
        pub ip: ::std::net::IpAddr,
        pub kind: IpKind,
    }

    impl ::std::convert::From<&ExternalIp> for ExternalIp {
        fn from(value: &ExternalIp) -> Self {
            value.clone()
        }
    }

    #[doc = "Parameters for creating an external IP address for instances."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Parameters for creating an external IP address for instances.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"An IP address providing both inbound and outbound access. The address is automatically-assigned from the provided IP Pool, or all available pools if not specified.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"pool_name\": {"]
    #[doc = "          \"oneOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"type\": \"null\""]
    #[doc = "            },"]
    #[doc = "            {"]
    #[doc = "              \"allOf\": ["]
    #[doc = "                {"]
    #[doc = "                  \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "                }"]
    #[doc = "              ]"]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ephemeral\""]
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
    pub enum ExternalIpCreate {
        #[doc = "An IP address providing both inbound and outbound access. The address is automatically-assigned from the provided IP Pool, or all available pools if not specified."]
        #[serde(rename = "ephemeral")]
        Ephemeral {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            pool_name: ::std::option::Option<Name>,
        },
    }

    impl ::std::convert::From<&Self> for ExternalIpCreate {
        fn from(value: &ExternalIpCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ExternalIp\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExternalIpResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<ExternalIp>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&ExternalIpResultsPage> for ExternalIpResultsPage {
        fn from(value: &ExternalIpResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "The name and type information for a field of a timeseries schema."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The name and type information for a field of a timeseries schema.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"name\","]
    #[doc = "    \"source\","]
    #[doc = "    \"ty\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"source\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/FieldSource\""]
    #[doc = "    },"]
    #[doc = "    \"ty\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/FieldType\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FieldSchema {
        pub name: ::std::string::String,
        pub source: FieldSource,
        pub ty: FieldType,
    }

    impl ::std::convert::From<&FieldSchema> for FieldSchema {
        fn from(value: &FieldSchema) -> Self {
            value.clone()
        }
    }

    #[doc = "The source from which a field is derived, the target or metric."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The source from which a field is derived, the target or metric.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"target\","]
    #[doc = "    \"metric\""]
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
    pub enum FieldSource {
        #[serde(rename = "target")]
        Target,
        #[serde(rename = "metric")]
        Metric,
    }

    impl ::std::convert::From<&Self> for FieldSource {
        fn from(value: &FieldSource) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FieldSource {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Target => f.write_str("target"),
                Self::Metric => f.write_str("metric"),
            }
        }
    }

    impl ::std::str::FromStr for FieldSource {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "target" => Ok(Self::Target),
                "metric" => Ok(Self::Metric),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FieldSource {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FieldSource {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FieldSource {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "The `FieldType` identifies the data type of a target or metric field."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The `FieldType` identifies the data type of a target or metric field.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"string\","]
    #[doc = "    \"i64\","]
    #[doc = "    \"ip_addr\","]
    #[doc = "    \"uuid\","]
    #[doc = "    \"bool\""]
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
    pub enum FieldType {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "i64")]
        I64,
        #[serde(rename = "ip_addr")]
        IpAddr,
        #[serde(rename = "uuid")]
        Uuid,
        #[serde(rename = "bool")]
        Bool,
    }

    impl ::std::convert::From<&Self> for FieldType {
        fn from(value: &FieldType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FieldType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::String => f.write_str("string"),
                Self::I64 => f.write_str("i64"),
                Self::IpAddr => f.write_str("ip_addr"),
                Self::Uuid => f.write_str("uuid"),
                Self::Bool => f.write_str("bool"),
            }
        }
    }

    impl ::std::str::FromStr for FieldType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "string" => Ok(Self::String),
                "i64" => Ok(Self::I64),
                "ip_addr" => Ok(Self::IpAddr),
                "uuid" => Ok(Self::Uuid),
                "bool" => Ok(Self::Bool),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FieldType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FieldType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FieldType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`FleetRole`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"admin\","]
    #[doc = "    \"collaborator\","]
    #[doc = "    \"viewer\""]
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
    pub enum FleetRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl ::std::convert::From<&Self> for FleetRole {
        fn from(value: &FleetRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FleetRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => f.write_str("admin"),
                Self::Collaborator => f.write_str("collaborator"),
                Self::Viewer => f.write_str("viewer"),
            }
        }
    }

    impl ::std::str::FromStr for FleetRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FleetRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FleetRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FleetRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a [`Policy`], which describes how this resource may be accessed\n\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Policy`], which describes how this resource may be accessed\\n\\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"role_assignments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"role_assignments\": {"]
    #[doc = "      \"description\": \"Roles directly assigned on this resource\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/FleetRoleRoleAssignment\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FleetRolePolicy {
        #[doc = "Roles directly assigned on this resource"]
        pub role_assignments: ::std::vec::Vec<FleetRoleRoleAssignment>,
    }

    impl ::std::convert::From<&FleetRolePolicy> for FleetRolePolicy {
        fn from(value: &FleetRolePolicy) -> Self {
            value.clone()
        }
    }

    #[doc = "Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\n\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\\n\\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"identity_id\","]
    #[doc = "    \"identity_type\","]
    #[doc = "    \"role_name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"identity_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"identity_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/IdentityType\""]
    #[doc = "    },"]
    #[doc = "    \"role_name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/FleetRole\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FleetRoleRoleAssignment {
        pub identity_id: ::uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: FleetRole,
    }

    impl ::std::convert::From<&FleetRoleRoleAssignment> for FleetRoleRoleAssignment {
        fn from(value: &FleetRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of global Images"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of global Images\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"block_size\","]
    #[doc = "    \"description\","]
    #[doc = "    \"distribution\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"size\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"block_size\": {"]
    #[doc = "      \"description\": \"size of blocks in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"digest\": {"]
    #[doc = "      \"description\": \"Hash of the image contents, if applicable\","]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Digest\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"distribution\": {"]
    #[doc = "      \"description\": \"Image distribution\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"description\": \"total size in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"url\": {"]
    #[doc = "      \"description\": \"URL source of this image, if any\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"description\": \"Image version\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalImage {
        #[doc = "size of blocks in bytes"]
        pub block_size: ByteCount,
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "Hash of the image contents, if applicable"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub digest: ::std::option::Option<Digest>,
        #[doc = "Image distribution"]
        pub distribution: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "total size in bytes"]
        pub size: ByteCount,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "URL source of this image, if any"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        #[doc = "Image version"]
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&GlobalImage> for GlobalImage {
        fn from(value: &GlobalImage) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for an [`GlobalImage`](crate::external_api::views::GlobalImage)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for an [`GlobalImage`](crate::external_api::views::GlobalImage)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"block_size\","]
    #[doc = "    \"description\","]
    #[doc = "    \"distribution\","]
    #[doc = "    \"name\","]
    #[doc = "    \"source\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"block_size\": {"]
    #[doc = "      \"description\": \"block size in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/BlockSize\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"distribution\": {"]
    #[doc = "      \"description\": \"OS image distribution\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Distribution\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"source\": {"]
    #[doc = "      \"description\": \"The source of the image's contents.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ImageSource\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalImageCreate {
        #[doc = "block size in bytes"]
        pub block_size: BlockSize,
        pub description: ::std::string::String,
        #[doc = "OS image distribution"]
        pub distribution: Distribution,
        pub name: Name,
        #[doc = "The source of the image's contents."]
        pub source: ImageSource,
    }

    impl ::std::convert::From<&GlobalImageCreate> for GlobalImageCreate {
        fn from(value: &GlobalImageCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/GlobalImage\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalImageResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<GlobalImage>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&GlobalImageResultsPage> for GlobalImageResultsPage {
        fn from(value: &GlobalImageResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`Group`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Group`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"display_name\","]
    #[doc = "    \"id\","]
    #[doc = "    \"silo_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"display_name\": {"]
    #[doc = "      \"description\": \"Human-readable name that can identify the group\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"silo_id\": {"]
    #[doc = "      \"description\": \"Uuid of the silo to which this group belongs\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Group {
        #[doc = "Human-readable name that can identify the group"]
        pub display_name: ::std::string::String,
        pub id: ::uuid::Uuid,
        #[doc = "Uuid of the silo to which this group belongs"]
        pub silo_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&Group> for Group {
        fn from(value: &Group) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Group\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GroupResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Group>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&GroupResultsPage> for GroupResultsPage {
        fn from(value: &GroupResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "A simple type for managing a histogram metric.\n\nA histogram maintains the count of any number of samples, over a set of bins. Bins are specified on construction via their _left_ edges, inclusive. There can't be any \"gaps\" in the bins, and an additional bin may be added to the left, right, or both so that the bins extend to the entire range of the support.\n\nNote that any gaps, unsorted bins, or non-finite values will result in an error.\n\nExample ------- ```rust use oximeter::histogram::{BinRange, Histogram};\n\nlet edges = [0i64, 10, 20]; let mut hist = Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..) assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);\n\nlet data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range, BinRange::range(i64::MIN, 0)); // An additional bin for `..0` assert_eq!(data[0].count, 0); // Nothing is in this bin\n\nassert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```\n\nNotes -----\n\nHistograms may be constructed either from their left bin edges, or from a sequence of ranges. In either case, the left-most bin may be converted upon construction. In particular, if the left-most value is not equal to the minimum of the support, a new bin will be added from the minimum to that provided value. If the left-most value _is_ the support's minimum, because the provided bin was unbounded below, such as `(..0)`, then that bin will be converted into one bounded below, `(MIN..0)` in this case.\n\nThe short of this is that, most of the time, it shouldn't matter. If one specifies the extremes of the support as their bins, be aware that the left-most may be converted from a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the first bin of a histogram is _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In fact, every bin is one of those variants, the `BinRange::RangeTo` is only provided as a convenience during construction."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A simple type for managing a histogram metric.\\n\\nA histogram maintains the count of any number of samples, over a set of bins. Bins are specified on construction via their _left_ edges, inclusive. There can't be any \\\"gaps\\\" in the bins, and an additional bin may be added to the left, right, or both so that the bins extend to the entire range of the support.\\n\\nNote that any gaps, unsorted bins, or non-finite values will result in an error.\\n\\nExample ------- ```rust use oximeter::histogram::{BinRange, Histogram};\\n\\nlet edges = [0i64, 10, 20]; let mut hist = Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..) assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);\\n\\nlet data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range, BinRange::range(i64::MIN, 0)); // An additional bin for `..0` assert_eq!(data[0].count, 0); // Nothing is in this bin\\n\\nassert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```\\n\\nNotes -----\\n\\nHistograms may be constructed either from their left bin edges, or from a sequence of ranges. In either case, the left-most bin may be converted upon construction. In particular, if the left-most value is not equal to the minimum of the support, a new bin will be added from the minimum to that provided value. If the left-most value _is_ the support's minimum, because the provided bin was unbounded below, such as `(..0)`, then that bin will be converted into one bounded below, `(MIN..0)` in this case.\\n\\nThe short of this is that, most of the time, it shouldn't matter. If one specifies the extremes of the support as their bins, be aware that the left-most may be converted from a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the first bin of a histogram is _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In fact, every bin is one of those variants, the `BinRange::RangeTo` is only provided as a convenience during construction.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"bins\","]
    #[doc = "    \"n_samples\","]
    #[doc = "    \"start_time\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"bins\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Bindouble\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"n_samples\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
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
    pub struct Histogramdouble {
        pub bins: ::std::vec::Vec<Bindouble>,
        pub n_samples: u64,
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Histogramdouble> for Histogramdouble {
        fn from(value: &Histogramdouble) -> Self {
            value.clone()
        }
    }

    #[doc = "A simple type for managing a histogram metric.\n\nA histogram maintains the count of any number of samples, over a set of bins. Bins are specified on construction via their _left_ edges, inclusive. There can't be any \"gaps\" in the bins, and an additional bin may be added to the left, right, or both so that the bins extend to the entire range of the support.\n\nNote that any gaps, unsorted bins, or non-finite values will result in an error.\n\nExample ------- ```rust use oximeter::histogram::{BinRange, Histogram};\n\nlet edges = [0i64, 10, 20]; let mut hist = Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..) assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);\n\nlet data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range, BinRange::range(i64::MIN, 0)); // An additional bin for `..0` assert_eq!(data[0].count, 0); // Nothing is in this bin\n\nassert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```\n\nNotes -----\n\nHistograms may be constructed either from their left bin edges, or from a sequence of ranges. In either case, the left-most bin may be converted upon construction. In particular, if the left-most value is not equal to the minimum of the support, a new bin will be added from the minimum to that provided value. If the left-most value _is_ the support's minimum, because the provided bin was unbounded below, such as `(..0)`, then that bin will be converted into one bounded below, `(MIN..0)` in this case.\n\nThe short of this is that, most of the time, it shouldn't matter. If one specifies the extremes of the support as their bins, be aware that the left-most may be converted from a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the first bin of a histogram is _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In fact, every bin is one of those variants, the `BinRange::RangeTo` is only provided as a convenience during construction."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A simple type for managing a histogram metric.\\n\\nA histogram maintains the count of any number of samples, over a set of bins. Bins are specified on construction via their _left_ edges, inclusive. There can't be any \\\"gaps\\\" in the bins, and an additional bin may be added to the left, right, or both so that the bins extend to the entire range of the support.\\n\\nNote that any gaps, unsorted bins, or non-finite values will result in an error.\\n\\nExample ------- ```rust use oximeter::histogram::{BinRange, Histogram};\\n\\nlet edges = [0i64, 10, 20]; let mut hist = Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..) assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);\\n\\nlet data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range, BinRange::range(i64::MIN, 0)); // An additional bin for `..0` assert_eq!(data[0].count, 0); // Nothing is in this bin\\n\\nassert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```\\n\\nNotes -----\\n\\nHistograms may be constructed either from their left bin edges, or from a sequence of ranges. In either case, the left-most bin may be converted upon construction. In particular, if the left-most value is not equal to the minimum of the support, a new bin will be added from the minimum to that provided value. If the left-most value _is_ the support's minimum, because the provided bin was unbounded below, such as `(..0)`, then that bin will be converted into one bounded below, `(MIN..0)` in this case.\\n\\nThe short of this is that, most of the time, it shouldn't matter. If one specifies the extremes of the support as their bins, be aware that the left-most may be converted from a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the first bin of a histogram is _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In fact, every bin is one of those variants, the `BinRange::RangeTo` is only provided as a convenience during construction.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"bins\","]
    #[doc = "    \"n_samples\","]
    #[doc = "    \"start_time\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"bins\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Binint64\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"n_samples\": {"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
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
    pub struct Histogramint64 {
        pub bins: ::std::vec::Vec<Binint64>,
        pub n_samples: u64,
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Histogramint64> for Histogramint64 {
        fn from(value: &Histogramint64) -> Self {
            value.clone()
        }
    }

    #[doc = "Supported set of sort modes for scanning by id only.\n\nCurrently, we only support scanning in ascending order."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Supported set of sort modes for scanning by id only.\\n\\nCurrently, we only support scanning in ascending order.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"sort in increasing order of \\\"id\\\"\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"id_ascending\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum IdSortMode {
        #[doc = "sort in increasing order of \"id\""]
        #[serde(rename = "id_ascending")]
        IdAscending,
    }

    impl ::std::convert::From<&Self> for IdSortMode {
        fn from(value: &IdSortMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IdSortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::IdAscending => f.write_str("id_ascending"),
            }
        }
    }

    impl ::std::str::FromStr for IdSortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "id_ascending" => Ok(Self::IdAscending),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of an [`IdentityProvider`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of an [`IdentityProvider`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"provider_type\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"provider_type\": {"]
    #[doc = "      \"description\": \"Identity provider type\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/IdentityProviderType\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityProvider {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "Identity provider type"]
        pub provider_type: IdentityProviderType,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&IdentityProvider> for IdentityProvider {
        fn from(value: &IdentityProvider) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/IdentityProvider\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityProviderResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<IdentityProvider>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&IdentityProviderResultsPage> for IdentityProviderResultsPage {
        fn from(value: &IdentityProviderResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`IdentityProviderType`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"SAML identity provider\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"saml\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum IdentityProviderType {
        #[doc = "SAML identity provider"]
        #[serde(rename = "saml")]
        Saml,
    }

    impl ::std::convert::From<&Self> for IdentityProviderType {
        fn from(value: &IdentityProviderType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IdentityProviderType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Saml => f.write_str("saml"),
            }
        }
    }

    impl ::std::str::FromStr for IdentityProviderType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "saml" => Ok(Self::Saml),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Describes what kind of identity is described by an id"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes what kind of identity is described by an id\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"silo_user\","]
    #[doc = "    \"silo_group\""]
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
    pub enum IdentityType {
        #[serde(rename = "silo_user")]
        SiloUser,
        #[serde(rename = "silo_group")]
        SiloGroup,
    }

    impl ::std::convert::From<&Self> for IdentityType {
        fn from(value: &IdentityType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IdentityType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SiloUser => f.write_str("silo_user"),
                Self::SiloGroup => f.write_str("silo_group"),
            }
        }
    }

    impl ::std::str::FromStr for IdentityType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "silo_user" => Ok(Self::SiloUser),
                "silo_group" => Ok(Self::SiloGroup),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IdentityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IdentityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IdentityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`IdpMetadataSource`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"url\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
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
    #[doc = "        \"data\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"data\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"base64_encoded_xml\""]
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
    pub enum IdpMetadataSource {
        #[serde(rename = "url")]
        Url { url: ::std::string::String },
        #[serde(rename = "base64_encoded_xml")]
        Base64EncodedXml { data: ::std::string::String },
    }

    impl ::std::convert::From<&Self> for IdpMetadataSource {
        fn from(value: &IdpMetadataSource) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of project Images"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of project Images\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"block_size\","]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"project_id\","]
    #[doc = "    \"size\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"block_size\": {"]
    #[doc = "      \"description\": \"size of blocks in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"digest\": {"]
    #[doc = "      \"description\": \"Hash of the image contents, if applicable\","]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Digest\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"project_id\": {"]
    #[doc = "      \"description\": \"The project the disk belongs to\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"description\": \"total size in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"url\": {"]
    #[doc = "      \"description\": \"URL source of this image, if any\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"description\": \"Version of this, if any\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Image {
        #[doc = "size of blocks in bytes"]
        pub block_size: ByteCount,
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "Hash of the image contents, if applicable"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub digest: ::std::option::Option<Digest>,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "The project the disk belongs to"]
        pub project_id: ::uuid::Uuid,
        #[doc = "total size in bytes"]
        pub size: ByteCount,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "URL source of this image, if any"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
        #[doc = "Version of this, if any"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&Image> for Image {
        fn from(value: &Image) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for an [`Image`](crate::external_api::views::Image)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for an [`Image`](crate::external_api::views::Image)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"block_size\","]
    #[doc = "    \"description\","]
    #[doc = "    \"name\","]
    #[doc = "    \"source\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"block_size\": {"]
    #[doc = "      \"description\": \"block size in bytes\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/BlockSize\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"source\": {"]
    #[doc = "      \"description\": \"The source of the image's contents.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ImageSource\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImageCreate {
        #[doc = "block size in bytes"]
        pub block_size: BlockSize,
        pub description: ::std::string::String,
        pub name: Name,
        #[doc = "The source of the image's contents."]
        pub source: ImageSource,
    }

    impl ::std::convert::From<&ImageCreate> for ImageCreate {
        fn from(value: &ImageCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Image\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImageResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Image>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&ImageResultsPage> for ImageResultsPage {
        fn from(value: &ImageResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "The source of the underlying image."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The source of the underlying image.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"url\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
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
    #[doc = "        \"id\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"id\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"snapshot\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Boot the Alpine ISO that ships with the Propolis zone. Intended for development purposes only.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"you_can_boot_anything_as_long_as_its_alpine\""]
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
    pub enum ImageSource {
        #[serde(rename = "url")]
        Url { url: ::std::string::String },
        #[serde(rename = "snapshot")]
        Snapshot { id: ::uuid::Uuid },
        #[serde(rename = "you_can_boot_anything_as_long_as_its_alpine")]
        YouCanBootAnythingAsLongAsItsAlpine,
    }

    impl ::std::convert::From<&Self> for ImageSource {
        fn from(value: &ImageSource) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of an [`Instance`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of an [`Instance`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"hostname\","]
    #[doc = "    \"id\","]
    #[doc = "    \"memory\","]
    #[doc = "    \"name\","]
    #[doc = "    \"ncpus\","]
    #[doc = "    \"project_id\","]
    #[doc = "    \"run_state\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"time_run_state_updated\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"hostname\": {"]
    #[doc = "      \"description\": \"RFC1035-compliant hostname for the Instance.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"memory\": {"]
    #[doc = "      \"description\": \"memory allocated for this Instance\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ncpus\": {"]
    #[doc = "      \"description\": \"number of CPUs allocated for this Instance\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/InstanceCpuCount\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"project_id\": {"]
    #[doc = "      \"description\": \"id for the project containing this Instance\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"run_state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/InstanceState\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_run_state_updated\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Instance {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "RFC1035-compliant hostname for the Instance."]
        pub hostname: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "memory allocated for this Instance"]
        pub memory: ByteCount,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "number of CPUs allocated for this Instance"]
        pub ncpus: InstanceCpuCount,
        #[doc = "id for the project containing this Instance"]
        pub project_id: ::uuid::Uuid,
        pub run_state: InstanceState,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        pub time_run_state_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Instance> for Instance {
        fn from(value: &Instance) -> Self {
            value.clone()
        }
    }

    #[doc = "The number of CPUs in an Instance"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The number of CPUs in an Instance\","]
    #[doc = "  \"type\": \"integer\","]
    #[doc = "  \"format\": \"uint16\","]
    #[doc = "  \"minimum\": 0.0"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct InstanceCpuCount(pub u16);
    impl ::std::ops::Deref for InstanceCpuCount {
        type Target = u16;
        fn deref(&self) -> &u16 {
            &self.0
        }
    }

    impl ::std::convert::From<InstanceCpuCount> for u16 {
        fn from(value: InstanceCpuCount) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&InstanceCpuCount> for InstanceCpuCount {
        fn from(value: &InstanceCpuCount) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<u16> for InstanceCpuCount {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for InstanceCpuCount {
        type Err = <u16 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceCpuCount {
        type Error = <u16 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for InstanceCpuCount {
        type Error = <u16 as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for InstanceCpuCount {
        type Error = <u16 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for InstanceCpuCount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    #[doc = "Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for an [`Instance`](omicron_common::api::external::Instance)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"hostname\","]
    #[doc = "    \"memory\","]
    #[doc = "    \"name\","]
    #[doc = "    \"ncpus\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"disks\": {"]
    #[doc = "      \"description\": \"The disks to be created or attached for this instance.\","]
    #[doc = "      \"default\": [],"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/InstanceDiskAttachment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"external_ips\": {"]
    #[doc = "      \"description\": \"The external IP addresses provided to this instance.\\n\\nBy default, all instances have outbound connectivity, but no inbound connectivity. These external addresses can be used to provide a fixed, known IP address for making inbound connections to the instance.\","]
    #[doc = "      \"default\": [],"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ExternalIpCreate\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"hostname\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"memory\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"ncpus\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/InstanceCpuCount\""]
    #[doc = "    },"]
    #[doc = "    \"network_interfaces\": {"]
    #[doc = "      \"description\": \"The network interfaces to be created for this instance.\","]
    #[doc = "      \"default\": {"]
    #[doc = "        \"type\": \"default\""]
    #[doc = "      },"]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/InstanceNetworkInterfaceAttachment\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"start\": {"]
    #[doc = "      \"description\": \"Should this instance be started upon creation; true by default.\","]
    #[doc = "      \"default\": true,"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"user_data\": {"]
    #[doc = "      \"description\": \"User data for instance initialization systems (such as cloud-init). Must be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and / characters with padding). Maximum 32 KiB unencoded data.\","]
    #[doc = "      \"default\": \"\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"byte\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceCreate {
        pub description: ::std::string::String,
        #[doc = "The disks to be created or attached for this instance."]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub disks: ::std::vec::Vec<InstanceDiskAttachment>,
        #[doc = "The external IP addresses provided to this instance.\n\nBy default, all instances have outbound connectivity, but no inbound connectivity. These external addresses can be used to provide a fixed, known IP address for making inbound connections to the instance."]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub external_ips: ::std::vec::Vec<ExternalIpCreate>,
        pub hostname: ::std::string::String,
        pub memory: ByteCount,
        pub name: Name,
        pub ncpus: InstanceCpuCount,
        #[doc = "The network interfaces to be created for this instance."]
        #[serde(default = "defaults::instance_create_network_interfaces")]
        pub network_interfaces: InstanceNetworkInterfaceAttachment,
        #[doc = "Should this instance be started upon creation; true by default."]
        #[serde(default = "defaults::default_bool::<true>")]
        pub start: bool,
        #[doc = "User data for instance initialization systems (such as cloud-init). Must be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and / characters with padding). Maximum 32 KiB unencoded data."]
        #[serde(default)]
        pub user_data: ::std::string::String,
    }

    impl ::std::convert::From<&InstanceCreate> for InstanceCreate {
        fn from(value: &InstanceCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "Describe the instance's disks at creation time"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describe the instance's disks at creation time\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"During instance creation, create and attach disks\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"description\","]
    #[doc = "        \"disk_source\","]
    #[doc = "        \"name\","]
    #[doc = "        \"size\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"description\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        },"]
    #[doc = "        \"disk_source\": {"]
    #[doc = "          \"description\": \"initial source for this disk\","]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/DiskSource\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"name\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        },"]
    #[doc = "        \"size\": {"]
    #[doc = "          \"description\": \"total size of the Disk in bytes\","]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"create\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"During instance creation, attach this disk\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"name\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"name\": {"]
    #[doc = "          \"description\": \"A disk name to attach\","]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"attach\""]
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
    pub enum InstanceDiskAttachment {
        #[doc = "During instance creation, create and attach disks"]
        #[serde(rename = "create")]
        Create {
            description: ::std::string::String,
            #[doc = "initial source for this disk"]
            disk_source: DiskSource,
            name: Name,
            #[doc = "total size of the Disk in bytes"]
            size: ByteCount,
        },
        #[doc = "During instance creation, attach this disk"]
        #[serde(rename = "attach")]
        Attach {
            #[doc = "A disk name to attach"]
            name: Name,
        },
    }

    impl ::std::convert::From<&Self> for InstanceDiskAttachment {
        fn from(value: &InstanceDiskAttachment) -> Self {
            value.clone()
        }
    }

    #[doc = "Migration parameters for an [`Instance`](omicron_common::api::external::Instance)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Migration parameters for an [`Instance`](omicron_common::api::external::Instance)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"dst_sled_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"dst_sled_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrate {
        pub dst_sled_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrate> for InstanceMigrate {
        fn from(value: &InstanceMigrate) -> Self {
            value.clone()
        }
    }

    #[doc = "Describes an attachment of a `NetworkInterface` to an `Instance`, at the time the instance is created."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes an attachment of a `NetworkInterface` to an `Instance`, at the time the instance is created.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Create one or more `NetworkInterface`s for the `Instance`.\\n\\nIf more than one interface is provided, then the first will be designated the primary interface for the instance.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"params\","]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"params\": {"]
    #[doc = "          \"type\": \"array\","]
    #[doc = "          \"items\": {"]
    #[doc = "            \"$ref\": \"#/components/schemas/NetworkInterfaceCreate\""]
    #[doc = "          }"]
    #[doc = "        },"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"create\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The default networking configuration for an instance is to create a single primary interface with an automatically-assigned IP address. The IP will be pulled from the Project's default VPC / VPC Subnet.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"default\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"No network interfaces at all will be created for the instance.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"none\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "params")]
    pub enum InstanceNetworkInterfaceAttachment {
        #[doc = "Create one or more `NetworkInterface`s for the `Instance`.\n\nIf more than one interface is provided, then the first will be designated the primary interface for the instance."]
        #[serde(rename = "create")]
        Create(::std::vec::Vec<NetworkInterfaceCreate>),
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "none")]
        None,
    }

    impl ::std::convert::From<&Self> for InstanceNetworkInterfaceAttachment {
        fn from(value: &InstanceNetworkInterfaceAttachment) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<NetworkInterfaceCreate>>
        for InstanceNetworkInterfaceAttachment
    {
        fn from(value: ::std::vec::Vec<NetworkInterfaceCreate>) -> Self {
            Self::Create(value)
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Instance\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Instance>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&InstanceResultsPage> for InstanceResultsPage {
        fn from(value: &InstanceResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Contents of an Instance's serial console buffer."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Contents of an Instance's serial console buffer.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"data\","]
    #[doc = "    \"last_byte_offset\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"data\": {"]
    #[doc = "      \"description\": \"The bytes starting from the requested offset up to either the end of the buffer or the request's `max_bytes`. Provided as a u8 array rather than a string, as it may not be UTF-8.\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"type\": \"integer\","]
    #[doc = "        \"format\": \"uint8\","]
    #[doc = "        \"minimum\": 0.0"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"last_byte_offset\": {"]
    #[doc = "      \"description\": \"The absolute offset since boot (suitable for use as `byte_offset` in a subsequent request) of the last byte returned in `data`.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint64\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSerialConsoleData {
        #[doc = "The bytes starting from the requested offset up to either the end of the buffer or the request's `max_bytes`. Provided as a u8 array rather than a string, as it may not be UTF-8."]
        pub data: ::std::vec::Vec<u8>,
        #[doc = "The absolute offset since boot (suitable for use as `byte_offset` in a subsequent request) of the last byte returned in `data`."]
        pub last_byte_offset: u64,
    }

    impl ::std::convert::From<&InstanceSerialConsoleData> for InstanceSerialConsoleData {
        fn from(value: &InstanceSerialConsoleData) -> Self {
            value.clone()
        }
    }

    #[doc = "Running state of an Instance (primarily: booted or stopped)\n\nThis typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Running state of an Instance (primarily: booted or stopped)\\n\\nThis typically reflects whether it's starting, running, stopping, or stopped, but also includes states related to the Instance's lifecycle\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is being created.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"creating\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is currently starting up.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"starting\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is currently running.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"running\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance has been requested to stop and a transition to \\\"Stopped\\\" is imminent.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"stopping\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is currently stopped.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"stopped\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is in the process of rebooting - it will remain in the \\\"rebooting\\\" state until the VM is starting once more.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"rebooting\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is in the process of migrating - it will remain in the \\\"migrating\\\" state until the migration process is complete and the destination propolis is ready to continue execution.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"migrating\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance is attempting to recover from a failure.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"repairing\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance has encountered a failure.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"failed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The instance has been deleted.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"destroyed\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
        #[doc = "The instance is being created."]
        #[serde(rename = "creating")]
        Creating,
        #[doc = "The instance is currently starting up."]
        #[serde(rename = "starting")]
        Starting,
        #[doc = "The instance is currently running."]
        #[serde(rename = "running")]
        Running,
        #[doc = "The instance has been requested to stop and a transition to \"Stopped\" is imminent."]
        #[serde(rename = "stopping")]
        Stopping,
        #[doc = "The instance is currently stopped."]
        #[serde(rename = "stopped")]
        Stopped,
        #[doc = "The instance is in the process of rebooting - it will remain in the \"rebooting\" state until the VM is starting once more."]
        #[serde(rename = "rebooting")]
        Rebooting,
        #[doc = "The instance is in the process of migrating - it will remain in the \"migrating\" state until the migration process is complete and the destination propolis is ready to continue execution."]
        #[serde(rename = "migrating")]
        Migrating,
        #[doc = "The instance is attempting to recover from a failure."]
        #[serde(rename = "repairing")]
        Repairing,
        #[doc = "The instance has encountered a failure."]
        #[serde(rename = "failed")]
        Failed,
        #[doc = "The instance has been deleted."]
        #[serde(rename = "destroyed")]
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
                Self::Creating => f.write_str("creating"),
                Self::Starting => f.write_str("starting"),
                Self::Running => f.write_str("running"),
                Self::Stopping => f.write_str("stopping"),
                Self::Stopped => f.write_str("stopped"),
                Self::Rebooting => f.write_str("rebooting"),
                Self::Migrating => f.write_str("migrating"),
                Self::Repairing => f.write_str("repairing"),
                Self::Failed => f.write_str("failed"),
                Self::Destroyed => f.write_str("destroyed"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "creating" => Ok(Self::Creating),
                "starting" => Ok(Self::Starting),
                "running" => Ok(Self::Running),
                "stopping" => Ok(Self::Stopping),
                "stopped" => Ok(Self::Stopped),
                "rebooting" => Ok(Self::Rebooting),
                "migrating" => Ok(Self::Migrating),
                "repairing" => Ok(Self::Repairing),
                "failed" => Ok(Self::Failed),
                "destroyed" => Ok(Self::Destroyed),
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

    #[doc = "The kind of an external IP address for an instance"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The kind of an external IP address for an instance\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"ephemeral\","]
    #[doc = "    \"floating\""]
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
    pub enum IpKind {
        #[serde(rename = "ephemeral")]
        Ephemeral,
        #[serde(rename = "floating")]
        Floating,
    }

    impl ::std::convert::From<&Self> for IpKind {
        fn from(value: &IpKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IpKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ephemeral => f.write_str("ephemeral"),
                Self::Floating => f.write_str("floating"),
            }
        }
    }

    impl ::std::str::FromStr for IpKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ephemeral" => Ok(Self::Ephemeral),
                "floating" => Ok(Self::Floating),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IpKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IpKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IpKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`IpNet`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"title\": \"v4\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv4Net\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"title\": \"v6\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv6Net\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum IpNet {
        V4(Ipv4Net),
        V6(Ipv6Net),
    }

    impl ::std::convert::From<&Self> for IpNet {
        fn from(value: &IpNet) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for IpNet {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::V4(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::V6(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IpNet {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IpNet {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IpNet {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for IpNet {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::V4(x) => x.fmt(f),
                Self::V6(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<Ipv4Net> for IpNet {
        fn from(value: Ipv4Net) -> Self {
            Self::V4(value)
        }
    }

    impl ::std::convert::From<Ipv6Net> for IpNet {
        fn from(value: Ipv6Net) -> Self {
            Self::V6(value)
        }
    }

    #[doc = "Identity-related metadata that's included in nearly all public API objects"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Identity-related metadata that's included in nearly all public API objects\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPool {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&IpPool> for IpPool {
        fn from(value: &IpPool) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for an IP Pool.\n\nSee [`IpPool`](crate::external_api::views::IpPool)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for an IP Pool.\\n\\nSee [`IpPool`](crate::external_api::views::IpPool)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolCreate {
        pub description: ::std::string::String,
        pub name: Name,
    }

    impl ::std::convert::From<&IpPoolCreate> for IpPoolCreate {
        fn from(value: &IpPoolCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "`IpPoolRange`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"range\","]
    #[doc = "    \"time_created\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"range\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/IpRange\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolRange {
        pub id: ::uuid::Uuid,
        pub range: IpRange,
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&IpPoolRange> for IpPoolRange {
        fn from(value: &IpPoolRange) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/IpPoolRange\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolRangeResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<IpPoolRange>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&IpPoolRangeResultsPage> for IpPoolRangeResultsPage {
        fn from(value: &IpPoolRangeResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/IpPool\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<IpPool>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&IpPoolResultsPage> for IpPoolResultsPage {
        fn from(value: &IpPoolResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Parameters for updating an IP Pool"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Parameters for updating an IP Pool\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
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
    pub struct IpPoolUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
    }

    impl ::std::convert::From<&IpPoolUpdate> for IpPoolUpdate {
        fn from(value: &IpPoolUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for IpPoolUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
            }
        }
    }

    #[doc = "`IpRange`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"title\": \"v4\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv4Range\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"title\": \"v6\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv6Range\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum IpRange {
        V4(Ipv4Range),
        V6(Ipv6Range),
    }

    impl ::std::convert::From<&Self> for IpRange {
        fn from(value: &IpRange) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<Ipv4Range> for IpRange {
        fn from(value: Ipv4Range) -> Self {
            Self::V4(value)
        }
    }

    impl ::std::convert::From<Ipv6Range> for IpRange {
        fn from(value: Ipv6Range) -> Self {
            Self::V6(value)
        }
    }

    #[doc = "An IPv4 subnet, including prefix and subnet mask"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"An IPv4 subnet\","]
    #[doc = "  \"description\": \"An IPv4 subnet, including prefix and subnet mask\","]
    #[doc = "  \"examples\": ["]
    #[doc = "    \"192.168.1.0/24\""]
    #[doc = "  ],"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"pattern\": \"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/([8-9]|1[0-9]|2[0-9]|3[0-2])$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct Ipv4Net(::std::string::String);
    impl ::std::ops::Deref for Ipv4Net {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Ipv4Net> for ::std::string::String {
        fn from(value: Ipv4Net) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Ipv4Net> for Ipv4Net {
        fn from(value: &Ipv4Net) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Ipv4Net {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
                || {
                    :: regress :: Regex :: new ("^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/([8-9]|1[0-9]|2[0-9]|3[0-2])$") . unwrap ()
                },
            );
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/([8-9]|1[0-9]|2[0-9]|3[0-2])$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Ipv4Net {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Ipv4Net {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Ipv4Net {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Ipv4Net {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "A non-decreasing IPv4 address range, inclusive of both ends.\n\nThe first address must be less than or equal to the last address."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A non-decreasing IPv4 address range, inclusive of both ends.\\n\\nThe first address must be less than or equal to the last address.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"first\","]
    #[doc = "    \"last\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"first\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"ipv4\""]
    #[doc = "    },"]
    #[doc = "    \"last\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"ipv4\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Ipv4Range {
        pub first: ::std::net::Ipv4Addr,
        pub last: ::std::net::Ipv4Addr,
    }

    impl ::std::convert::From<&Ipv4Range> for Ipv4Range {
        fn from(value: &Ipv4Range) -> Self {
            value.clone()
        }
    }

    #[doc = "An IPv6 subnet, including prefix and subnet mask"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"An IPv6 subnet\","]
    #[doc = "  \"description\": \"An IPv6 subnet, including prefix and subnet mask\","]
    #[doc = "  \"examples\": ["]
    #[doc = "    \"fd12:3456::/64\""]
    #[doc = "  ],"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"pattern\": \"^([fF][dD])[0-9a-fA-F]{2}:(([0-9a-fA-F]{1,4}:){6}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,6}:)\\\\/([1-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct Ipv6Net(::std::string::String);
    impl ::std::ops::Deref for Ipv6Net {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Ipv6Net> for ::std::string::String {
        fn from(value: Ipv6Net) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Ipv6Net> for Ipv6Net {
        fn from(value: &Ipv6Net) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Ipv6Net {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
                || {
                    :: regress :: Regex :: new ("^([fF][dD])[0-9a-fA-F]{2}:(([0-9a-fA-F]{1,4}:){6}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,6}:)\\/([1-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$") . unwrap ()
                },
            );
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^([fF][dD])[0-9a-fA-F]{2}:(([0-9a-fA-F]{1,4}:){6}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,6}:)\\/([1-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Ipv6Net {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Ipv6Net {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Ipv6Net {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Ipv6Net {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "A non-decreasing IPv6 address range, inclusive of both ends.\n\nThe first address must be less than or equal to the last address."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A non-decreasing IPv6 address range, inclusive of both ends.\\n\\nThe first address must be less than or equal to the last address.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"first\","]
    #[doc = "    \"last\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"first\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"ipv6\""]
    #[doc = "    },"]
    #[doc = "    \"last\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"ipv6\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Ipv6Range {
        pub first: ::std::net::Ipv6Addr,
        pub last: ::std::net::Ipv6Addr,
    }

    impl ::std::convert::From<&Ipv6Range> for Ipv6Range {
        fn from(value: &Ipv6Range) -> Self {
            value.clone()
        }
    }

    #[doc = "An inclusive-inclusive range of IP ports. The second port may be omitted to represent a single port"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"A range of IP ports\","]
    #[doc = "  \"description\": \"An inclusive-inclusive range of IP ports. The second port may be omitted to represent a single port\","]
    #[doc = "  \"examples\": ["]
    #[doc = "    \"22\""]
    #[doc = "  ],"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"maxLength\": 11,"]
    #[doc = "  \"minLength\": 1,"]
    #[doc = "  \"pattern\": \"^[0-9]{1,5}(-[0-9]{1,5})?$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct L4PortRange(::std::string::String);
    impl ::std::ops::Deref for L4PortRange {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<L4PortRange> for ::std::string::String {
        fn from(value: L4PortRange) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&L4PortRange> for L4PortRange {
        fn from(value: &L4PortRange) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for L4PortRange {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 11usize {
                return Err("longer than 11 characters".into());
            }
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[0-9]{1,5}(-[0-9]{1,5})?$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[0-9]{1,5}(-[0-9]{1,5})?$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for L4PortRange {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for L4PortRange {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for L4PortRange {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for L4PortRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "A Media Access Control address, in EUI-48 format"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"A MAC address\","]
    #[doc = "  \"description\": \"A Media Access Control address, in EUI-48 format\","]
    #[doc = "  \"examples\": ["]
    #[doc = "    \"ff:ff:ff:ff:ff:ff\""]
    #[doc = "  ],"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"maxLength\": 17,"]
    #[doc = "  \"minLength\": 17,"]
    #[doc = "  \"pattern\": \"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MacAddr(::std::string::String);
    impl ::std::ops::Deref for MacAddr {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<MacAddr> for ::std::string::String {
        fn from(value: MacAddr) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&MacAddr> for MacAddr {
        fn from(value: &MacAddr) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for MacAddr {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 17usize {
                return Err("longer than 17 characters".into());
            }
            if value.chars().count() < 17usize {
                return Err("shorter than 17 characters".into());
            }
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err(
                    "doesn't match pattern \"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$\"".into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for MacAddr {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MacAddr {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MacAddr {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for MacAddr {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "A `Measurement` is a timestamped datum from a single metric"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A `Measurement` is a timestamped datum from a single metric\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"datum\","]
    #[doc = "    \"timestamp\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"datum\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Datum\""]
    #[doc = "    },"]
    #[doc = "    \"timestamp\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Measurement {
        pub datum: Datum,
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Measurement> for Measurement {
        fn from(value: &Measurement) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Measurement\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MeasurementResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Measurement>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&MeasurementResultsPage> for MeasurementResultsPage {
        fn from(value: &MeasurementResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"A name unique within the parent collection\","]
    #[doc = "  \"description\": \"Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"maxLength\": 63,"]
    #[doc = "  \"pattern\": \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct Name(::std::string::String);
    impl ::std::ops::Deref for Name {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Name> for ::std::string::String {
        fn from(value: Name) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Name> for Name {
        fn from(value: &Name) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Name {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 63usize {
                return Err("longer than 63 characters".into());
            }
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
                || {
                    :: regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap ()
                },
            );
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Name {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Name {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "`NameOrId`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"title\": \"id\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"uuid\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"title\": \"name\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum NameOrId {
        Id(::uuid::Uuid),
        Name(Name),
    }

    impl ::std::convert::From<&Self> for NameOrId {
        fn from(value: &NameOrId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for NameOrId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Id(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Name(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NameOrId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NameOrId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NameOrId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for NameOrId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Id(x) => x.fmt(f),
                Self::Name(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for NameOrId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self::Id(value)
        }
    }

    impl ::std::convert::From<Name> for NameOrId {
        fn from(value: Name) -> Self {
            Self::Name(value)
        }
    }

    #[doc = "Supported set of sort modes for scanning by name or id"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Supported set of sort modes for scanning by name or id\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"sort in increasing order of \\\"name\\\"\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"name_ascending\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"sort in decreasing order of \\\"name\\\"\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"name_descending\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"sort in increasing order of \\\"id\\\"\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"id_ascending\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum NameOrIdSortMode {
        #[doc = "sort in increasing order of \"name\""]
        #[serde(rename = "name_ascending")]
        NameAscending,
        #[doc = "sort in decreasing order of \"name\""]
        #[serde(rename = "name_descending")]
        NameDescending,
        #[doc = "sort in increasing order of \"id\""]
        #[serde(rename = "id_ascending")]
        IdAscending,
    }

    impl ::std::convert::From<&Self> for NameOrIdSortMode {
        fn from(value: &NameOrIdSortMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NameOrIdSortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NameAscending => f.write_str("name_ascending"),
                Self::NameDescending => f.write_str("name_descending"),
                Self::IdAscending => f.write_str("id_ascending"),
            }
        }
    }

    impl ::std::str::FromStr for NameOrIdSortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name_ascending" => Ok(Self::NameAscending),
                "name_descending" => Ok(Self::NameDescending),
                "id_ascending" => Ok(Self::IdAscending),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NameOrIdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NameOrIdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NameOrIdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Supported set of sort modes for scanning by name only\n\nCurrently, we only support scanning in ascending order."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Supported set of sort modes for scanning by name only\\n\\nCurrently, we only support scanning in ascending order.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"sort in increasing order of \\\"name\\\"\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"name_ascending\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum NameSortMode {
        #[doc = "sort in increasing order of \"name\""]
        #[serde(rename = "name_ascending")]
        NameAscending,
    }

    impl ::std::convert::From<&Self> for NameSortMode {
        fn from(value: &NameSortMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NameSortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NameAscending => f.write_str("name_ascending"),
            }
        }
    }

    impl ::std::str::FromStr for NameSortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "name_ascending" => Ok(Self::NameAscending),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NameSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NameSortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NameSortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "A `NetworkInterface` represents a virtual network interface device."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A `NetworkInterface` represents a virtual network interface device.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"instance_id\","]
    #[doc = "    \"ip\","]
    #[doc = "    \"mac\","]
    #[doc = "    \"name\","]
    #[doc = "    \"primary\","]
    #[doc = "    \"subnet_id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"vpc_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"instance_id\": {"]
    #[doc = "      \"description\": \"The Instance to which the interface belongs.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"ip\": {"]
    #[doc = "      \"description\": \"The IP address assigned to this interface.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"ip\""]
    #[doc = "    },"]
    #[doc = "    \"mac\": {"]
    #[doc = "      \"description\": \"The MAC address assigned to this interface.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/MacAddr\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"primary\": {"]
    #[doc = "      \"description\": \"True if this interface is the primary for the instance to which it's attached.\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"subnet_id\": {"]
    #[doc = "      \"description\": \"The subnet to which the interface belongs.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"vpc_id\": {"]
    #[doc = "      \"description\": \"The VPC to which the interface belongs.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterface {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "The Instance to which the interface belongs."]
        pub instance_id: ::uuid::Uuid,
        #[doc = "The IP address assigned to this interface."]
        pub ip: ::std::net::IpAddr,
        #[doc = "The MAC address assigned to this interface."]
        pub mac: MacAddr,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "True if this interface is the primary for the instance to which it's attached."]
        pub primary: bool,
        #[doc = "The subnet to which the interface belongs."]
        pub subnet_id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "The VPC to which the interface belongs."]
        pub vpc_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&NetworkInterface> for NetworkInterface {
        fn from(value: &NetworkInterface) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`NetworkInterface`](omicron_common::api::external::NetworkInterface)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`NetworkInterface`](omicron_common::api::external::NetworkInterface)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\","]
    #[doc = "    \"subnet_name\","]
    #[doc = "    \"vpc_name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"ip\": {"]
    #[doc = "      \"description\": \"The IP address for the interface. One will be auto-assigned if not provided.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"ip\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"subnet_name\": {"]
    #[doc = "      \"description\": \"The VPC Subnet in which to create the interface.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"vpc_name\": {"]
    #[doc = "      \"description\": \"The VPC in which to create the interface.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceCreate {
        pub description: ::std::string::String,
        #[doc = "The IP address for the interface. One will be auto-assigned if not provided."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ip: ::std::option::Option<::std::net::IpAddr>,
        pub name: Name,
        #[doc = "The VPC Subnet in which to create the interface."]
        pub subnet_name: Name,
        #[doc = "The VPC in which to create the interface."]
        pub vpc_name: Name,
    }

    impl ::std::convert::From<&NetworkInterfaceCreate> for NetworkInterfaceCreate {
        fn from(value: &NetworkInterfaceCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/NetworkInterface\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<NetworkInterface>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&NetworkInterfaceResultsPage> for NetworkInterfaceResultsPage {
        fn from(value: &NetworkInterfaceResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Parameters for updating a [`NetworkInterface`](omicron_common::api::external::NetworkInterface).\n\nNote that modifying IP addresses for an interface is not yet supported, a new interface must be created instead."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Parameters for updating a [`NetworkInterface`](omicron_common::api::external::NetworkInterface).\\n\\nNote that modifying IP addresses for an interface is not yet supported, a new interface must be created instead.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"primary\": {"]
    #[doc = "      \"description\": \"Make a secondary interface the instance's primary interface.\\n\\nIf applied to a secondary interface, that interface will become the primary on the next reboot of the instance. Note that this may have implications for routing between instances, as the new primary interface will be on a distinct subnet from the previous primary interface.\\n\\nNote that this can only be used to select a new primary interface for an instance. Requests to change the primary interface into a secondary will return an error.\","]
    #[doc = "      \"default\": false,"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
        #[doc = "Make a secondary interface the instance's primary interface.\n\nIf applied to a secondary interface, that interface will become the primary on the next reboot of the instance. Note that this may have implications for routing between instances, as the new primary interface will be on a distinct subnet from the previous primary interface.\n\nNote that this can only be used to select a new primary interface for an instance. Requests to change the primary interface into a secondary will return an error."]
        #[serde(default)]
        pub primary: bool,
    }

    impl ::std::convert::From<&NetworkInterfaceUpdate> for NetworkInterfaceUpdate {
        fn from(value: &NetworkInterfaceUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for NetworkInterfaceUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
                primary: Default::default(),
            }
        }
    }

    #[doc = "Unique name for a saga [`Node`]\n\nEach node requires a string name that's unique within its DAG.  The name is used to identify its output.  Nodes that depend on a given node (either directly or indirectly) can access the node's output using its name."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Unique name for a saga [`Node`]\\n\\nEach node requires a string name that's unique within its DAG.  The name is used to identify its output.  Nodes that depend on a given node (either directly or indirectly) can access the node's output using its name.\","]
    #[doc = "  \"type\": \"string\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct NodeName(pub ::std::string::String);
    impl ::std::ops::Deref for NodeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<NodeName> for ::std::string::String {
        fn from(value: NodeName) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&NodeName> for NodeName {
        fn from(value: &NodeName) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for NodeName {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for NodeName {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for NodeName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    #[doc = "Client view of an [`Organization`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of an [`Organization`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Organization {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Organization> for Organization {
        fn from(value: &Organization) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for an [`Organization`](crate::external_api::views::Organization)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for an [`Organization`](crate::external_api::views::Organization)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationCreate {
        pub description: ::std::string::String,
        pub name: Name,
    }

    impl ::std::convert::From<&OrganizationCreate> for OrganizationCreate {
        fn from(value: &OrganizationCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Organization\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Organization>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&OrganizationResultsPage> for OrganizationResultsPage {
        fn from(value: &OrganizationResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`OrganizationRole`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"admin\","]
    #[doc = "    \"collaborator\","]
    #[doc = "    \"viewer\""]
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
    pub enum OrganizationRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl ::std::convert::From<&Self> for OrganizationRole {
        fn from(value: &OrganizationRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for OrganizationRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => f.write_str("admin"),
                Self::Collaborator => f.write_str("collaborator"),
                Self::Viewer => f.write_str("viewer"),
            }
        }
    }

    impl ::std::str::FromStr for OrganizationRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrganizationRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrganizationRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrganizationRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a [`Policy`], which describes how this resource may be accessed\n\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Policy`], which describes how this resource may be accessed\\n\\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"role_assignments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"role_assignments\": {"]
    #[doc = "      \"description\": \"Roles directly assigned on this resource\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/OrganizationRoleRoleAssignment\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationRolePolicy {
        #[doc = "Roles directly assigned on this resource"]
        pub role_assignments: ::std::vec::Vec<OrganizationRoleRoleAssignment>,
    }

    impl ::std::convert::From<&OrganizationRolePolicy> for OrganizationRolePolicy {
        fn from(value: &OrganizationRolePolicy) -> Self {
            value.clone()
        }
    }

    #[doc = "Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\n\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\\n\\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"identity_id\","]
    #[doc = "    \"identity_type\","]
    #[doc = "    \"role_name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"identity_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"identity_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/IdentityType\""]
    #[doc = "    },"]
    #[doc = "    \"role_name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/OrganizationRole\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationRoleRoleAssignment {
        pub identity_id: ::uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: OrganizationRole,
    }

    impl ::std::convert::From<&OrganizationRoleRoleAssignment> for OrganizationRoleRoleAssignment {
        fn from(value: &OrganizationRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    #[doc = "Updateable properties of an [`Organization`](crate::external_api::views::Organization)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of an [`Organization`](crate::external_api::views::Organization)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
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
    pub struct OrganizationUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
    }

    impl ::std::convert::From<&OrganizationUpdate> for OrganizationUpdate {
        fn from(value: &OrganizationUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for OrganizationUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
            }
        }
    }

    #[doc = "Passwords may be subject to additional constraints."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"A password used to authenticate a user\","]
    #[doc = "  \"description\": \"Passwords may be subject to additional constraints.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"maxLength\": 512"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct Password(::std::string::String);
    impl ::std::ops::Deref for Password {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Password> for ::std::string::String {
        fn from(value: Password) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Password> for Password {
        fn from(value: &Password) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Password {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 512usize {
                return Err("longer than 512 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Password {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Password {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Password {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Password {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "Client view of a [`PhysicalDisk`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`PhysicalDisk`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"disk_type\","]
    #[doc = "    \"id\","]
    #[doc = "    \"model\","]
    #[doc = "    \"serial\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"vendor\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"disk_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/PhysicalDiskType\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"model\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"serial\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"sled_id\": {"]
    #[doc = "      \"description\": \"The sled to which this disk is attached, if any.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"vendor\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PhysicalDisk {
        pub disk_type: PhysicalDiskType,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        pub model: ::std::string::String,
        pub serial: ::std::string::String,
        #[doc = "The sled to which this disk is attached, if any."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sled_id: ::std::option::Option<::uuid::Uuid>,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        pub vendor: ::std::string::String,
    }

    impl ::std::convert::From<&PhysicalDisk> for PhysicalDisk {
        fn from(value: &PhysicalDisk) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/PhysicalDisk\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PhysicalDiskResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<PhysicalDisk>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&PhysicalDiskResultsPage> for PhysicalDiskResultsPage {
        fn from(value: &PhysicalDiskResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`PhysicalDiskType`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"internal\","]
    #[doc = "    \"external\""]
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
    pub enum PhysicalDiskType {
        #[serde(rename = "internal")]
        Internal,
        #[serde(rename = "external")]
        External,
    }

    impl ::std::convert::From<&Self> for PhysicalDiskType {
        fn from(value: &PhysicalDiskType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for PhysicalDiskType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Internal => f.write_str("internal"),
                Self::External => f.write_str("external"),
            }
        }
    }

    impl ::std::str::FromStr for PhysicalDiskType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "internal" => Ok(Self::Internal),
                "external" => Ok(Self::External),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PhysicalDiskType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PhysicalDiskType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PhysicalDiskType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a [`Project`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Project`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"organization_id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"organization_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Project {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        pub organization_id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Project> for Project {
        fn from(value: &Project) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`Project`](crate::external_api::views::Project)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`Project`](crate::external_api::views::Project)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectCreate {
        pub description: ::std::string::String,
        pub name: Name,
    }

    impl ::std::convert::From<&ProjectCreate> for ProjectCreate {
        fn from(value: &ProjectCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Project\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Project>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&ProjectResultsPage> for ProjectResultsPage {
        fn from(value: &ProjectResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`ProjectRole`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"admin\","]
    #[doc = "    \"collaborator\","]
    #[doc = "    \"viewer\""]
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
    pub enum ProjectRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl ::std::convert::From<&Self> for ProjectRole {
        fn from(value: &ProjectRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ProjectRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => f.write_str("admin"),
                Self::Collaborator => f.write_str("collaborator"),
                Self::Viewer => f.write_str("viewer"),
            }
        }
    }

    impl ::std::str::FromStr for ProjectRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ProjectRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ProjectRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ProjectRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a [`Policy`], which describes how this resource may be accessed\n\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Policy`], which describes how this resource may be accessed\\n\\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"role_assignments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"role_assignments\": {"]
    #[doc = "      \"description\": \"Roles directly assigned on this resource\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ProjectRoleRoleAssignment\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectRolePolicy {
        #[doc = "Roles directly assigned on this resource"]
        pub role_assignments: ::std::vec::Vec<ProjectRoleRoleAssignment>,
    }

    impl ::std::convert::From<&ProjectRolePolicy> for ProjectRolePolicy {
        fn from(value: &ProjectRolePolicy) -> Self {
            value.clone()
        }
    }

    #[doc = "Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\n\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\\n\\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"identity_id\","]
    #[doc = "    \"identity_type\","]
    #[doc = "    \"role_name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"identity_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"identity_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/IdentityType\""]
    #[doc = "    },"]
    #[doc = "    \"role_name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ProjectRole\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectRoleRoleAssignment {
        pub identity_id: ::uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: ProjectRole,
    }

    impl ::std::convert::From<&ProjectRoleRoleAssignment> for ProjectRoleRoleAssignment {
        fn from(value: &ProjectRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    #[doc = "Updateable properties of a [`Project`](crate::external_api::views::Project)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of a [`Project`](crate::external_api::views::Project)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
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
    pub struct ProjectUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
    }

    impl ::std::convert::From<&ProjectUpdate> for ProjectUpdate {
        fn from(value: &ProjectUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for ProjectUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
            }
        }
    }

    #[doc = "Client view of an [`Rack`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of an [`Rack`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Rack {
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Rack> for Rack {
        fn from(value: &Rack) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Rack\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RackResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Rack>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RackResultsPage> for RackResultsPage {
        fn from(value: &RackResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`Role`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Role`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RoleName\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Role {
        pub description: ::std::string::String,
        pub name: RoleName,
    }

    impl ::std::convert::From<&Role> for Role {
        fn from(value: &Role) -> Self {
            value.clone()
        }
    }

    #[doc = "Role names consist of two string components separated by dot (\".\")."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"A name for a built-in role\","]
    #[doc = "  \"description\": \"Role names consist of two string components separated by dot (\\\".\\\").\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"maxLength\": 63,"]
    #[doc = "  \"pattern\": \"[a-z-]+\\\\.[a-z-]+\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RoleName(::std::string::String);
    impl ::std::ops::Deref for RoleName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RoleName> for ::std::string::String {
        fn from(value: RoleName) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&RoleName> for RoleName {
        fn from(value: &RoleName) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for RoleName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 63usize {
                return Err("longer than 63 characters".into());
            }
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| ::regress::Regex::new("[a-z-]+\\.[a-z-]+").unwrap());
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"[a-z-]+\\.[a-z-]+\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RoleName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for RoleName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RoleName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RoleName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Role\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RoleResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Role>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RoleResultsPage> for RoleResultsPage {
        fn from(value: &RoleResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "A `RouteDestination` is used to match traffic with a routing rule, on the destination of that traffic.\n\nWhen traffic is to be sent to a destination that is within a given `RouteDestination`, the corresponding [`RouterRoute`] applies, and traffic will be forward to the [`RouteTarget`] for that rule."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A `RouteDestination` is used to match traffic with a routing rule, on the destination of that traffic.\\n\\nWhen traffic is to be sent to a destination that is within a given `RouteDestination`, the corresponding [`RouterRoute`] applies, and traffic will be forward to the [`RouteTarget`] for that rule.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Route applies to traffic destined for a specific IP address\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"ip\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Route applies to traffic destined for a specific IP subnet\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip_net\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/IpNet\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Route applies to traffic destined for the given VPC.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"vpc\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Route applies to traffic\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"subnet\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum RouteDestination {
        #[doc = "Route applies to traffic destined for a specific IP address"]
        #[serde(rename = "ip")]
        Ip(::std::net::IpAddr),
        #[doc = "Route applies to traffic destined for a specific IP subnet"]
        #[serde(rename = "ip_net")]
        IpNet(IpNet),
        #[doc = "Route applies to traffic destined for the given VPC."]
        #[serde(rename = "vpc")]
        Vpc(Name),
        #[doc = "Route applies to traffic"]
        #[serde(rename = "subnet")]
        Subnet(Name),
    }

    impl ::std::convert::From<&Self> for RouteDestination {
        fn from(value: &RouteDestination) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::net::IpAddr> for RouteDestination {
        fn from(value: ::std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    impl ::std::convert::From<IpNet> for RouteDestination {
        fn from(value: IpNet) -> Self {
            Self::IpNet(value)
        }
    }

    #[doc = "A `RouteTarget` describes the possible locations that traffic matching a route destination can be sent."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A `RouteTarget` describes the possible locations that traffic matching a route destination can be sent.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Forward traffic to a particular IP address.\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"ip\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Forward traffic to a VPC\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"vpc\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Forward traffic to a VPC Subnet\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"subnet\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Forward traffic to a specific instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"instance\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Forward traffic to an internet gateway\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"internet_gateway\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum RouteTarget {
        #[doc = "Forward traffic to a particular IP address."]
        #[serde(rename = "ip")]
        Ip(::std::net::IpAddr),
        #[doc = "Forward traffic to a VPC"]
        #[serde(rename = "vpc")]
        Vpc(Name),
        #[doc = "Forward traffic to a VPC Subnet"]
        #[serde(rename = "subnet")]
        Subnet(Name),
        #[doc = "Forward traffic to a specific instance"]
        #[serde(rename = "instance")]
        Instance(Name),
        #[doc = "Forward traffic to an internet gateway"]
        #[serde(rename = "internet_gateway")]
        InternetGateway(Name),
    }

    impl ::std::convert::From<&Self> for RouteTarget {
        fn from(value: &RouteTarget) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::net::IpAddr> for RouteTarget {
        fn from(value: ::std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    #[doc = "A route defines a rule that governs where traffic should be sent based on its destination."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A route defines a rule that governs where traffic should be sent based on its destination.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"destination\","]
    #[doc = "    \"id\","]
    #[doc = "    \"kind\","]
    #[doc = "    \"name\","]
    #[doc = "    \"target\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"vpc_router_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"destination\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RouteDestination\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"kind\": {"]
    #[doc = "      \"description\": \"Describes the kind of router. Set at creation. `read-only`\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/RouterRouteKind\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RouteTarget\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"vpc_router_id\": {"]
    #[doc = "      \"description\": \"The VPC Router to which the route belongs.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRoute {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        pub destination: RouteDestination,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "Describes the kind of router. Set at creation. `read-only`"]
        pub kind: RouterRouteKind,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        pub target: RouteTarget,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "The VPC Router to which the route belongs."]
        pub vpc_router_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&RouterRoute> for RouterRoute {
        fn from(value: &RouterRoute) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`RouterRoute`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`RouterRoute`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"destination\","]
    #[doc = "    \"name\","]
    #[doc = "    \"target\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"destination\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RouteDestination\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RouteTarget\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRouteCreateParams {
        pub description: ::std::string::String,
        pub destination: RouteDestination,
        pub name: Name,
        pub target: RouteTarget,
    }

    impl ::std::convert::From<&RouterRouteCreateParams> for RouterRouteCreateParams {
        fn from(value: &RouterRouteCreateParams) -> Self {
            value.clone()
        }
    }

    #[doc = "The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.\n\nSee [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.\\n\\nSee [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Determines the default destination of traffic, such as whether it goes to the internet or not.\\n\\n`Destination: An Internet Gateway` `Modifiable: true`\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"default\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Automatically added for each VPC Subnet in the VPC\\n\\n`Destination: A VPC Subnet` `Modifiable: false`\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"vpc_subnet\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Automatically added when VPC peering is established\\n\\n`Destination: A different VPC` `Modifiable: false`\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"vpc_peering\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Created by a user See [`RouteTarget`]\\n\\n`Destination: User defined` `Modifiable: true`\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"custom\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum RouterRouteKind {
        #[doc = "Determines the default destination of traffic, such as whether it goes to the internet or not.\n\n`Destination: An Internet Gateway` `Modifiable: true`"]
        #[serde(rename = "default")]
        Default,
        #[doc = "Automatically added for each VPC Subnet in the VPC\n\n`Destination: A VPC Subnet` `Modifiable: false`"]
        #[serde(rename = "vpc_subnet")]
        VpcSubnet,
        #[doc = "Automatically added when VPC peering is established\n\n`Destination: A different VPC` `Modifiable: false`"]
        #[serde(rename = "vpc_peering")]
        VpcPeering,
        #[doc = "Created by a user See [`RouteTarget`]\n\n`Destination: User defined` `Modifiable: true`"]
        #[serde(rename = "custom")]
        Custom,
    }

    impl ::std::convert::From<&Self> for RouterRouteKind {
        fn from(value: &RouterRouteKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for RouterRouteKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Default => f.write_str("default"),
                Self::VpcSubnet => f.write_str("vpc_subnet"),
                Self::VpcPeering => f.write_str("vpc_peering"),
                Self::Custom => f.write_str("custom"),
            }
        }
    }

    impl ::std::str::FromStr for RouterRouteKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "default" => Ok(Self::Default),
                "vpc_subnet" => Ok(Self::VpcSubnet),
                "vpc_peering" => Ok(Self::VpcPeering),
                "custom" => Ok(Self::Custom),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for RouterRouteKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for RouterRouteKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RouterRouteKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/RouterRoute\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRouteResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<RouterRoute>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RouterRouteResultsPage> for RouterRouteResultsPage {
        fn from(value: &RouterRouteResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Updateable properties of a [`RouterRoute`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of a [`RouterRoute`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"destination\","]
    #[doc = "    \"target\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"destination\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RouteDestination\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/RouteTarget\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRouteUpdateParams {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        pub destination: RouteDestination,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
        pub target: RouteTarget,
    }

    impl ::std::convert::From<&RouterRouteUpdateParams> for RouterRouteUpdateParams {
        fn from(value: &RouterRouteUpdateParams) -> Self {
            value.clone()
        }
    }

    #[doc = "`Saga`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"state\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SagaState\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Saga {
        pub id: ::uuid::Uuid,
        pub state: SagaState,
    }

    impl ::std::convert::From<&Saga> for Saga {
        fn from(value: &Saga) -> Self {
            value.clone()
        }
    }

    #[doc = "`SagaErrorInfo`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"error\","]
    #[doc = "        \"source_error\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"error\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"action_failed\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"source_error\": {}"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"error\","]
    #[doc = "        \"message\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"error\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"deserialize_failed\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"message\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"error\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"error\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"injected_error\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"error\","]
    #[doc = "        \"message\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"error\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"serialize_failed\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"message\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"error\","]
    #[doc = "        \"message\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"error\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"subsaga_create_failed\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"message\": {"]
    #[doc = "          \"type\": \"string\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "error")]
    pub enum SagaErrorInfo {
        #[serde(rename = "action_failed")]
        ActionFailed { source_error: ::serde_json::Value },
        #[serde(rename = "deserialize_failed")]
        DeserializeFailed { message: ::std::string::String },
        #[serde(rename = "injected_error")]
        InjectedError,
        #[serde(rename = "serialize_failed")]
        SerializeFailed { message: ::std::string::String },
        #[serde(rename = "subsaga_create_failed")]
        SubsagaCreateFailed { message: ::std::string::String },
    }

    impl ::std::convert::From<&Self> for SagaErrorInfo {
        fn from(value: &SagaErrorInfo) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Saga\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SagaResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Saga>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SagaResultsPage> for SagaResultsPage {
        fn from(value: &SagaResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`SagaState`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"running\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"succeeded\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"error_info\","]
    #[doc = "        \"error_node_name\","]
    #[doc = "        \"state\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"error_info\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/SagaErrorInfo\""]
    #[doc = "        },"]
    #[doc = "        \"error_node_name\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/NodeName\""]
    #[doc = "        },"]
    #[doc = "        \"state\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"failed\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "state")]
    pub enum SagaState {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed {
            error_info: SagaErrorInfo,
            error_node_name: NodeName,
        },
    }

    impl ::std::convert::From<&Self> for SagaState {
        fn from(value: &SagaState) -> Self {
            value.clone()
        }
    }

    #[doc = "Identity-related metadata that's included in nearly all public API objects"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Identity-related metadata that's included in nearly all public API objects\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"acs_url\","]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"idp_entity_id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"slo_url\","]
    #[doc = "    \"sp_client_id\","]
    #[doc = "    \"technical_contact_email\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"acs_url\": {"]
    #[doc = "      \"description\": \"service provider endpoint where the response will be sent\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"idp_entity_id\": {"]
    #[doc = "      \"description\": \"idp's entity id\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"public_cert\": {"]
    #[doc = "      \"description\": \"optional request signing public certificate (base64 encoded der file)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"slo_url\": {"]
    #[doc = "      \"description\": \"service provider endpoint where the idp should send log out requests\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"sp_client_id\": {"]
    #[doc = "      \"description\": \"sp's client id\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"technical_contact_email\": {"]
    #[doc = "      \"description\": \"customer's technical contact for saml configuration\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SamlIdentityProvider {
        #[doc = "service provider endpoint where the response will be sent"]
        pub acs_url: ::std::string::String,
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "idp's entity id"]
        pub idp_entity_id: ::std::string::String,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "optional request signing public certificate (base64 encoded der file)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub public_cert: ::std::option::Option<::std::string::String>,
        #[doc = "service provider endpoint where the idp should send log out requests"]
        pub slo_url: ::std::string::String,
        #[doc = "sp's client id"]
        pub sp_client_id: ::std::string::String,
        #[doc = "customer's technical contact for saml configuration"]
        pub technical_contact_email: ::std::string::String,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&SamlIdentityProvider> for SamlIdentityProvider {
        fn from(value: &SamlIdentityProvider) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time identity-related parameters"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time identity-related parameters\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"acs_url\","]
    #[doc = "    \"description\","]
    #[doc = "    \"idp_entity_id\","]
    #[doc = "    \"idp_metadata_source\","]
    #[doc = "    \"name\","]
    #[doc = "    \"slo_url\","]
    #[doc = "    \"sp_client_id\","]
    #[doc = "    \"technical_contact_email\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"acs_url\": {"]
    #[doc = "      \"description\": \"service provider endpoint where the response will be sent\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"group_attribute_name\": {"]
    #[doc = "      \"description\": \"If set, SAML attributes with this name will be considered to denote a user's group membership, where the attribute value(s) should be a comma-separated list of group names.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"idp_entity_id\": {"]
    #[doc = "      \"description\": \"idp's entity id\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"idp_metadata_source\": {"]
    #[doc = "      \"description\": \"the source of an identity provider metadata descriptor\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/IdpMetadataSource\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"signing_keypair\": {"]
    #[doc = "      \"description\": \"optional request signing key pair\","]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/DerEncodedKeyPair\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"slo_url\": {"]
    #[doc = "      \"description\": \"service provider endpoint where the idp should send log out requests\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"sp_client_id\": {"]
    #[doc = "      \"description\": \"sp's client id\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"technical_contact_email\": {"]
    #[doc = "      \"description\": \"customer's technical contact for saml configuration\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SamlIdentityProviderCreate {
        #[doc = "service provider endpoint where the response will be sent"]
        pub acs_url: ::std::string::String,
        pub description: ::std::string::String,
        #[doc = "If set, SAML attributes with this name will be considered to denote a user's group membership, where the attribute value(s) should be a comma-separated list of group names."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_attribute_name: ::std::option::Option<::std::string::String>,
        #[doc = "idp's entity id"]
        pub idp_entity_id: ::std::string::String,
        #[doc = "the source of an identity provider metadata descriptor"]
        pub idp_metadata_source: IdpMetadataSource,
        pub name: Name,
        #[doc = "optional request signing key pair"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub signing_keypair: ::std::option::Option<DerEncodedKeyPair>,
        #[doc = "service provider endpoint where the idp should send log out requests"]
        pub slo_url: ::std::string::String,
        #[doc = "sp's client id"]
        pub sp_client_id: ::std::string::String,
        #[doc = "customer's technical contact for saml configuration"]
        pub technical_contact_email: ::std::string::String,
    }

    impl ::std::convert::From<&SamlIdentityProviderCreate> for SamlIdentityProviderCreate {
        fn from(value: &SamlIdentityProviderCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "`SemverVersion`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"pattern\": \"^\\\\d+\\\\.\\\\d+\\\\.\\\\d+([\\\\-\\\\+].+)?$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SemverVersion(::std::string::String);
    impl ::std::ops::Deref for SemverVersion {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SemverVersion> for ::std::string::String {
        fn from(value: SemverVersion) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&SemverVersion> for SemverVersion {
        fn from(value: &SemverVersion) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for SemverVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+([\\-\\+].+)?$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+([\\-\\+].+)?$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SemverVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SemverVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SemverVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SemverVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "The service intended to use this certificate."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The service intended to use this certificate.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"This certificate is intended for access to the external API.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"external_api\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum ServiceUsingCertificate {
        #[doc = "This certificate is intended for access to the external API."]
        #[serde(rename = "external_api")]
        ExternalApi,
    }

    impl ::std::convert::From<&Self> for ServiceUsingCertificate {
        fn from(value: &ServiceUsingCertificate) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ServiceUsingCertificate {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExternalApi => f.write_str("external_api"),
            }
        }
    }

    impl ::std::str::FromStr for ServiceUsingCertificate {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "external_api" => Ok(Self::ExternalApi),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ServiceUsingCertificate {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ServiceUsingCertificate {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ServiceUsingCertificate {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a ['Silo']"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a ['Silo']\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"discoverable\","]
    #[doc = "    \"id\","]
    #[doc = "    \"identity_mode\","]
    #[doc = "    \"name\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"discoverable\": {"]
    #[doc = "      \"description\": \"A silo where discoverable is false can be retrieved only by its id - it will not be part of the \\\"list all silos\\\" output.\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"identity_mode\": {"]
    #[doc = "      \"description\": \"How users and groups are managed in this Silo\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/SiloIdentityMode\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Silo {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "A silo where discoverable is false can be retrieved only by its id - it will not be part of the \"list all silos\" output."]
        pub discoverable: bool,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "How users and groups are managed in this Silo"]
        pub identity_mode: SiloIdentityMode,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Silo> for Silo {
        fn from(value: &Silo) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`Silo`](crate::external_api::views::Silo)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`Silo`](crate::external_api::views::Silo)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"discoverable\","]
    #[doc = "    \"identity_mode\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"admin_group_name\": {"]
    #[doc = "      \"description\": \"If set, this group will be created during Silo creation and granted the \\\"Silo Admin\\\" role. Identity providers can assert that users belong to this group and those users can log in and further initialize the Silo.\\n\\nNote that if configuring a SAML based identity provider, group_attribute_name must be set for users to be considered part of a group. See [`SamlIdentityProviderCreate`] for more information.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"discoverable\": {"]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"identity_mode\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SiloIdentityMode\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloCreate {
        #[doc = "If set, this group will be created during Silo creation and granted the \"Silo Admin\" role. Identity providers can assert that users belong to this group and those users can log in and further initialize the Silo.\n\nNote that if configuring a SAML based identity provider, group_attribute_name must be set for users to be considered part of a group. See [`SamlIdentityProviderCreate`] for more information."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub admin_group_name: ::std::option::Option<::std::string::String>,
        pub description: ::std::string::String,
        pub discoverable: bool,
        pub identity_mode: SiloIdentityMode,
        pub name: Name,
    }

    impl ::std::convert::From<&SiloCreate> for SiloCreate {
        fn from(value: &SiloCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "Describes how identities are managed and users are authenticated in this Silo"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes how identities are managed and users are authenticated in this Silo\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Users are authenticated with SAML using an external authentication provider.  The system updates information about users and groups only during successful authentication (i.e,. \\\"JIT provisioning\\\" of users and groups).\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"saml_jit\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The system is the source of truth about users.  There is no linkage to an external authentication provider or identity provider.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"local_only\""]
    #[doc = "      ]"]
    #[doc = "    }"]
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
    pub enum SiloIdentityMode {
        #[doc = "Users are authenticated with SAML using an external authentication provider.  The system updates information about users and groups only during successful authentication (i.e,. \"JIT provisioning\" of users and groups)."]
        #[serde(rename = "saml_jit")]
        SamlJit,
        #[doc = "The system is the source of truth about users.  There is no linkage to an external authentication provider or identity provider."]
        #[serde(rename = "local_only")]
        LocalOnly,
    }

    impl ::std::convert::From<&Self> for SiloIdentityMode {
        fn from(value: &SiloIdentityMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SiloIdentityMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SamlJit => f.write_str("saml_jit"),
                Self::LocalOnly => f.write_str("local_only"),
            }
        }
    }

    impl ::std::str::FromStr for SiloIdentityMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "saml_jit" => Ok(Self::SamlJit),
                "local_only" => Ok(Self::LocalOnly),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SiloIdentityMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SiloIdentityMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SiloIdentityMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Silo\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Silo>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SiloResultsPage> for SiloResultsPage {
        fn from(value: &SiloResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`SiloRole`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"admin\","]
    #[doc = "    \"collaborator\","]
    #[doc = "    \"viewer\""]
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
    pub enum SiloRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl ::std::convert::From<&Self> for SiloRole {
        fn from(value: &SiloRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SiloRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => f.write_str("admin"),
                Self::Collaborator => f.write_str("collaborator"),
                Self::Viewer => f.write_str("viewer"),
            }
        }
    }

    impl ::std::str::FromStr for SiloRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SiloRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SiloRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SiloRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a [`Policy`], which describes how this resource may be accessed\n\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Policy`], which describes how this resource may be accessed\\n\\nNote that the Policy only describes access granted explicitly for this resource.  The policies of parent resources can also cause a user to have access to this resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"role_assignments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"role_assignments\": {"]
    #[doc = "      \"description\": \"Roles directly assigned on this resource\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SiloRoleRoleAssignment\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloRolePolicy {
        #[doc = "Roles directly assigned on this resource"]
        pub role_assignments: ::std::vec::Vec<SiloRoleRoleAssignment>,
    }

    impl ::std::convert::From<&SiloRolePolicy> for SiloRolePolicy {
        fn from(value: &SiloRolePolicy) -> Self {
            value.clone()
        }
    }

    #[doc = "Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\n\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Describes the assignment of a particular role on a particular resource to a particular identity (user, group, etc.)\\n\\nThe resource is not part of this structure.  Rather, [`RoleAssignment`]s are put into a [`Policy`] and that Policy is applied to a particular resource.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"identity_id\","]
    #[doc = "    \"identity_type\","]
    #[doc = "    \"role_name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"identity_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"identity_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/IdentityType\""]
    #[doc = "    },"]
    #[doc = "    \"role_name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SiloRole\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloRoleRoleAssignment {
        pub identity_id: ::uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: SiloRole,
    }

    impl ::std::convert::From<&SiloRoleRoleAssignment> for SiloRoleRoleAssignment {
        fn from(value: &SiloRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`Sled`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Sled`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"baseboard\","]
    #[doc = "    \"id\","]
    #[doc = "    \"service_address\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"baseboard\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Baseboard\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"service_address\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Sled {
        pub baseboard: Baseboard,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        pub service_address: ::std::string::String,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Sled> for Sled {
        fn from(value: &Sled) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Sled\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SledResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Sled>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SledResultsPage> for SledResultsPage {
        fn from(value: &SledResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a Snapshot"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a Snapshot\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"disk_id\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"project_id\","]
    #[doc = "    \"size\","]
    #[doc = "    \"state\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"disk_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"project_id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"size\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ByteCount\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SnapshotState\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Snapshot {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        pub disk_id: ::uuid::Uuid,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        pub project_id: ::uuid::Uuid,
        pub size: ByteCount,
        pub state: SnapshotState,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Snapshot> for Snapshot {
        fn from(value: &Snapshot) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`Snapshot`](crate::external_api::views::Snapshot)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`Snapshot`](crate::external_api::views::Snapshot)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"disk\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"disk\": {"]
    #[doc = "      \"description\": \"The name of the disk to be snapshotted\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotCreate {
        pub description: ::std::string::String,
        #[doc = "The name of the disk to be snapshotted"]
        pub disk: Name,
        pub name: Name,
    }

    impl ::std::convert::From<&SnapshotCreate> for SnapshotCreate {
        fn from(value: &SnapshotCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Snapshot\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Snapshot>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SnapshotResultsPage> for SnapshotResultsPage {
        fn from(value: &SnapshotResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`SnapshotState`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"creating\","]
    #[doc = "    \"ready\","]
    #[doc = "    \"faulted\","]
    #[doc = "    \"destroyed\""]
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
    pub enum SnapshotState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "faulted")]
        Faulted,
        #[serde(rename = "destroyed")]
        Destroyed,
    }

    impl ::std::convert::From<&Self> for SnapshotState {
        fn from(value: &SnapshotState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SnapshotState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => f.write_str("creating"),
                Self::Ready => f.write_str("ready"),
                Self::Faulted => f.write_str("faulted"),
                Self::Destroyed => f.write_str("destroyed"),
            }
        }
    }

    impl ::std::str::FromStr for SnapshotState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "creating" => Ok(Self::Creating),
                "ready" => Ok(Self::Ready),
                "faulted" => Ok(Self::Faulted),
                "destroyed" => Ok(Self::Destroyed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SnapshotState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SnapshotState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SnapshotState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`SpoofLoginBody`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"username\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"username\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SpoofLoginBody {
        pub username: ::std::string::String,
    }

    impl ::std::convert::From<&SpoofLoginBody> for SpoofLoginBody {
        fn from(value: &SpoofLoginBody) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`SshKey`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`SshKey`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"public_key\","]
    #[doc = "    \"silo_user_id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"public_key\": {"]
    #[doc = "      \"description\": \"SSH public key, e.g., `\\\"ssh-ed25519 AAAAC3NzaC...\\\"`\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"silo_user_id\": {"]
    #[doc = "      \"description\": \"The user to whom this key belongs\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SshKey {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "SSH public key, e.g., `\"ssh-ed25519 AAAAC3NzaC...\"`"]
        pub public_key: ::std::string::String,
        #[doc = "The user to whom this key belongs"]
        pub silo_user_id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&SshKey> for SshKey {
        fn from(value: &SshKey) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for an [`SshKey`](crate::external_api::views::SshKey)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for an [`SshKey`](crate::external_api::views::SshKey)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\","]
    #[doc = "    \"public_key\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"public_key\": {"]
    #[doc = "      \"description\": \"SSH public key, e.g., `\\\"ssh-ed25519 AAAAC3NzaC...\\\"`\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SshKeyCreate {
        pub description: ::std::string::String,
        pub name: Name,
        #[doc = "SSH public key, e.g., `\"ssh-ed25519 AAAAC3NzaC...\"`"]
        pub public_key: ::std::string::String,
    }

    impl ::std::convert::From<&SshKeyCreate> for SshKeyCreate {
        fn from(value: &SshKeyCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SshKey\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SshKeyResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<SshKey>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SshKeyResultsPage> for SshKeyResultsPage {
        fn from(value: &SshKeyResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`SystemMetricName`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"virtual_disk_space_provisioned\","]
    #[doc = "    \"cpus_provisioned\","]
    #[doc = "    \"ram_provisioned\""]
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
    pub enum SystemMetricName {
        #[serde(rename = "virtual_disk_space_provisioned")]
        VirtualDiskSpaceProvisioned,
        #[serde(rename = "cpus_provisioned")]
        CpusProvisioned,
        #[serde(rename = "ram_provisioned")]
        RamProvisioned,
    }

    impl ::std::convert::From<&Self> for SystemMetricName {
        fn from(value: &SystemMetricName) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SystemMetricName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VirtualDiskSpaceProvisioned => f.write_str("virtual_disk_space_provisioned"),
                Self::CpusProvisioned => f.write_str("cpus_provisioned"),
                Self::RamProvisioned => f.write_str("ram_provisioned"),
            }
        }
    }

    impl ::std::str::FromStr for SystemMetricName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "virtual_disk_space_provisioned" => Ok(Self::VirtualDiskSpaceProvisioned),
                "cpus_provisioned" => Ok(Self::CpusProvisioned),
                "ram_provisioned" => Ok(Self::RamProvisioned),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SystemMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SystemMetricName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SystemMetricName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Identity-related metadata that's included in \"asset\" public API objects (which generally have no name or description)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Identity-related metadata that's included in \\\"asset\\\" public API objects (which generally have no name or description)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemUpdate {
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl ::std::convert::From<&SystemUpdate> for SystemUpdate {
        fn from(value: &SystemUpdate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/SystemUpdate\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemUpdateResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<SystemUpdate>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SystemUpdateResultsPage> for SystemUpdateResultsPage {
        fn from(value: &SystemUpdateResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`SystemUpdateStart`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemUpdateStart {
        pub version: SemverVersion,
    }

    impl ::std::convert::From<&SystemUpdateStart> for SystemUpdateStart {
        fn from(value: &SystemUpdateStart) -> Self {
            value.clone()
        }
    }

    #[doc = "`SystemVersion`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"status\","]
    #[doc = "    \"version_range\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"status\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/UpdateStatus\""]
    #[doc = "    },"]
    #[doc = "    \"version_range\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/VersionRange\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemVersion {
        pub status: UpdateStatus,
        pub version_range: VersionRange,
    }

    impl ::std::convert::From<&SystemVersion> for SystemVersion {
        fn from(value: &SystemVersion) -> Self {
            value.clone()
        }
    }

    #[doc = "Names are constructed by concatenating the target and metric names with ':'. Target and metric names must be lowercase alphanumeric characters with '_' separating words."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"The name of a timeseries\","]
    #[doc = "  \"description\": \"Names are constructed by concatenating the target and metric names with ':'. Target and metric names must be lowercase alphanumeric characters with '_' separating words.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"pattern\": \"(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*)\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct TimeseriesName(::std::string::String);
    impl ::std::ops::Deref for TimeseriesName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<TimeseriesName> for ::std::string::String {
        fn from(value: TimeseriesName) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TimeseriesName> for TimeseriesName {
        fn from(value: &TimeseriesName) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for TimeseriesName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new(
                        "(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*)",
                    )
                    .unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*)\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for TimeseriesName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TimeseriesName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TimeseriesName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for TimeseriesName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "The schema for a timeseries.\n\nThis includes the name of the timeseries, as well as the datum type of its metric and the schema for each field."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The schema for a timeseries.\\n\\nThis includes the name of the timeseries, as well as the datum type of its metric and the schema for each field.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"created\","]
    #[doc = "    \"datum_type\","]
    #[doc = "    \"field_schema\","]
    #[doc = "    \"timeseries_name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"created\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"datum_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/DatumType\""]
    #[doc = "    },"]
    #[doc = "    \"field_schema\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/FieldSchema\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"timeseries_name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/TimeseriesName\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TimeseriesSchema {
        pub created: ::chrono::DateTime<::chrono::offset::Utc>,
        pub datum_type: DatumType,
        pub field_schema: ::std::vec::Vec<FieldSchema>,
        pub timeseries_name: TimeseriesName,
    }

    impl ::std::convert::From<&TimeseriesSchema> for TimeseriesSchema {
        fn from(value: &TimeseriesSchema) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/TimeseriesSchema\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TimeseriesSchemaResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<TimeseriesSchema>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&TimeseriesSchemaResultsPage> for TimeseriesSchemaResultsPage {
        fn from(value: &TimeseriesSchemaResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Identity-related metadata that's included in \"asset\" public API objects (which generally have no name or description)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Identity-related metadata that's included in \\\"asset\\\" public API objects (which generally have no name or description)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"id\","]
    #[doc = "    \"status\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"status\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/UpdateStatus\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateDeployment {
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        pub status: UpdateStatus,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl ::std::convert::From<&UpdateDeployment> for UpdateDeployment {
        fn from(value: &UpdateDeployment) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/UpdateDeployment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateDeploymentResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<UpdateDeployment>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&UpdateDeploymentResultsPage> for UpdateDeploymentResultsPage {
        fn from(value: &UpdateDeploymentResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`UpdateStatus`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"status\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"status\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"updating\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"status\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"status\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"steady\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
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
    #[serde(tag = "status")]
    pub enum UpdateStatus {
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "steady")]
        Steady,
    }

    impl ::std::convert::From<&Self> for UpdateStatus {
        fn from(value: &UpdateStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for UpdateStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Updating => f.write_str("updating"),
                Self::Steady => f.write_str("steady"),
            }
        }
    }

    impl ::std::str::FromStr for UpdateStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "updating" => Ok(Self::Updating),
                "steady" => Ok(Self::Steady),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdateStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UpdateStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdateStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Identity-related metadata that's included in \"asset\" public API objects (which generally have no name or description)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Identity-related metadata that's included in \\\"asset\\\" public API objects (which generally have no name or description)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"component_type\","]
    #[doc = "    \"device_id\","]
    #[doc = "    \"id\","]
    #[doc = "    \"status\","]
    #[doc = "    \"system_version\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"version\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"component_type\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/UpdateableComponentType\""]
    #[doc = "    },"]
    #[doc = "    \"device_id\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"status\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/UpdateStatus\""]
    #[doc = "    },"]
    #[doc = "    \"system_version\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"version\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateableComponent {
        pub component_type: UpdateableComponentType,
        pub device_id: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        pub status: UpdateStatus,
        pub system_version: SemverVersion,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl ::std::convert::From<&UpdateableComponent> for UpdateableComponent {
        fn from(value: &UpdateableComponent) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/UpdateableComponent\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateableComponentResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<UpdateableComponent>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&UpdateableComponentResultsPage> for UpdateableComponentResultsPage {
        fn from(value: &UpdateableComponentResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "`UpdateableComponentType`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"bootloader_for_rot\","]
    #[doc = "    \"bootloader_for_sp\","]
    #[doc = "    \"bootloader_for_host_proc\","]
    #[doc = "    \"hubris_for_psc_rot\","]
    #[doc = "    \"hubris_for_psc_sp\","]
    #[doc = "    \"hubris_for_sidecar_rot\","]
    #[doc = "    \"hubris_for_sidecar_sp\","]
    #[doc = "    \"hubris_for_gimlet_rot\","]
    #[doc = "    \"hubris_for_gimlet_sp\","]
    #[doc = "    \"helios_host_phase1\","]
    #[doc = "    \"helios_host_phase2\","]
    #[doc = "    \"host_omicron\""]
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
    pub enum UpdateableComponentType {
        #[serde(rename = "bootloader_for_rot")]
        BootloaderForRot,
        #[serde(rename = "bootloader_for_sp")]
        BootloaderForSp,
        #[serde(rename = "bootloader_for_host_proc")]
        BootloaderForHostProc,
        #[serde(rename = "hubris_for_psc_rot")]
        HubrisForPscRot,
        #[serde(rename = "hubris_for_psc_sp")]
        HubrisForPscSp,
        #[serde(rename = "hubris_for_sidecar_rot")]
        HubrisForSidecarRot,
        #[serde(rename = "hubris_for_sidecar_sp")]
        HubrisForSidecarSp,
        #[serde(rename = "hubris_for_gimlet_rot")]
        HubrisForGimletRot,
        #[serde(rename = "hubris_for_gimlet_sp")]
        HubrisForGimletSp,
        #[serde(rename = "helios_host_phase1")]
        HeliosHostPhase1,
        #[serde(rename = "helios_host_phase2")]
        HeliosHostPhase2,
        #[serde(rename = "host_omicron")]
        HostOmicron,
    }

    impl ::std::convert::From<&Self> for UpdateableComponentType {
        fn from(value: &UpdateableComponentType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for UpdateableComponentType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BootloaderForRot => f.write_str("bootloader_for_rot"),
                Self::BootloaderForSp => f.write_str("bootloader_for_sp"),
                Self::BootloaderForHostProc => f.write_str("bootloader_for_host_proc"),
                Self::HubrisForPscRot => f.write_str("hubris_for_psc_rot"),
                Self::HubrisForPscSp => f.write_str("hubris_for_psc_sp"),
                Self::HubrisForSidecarRot => f.write_str("hubris_for_sidecar_rot"),
                Self::HubrisForSidecarSp => f.write_str("hubris_for_sidecar_sp"),
                Self::HubrisForGimletRot => f.write_str("hubris_for_gimlet_rot"),
                Self::HubrisForGimletSp => f.write_str("hubris_for_gimlet_sp"),
                Self::HeliosHostPhase1 => f.write_str("helios_host_phase1"),
                Self::HeliosHostPhase2 => f.write_str("helios_host_phase2"),
                Self::HostOmicron => f.write_str("host_omicron"),
            }
        }
    }

    impl ::std::str::FromStr for UpdateableComponentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "bootloader_for_rot" => Ok(Self::BootloaderForRot),
                "bootloader_for_sp" => Ok(Self::BootloaderForSp),
                "bootloader_for_host_proc" => Ok(Self::BootloaderForHostProc),
                "hubris_for_psc_rot" => Ok(Self::HubrisForPscRot),
                "hubris_for_psc_sp" => Ok(Self::HubrisForPscSp),
                "hubris_for_sidecar_rot" => Ok(Self::HubrisForSidecarRot),
                "hubris_for_sidecar_sp" => Ok(Self::HubrisForSidecarSp),
                "hubris_for_gimlet_rot" => Ok(Self::HubrisForGimletRot),
                "hubris_for_gimlet_sp" => Ok(Self::HubrisForGimletSp),
                "helios_host_phase1" => Ok(Self::HeliosHostPhase1),
                "helios_host_phase2" => Ok(Self::HeliosHostPhase2),
                "host_omicron" => Ok(Self::HostOmicron),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdateableComponentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UpdateableComponentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdateableComponentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Client view of a [`User`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`User`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"display_name\","]
    #[doc = "    \"id\","]
    #[doc = "    \"silo_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"display_name\": {"]
    #[doc = "      \"description\": \"Human-readable name that can identify the user\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"silo_id\": {"]
    #[doc = "      \"description\": \"Uuid of the silo to which this user belongs\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct User {
        #[doc = "Human-readable name that can identify the user"]
        pub display_name: ::std::string::String,
        pub id: ::uuid::Uuid,
        #[doc = "Uuid of the silo to which this user belongs"]
        pub silo_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`UserBuiltin`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`UserBuiltin`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserBuiltin {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&UserBuiltin> for UserBuiltin {
        fn from(value: &UserBuiltin) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/UserBuiltin\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserBuiltinResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<UserBuiltin>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&UserBuiltinResultsPage> for UserBuiltinResultsPage {
        fn from(value: &UserBuiltinResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`User`](crate::external_api::views::User)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`User`](crate::external_api::views::User)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"external_id\","]
    #[doc = "    \"password\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"external_id\": {"]
    #[doc = "      \"description\": \"username used to log in\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/UserId\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"password\": {"]
    #[doc = "      \"description\": \"password used to log in\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/UserPassword\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserCreate {
        #[doc = "username used to log in"]
        pub external_id: UserId,
        #[doc = "password used to log in"]
        pub password: UserPassword,
    }

    impl ::std::convert::From<&UserCreate> for UserCreate {
        fn from(value: &UserCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"A name unique within the parent collection\","]
    #[doc = "  \"description\": \"Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"maxLength\": 63,"]
    #[doc = "  \"pattern\": \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\""]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UserId(::std::string::String);
    impl ::std::ops::Deref for UserId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UserId> for ::std::string::String {
        fn from(value: UserId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&UserId> for UserId {
        fn from(value: &UserId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for UserId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 63usize {
                return Err("longer than 63 characters".into());
            }
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
                || {
                    :: regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap ()
                },
            );
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UserId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UserId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    #[doc = "Parameters for setting a user's password"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Parameters for setting a user's password\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Sets the user's password to the provided value\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"details\","]
    #[doc = "        \"user_password_value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"details\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Password\""]
    #[doc = "        },"]
    #[doc = "        \"user_password_value\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"password\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"Invalidates any current password (disabling password authentication)\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"user_password_value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"user_password_value\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"invalid_password\""]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "user_password_value", content = "details")]
    pub enum UserPassword {
        #[doc = "Sets the user's password to the provided value"]
        #[serde(rename = "password")]
        Password(Password),
        #[serde(rename = "invalid_password")]
        InvalidPassword,
    }

    impl ::std::convert::From<&Self> for UserPassword {
        fn from(value: &UserPassword) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<Password> for UserPassword {
        fn from(value: Password) -> Self {
            Self::Password(value)
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/User\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<User>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&UserResultsPage> for UserResultsPage {
        fn from(value: &UserResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Credentials for local user login"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Credentials for local user login\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"password\","]
    #[doc = "    \"username\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"password\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Password\""]
    #[doc = "    },"]
    #[doc = "    \"username\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/UserId\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UsernamePasswordCredentials {
        pub password: Password,
        pub username: UserId,
    }

    impl ::std::convert::From<&UsernamePasswordCredentials> for UsernamePasswordCredentials {
        fn from(value: &UsernamePasswordCredentials) -> Self {
            value.clone()
        }
    }

    #[doc = "`VersionRange`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"high\","]
    #[doc = "    \"low\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"high\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    },"]
    #[doc = "    \"low\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/SemverVersion\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VersionRange {
        pub high: SemverVersion,
        pub low: SemverVersion,
    }

    impl ::std::convert::From<&VersionRange> for VersionRange {
        fn from(value: &VersionRange) -> Self {
            value.clone()
        }
    }

    #[doc = "Client view of a [`Vpc`]"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Client view of a [`Vpc`]\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"dns_name\","]
    #[doc = "    \"id\","]
    #[doc = "    \"ipv6_prefix\","]
    #[doc = "    \"name\","]
    #[doc = "    \"project_id\","]
    #[doc = "    \"system_router_id\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"dns_name\": {"]
    #[doc = "      \"description\": \"The name used for the VPC in DNS.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"ipv6_prefix\": {"]
    #[doc = "      \"description\": \"The unique local IPv6 address range for subnets in this VPC\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv6Net\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"project_id\": {"]
    #[doc = "      \"description\": \"id for the project containing this VPC\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"system_router_id\": {"]
    #[doc = "      \"description\": \"id for the system router where subnet default routes are registered\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Vpc {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "The name used for the VPC in DNS."]
        pub dns_name: Name,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "The unique local IPv6 address range for subnets in this VPC"]
        pub ipv6_prefix: Ipv6Net,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "id for the project containing this VPC"]
        pub project_id: ::uuid::Uuid,
        #[doc = "id for the system router where subnet default routes are registered"]
        pub system_router_id: ::uuid::Uuid,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Vpc> for Vpc {
        fn from(value: &Vpc) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`Vpc`](crate::external_api::views::Vpc)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`Vpc`](crate::external_api::views::Vpc)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"dns_name\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"dns_name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    },"]
    #[doc = "    \"ipv6_prefix\": {"]
    #[doc = "      \"description\": \"The IPv6 prefix for this VPC.\\n\\nAll IPv6 subnets created from this VPC must be taken from this range, which sould be a Unique Local Address in the range `fd00::/48`. The default VPC Subnet will have the first `/64` range from this prefix.\","]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Ipv6Net\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcCreate {
        pub description: ::std::string::String,
        pub dns_name: Name,
        #[doc = "The IPv6 prefix for this VPC.\n\nAll IPv6 subnets created from this VPC must be taken from this range, which sould be a Unique Local Address in the range `fd00::/48`. The default VPC Subnet will have the first `/64` range from this prefix."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ipv6_prefix: ::std::option::Option<Ipv6Net>,
        pub name: Name,
    }

    impl ::std::convert::From<&VpcCreate> for VpcCreate {
        fn from(value: &VpcCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single rule in a VPC firewall"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single rule in a VPC firewall\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"action\","]
    #[doc = "    \"description\","]
    #[doc = "    \"direction\","]
    #[doc = "    \"filters\","]
    #[doc = "    \"id\","]
    #[doc = "    \"name\","]
    #[doc = "    \"priority\","]
    #[doc = "    \"status\","]
    #[doc = "    \"targets\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"vpc_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"action\": {"]
    #[doc = "      \"description\": \"whether traffic matching the rule should be allowed or dropped\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleAction\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"direction\": {"]
    #[doc = "      \"description\": \"whether this rule is for incoming or outgoing traffic\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleDirection\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"filters\": {"]
    #[doc = "      \"description\": \"reductions on the scope of the rule\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleFilter\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"priority\": {"]
    #[doc = "      \"description\": \"the relative priority of this rule\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint16\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"status\": {"]
    #[doc = "      \"description\": \"whether this rule is in effect\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleStatus\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"targets\": {"]
    #[doc = "      \"description\": \"list of sets of instances that the rule applies to\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcFirewallRuleTarget\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"vpc_id\": {"]
    #[doc = "      \"description\": \"the VPC to which this rule belongs\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRule {
        #[doc = "whether traffic matching the rule should be allowed or dropped"]
        pub action: VpcFirewallRuleAction,
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "whether this rule is for incoming or outgoing traffic"]
        pub direction: VpcFirewallRuleDirection,
        #[doc = "reductions on the scope of the rule"]
        pub filters: VpcFirewallRuleFilter,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "the relative priority of this rule"]
        pub priority: u16,
        #[doc = "whether this rule is in effect"]
        pub status: VpcFirewallRuleStatus,
        #[doc = "list of sets of instances that the rule applies to"]
        pub targets: ::std::vec::Vec<VpcFirewallRuleTarget>,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "the VPC to which this rule belongs"]
        pub vpc_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&VpcFirewallRule> for VpcFirewallRule {
        fn from(value: &VpcFirewallRule) -> Self {
            value.clone()
        }
    }

    #[doc = "`VpcFirewallRuleAction`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"allow\","]
    #[doc = "    \"deny\""]
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
    pub enum VpcFirewallRuleAction {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
    }

    impl ::std::convert::From<&Self> for VpcFirewallRuleAction {
        fn from(value: &VpcFirewallRuleAction) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Allow => f.write_str("allow"),
                Self::Deny => f.write_str("deny"),
            }
        }
    }

    impl ::std::str::FromStr for VpcFirewallRuleAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "allow" => Ok(Self::Allow),
                "deny" => Ok(Self::Deny),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VpcFirewallRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VpcFirewallRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`VpcFirewallRuleDirection`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"inbound\","]
    #[doc = "    \"outbound\""]
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
    pub enum VpcFirewallRuleDirection {
        #[serde(rename = "inbound")]
        Inbound,
        #[serde(rename = "outbound")]
        Outbound,
    }

    impl ::std::convert::From<&Self> for VpcFirewallRuleDirection {
        fn from(value: &VpcFirewallRuleDirection) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleDirection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Inbound => f.write_str("inbound"),
                Self::Outbound => f.write_str("outbound"),
            }
        }
    }

    impl ::std::str::FromStr for VpcFirewallRuleDirection {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "inbound" => Ok(Self::Inbound),
                "outbound" => Ok(Self::Outbound),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VpcFirewallRuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VpcFirewallRuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Filter for a firewall rule. A given packet must match every field that is present for the rule to apply to it. A packet matches a field if any entry in that field matches the packet.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"hosts\": {"]
    #[doc = "      \"description\": \"If present, the sources (if incoming) or destinations (if outgoing) this rule applies to.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcFirewallRuleHostFilter\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"ports\": {"]
    #[doc = "      \"description\": \"If present, the destination ports this rule applies to.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/L4PortRange\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"protocols\": {"]
    #[doc = "      \"description\": \"If present, the networking protocols this rule applies to.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcFirewallRuleProtocol\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRuleFilter {
        #[doc = "If present, the sources (if incoming) or destinations (if outgoing) this rule applies to."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub hosts: ::std::option::Option<::std::vec::Vec<VpcFirewallRuleHostFilter>>,
        #[doc = "If present, the destination ports this rule applies to."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ports: ::std::option::Option<::std::vec::Vec<L4PortRange>>,
        #[doc = "If present, the networking protocols this rule applies to."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub protocols: ::std::option::Option<::std::vec::Vec<VpcFirewallRuleProtocol>>,
    }

    impl ::std::convert::From<&VpcFirewallRuleFilter> for VpcFirewallRuleFilter {
        fn from(value: &VpcFirewallRuleFilter) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for VpcFirewallRuleFilter {
        fn default() -> Self {
            Self {
                hosts: Default::default(),
                ports: Default::default(),
                protocols: Default::default(),
            }
        }
    }

    #[doc = "The `VpcFirewallRuleHostFilter` is used to filter traffic on the basis of its source or destination host."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The `VpcFirewallRuleHostFilter` is used to filter traffic on the basis of its source or destination host.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to traffic from/to all instances in the VPC\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"vpc\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to traffic from/to all instances in the VPC Subnet\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"subnet\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to traffic from/to this specific instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"instance\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to traffic from/to a specific IP address\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"ip\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to traffic from/to a specific IP subnet\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip_net\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/IpNet\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum VpcFirewallRuleHostFilter {
        #[doc = "The rule applies to traffic from/to all instances in the VPC"]
        #[serde(rename = "vpc")]
        Vpc(Name),
        #[doc = "The rule applies to traffic from/to all instances in the VPC Subnet"]
        #[serde(rename = "subnet")]
        Subnet(Name),
        #[doc = "The rule applies to traffic from/to this specific instance"]
        #[serde(rename = "instance")]
        Instance(Name),
        #[doc = "The rule applies to traffic from/to a specific IP address"]
        #[serde(rename = "ip")]
        Ip(::std::net::IpAddr),
        #[doc = "The rule applies to traffic from/to a specific IP subnet"]
        #[serde(rename = "ip_net")]
        IpNet(IpNet),
    }

    impl ::std::convert::From<&Self> for VpcFirewallRuleHostFilter {
        fn from(value: &VpcFirewallRuleHostFilter) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::net::IpAddr> for VpcFirewallRuleHostFilter {
        fn from(value: ::std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    impl ::std::convert::From<IpNet> for VpcFirewallRuleHostFilter {
        fn from(value: IpNet) -> Self {
            Self::IpNet(value)
        }
    }

    #[doc = "The protocols that may be specified in a firewall rule's filter"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"The protocols that may be specified in a firewall rule's filter\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"TCP\","]
    #[doc = "    \"UDP\","]
    #[doc = "    \"ICMP\""]
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
    pub enum VpcFirewallRuleProtocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
        #[serde(rename = "ICMP")]
        Icmp,
    }

    impl ::std::convert::From<&Self> for VpcFirewallRuleProtocol {
        fn from(value: &VpcFirewallRuleProtocol) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("TCP"),
                Self::Udp => f.write_str("UDP"),
                Self::Icmp => f.write_str("ICMP"),
            }
        }
    }

    impl ::std::str::FromStr for VpcFirewallRuleProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TCP" => Ok(Self::Tcp),
                "UDP" => Ok(Self::Udp),
                "ICMP" => Ok(Self::Icmp),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VpcFirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VpcFirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "`VpcFirewallRuleStatus`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"disabled\","]
    #[doc = "    \"enabled\""]
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
    pub enum VpcFirewallRuleStatus {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "enabled")]
        Enabled,
    }

    impl ::std::convert::From<&Self> for VpcFirewallRuleStatus {
        fn from(value: &VpcFirewallRuleStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => f.write_str("disabled"),
                Self::Enabled => f.write_str("enabled"),
            }
        }
    }

    impl ::std::str::FromStr for VpcFirewallRuleStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VpcFirewallRuleStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VpcFirewallRuleStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "A `VpcFirewallRuleTarget` is used to specify the set of [`Instance`]s to which a firewall rule applies."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A `VpcFirewallRuleTarget` is used to specify the set of [`Instance`]s to which a firewall rule applies.\","]
    #[doc = "  \"oneOf\": ["]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to all instances in the VPC\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"vpc\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to all instances in the VPC Subnet\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"subnet\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to this specific instance\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"instance\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to a specific IP address\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"format\": \"ip\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    {"]
    #[doc = "      \"description\": \"The rule applies to a specific IP subnet\","]
    #[doc = "      \"type\": \"object\","]
    #[doc = "      \"required\": ["]
    #[doc = "        \"type\","]
    #[doc = "        \"value\""]
    #[doc = "      ],"]
    #[doc = "      \"properties\": {"]
    #[doc = "        \"type\": {"]
    #[doc = "          \"type\": \"string\","]
    #[doc = "          \"enum\": ["]
    #[doc = "            \"ip_net\""]
    #[doc = "          ]"]
    #[doc = "        },"]
    #[doc = "        \"value\": {"]
    #[doc = "          \"$ref\": \"#/components/schemas/IpNet\""]
    #[doc = "        }"]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum VpcFirewallRuleTarget {
        #[doc = "The rule applies to all instances in the VPC"]
        #[serde(rename = "vpc")]
        Vpc(Name),
        #[doc = "The rule applies to all instances in the VPC Subnet"]
        #[serde(rename = "subnet")]
        Subnet(Name),
        #[doc = "The rule applies to this specific instance"]
        #[serde(rename = "instance")]
        Instance(Name),
        #[doc = "The rule applies to a specific IP address"]
        #[serde(rename = "ip")]
        Ip(::std::net::IpAddr),
        #[doc = "The rule applies to a specific IP subnet"]
        #[serde(rename = "ip_net")]
        IpNet(IpNet),
    }

    impl ::std::convert::From<&Self> for VpcFirewallRuleTarget {
        fn from(value: &VpcFirewallRuleTarget) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::net::IpAddr> for VpcFirewallRuleTarget {
        fn from(value: ::std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    impl ::std::convert::From<IpNet> for VpcFirewallRuleTarget {
        fn from(value: IpNet) -> Self {
            Self::IpNet(value)
        }
    }

    #[doc = "A single rule in a VPC firewall"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single rule in a VPC firewall\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"action\","]
    #[doc = "    \"description\","]
    #[doc = "    \"direction\","]
    #[doc = "    \"filters\","]
    #[doc = "    \"name\","]
    #[doc = "    \"priority\","]
    #[doc = "    \"status\","]
    #[doc = "    \"targets\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"action\": {"]
    #[doc = "      \"description\": \"whether traffic matching the rule should be allowed or dropped\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleAction\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"direction\": {"]
    #[doc = "      \"description\": \"whether this rule is for incoming or outgoing traffic\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleDirection\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"filters\": {"]
    #[doc = "      \"description\": \"reductions on the scope of the rule\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleFilter\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"name of the rule, unique to this VPC\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"priority\": {"]
    #[doc = "      \"description\": \"the relative priority of this rule\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"uint16\","]
    #[doc = "      \"minimum\": 0.0"]
    #[doc = "    },"]
    #[doc = "    \"status\": {"]
    #[doc = "      \"description\": \"whether this rule is in effect\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/VpcFirewallRuleStatus\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"targets\": {"]
    #[doc = "      \"description\": \"list of sets of instances that the rule applies to\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcFirewallRuleTarget\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRuleUpdate {
        #[doc = "whether traffic matching the rule should be allowed or dropped"]
        pub action: VpcFirewallRuleAction,
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "whether this rule is for incoming or outgoing traffic"]
        pub direction: VpcFirewallRuleDirection,
        #[doc = "reductions on the scope of the rule"]
        pub filters: VpcFirewallRuleFilter,
        #[doc = "name of the rule, unique to this VPC"]
        pub name: Name,
        #[doc = "the relative priority of this rule"]
        pub priority: u16,
        #[doc = "whether this rule is in effect"]
        pub status: VpcFirewallRuleStatus,
        #[doc = "list of sets of instances that the rule applies to"]
        pub targets: ::std::vec::Vec<VpcFirewallRuleTarget>,
    }

    impl ::std::convert::From<&VpcFirewallRuleUpdate> for VpcFirewallRuleUpdate {
        fn from(value: &VpcFirewallRuleUpdate) -> Self {
            value.clone()
        }
    }

    #[doc = "Updateable properties of a `Vpc`'s firewall Note that VpcFirewallRules are implicitly created along with a Vpc, so there is no explicit creation."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of a `Vpc`'s firewall Note that VpcFirewallRules are implicitly created along with a Vpc, so there is no explicit creation.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"rules\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"rules\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcFirewallRuleUpdate\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRuleUpdateParams {
        pub rules: ::std::vec::Vec<VpcFirewallRuleUpdate>,
    }

    impl ::std::convert::From<&VpcFirewallRuleUpdateParams> for VpcFirewallRuleUpdateParams {
        fn from(value: &VpcFirewallRuleUpdateParams) -> Self {
            value.clone()
        }
    }

    #[doc = "Collection of a Vpc's firewall rules"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Collection of a Vpc's firewall rules\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"rules\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"rules\": {"]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcFirewallRule\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRules {
        pub rules: ::std::vec::Vec<VpcFirewallRule>,
    }

    impl ::std::convert::From<&VpcFirewallRules> for VpcFirewallRules {
        fn from(value: &VpcFirewallRules) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/Vpc\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<Vpc>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&VpcResultsPage> for VpcResultsPage {
        fn from(value: &VpcResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "A VPC router defines a series of rules that indicate where traffic should be sent depending on its destination."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A VPC router defines a series of rules that indicate where traffic should be sent depending on its destination.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"kind\","]
    #[doc = "    \"name\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"vpc_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"kind\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/VpcRouterKind\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"vpc_id\": {"]
    #[doc = "      \"description\": \"The VPC to which the router belongs.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcRouter {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        pub kind: VpcRouterKind,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "The VPC to which the router belongs."]
        pub vpc_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&VpcRouter> for VpcRouter {
        fn from(value: &VpcRouter) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`VpcRouter`](crate::external_api::views::VpcRouter)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`VpcRouter`](crate::external_api::views::VpcRouter)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcRouterCreate {
        pub description: ::std::string::String,
        pub name: Name,
    }

    impl ::std::convert::From<&VpcRouterCreate> for VpcRouterCreate {
        fn from(value: &VpcRouterCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "`VpcRouterKind`"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"system\","]
    #[doc = "    \"custom\""]
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
    pub enum VpcRouterKind {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "custom")]
        Custom,
    }

    impl ::std::convert::From<&Self> for VpcRouterKind {
        fn from(value: &VpcRouterKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcRouterKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => f.write_str("system"),
                Self::Custom => f.write_str("custom"),
            }
        }
    }

    impl ::std::str::FromStr for VpcRouterKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "custom" => Ok(Self::Custom),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VpcRouterKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for VpcRouterKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VpcRouterKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcRouter\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcRouterResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<VpcRouter>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&VpcRouterResultsPage> for VpcRouterResultsPage {
        fn from(value: &VpcRouterResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Updateable properties of a [`VpcRouter`](crate::external_api::views::VpcRouter)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of a [`VpcRouter`](crate::external_api::views::VpcRouter)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
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
    pub struct VpcRouterUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
    }

    impl ::std::convert::From<&VpcRouterUpdate> for VpcRouterUpdate {
        fn from(value: &VpcRouterUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for VpcRouterUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
            }
        }
    }

    #[doc = "A VPC subnet represents a logical grouping for instances that allows network traffic between them, within a IPv4 subnetwork or optionall an IPv6 subnetwork."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A VPC subnet represents a logical grouping for instances that allows network traffic between them, within a IPv4 subnetwork or optionall an IPv6 subnetwork.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"id\","]
    #[doc = "    \"ipv4_block\","]
    #[doc = "    \"ipv6_block\","]
    #[doc = "    \"name\","]
    #[doc = "    \"time_created\","]
    #[doc = "    \"time_modified\","]
    #[doc = "    \"vpc_id\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"description\": \"human-readable free-form text about a resource\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"id\": {"]
    #[doc = "      \"description\": \"unique, immutable, system-controlled identifier for each resource\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    },"]
    #[doc = "    \"ipv4_block\": {"]
    #[doc = "      \"description\": \"The IPv4 subnet CIDR block.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv4Net\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ipv6_block\": {"]
    #[doc = "      \"description\": \"The IPv6 subnet CIDR block.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv6Net\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"description\": \"unique, mutable, user-controlled identifier for each resource\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"time_created\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was created\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"time_modified\": {"]
    #[doc = "      \"description\": \"timestamp when this resource was last modified\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"vpc_id\": {"]
    #[doc = "      \"description\": \"The VPC to which the subnet belongs.\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"uuid\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcSubnet {
        #[doc = "human-readable free-form text about a resource"]
        pub description: ::std::string::String,
        #[doc = "unique, immutable, system-controlled identifier for each resource"]
        pub id: ::uuid::Uuid,
        #[doc = "The IPv4 subnet CIDR block."]
        pub ipv4_block: Ipv4Net,
        #[doc = "The IPv6 subnet CIDR block."]
        pub ipv6_block: Ipv6Net,
        #[doc = "unique, mutable, user-controlled identifier for each resource"]
        pub name: Name,
        #[doc = "timestamp when this resource was created"]
        pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "timestamp when this resource was last modified"]
        pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "The VPC to which the subnet belongs."]
        pub vpc_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&VpcSubnet> for VpcSubnet {
        fn from(value: &VpcSubnet) -> Self {
            value.clone()
        }
    }

    #[doc = "Create-time parameters for a [`VpcSubnet`](crate::external_api::views::VpcSubnet)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Create-time parameters for a [`VpcSubnet`](crate::external_api::views::VpcSubnet)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"ipv4_block\","]
    #[doc = "    \"name\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"ipv4_block\": {"]
    #[doc = "      \"description\": \"The IPv4 address range for this subnet.\\n\\nIt must be allocated from an RFC 1918 private address range, and must not overlap with any other existing subnet in the VPC.\","]
    #[doc = "      \"allOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"$ref\": \"#/components/schemas/Ipv4Net\""]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ipv6_block\": {"]
    #[doc = "      \"description\": \"The IPv6 address range for this subnet.\\n\\nIt must be allocated from the RFC 4193 Unique Local Address range, with the prefix equal to the parent VPC's prefix. A random `/64` block will be assigned if one is not provided. It must not overlap with any existing subnet in the VPC.\","]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Ipv6Net\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcSubnetCreate {
        pub description: ::std::string::String,
        #[doc = "The IPv4 address range for this subnet.\n\nIt must be allocated from an RFC 1918 private address range, and must not overlap with any other existing subnet in the VPC."]
        pub ipv4_block: Ipv4Net,
        #[doc = "The IPv6 address range for this subnet.\n\nIt must be allocated from the RFC 4193 Unique Local Address range, with the prefix equal to the parent VPC's prefix. A random `/64` block will be assigned if one is not provided. It must not overlap with any existing subnet in the VPC."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ipv6_block: ::std::option::Option<Ipv6Net>,
        pub name: Name,
    }

    impl ::std::convert::From<&VpcSubnetCreate> for VpcSubnetCreate {
        fn from(value: &VpcSubnetCreate) -> Self {
            value.clone()
        }
    }

    #[doc = "A single page of results"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"A single page of results\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"items\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"items\": {"]
    #[doc = "      \"description\": \"list of items on this page of results\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/VpcSubnet\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"next_page\": {"]
    #[doc = "      \"description\": \"token used to fetch the next page of results (if any)\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  }"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcSubnetResultsPage {
        #[doc = "list of items on this page of results"]
        pub items: ::std::vec::Vec<VpcSubnet>,
        #[doc = "token used to fetch the next page of results (if any)"]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&VpcSubnetResultsPage> for VpcSubnetResultsPage {
        fn from(value: &VpcSubnetResultsPage) -> Self {
            value.clone()
        }
    }

    #[doc = "Updateable properties of a [`VpcSubnet`](crate::external_api::views::VpcSubnet)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of a [`VpcSubnet`](crate::external_api::views::VpcSubnet)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
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
    pub struct VpcSubnetUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
    }

    impl ::std::convert::From<&VpcSubnetUpdate> for VpcSubnetUpdate {
        fn from(value: &VpcSubnetUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for VpcSubnetUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                name: Default::default(),
            }
        }
    }

    #[doc = "Updateable properties of a [`Vpc`](crate::external_api::views::Vpc)"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"description\": \"Updateable properties of a [`Vpc`](crate::external_api::views::Vpc)\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"dns_name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
    #[doc = "            }"]
    #[doc = "          ]"]
    #[doc = "        }"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"oneOf\": ["]
    #[doc = "        {"]
    #[doc = "          \"type\": \"null\""]
    #[doc = "        },"]
    #[doc = "        {"]
    #[doc = "          \"allOf\": ["]
    #[doc = "            {"]
    #[doc = "              \"$ref\": \"#/components/schemas/Name\""]
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
    pub struct VpcUpdate {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub dns_name: ::std::option::Option<Name>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<Name>,
    }

    impl ::std::convert::From<&VpcUpdate> for VpcUpdate {
        fn from(value: &VpcUpdate) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for VpcUpdate {
        fn default() -> Self {
            Self {
                description: Default::default(),
                dns_name: Default::default(),
                name: Default::default(),
            }
        }
    }

    #[doc = r" Generation of default values for serde."]
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }

        pub(super) fn instance_create_network_interfaces(
        ) -> super::InstanceNetworkInterfaceAttachment {
            super::InstanceNetworkInterfaceAttachment::Default
        }
    }
}

#[derive(Clone, Debug)]
#[doc = "Client for Oxide Region API\n\nAPI for interacting with the Oxide control plane\n\nVersion: 0.0.1"]
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
    #[doc = "Fetch a disk by id\n\nUse `GET /v1/disks/{disk}` instead\n\nSends a `GET` request to `/by-id/disks/{id}`\n\n"]
    pub async fn disk_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/disks/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "disk_view_by_id",
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

    #[doc = "Fetch an image by id\n\nSends a `GET` request to `/by-id/images/{id}`\n\n"]
    pub async fn image_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Image>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/images/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "image_view_by_id",
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

    #[doc = "Fetch an instance by id\n\nSends a `GET` request to `/by-id/instances/{id}`\n\n"]
    pub async fn instance_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/instances/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "instance_view_by_id",
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

    #[doc = "Fetch a network interface by id\n\nSends a `GET` request to `/by-id/network-interfaces/{id}`\n\n"]
    pub async fn instance_network_interface_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/network-interfaces/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "instance_network_interface_view_by_id",
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

    #[doc = "Fetch an organization by id\n\nUse `GET /v1/organizations/{organization}` instead\n\nSends a `GET` request to `/by-id/organizations/{id}`\n\n"]
    pub async fn organization_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/organizations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "organization_view_by_id",
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

    #[doc = "Fetch a project by id\n\nUse `GET /v1/projects/{project}` instead\n\nSends a `GET` request to `/by-id/projects/{id}`\n\n"]
    pub async fn project_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/projects/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "project_view_by_id",
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

    #[doc = "Fetch a snapshot by id\n\nSends a `GET` request to `/by-id/snapshots/{id}`\n\n"]
    pub async fn snapshot_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Snapshot>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/snapshots/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "snapshot_view_by_id",
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

    #[doc = "Fetch a route by id\n\nSends a `GET` request to `/by-id/vpc-router-routes/{id}`\n\n"]
    pub async fn vpc_router_route_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpc-router-routes/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "vpc_router_route_view_by_id",
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

    #[doc = "Get a router by id\n\nSends a `GET` request to `/by-id/vpc-routers/{id}`\n\n"]
    pub async fn vpc_router_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpc-routers/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "vpc_router_view_by_id",
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

    #[doc = "Fetch a subnet by id\n\nSends a `GET` request to `/by-id/vpc-subnets/{id}`\n\n"]
    pub async fn vpc_subnet_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpc-subnets/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "vpc_subnet_view_by_id",
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

    #[doc = "Fetch a VPC\n\nSends a `GET` request to `/by-id/vpcs/{id}`\n\n"]
    pub async fn vpc_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpcs/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "vpc_view_by_id",
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

    #[doc = "Start an OAuth 2.0 Device Authorization Grant\n\nThis endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted.\n\nSends a `POST` request to `/device/auth`\n\n"]
    pub async fn device_auth_request<'a>(
        &'a self,
        body: &'a types::DeviceAuthRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
        let url = format!("{}/device/auth", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "device_auth_request",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
        }
    }

    #[doc = "Confirm an OAuth 2.0 Device Authorization Grant\n\nThis endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/device/token`.\n\nSends a `POST` request to `/device/confirm`\n\n"]
    pub async fn device_auth_confirm<'a>(
        &'a self,
        body: &'a types::DeviceAuthVerify,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/device/confirm", self.baseurl,);
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
            operation_id: "device_auth_confirm",
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

    #[doc = "Request a device access token\n\nThis endpoint should be polled by the client until the user code is verified and the grant is confirmed.\n\nSends a `POST` request to `/device/token`\n\n"]
    pub async fn device_access_token<'a>(
        &'a self,
        body: &'a types::DeviceAccessTokenRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
        let url = format!("{}/device/token", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .form_urlencoded(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "device_access_token",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
        }
    }

    #[doc = "List groups\n\nSends a `GET` request to `/groups`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn group_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::GroupResultsPage>, Error<types::Error>> {
        let url = format!("{}/groups", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "group_list",
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

    #[doc = "List groups as a Stream\n\nSends repeated `GET` requests to `/groups` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn group_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Group, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.group_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.group_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Sends a `POST` request to `/login`\n\n"]
    pub async fn login_spoof<'a>(
        &'a self,
        body: &'a types::SpoofLoginBody,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/login", self.baseurl,);
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
            operation_id: "login_spoof",
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

    #[doc = "Authenticate a user (i.e., log in) via username and password\n\nSends a `POST` request to `/login/{silo_name}/local`\n\n"]
    pub async fn login_local<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::UsernamePasswordCredentials,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/login/{}/local",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            operation_id: "login_local",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Prompt user login\n\nEither display a page asking a user for their credentials, or redirect them to their identity provider.\n\nSends a `GET` request to `/login/{silo_name}/saml/{provider_name}`\n\n"]
    pub async fn login_saml_begin<'a>(
        &'a self,
        silo_name: &'a types::Name,
        provider_name: &'a types::Name,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/login/{}/saml/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&provider_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "login_saml_begin",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Authenticate a user (i.e., log in) via SAML\n\nSends a `POST` request to `/login/{silo_name}/saml/{provider_name}`\n\n"]
    pub async fn login_saml<'a, B: Into<reqwest::Body>>(
        &'a self,
        silo_name: &'a types::Name,
        provider_name: &'a types::Name,
        body: B,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/login/{}/saml/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&provider_name.to_string()),
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
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("application/octet-stream"),
            )
            .body(body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "login_saml",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Sends a `POST` request to `/logout`\n\n"]
    pub async fn logout<'a>(&'a self) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/logout", self.baseurl,);
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
            operation_id: "logout",
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

    #[doc = "List organizations\n\nUse `GET /v1/organizations` instead\n\nSends a `GET` request to `/organizations`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn organization_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::OrganizationResultsPage>, Error<types::Error>> {
        let url = format!("{}/organizations", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "organization_list",
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

    #[doc = "List organizations as a Stream\n\nUse `GET /v1/organizations` instead\n\nSends repeated `GET` requests to `/organizations` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn organization_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Organization, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.organization_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.organization_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an organization\n\nUse `POST /v1/organizations` instead\n\nSends a `POST` request to `/organizations`\n\n"]
    pub async fn organization_create<'a>(
        &'a self,
        body: &'a types::OrganizationCreate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!("{}/organizations", self.baseurl,);
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
            operation_id: "organization_create",
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

    #[doc = "Fetch an organization\n\nUse `GET /v1/organizations/{organization}` instead\n\nSends a `GET` request to `/organizations/{organization_name}`\n\nArguments:\n- `organization_name`: The organization's unique name.\n"]
    pub async fn organization_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
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
            operation_id: "organization_view",
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

    #[doc = "Update an organization\n\nUse `PUT /v1/organizations/{organization}` instead\n\nSends a `PUT` request to `/organizations/{organization_name}`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `body`\n"]
    pub async fn organization_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        body: &'a types::OrganizationUpdate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
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
            operation_id: "organization_update",
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

    #[doc = "Delete an organization\n\nUse `DELETE /v1/organizations/{organization}` instead\n\nSends a `DELETE` request to `/organizations/{organization_name}`\n\nArguments:\n- `organization_name`: The organization's unique name.\n"]
    pub async fn organization_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "organization_delete",
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

    #[doc = "Fetch an organization's IAM policy\n\nUse `GET /v1/organizations/{organization}/policy` instead\n\nSends a `GET` request to `/organizations/{organization_name}/policy`\n\nArguments:\n- `organization_name`: The organization's unique name.\n"]
    pub async fn organization_policy_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
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
            operation_id: "organization_policy_view",
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

    #[doc = "Update an organization's IAM policy\n\nUse `PUT /v1/organizations/{organization}/policy` instead\n\nSends a `PUT` request to `/organizations/{organization_name}/policy`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `body`\n"]
    pub async fn organization_policy_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        body: &'a types::OrganizationRolePolicy,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
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
            operation_id: "organization_policy_update",
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

    #[doc = "List projects\n\nUse `GET /v1/projects` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn project_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::ProjectResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects",
            self.baseurl,
            encode_path(&organization_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_list",
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

    #[doc = "List projects as a Stream\n\nUse `GET /v1/projects` instead\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects` until there are no more results.\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn project_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Project, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.project_list(organization_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.project_list(organization_name, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a project\n\nUse `POST /v1/projects` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `body`\n"]
    pub async fn project_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        body: &'a types::ProjectCreate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects",
            self.baseurl,
            encode_path(&organization_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_create",
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

    #[doc = "Fetch a project\n\nUse `GET /v1/projects/{project}` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n"]
    pub async fn project_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            operation_id: "project_view",
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

    #[doc = "Update a project\n\nUse `PUT /v1/projects/{project}` instead\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn project_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::ProjectUpdate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
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
            operation_id: "project_update",
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

    #[doc = "Delete a project\n\nUse `DELETE /v1/projects/{project}` instead\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n"]
    pub async fn project_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_delete",
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

    #[doc = "List disks\n\nUse `GET /v1/disks` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/disks`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn disk_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_list",
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

    #[doc = "List disks as a Stream\n\nUse `GET /v1/disks` instead\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/disks` until there are no more results.\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn disk_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.disk_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.disk_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Use `POST /v1/disks` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/disks`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn disk_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::DiskCreate,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_create",
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

    #[doc = "Fetch a disk\n\nUse `GET /v1/disks/{disk}` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}`\n\n"]
    pub async fn disk_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&disk_name.to_string()),
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
            operation_id: "disk_view",
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

    #[doc = "Use `DELETE /v1/disks/{disk}` instead\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}`\n\n"]
    pub async fn disk_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&disk_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_delete",
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

    #[doc = "Fetch disk metrics\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}/metrics/{metric_name}`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `disk_name`\n- `metric_name`\n- `end_time`: An exclusive end time of metrics.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `start_time`: An inclusive start time of metrics.\n"]
    pub async fn disk_metrics_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
        metric_name: types::DiskMetricName,
        end_time: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        start_time: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
    ) -> Result<ResponseValue<types::MeasurementResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks/{}/metrics/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&disk_name.to_string()),
            encode_path(&metric_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("end_time", &end_time))
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "start_time",
                &start_time,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_metrics_list",
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

    #[doc = "Fetch disk metrics as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/disks/{disk_name}/metrics/{metric_name}` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `disk_name`\n- `metric_name`\n- `end_time`: An exclusive end time of metrics.\n- `limit`: Maximum number of items returned by a single call\n- `start_time`: An inclusive start time of metrics.\n"]
    pub fn disk_metrics_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
        metric_name: types::DiskMetricName,
        end_time: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
        limit: Option<::std::num::NonZeroU32>,
        start_time: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
    ) -> impl futures::Stream<Item = Result<types::Measurement, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.disk_metrics_list(
            organization_name,
            project_name,
            disk_name,
            metric_name,
            end_time,
            limit,
            None,
            start_time,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.disk_metrics_list(
                        organization_name,
                        project_name,
                        disk_name,
                        metric_name,
                        None,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "List images\n\nList images in a project. The images are returned sorted by creation date, with the most recent images appearing first.\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/images`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn image_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::ImageResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "image_list",
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

    #[doc = "List images as a Stream\n\nList images in a project. The images are returned sorted by creation date, with the most recent images appearing first.\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/images` until there are no more results.\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn image_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Image, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.image_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.image_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an image\n\nCreate a new image in a project.\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/images`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn image_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::ImageCreate,
    ) -> Result<ResponseValue<types::Image>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "image_create",
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

    #[doc = "Fetch an image\n\nFetch the details for a specific image in a project.\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/images/{image_name}`\n\n"]
    pub async fn image_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Image>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&image_name.to_string()),
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
            operation_id: "image_view",
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

    #[doc = "Delete an image\n\nPermanently delete an image from a project. This operation cannot be undone. Any instances in the project using the image will continue to run, however new instances can not be created with this image.\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/images/{image_name}`\n\n"]
    pub async fn image_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&image_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "image_delete",
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

    #[doc = "List instances\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn instance_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::InstanceResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_list",
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

    #[doc = "List instances as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/instances` until there are no more results.\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn instance_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Instance, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.instance_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an instance\n\nUse `POST /v1/instances` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn instance_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::InstanceCreate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_create",
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

    #[doc = "Fetch an instance\n\nUse `GET /v1/instances/{instance}` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}`\n\n"]
    pub async fn instance_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            operation_id: "instance_view",
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

    #[doc = "Delete an instance\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}`\n\n"]
    pub async fn instance_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_delete",
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

    #[doc = "List an instance's disks\n\nUse `GET /v1/instances/{instance}/disks` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `instance_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn instance_disk_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/disks",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_disk_list",
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

    #[doc = "List an instance's disks as a Stream\n\nUse `GET /v1/instances/{instance}/disks` instead\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `instance_name`\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn instance_disk_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_disk_list(
            organization_name,
            project_name,
            instance_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.instance_disk_list(
                        organization_name,
                        project_name,
                        instance_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "Attach a disk to an instance\n\nUse `POST /v1/instances/{instance}/disks/attach` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks/attach`\n\n"]
    pub async fn instance_disk_attach<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::DiskIdentifier,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/disks/attach",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_disk_attach",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Detach a disk from an instance\n\nUse `POST /v1/disks/{disk}/detach` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/disks/detach`\n\n"]
    pub async fn instance_disk_detach<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::DiskIdentifier,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/disks/detach",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_disk_detach",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List external IP addresses\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/external-ips`\n\n"]
    pub async fn instance_external_ip_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::ExternalIpResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/external-ips",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            operation_id: "instance_external_ip_list",
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

    #[doc = "Migrate an instance\n\nUse `POST /v1/instances/{instance}/migrate` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/migrate`\n\n"]
    pub async fn instance_migrate<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::InstanceMigrate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/migrate",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_migrate",
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

    #[doc = "List network interfaces\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `instance_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn instance_network_interface_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::NetworkInterfaceResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_network_interface_list",
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

    #[doc = "List network interfaces as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `instance_name`\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn instance_network_interface_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::NetworkInterface, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_network_interface_list(
            organization_name,
            project_name,
            instance_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.instance_network_interface_list(
                        organization_name,
                        project_name,
                        instance_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "Create a network interface\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces`\n\n"]
    pub async fn instance_network_interface_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::NetworkInterfaceCreate,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_network_interface_create",
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

    #[doc = "Fetch a network interface\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}`\n\n"]
    pub async fn instance_network_interface_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        interface_name: &'a types::Name,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
            encode_path(&interface_name.to_string()),
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
            operation_id: "instance_network_interface_view",
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

    #[doc = "Update a network interface\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}`\n\n"]
    pub async fn instance_network_interface_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        interface_name: &'a types::Name,
        body: &'a types::NetworkInterfaceUpdate,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
            encode_path(&interface_name.to_string()),
        );
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
            operation_id: "instance_network_interface_update",
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

    #[doc = "Delete a network interface\n\nNote that the primary interface for an instance cannot be deleted if there are any secondary interfaces. A new primary interface must be designated first. The primary interface can be deleted if there are no secondary interfaces.\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/network-interfaces/{interface_name}`\n\n"]
    pub async fn instance_network_interface_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        interface_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
            encode_path(&interface_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_network_interface_delete",
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

    #[doc = "Reboot an instance\n\nUse `POST /v1/instances/{instance}/reboot` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/reboot`\n\n"]
    pub async fn instance_reboot<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/reboot",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            operation_id: "instance_reboot",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Fetch an instance's serial console\n\nUse `GET /v1/instances/{instance}/serial-console` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/serial-console`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `instance_name`\n- `from_start`: Character index in the serial buffer from which to read, counting the bytes output since instance start. If this is not provided, `most_recent` must be provided, and if this *is* provided, `most_recent` must *not* be provided.\n- `max_bytes`: Maximum number of bytes of buffered serial console contents to return. If the requested range runs to the end of the available buffer, the data returned will be shorter than `max_bytes`.\n- `most_recent`: Character index in the serial buffer from which to read, counting *backward* from the most recently buffered data retrieved from the instance. (See note on `from_start` about mutual exclusivity)\n"]
    pub async fn instance_serial_console<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        from_start: Option<u64>,
        max_bytes: Option<u64>,
        most_recent: Option<u64>,
    ) -> Result<ResponseValue<types::InstanceSerialConsoleData>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/serial-console",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "from_start",
                &from_start,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("max_bytes", &max_bytes))
            .query(&progenitor_middleware_client::QueryParam::new(
                "most_recent",
                &most_recent,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_serial_console",
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

    #[doc = "Connect to an instance's serial console\n\nUse `GET /v1/instances/{instance}/serial-console/stream` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/serial-console/stream`\n\n"]
    pub async fn instance_serial_console_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/serial-console/stream",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            operation_id: "instance_serial_console_stream",
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

    #[doc = "Boot an instance\n\nUse `POST /v1/instances/{instance}/start` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/start`\n\n"]
    pub async fn instance_start<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/start",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            operation_id: "instance_start",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Halt an instance\n\nUse `POST /v1/instances/{instance}/stop` instead\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/instances/{instance_name}/stop`\n\n"]
    pub async fn instance_stop<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/stop",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            operation_id: "instance_stop",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Fetch a project's IAM policy\n\nUse `GET /v1/projects/{project}/policy` instead\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/policy`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n"]
    pub async fn project_policy_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            operation_id: "project_policy_view",
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

    #[doc = "Update a project's IAM policy\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/policy`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn project_policy_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::ProjectRolePolicy,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
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
            operation_id: "project_policy_update",
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

    #[doc = "List snapshots\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/snapshots`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn snapshot_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::SnapshotResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "snapshot_list",
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

    #[doc = "List snapshots as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/snapshots` until there are no more results.\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn snapshot_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Snapshot, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.snapshot_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.snapshot_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a snapshot\n\nCreates a point-in-time snapshot from a disk.\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/snapshots`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn snapshot_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::SnapshotCreate,
    ) -> Result<ResponseValue<types::Snapshot>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "snapshot_create",
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

    #[doc = "Fetch a snapshot\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/snapshots/{snapshot_name}`\n\n"]
    pub async fn snapshot_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        snapshot_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Snapshot>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&snapshot_name.to_string()),
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
            operation_id: "snapshot_view",
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

    #[doc = "Delete a snapshot\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/snapshots/{snapshot_name}`\n\n"]
    pub async fn snapshot_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        snapshot_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&snapshot_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "snapshot_delete",
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

    #[doc = "List VPCs\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn vpc_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::VpcResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_list",
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

    #[doc = "List VPCs as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/vpcs` until there are no more results.\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn vpc_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Vpc, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.vpc_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a VPC\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/vpcs`\n\nArguments:\n- `organization_name`: The organization's unique name.\n- `project_name`: The project's unique name within the organization.\n- `body`\n"]
    pub async fn vpc_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::VpcCreate,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_create",
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

    #[doc = "Fetch a VPC\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}`\n\n"]
    pub async fn vpc_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
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
            operation_id: "vpc_view",
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

    #[doc = "Update a VPC\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}`\n\n"]
    pub async fn vpc_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcUpdate,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
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
            operation_id: "vpc_update",
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

    #[doc = "Delete a VPC\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}`\n\n"]
    pub async fn vpc_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_delete",
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

    #[doc = "List firewall rules\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules`\n\n"]
    pub async fn vpc_firewall_rules_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
    ) -> Result<ResponseValue<types::VpcFirewallRules>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
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
            operation_id: "vpc_firewall_rules_view",
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

    #[doc = "Replace firewall rules\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/firewall/rules`\n\n"]
    pub async fn vpc_firewall_rules_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcFirewallRuleUpdateParams,
    ) -> Result<ResponseValue<types::VpcFirewallRules>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
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
            operation_id: "vpc_firewall_rules_update",
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

    #[doc = "List routers\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn vpc_router_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::VpcRouterResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_router_list",
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

    #[doc = "List routers as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn vpc_router_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::VpcRouter, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_router_list(
            organization_name,
            project_name,
            vpc_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_router_list(
                        organization_name,
                        project_name,
                        vpc_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "Create a router\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers`\n\n"]
    pub async fn vpc_router_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcRouterCreate,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_router_create",
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

    #[doc = "Get a router\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}`\n\n"]
    pub async fn vpc_router_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
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
            operation_id: "vpc_router_view",
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

    #[doc = "Update a router\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}`\n\n"]
    pub async fn vpc_router_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        body: &'a types::VpcRouterUpdate,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
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
            operation_id: "vpc_router_update",
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

    #[doc = "Delete a router\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}`\n\n"]
    pub async fn vpc_router_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_router_delete",
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

    #[doc = "List routes\n\nList the routes associated with a router in a particular VPC.\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `router_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn vpc_router_route_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::RouterRouteResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_router_route_list",
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

    #[doc = "List routes as a Stream\n\nList the routes associated with a router in a particular VPC.\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `router_name`\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn vpc_router_route_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::RouterRoute, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_router_route_list(
            organization_name,
            project_name,
            vpc_name,
            router_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_router_route_list(
                        organization_name,
                        project_name,
                        vpc_name,
                        router_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "Create a router\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes`\n\n"]
    pub async fn vpc_router_route_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        body: &'a types::RouterRouteCreateParams,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_router_route_create",
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

    #[doc = "Fetch a route\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes/{route_name}`\n\n"]
    pub async fn vpc_router_route_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        route_name: &'a types::Name,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
            encode_path(&route_name.to_string()),
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
            operation_id: "vpc_router_route_view",
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

    #[doc = "Update a route\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes/{route_name}`\n\n"]
    pub async fn vpc_router_route_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        route_name: &'a types::Name,
        body: &'a types::RouterRouteUpdateParams,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
            encode_path(&route_name.to_string()),
        );
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
            operation_id: "vpc_router_route_update",
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

    #[doc = "Delete a route\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/routers/{router_name}/routes/{route_name}`\n\n"]
    pub async fn vpc_router_route_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        route_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
            encode_path(&route_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_router_route_delete",
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

    #[doc = "List subnets\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn vpc_subnet_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::VpcSubnetResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_subnet_list",
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

    #[doc = "List subnets as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn vpc_subnet_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::VpcSubnet, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_subnet_list(
            organization_name,
            project_name,
            vpc_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_subnet_list(
                        organization_name,
                        project_name,
                        vpc_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "Create a subnet\n\nSends a `POST` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets`\n\n"]
    pub async fn vpc_subnet_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcSubnetCreate,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_subnet_create",
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

    #[doc = "Fetch a subnet\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}`\n\n"]
    pub async fn vpc_subnet_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
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
            operation_id: "vpc_subnet_view",
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

    #[doc = "Update a subnet\n\nSends a `PUT` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}`\n\n"]
    pub async fn vpc_subnet_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
        body: &'a types::VpcSubnetUpdate,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
        );
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
            operation_id: "vpc_subnet_update",
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

    #[doc = "Delete a subnet\n\nSends a `DELETE` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}`\n\n"]
    pub async fn vpc_subnet_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_subnet_delete",
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

    #[doc = "List network interfaces\n\nSends a `GET` request to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}/network-interfaces`\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `subnet_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn vpc_subnet_list_network_interfaces<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::NetworkInterfaceResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}/network-interfaces",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vpc_subnet_list_network_interfaces",
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

    #[doc = "List network interfaces as a Stream\n\nSends repeated `GET` requests to `/organizations/{organization_name}/projects/{project_name}/vpcs/{vpc_name}/subnets/{subnet_name}/network-interfaces` until there are no more results.\n\nArguments:\n- `organization_name`\n- `project_name`\n- `vpc_name`\n- `subnet_name`\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn vpc_subnet_list_network_interfaces_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::NetworkInterface, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_subnet_list_network_interfaces(
            organization_name,
            project_name,
            vpc_name,
            subnet_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_subnet_list_network_interfaces(
                        organization_name,
                        project_name,
                        vpc_name,
                        subnet_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    #[doc = "Fetch the current silo's IAM policy\n\nSends a `GET` request to `/policy`\n\n"]
    pub async fn policy_view<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!("{}/policy", self.baseurl,);
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
            operation_id: "policy_view",
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

    #[doc = "Update the current silo's IAM policy\n\nSends a `PUT` request to `/policy`\n\n"]
    pub async fn policy_update<'a>(
        &'a self,
        body: &'a types::SiloRolePolicy,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!("{}/policy", self.baseurl,);
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
            operation_id: "policy_update",
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

    #[doc = "List built-in roles\n\nSends a `GET` request to `/roles`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n"]
    pub async fn role_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::RoleResultsPage>, Error<types::Error>> {
        let url = format!("{}/roles", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "role_list",
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

    #[doc = "List built-in roles as a Stream\n\nSends repeated `GET` requests to `/roles` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n"]
    pub fn role_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::Role, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.role_list(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.role_list(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a built-in role\n\nSends a `GET` request to `/roles/{role_name}`\n\nArguments:\n- `role_name`: The built-in role's unique name.\n"]
    pub async fn role_view<'a>(
        &'a self,
        role_name: &'a str,
    ) -> Result<ResponseValue<types::Role>, Error<types::Error>> {
        let url = format!(
            "{}/roles/{}",
            self.baseurl,
            encode_path(&role_name.to_string()),
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
            operation_id: "role_view",
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

    #[doc = "Fetch the user associated with the current session\n\nSends a `GET` request to `/session/me`\n\n"]
    pub async fn session_me<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::User>, Error<types::Error>> {
        let url = format!("{}/session/me", self.baseurl,);
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
            operation_id: "session_me",
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

    #[doc = "Fetch the silo\u{a0}groups the current user belongs to\n\nSends a `GET` request to `/session/me/groups`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn session_me_groups<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::GroupResultsPage>, Error<types::Error>> {
        let url = format!("{}/session/me/groups", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "session_me_groups",
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

    #[doc = "Fetch the silo\u{a0}groups the current user belongs to as a Stream\n\nSends repeated `GET` requests to `/session/me/groups` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn session_me_groups_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Group, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.session_me_groups(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.session_me_groups(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List SSH public keys\n\nLists SSH public keys for the currently authenticated user.\n\nSends a `GET` request to `/session/me/sshkeys`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn session_sshkey_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::SshKeyResultsPage>, Error<types::Error>> {
        let url = format!("{}/session/me/sshkeys", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "session_sshkey_list",
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

    #[doc = "List SSH public keys as a Stream\n\nLists SSH public keys for the currently authenticated user.\n\nSends repeated `GET` requests to `/session/me/sshkeys` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn session_sshkey_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::SshKey, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.session_sshkey_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.session_sshkey_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an SSH public key\n\nCreate an SSH public key for the currently authenticated user.\n\nSends a `POST` request to `/session/me/sshkeys`\n\n"]
    pub async fn session_sshkey_create<'a>(
        &'a self,
        body: &'a types::SshKeyCreate,
    ) -> Result<ResponseValue<types::SshKey>, Error<types::Error>> {
        let url = format!("{}/session/me/sshkeys", self.baseurl,);
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
            operation_id: "session_sshkey_create",
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

    #[doc = "Fetch an SSH public key\n\nFetch an SSH public key associated with the currently authenticated user.\n\nSends a `GET` request to `/session/me/sshkeys/{ssh_key_name}`\n\n"]
    pub async fn session_sshkey_view<'a>(
        &'a self,
        ssh_key_name: &'a types::Name,
    ) -> Result<ResponseValue<types::SshKey>, Error<types::Error>> {
        let url = format!(
            "{}/session/me/sshkeys/{}",
            self.baseurl,
            encode_path(&ssh_key_name.to_string()),
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
            operation_id: "session_sshkey_view",
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

    #[doc = "Delete an SSH public key\n\nDelete an SSH public key associated with the currently authenticated user.\n\nSends a `DELETE` request to `/session/me/sshkeys/{ssh_key_name}`\n\n"]
    pub async fn session_sshkey_delete<'a>(
        &'a self,
        ssh_key_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/session/me/sshkeys/{}",
            self.baseurl,
            encode_path(&ssh_key_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "session_sshkey_delete",
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

    #[doc = "Fetch a system-wide image by id\n\nSends a `GET` request to `/system/by-id/images/{id}`\n\n"]
    pub async fn system_image_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::GlobalImage>, Error<types::Error>> {
        let url = format!(
            "{}/system/by-id/images/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "system_image_view_by_id",
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

    #[doc = "Fetch an IP pool by id\n\nSends a `GET` request to `/system/by-id/ip-pools/{id}`\n\n"]
    pub async fn ip_pool_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!(
            "{}/system/by-id/ip-pools/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "ip_pool_view_by_id",
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

    #[doc = "Fetch a silo by id\n\nSends a `GET` request to `/system/by-id/silos/{id}`\n\n"]
    pub async fn silo_view_by_id<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Silo>, Error<types::Error>> {
        let url = format!(
            "{}/system/by-id/silos/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "silo_view_by_id",
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

    #[doc = "List system-wide certificates\n\nReturns a list of all the system-wide certificates. System-wide certificates are returned sorted by creation date, with the most recent certificates appearing first.\n\nSends a `GET` request to `/system/certificates`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn certificate_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::CertificateResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/certificates", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "certificate_list",
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

    #[doc = "List system-wide certificates as a Stream\n\nReturns a list of all the system-wide certificates. System-wide certificates are returned sorted by creation date, with the most recent certificates appearing first.\n\nSends repeated `GET` requests to `/system/certificates` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn certificate_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Certificate, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.certificate_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.certificate_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a new system-wide x.509 certificate\n\nThis certificate is automatically used by the Oxide Control plane to serve external connections.\n\nSends a `POST` request to `/system/certificates`\n\n"]
    pub async fn certificate_create<'a>(
        &'a self,
        body: &'a types::CertificateCreate,
    ) -> Result<ResponseValue<types::Certificate>, Error<types::Error>> {
        let url = format!("{}/system/certificates", self.baseurl,);
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
            operation_id: "certificate_create",
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

    #[doc = "Fetch a certificate\n\nReturns the details of a specific certificate\n\nSends a `GET` request to `/system/certificates/{certificate}`\n\n"]
    pub async fn certificate_view<'a>(
        &'a self,
        certificate: &'a types::NameOrId,
    ) -> Result<ResponseValue<types::Certificate>, Error<types::Error>> {
        let url = format!(
            "{}/system/certificates/{}",
            self.baseurl,
            encode_path(&certificate.to_string()),
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
            operation_id: "certificate_view",
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

    #[doc = "Delete a certificate\n\nPermanently delete a certificate. This operation cannot be undone.\n\nSends a `DELETE` request to `/system/certificates/{certificate}`\n\n"]
    pub async fn certificate_delete<'a>(
        &'a self,
        certificate: &'a types::NameOrId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/certificates/{}",
            self.baseurl,
            encode_path(&certificate.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "certificate_delete",
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

    #[doc = "List physical disks\n\nSends a `GET` request to `/system/hardware/disks`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn physical_disk_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::PhysicalDiskResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/hardware/disks", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "physical_disk_list",
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

    #[doc = "List physical disks as a Stream\n\nSends repeated `GET` requests to `/system/hardware/disks` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn physical_disk_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::PhysicalDisk, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.physical_disk_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.physical_disk_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List racks\n\nSends a `GET` request to `/system/hardware/racks`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn rack_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::RackResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/hardware/racks", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rack_list",
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

    #[doc = "List racks as a Stream\n\nSends repeated `GET` requests to `/system/hardware/racks` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn rack_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Rack, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.rack_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.rack_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a rack\n\nSends a `GET` request to `/system/hardware/racks/{rack_id}`\n\nArguments:\n- `rack_id`: The rack's unique ID.\n"]
    pub async fn rack_view<'a>(
        &'a self,
        rack_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Rack>, Error<types::Error>> {
        let url = format!(
            "{}/system/hardware/racks/{}",
            self.baseurl,
            encode_path(&rack_id.to_string()),
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
            operation_id: "rack_view",
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

    #[doc = "List sleds\n\nSends a `GET` request to `/system/hardware/sleds`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn sled_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::SledResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/hardware/sleds", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "sled_list",
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

    #[doc = "List sleds as a Stream\n\nSends repeated `GET` requests to `/system/hardware/sleds` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn sled_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Sled, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.sled_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.sled_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a sled\n\nSends a `GET` request to `/system/hardware/sleds/{sled_id}`\n\nArguments:\n- `sled_id`: The sled's unique ID.\n"]
    pub async fn sled_view<'a>(
        &'a self,
        sled_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Sled>, Error<types::Error>> {
        let url = format!(
            "{}/system/hardware/sleds/{}",
            self.baseurl,
            encode_path(&sled_id.to_string()),
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
            operation_id: "sled_view",
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

    #[doc = "List physical disks attached to sleds\n\nSends a `GET` request to `/system/hardware/sleds/{sled_id}/disks`\n\nArguments:\n- `sled_id`: The sled's unique ID.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn sled_physical_disk_list<'a>(
        &'a self,
        sled_id: &'a ::uuid::Uuid,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::PhysicalDiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/hardware/sleds/{}/disks",
            self.baseurl,
            encode_path(&sled_id.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "sled_physical_disk_list",
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

    #[doc = "List physical disks attached to sleds as a Stream\n\nSends repeated `GET` requests to `/system/hardware/sleds/{sled_id}/disks` until there are no more results.\n\nArguments:\n- `sled_id`: The sled's unique ID.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn sled_physical_disk_list_stream<'a>(
        &'a self,
        sled_id: &'a ::uuid::Uuid,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::PhysicalDisk, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.sled_physical_disk_list(sled_id, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.sled_physical_disk_list(sled_id, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List system-wide images\n\nReturns a list of all the system-wide images. System-wide images are returned sorted by creation date, with the most recent images appearing first.\n\nSends a `GET` request to `/system/images`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn system_image_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::GlobalImageResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/images", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "system_image_list",
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

    #[doc = "List system-wide images as a Stream\n\nReturns a list of all the system-wide images. System-wide images are returned sorted by creation date, with the most recent images appearing first.\n\nSends repeated `GET` requests to `/system/images` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn system_image_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::GlobalImage, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_image_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_image_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a system-wide image\n\nCreate a new system-wide image. This image can then be used by any user in any silo as a base for instances.\n\nSends a `POST` request to `/system/images`\n\n"]
    pub async fn system_image_create<'a>(
        &'a self,
        body: &'a types::GlobalImageCreate,
    ) -> Result<ResponseValue<types::GlobalImage>, Error<types::Error>> {
        let url = format!("{}/system/images", self.baseurl,);
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
            operation_id: "system_image_create",
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

    #[doc = "Fetch a system-wide image\n\nReturns the details of a specific system-wide image.\n\nSends a `GET` request to `/system/images/{image_name}`\n\n"]
    pub async fn system_image_view<'a>(
        &'a self,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<types::GlobalImage>, Error<types::Error>> {
        let url = format!(
            "{}/system/images/{}",
            self.baseurl,
            encode_path(&image_name.to_string()),
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
            operation_id: "system_image_view",
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

    #[doc = "Delete a system-wide image\n\nPermanently delete a system-wide image. This operation cannot be undone. Any instances using the system-wide image will continue to run, however new instances can not be created with this image.\n\nSends a `DELETE` request to `/system/images/{image_name}`\n\n"]
    pub async fn system_image_delete<'a>(
        &'a self,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/images/{}",
            self.baseurl,
            encode_path(&image_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "system_image_delete",
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

    #[doc = "List IP pools\n\nSends a `GET` request to `/system/ip-pools`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn ip_pool_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::IpPoolResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ip_pool_list",
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

    #[doc = "List IP pools as a Stream\n\nSends repeated `GET` requests to `/system/ip-pools` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn ip_pool_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::IpPool, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.ip_pool_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.ip_pool_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an IP pool\n\nSends a `POST` request to `/system/ip-pools`\n\n"]
    pub async fn ip_pool_create<'a>(
        &'a self,
        body: &'a types::IpPoolCreate,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools", self.baseurl,);
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
            operation_id: "ip_pool_create",
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

    #[doc = "Fetch an IP pool\n\nSends a `GET` request to `/system/ip-pools/{pool_name}`\n\n"]
    pub async fn ip_pool_view<'a>(
        &'a self,
        pool_name: &'a types::Name,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}",
            self.baseurl,
            encode_path(&pool_name.to_string()),
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
            operation_id: "ip_pool_view",
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

    #[doc = "Update an IP Pool\n\nSends a `PUT` request to `/system/ip-pools/{pool_name}`\n\n"]
    pub async fn ip_pool_update<'a>(
        &'a self,
        pool_name: &'a types::Name,
        body: &'a types::IpPoolUpdate,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
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
            operation_id: "ip_pool_update",
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

    #[doc = "Delete an IP Pool\n\nSends a `DELETE` request to `/system/ip-pools/{pool_name}`\n\n"]
    pub async fn ip_pool_delete<'a>(
        &'a self,
        pool_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ip_pool_delete",
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

    #[doc = "List ranges for an IP pool\n\nRanges are ordered by their first address.\n\nSends a `GET` request to `/system/ip-pools/{pool_name}/ranges`\n\nArguments:\n- `pool_name`\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n"]
    pub async fn ip_pool_range_list<'a>(
        &'a self,
        pool_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::IpPoolRangeResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}/ranges",
            self.baseurl,
            encode_path(&pool_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ip_pool_range_list",
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

    #[doc = "List ranges for an IP pool as a Stream\n\nRanges are ordered by their first address.\n\nSends repeated `GET` requests to `/system/ip-pools/{pool_name}/ranges` until there are no more results.\n\nArguments:\n- `pool_name`\n- `limit`: Maximum number of items returned by a single call\n"]
    pub fn ip_pool_range_list_stream<'a>(
        &'a self,
        pool_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::IpPoolRange, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.ip_pool_range_list(pool_name, limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.ip_pool_range_list(pool_name, limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Add a range to an IP pool\n\nSends a `POST` request to `/system/ip-pools/{pool_name}/ranges/add`\n\n"]
    pub async fn ip_pool_range_add<'a>(
        &'a self,
        pool_name: &'a types::Name,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<types::IpPoolRange>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}/ranges/add",
            self.baseurl,
            encode_path(&pool_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ip_pool_range_add",
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

    #[doc = "Remove a range from an IP pool\n\nSends a `POST` request to `/system/ip-pools/{pool_name}/ranges/remove`\n\n"]
    pub async fn ip_pool_range_remove<'a>(
        &'a self,
        pool_name: &'a types::Name,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}/ranges/remove",
            self.baseurl,
            encode_path(&pool_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ip_pool_range_remove",
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

    #[doc = "Fetch the IP pool used for Oxide services\n\nSends a `GET` request to `/system/ip-pools-service`\n\n"]
    pub async fn ip_pool_service_view<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service", self.baseurl,);
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
            operation_id: "ip_pool_service_view",
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

    #[doc = "List ranges for the IP pool used for Oxide services\n\nRanges are ordered by their first address.\n\nSends a `GET` request to `/system/ip-pools-service/ranges`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n"]
    pub async fn ip_pool_service_range_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::IpPoolRangeResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service/ranges", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "ip_pool_service_range_list",
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

    #[doc = "List ranges for the IP pool used for Oxide services as a Stream\n\nRanges are ordered by their first address.\n\nSends repeated `GET` requests to `/system/ip-pools-service/ranges` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n"]
    pub fn ip_pool_service_range_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::IpPoolRange, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.ip_pool_service_range_list(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.ip_pool_service_range_list(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Add a range to an IP pool used for Oxide services\n\nSends a `POST` request to `/system/ip-pools-service/ranges/add`\n\n"]
    pub async fn ip_pool_service_range_add<'a>(
        &'a self,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<types::IpPoolRange>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service/ranges/add", self.baseurl,);
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
            operation_id: "ip_pool_service_range_add",
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

    #[doc = "Remove a range from an IP pool used for Oxide services\n\nSends a `POST` request to `/system/ip-pools-service/ranges/remove`\n\n"]
    pub async fn ip_pool_service_range_remove<'a>(
        &'a self,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service/ranges/remove", self.baseurl,);
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
            operation_id: "ip_pool_service_range_remove",
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

    #[doc = "Access metrics data\n\nSends a `GET` request to `/system/metrics/{metric_name}`\n\nArguments:\n- `metric_name`\n- `end_time`: An exclusive end time of metrics.\n- `id`: The UUID of the container being queried\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `start_time`: An inclusive start time of metrics.\n"]
    pub async fn system_metric<'a>(
        &'a self,
        metric_name: types::SystemMetricName,
        end_time: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
        id: &'a ::uuid::Uuid,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        start_time: Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>,
    ) -> Result<ResponseValue<types::MeasurementResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/metrics/{}",
            self.baseurl,
            encode_path(&metric_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("end_time", &end_time))
            .query(&progenitor_middleware_client::QueryParam::new("id", &id))
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "start_time",
                &start_time,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "system_metric",
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

    #[doc = "Fetch the top-level IAM policy\n\nSends a `GET` request to `/system/policy`\n\n"]
    pub async fn system_policy_view<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::FleetRolePolicy>, Error<types::Error>> {
        let url = format!("{}/system/policy", self.baseurl,);
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
            operation_id: "system_policy_view",
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

    #[doc = "Update the top-level IAM policy\n\nSends a `PUT` request to `/system/policy`\n\n"]
    pub async fn system_policy_update<'a>(
        &'a self,
        body: &'a types::FleetRolePolicy,
    ) -> Result<ResponseValue<types::FleetRolePolicy>, Error<types::Error>> {
        let url = format!("{}/system/policy", self.baseurl,);
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
            operation_id: "system_policy_update",
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

    #[doc = "List sagas\n\nSends a `GET` request to `/system/sagas`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn saga_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::SagaResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/sagas", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "saga_list",
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

    #[doc = "List sagas as a Stream\n\nSends repeated `GET` requests to `/system/sagas` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn saga_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Saga, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.saga_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.saga_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a saga\n\nSends a `GET` request to `/system/sagas/{saga_id}`\n\n"]
    pub async fn saga_view<'a>(
        &'a self,
        saga_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::Saga>, Error<types::Error>> {
        let url = format!(
            "{}/system/sagas/{}",
            self.baseurl,
            encode_path(&saga_id.to_string()),
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
            operation_id: "saga_view",
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

    #[doc = "List silos\n\nLists silos that are discoverable based on the current permissions.\n\nSends a `GET` request to `/system/silos`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn silo_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::SiloResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/silos", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "silo_list",
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

    #[doc = "List silos as a Stream\n\nLists silos that are discoverable based on the current permissions.\n\nSends repeated `GET` requests to `/system/silos` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn silo_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Silo, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.silo_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.silo_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a silo\n\nSends a `POST` request to `/system/silos`\n\n"]
    pub async fn silo_create<'a>(
        &'a self,
        body: &'a types::SiloCreate,
    ) -> Result<ResponseValue<types::Silo>, Error<types::Error>> {
        let url = format!("{}/system/silos", self.baseurl,);
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
            operation_id: "silo_create",
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

    #[doc = "Fetch a silo\n\nFetch a silo by name.\n\nSends a `GET` request to `/system/silos/{silo_name}`\n\nArguments:\n- `silo_name`: The silo's unique name.\n"]
    pub async fn silo_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Silo>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            operation_id: "silo_view",
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

    #[doc = "Delete a silo\n\nDelete a silo by name.\n\nSends a `DELETE` request to `/system/silos/{silo_name}`\n\nArguments:\n- `silo_name`: The silo's unique name.\n"]
    pub async fn silo_delete<'a>(
        &'a self,
        silo_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "silo_delete",
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

    #[doc = "List a silo's IDPs\n\nSends a `GET` request to `/system/silos/{silo_name}/identity-providers`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn silo_identity_provider_list<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::IdentityProviderResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "silo_identity_provider_list",
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

    #[doc = "List a silo's IDPs as a Stream\n\nSends repeated `GET` requests to `/system/silos/{silo_name}/identity-providers` until there are no more results.\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn silo_identity_provider_list_stream<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::IdentityProvider, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.silo_identity_provider_list(silo_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.silo_identity_provider_list(silo_name, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a user\n\nUsers can only be created in Silos with `provision_type` == `Fixed`. Otherwise, Silo users are just-in-time (JIT) provisioned when a user first logs in using an external Identity Provider.\n\nSends a `POST` request to `/system/silos/{silo_name}/identity-providers/local/users`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `body`\n"]
    pub async fn local_idp_user_create<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::UserCreate,
    ) -> Result<ResponseValue<types::User>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/local/users",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "local_idp_user_create",
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

    #[doc = "Delete a user\n\nSends a `DELETE` request to `/system/silos/{silo_name}/identity-providers/local/users/{user_id}`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `user_id`: The user's internal id\n"]
    pub async fn local_idp_user_delete<'a>(
        &'a self,
        silo_name: &'a types::Name,
        user_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/local/users/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "local_idp_user_delete",
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

    #[doc = "Set or invalidate a user's password\n\nPasswords can only be updated for users in Silos with identity mode `LocalOnly`.\n\nSends a `POST` request to `/system/silos/{silo_name}/identity-providers/local/users/{user_id}/set-password`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `user_id`: The user's internal id\n- `body`\n"]
    pub async fn local_idp_user_set_password<'a>(
        &'a self,
        silo_name: &'a types::Name,
        user_id: &'a ::uuid::Uuid,
        body: &'a types::UserPassword,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/local/users/{}/set-password",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&user_id.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "local_idp_user_set_password",
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

    #[doc = "Create a SAML IDP\n\nSends a `POST` request to `/system/silos/{silo_name}/identity-providers/saml`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `body`\n"]
    pub async fn saml_identity_provider_create<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::SamlIdentityProviderCreate,
    ) -> Result<ResponseValue<types::SamlIdentityProvider>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/saml",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "saml_identity_provider_create",
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

    #[doc = "Fetch a SAML IDP\n\nSends a `GET` request to `/system/silos/{silo_name}/identity-providers/saml/{provider_name}`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `provider_name`: The SAML identity provider's name\n"]
    pub async fn saml_identity_provider_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
        provider_name: &'a types::Name,
    ) -> Result<ResponseValue<types::SamlIdentityProvider>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/saml/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&provider_name.to_string()),
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
            operation_id: "saml_identity_provider_view",
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

    #[doc = "Fetch a silo's IAM policy\n\nSends a `GET` request to `/system/silos/{silo_name}/policy`\n\nArguments:\n- `silo_name`: The silo's unique name.\n"]
    pub async fn silo_policy_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/policy",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            operation_id: "silo_policy_view",
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

    #[doc = "Update a silo's IAM policy\n\nSends a `PUT` request to `/system/silos/{silo_name}/policy`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `body`\n"]
    pub async fn silo_policy_update<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::SiloRolePolicy,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/policy",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
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
            operation_id: "silo_policy_update",
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

    #[doc = "List users in a silo\n\nSends a `GET` request to `/system/silos/{silo_name}/users/all`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn silo_users_list<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UserResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/users/all",
            self.baseurl,
            encode_path(&silo_name.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "silo_users_list",
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

    #[doc = "List users in a silo as a Stream\n\nSends repeated `GET` requests to `/system/silos/{silo_name}/users/all` until there are no more results.\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn silo_users_list_stream<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::User, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.silo_users_list(silo_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.silo_users_list(silo_name, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a user\n\nSends a `GET` request to `/system/silos/{silo_name}/users/id/{user_id}`\n\nArguments:\n- `silo_name`: The silo's unique name.\n- `user_id`: The user's internal id\n"]
    pub async fn silo_user_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
        user_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::User>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/users/id/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&user_id.to_string()),
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
            operation_id: "silo_user_view",
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

    #[doc = "List built-in users\n\nSends a `GET` request to `/system/user`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn system_user_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::UserBuiltinResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/user", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "system_user_list",
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

    #[doc = "List built-in users as a Stream\n\nSends repeated `GET` requests to `/system/user` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn system_user_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::UserBuiltin, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_user_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_user_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a built-in user\n\nSends a `GET` request to `/system/user/{user_name}`\n\nArguments:\n- `user_name`: The built-in user's unique name.\n"]
    pub async fn system_user_view<'a>(
        &'a self,
        user_name: &'a types::Name,
    ) -> Result<ResponseValue<types::UserBuiltin>, Error<types::Error>> {
        let url = format!(
            "{}/system/user/{}",
            self.baseurl,
            encode_path(&user_name.to_string()),
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
            operation_id: "system_user_view",
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

    #[doc = "List timeseries schema\n\nSends a `GET` request to `/timeseries/schema`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n"]
    pub async fn timeseries_schema_get<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::TimeseriesSchemaResultsPage>, Error<types::Error>> {
        let url = format!("{}/timeseries/schema", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "timeseries_schema_get",
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

    #[doc = "List timeseries schema as a Stream\n\nSends repeated `GET` requests to `/timeseries/schema` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n"]
    pub fn timeseries_schema_get_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::TimeseriesSchema, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.timeseries_schema_get(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.timeseries_schema_get(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List users\n\nSends a `GET` request to `/users`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn user_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UserResultsPage>, Error<types::Error>> {
        let url = format!("{}/users", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_list",
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

    #[doc = "List users as a Stream\n\nSends repeated `GET` requests to `/users` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn user_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::User, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.user_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.user_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List disks\n\nSends a `GET` request to `/v1/disks`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `project`\n- `sort_by`\n"]
    pub async fn disk_list_v1<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/disks", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_list_v1",
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

    #[doc = "List disks as a Stream\n\nSends repeated `GET` requests to `/v1/disks` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `project`\n- `sort_by`\n"]
    pub fn disk_list_v1_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.disk_list_v1(limit, organization, None, project, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.disk_list_v1(limit, None, state.as_deref(), None, None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a disk\n\nSends a `POST` request to `/v1/disks`\n\n"]
    pub async fn disk_create_v1<'a>(
        &'a self,
        organization: Option<&'a types::NameOrId>,
        project: &'a types::NameOrId,
        body: &'a types::DiskCreate,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!("{}/v1/disks", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_create_v1",
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

    #[doc = "Fetch a disk\n\nSends a `GET` request to `/v1/disks/{disk}`\n\n"]
    pub async fn disk_view_v1<'a>(
        &'a self,
        disk: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/v1/disks/{}",
            self.baseurl,
            encode_path(&disk.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_view_v1",
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

    #[doc = "Delete a disk\n\nSends a `DELETE` request to `/v1/disks/{disk}`\n\n"]
    pub async fn disk_delete_v1<'a>(
        &'a self,
        disk: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/disks/{}",
            self.baseurl,
            encode_path(&disk.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disk_delete_v1",
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

    #[doc = "List instances\n\nSends a `GET` request to `/v1/instances`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `project`\n- `sort_by`\n"]
    pub async fn instance_list_v1<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::InstanceResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/instances", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_list_v1",
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

    #[doc = "List instances as a Stream\n\nSends repeated `GET` requests to `/v1/instances` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `project`\n- `sort_by`\n"]
    pub fn instance_list_v1_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Instance, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_list_v1(limit, organization, None, project, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.instance_list_v1(limit, None, state.as_deref(), None, None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an instance\n\nSends a `POST` request to `/v1/instances`\n\n"]
    pub async fn instance_create_v1<'a>(
        &'a self,
        organization: Option<&'a types::NameOrId>,
        project: &'a types::NameOrId,
        body: &'a types::InstanceCreate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!("{}/v1/instances", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_create_v1",
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

    #[doc = "Fetch an instance\n\nSends a `GET` request to `/v1/instances/{instance}`\n\n"]
    pub async fn instance_view_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_view_v1",
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

    #[doc = "Delete an instance\n\nSends a `DELETE` request to `/v1/instances/{instance}`\n\n"]
    pub async fn instance_delete_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_delete_v1",
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

    #[doc = "List an instance's disks\n\nSends a `GET` request to `/v1/instances/{instance}/disks`\n\nArguments:\n- `instance`\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `project`\n- `sort_by`\n"]
    pub async fn instance_disk_list_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/disks",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_disk_list_v1",
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

    #[doc = "List an instance's disks as a Stream\n\nSends repeated `GET` requests to `/v1/instances/{instance}/disks` until there are no more results.\n\nArguments:\n- `instance`\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `project`\n- `sort_by`\n"]
    pub fn instance_disk_list_v1_stream<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_disk_list_v1(instance, limit, organization, None, project, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.instance_disk_list_v1(
                            instance,
                            limit,
                            None,
                            state.as_deref(),
                            None,
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Attach a disk to an instance\n\nSends a `POST` request to `/v1/instances/{instance}/disks/attach`\n\n"]
    pub async fn instance_disk_attach_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        body: &'a types::DiskPath,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/disks/attach",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .json(&body)
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_disk_attach_v1",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Detach a disk from an instance\n\nSends a `POST` request to `/v1/instances/{instance}/disks/detach`\n\n"]
    pub async fn instance_disk_detach_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        body: &'a types::DiskPath,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/disks/detach",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .json(&body)
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_disk_detach_v1",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Migrate an instance\n\nSends a `POST` request to `/v1/instances/{instance}/migrate`\n\n"]
    pub async fn instance_migrate_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        body: &'a types::InstanceMigrate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/migrate",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .json(&body)
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_migrate_v1",
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

    #[doc = "Reboot an instance\n\nSends a `POST` request to `/v1/instances/{instance}/reboot`\n\n"]
    pub async fn instance_reboot_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/reboot",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_reboot_v1",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Fetch an instance's serial console\n\nSends a `GET` request to `/v1/instances/{instance}/serial-console`\n\nArguments:\n- `instance`\n- `from_start`: Character index in the serial buffer from which to read, counting the bytes output since instance start. If this is not provided, `most_recent` must be provided, and if this *is* provided, `most_recent` must *not* be provided.\n- `max_bytes`: Maximum number of bytes of buffered serial console contents to return. If the requested range runs to the end of the available buffer, the data returned will be shorter than `max_bytes`.\n- `most_recent`: Character index in the serial buffer from which to read, counting *backward* from the most recently buffered data retrieved from the instance. (See note on `from_start` about mutual exclusivity)\n- `organization`\n- `project`\n"]
    pub async fn instance_serial_console_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        from_start: Option<u64>,
        max_bytes: Option<u64>,
        most_recent: Option<u64>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::InstanceSerialConsoleData>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/serial-console",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "from_start",
                &from_start,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("max_bytes", &max_bytes))
            .query(&progenitor_middleware_client::QueryParam::new(
                "most_recent",
                &most_recent,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_serial_console_v1",
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

    #[doc = "Stream an instance's serial console\n\nSends a `GET` request to `/v1/instances/{instance}/serial-console/stream`\n\n"]
    pub async fn instance_serial_console_stream_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
        let url = format!(
            "{}/v1/instances/{}/serial-console/stream",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
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
            operation_id: "instance_serial_console_stream_v1",
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

    #[doc = "Boot an instance\n\nSends a `POST` request to `/v1/instances/{instance}/start`\n\n"]
    pub async fn instance_start_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/start",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_start_v1",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Stop an instance\n\nSends a `POST` request to `/v1/instances/{instance}/stop`\n\n"]
    pub async fn instance_stop_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/stop",
            self.baseurl,
            encode_path(&instance.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("project", &project))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_stop_v1",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "List organizations\n\nSends a `GET` request to `/v1/organizations`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn organization_list_v1<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::OrganizationResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/organizations", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "organization_list_v1",
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

    #[doc = "List organizations as a Stream\n\nSends repeated `GET` requests to `/v1/organizations` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn organization_list_v1_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Organization, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.organization_list_v1(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.organization_list_v1(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create an organization\n\nSends a `POST` request to `/v1/organizations`\n\n"]
    pub async fn organization_create_v1<'a>(
        &'a self,
        body: &'a types::OrganizationCreate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!("{}/v1/organizations", self.baseurl,);
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
            operation_id: "organization_create_v1",
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

    #[doc = "Fetch an organization\n\nSends a `GET` request to `/v1/organizations/{organization}`\n\n"]
    pub async fn organization_view_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.baseurl,
            encode_path(&organization.to_string()),
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
            operation_id: "organization_view_v1",
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

    #[doc = "Update an organization\n\nSends a `PUT` request to `/v1/organizations/{organization}`\n\n"]
    pub async fn organization_update_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
        body: &'a types::OrganizationUpdate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
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
            operation_id: "organization_update_v1",
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

    #[doc = "Delete an organization\n\nSends a `DELETE` request to `/v1/organizations/{organization}`\n\n"]
    pub async fn organization_delete_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "organization_delete_v1",
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

    #[doc = "Fetch an organization's IAM policy\n\nSends a `GET` request to `/v1/organizations/{organization}/policy`\n\n"]
    pub async fn organization_policy_view_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization.to_string()),
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
            operation_id: "organization_policy_view_v1",
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

    #[doc = "Update an organization's IAM policy\n\nSends a `PUT` request to `/v1/organizations/{organization}/policy`\n\n"]
    pub async fn organization_policy_update_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
        body: &'a types::OrganizationRolePolicy,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
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
            operation_id: "organization_policy_update_v1",
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

    #[doc = "List projects\n\nSends a `GET` request to `/v1/projects`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn project_list_v1<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::ProjectResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/projects", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_list_v1",
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

    #[doc = "List projects as a Stream\n\nSends repeated `GET` requests to `/v1/projects` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `organization`\n- `sort_by`\n"]
    pub fn project_list_v1_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Project, Error<types::Error>>> + Unpin + 'a {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.project_list_v1(limit, organization, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.project_list_v1(limit, None, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a project\n\nSends a `POST` request to `/v1/projects`\n\n"]
    pub async fn project_create_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
        body: &'a types::ProjectCreate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!("{}/v1/projects", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_create_v1",
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

    #[doc = "Fetch a project\n\nSends a `GET` request to `/v1/projects/{project}`\n\n"]
    pub async fn project_view_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}",
            self.baseurl,
            encode_path(&project.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_view_v1",
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

    #[doc = "Update a project\n\nSends a `PUT` request to `/v1/projects/{project}`\n\n"]
    pub async fn project_update_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        body: &'a types::ProjectUpdate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}",
            self.baseurl,
            encode_path(&project.to_string()),
        );
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_update_v1",
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

    #[doc = "Delete a project\n\nSends a `DELETE` request to `/v1/projects/{project}`\n\n"]
    pub async fn project_delete_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}",
            self.baseurl,
            encode_path(&project.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_delete_v1",
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

    #[doc = "Fetch a project's IAM policy\n\nSends a `GET` request to `/v1/projects/{project}/policy`\n\n"]
    pub async fn project_policy_view_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}/policy",
            self.baseurl,
            encode_path(&project.to_string()),
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_policy_view_v1",
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

    #[doc = "Update a project's IAM policy\n\nSends a `PUT` request to `/v1/projects/{project}/policy`\n\n"]
    pub async fn project_policy_update_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        body: &'a types::ProjectRolePolicy,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}/policy",
            self.baseurl,
            encode_path(&project.to_string()),
        );
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
            .query(&progenitor_middleware_client::QueryParam::new(
                "organization",
                &organization,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "project_policy_update_v1",
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

    #[doc = "View version and update status of component tree\n\nSends a `GET` request to `/v1/system/update/components`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn system_component_version_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UpdateableComponentResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/components", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "system_component_version_list",
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

    #[doc = "View version and update status of component tree as a Stream\n\nSends repeated `GET` requests to `/v1/system/update/components` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn system_component_version_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::UpdateableComponent, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_component_version_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_component_version_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List all update deployments\n\nSends a `GET` request to `/v1/system/update/deployments`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn update_deployments_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UpdateDeploymentResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/deployments", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_deployments_list",
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

    #[doc = "List all update deployments as a Stream\n\nSends repeated `GET` requests to `/v1/system/update/deployments` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn update_deployments_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::UpdateDeployment, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.update_deployments_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.update_deployments_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Fetch a system update deployment\n\nSends a `GET` request to `/v1/system/update/deployments/{id}`\n\n"]
    pub async fn update_deployment_view<'a>(
        &'a self,
        id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<types::UpdateDeployment>, Error<types::Error>> {
        let url = format!(
            "{}/v1/system/update/deployments/{}",
            self.baseurl,
            encode_path(&id.to_string()),
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
            operation_id: "update_deployment_view",
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

    #[doc = "Refresh update data\n\nSends a `POST` request to `/v1/system/update/refresh`\n\n"]
    pub async fn system_update_refresh<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/refresh", self.baseurl,);
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
            operation_id: "system_update_refresh",
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

    #[doc = "Start system update\n\nSends a `POST` request to `/v1/system/update/start`\n\n"]
    pub async fn system_update_start<'a>(
        &'a self,
        body: &'a types::SystemUpdateStart,
    ) -> Result<ResponseValue<types::UpdateDeployment>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/start", self.baseurl,);
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
            operation_id: "system_update_start",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Stop system update\n\nIf there is no update in progress, do nothing.\n\nSends a `POST` request to `/v1/system/update/stop`\n\n"]
    pub async fn system_update_stop<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/stop", self.baseurl,);
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
            operation_id: "system_update_stop",
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

    #[doc = "List all updates\n\nSends a `GET` request to `/v1/system/update/updates`\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `page_token`: Token returned by previous call to retrieve the subsequent page\n- `sort_by`\n"]
    pub async fn system_update_list<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::SystemUpdateResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/updates", self.baseurl,);
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
            .query(&progenitor_middleware_client::QueryParam::new("limit", &limit))
            .query(&progenitor_middleware_client::QueryParam::new(
                "page_token",
                &page_token,
            ))
            .query(&progenitor_middleware_client::QueryParam::new("sort_by", &sort_by))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "system_update_list",
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

    #[doc = "List all updates as a Stream\n\nSends repeated `GET` requests to `/v1/system/update/updates` until there are no more results.\n\nArguments:\n- `limit`: Maximum number of items returned by a single call\n- `sort_by`\n"]
    pub fn system_update_list_stream<'a>(
        &'a self,
        limit: Option<::std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::SystemUpdate, Error<types::Error>>> + Unpin + 'a
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_update_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_update_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "View system update\n\nSends a `GET` request to `/v1/system/update/updates/{version}`\n\n"]
    pub async fn system_update_view<'a>(
        &'a self,
        version: &'a types::SemverVersion,
    ) -> Result<ResponseValue<types::SystemUpdate>, Error<types::Error>> {
        let url = format!(
            "{}/v1/system/update/updates/{}",
            self.baseurl,
            encode_path(&version.to_string()),
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
            operation_id: "system_update_view",
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

    #[doc = "View system update component tree\n\nSends a `GET` request to `/v1/system/update/updates/{version}/components`\n\n"]
    pub async fn system_update_components_list<'a>(
        &'a self,
        version: &'a types::SemverVersion,
    ) -> Result<ResponseValue<types::ComponentUpdateResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/v1/system/update/updates/{}/components",
            self.baseurl,
            encode_path(&version.to_string()),
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
            operation_id: "system_update_components_list",
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

    #[doc = "View system version and update status\n\nSends a `GET` request to `/v1/system/update/version`\n\n"]
    pub async fn system_version<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SystemVersion>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/version", self.baseurl,);
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
            operation_id: "system_version",
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
