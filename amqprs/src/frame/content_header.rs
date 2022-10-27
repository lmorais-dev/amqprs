use std::fmt;

use amqp_serde::types::{FieldTable, LongLongUint, Octect, ShortStr, ShortUint, TimeStamp};
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentHeader {
    pub common: ContentHeaderCommon,
    pub basic_propertities: BasicPropertities,
}

impl ContentHeader {
    pub fn new(common: ContentHeaderCommon, basic_propertities: BasicPropertities) -> Self {
        Self {
            common,
            basic_propertities,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentHeaderCommon {
    class: ShortUint,
    weight: ShortUint,
    body_size: LongLongUint,
}

#[derive(Debug, Serialize)]
pub struct BasicPropertities {
    // // property flags is included in this type
    // // in order to manage the value according to optional property
    property_flags: [Octect; 2],

    content_type: Option<ShortStr>,
    content_encoding: Option<ShortStr>,
    headers: Option<FieldTable>,
    delivery_mode: Option<Octect>,
    priority: Option<Octect>,
    correlation_id: Option<ShortStr>,
    reply_to: Option<ShortStr>,
    expiration: Option<ShortStr>,
    message_id: Option<ShortStr>,
    timestamp: Option<TimeStamp>,
    typ: Option<ShortStr>,
    user_id: Option<ShortStr>,
    app_id: Option<ShortStr>,
    cluster_id: Option<ShortStr>,
}

impl<'de> Deserialize<'de> for BasicPropertities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &[
            "content_type",
            "content_encoding",
            "headers",
            "delivery_mode",
            "priority",
            "correlation_id",
            "reply_to",
            "expiration",
            "message_id",
            "timestamp",
            "typ",
            "user_id",
            "app_id",
            "cluster_id",
        ];

        struct BasicPropertitiesVisitor;

        impl<'de> Visitor<'de> for BasicPropertitiesVisitor {
            type Value = BasicPropertities;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BasicPropertities")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let flags: [Octect; 2] = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let mut basic_propertities = BasicPropertities {
                    property_flags: flags,
                    content_type: None,
                    content_encoding: None,
                    headers: None,
                    delivery_mode: None,
                    priority: None,
                    correlation_id: None,
                    reply_to: None,
                    expiration: None,
                    message_id: None,
                    timestamp: None,
                    typ: None,
                    user_id: None,
                    app_id: None,
                    cluster_id: None,
                };
                if (flags[0] & 1 << 7) != 0 {
                    basic_propertities.content_type = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 6) != 0 {
                    basic_propertities.content_encoding = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 5) != 0 {
                    basic_propertities.headers = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 4) != 0 {
                    basic_propertities.delivery_mode = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 3) != 0 {
                    basic_propertities.priority = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 2) != 0 {
                    basic_propertities.correlation_id = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 1) != 0 {
                    basic_propertities.reply_to = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[0] & 1 << 0) != 0 {
                    basic_propertities.expiration = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                // 2nd byte of flags
                if (flags[1] & 1 << 7) != 0 {
                    basic_propertities.message_id = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[1] & 1 << 6) != 0 {
                    basic_propertities.timestamp = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[1] & 1 << 5) != 0 {
                    basic_propertities.typ = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[1] & 1 << 4) != 0 {
                    basic_propertities.user_id = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[1] & 1 << 3) != 0 {
                    basic_propertities.app_id = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }
                if (flags[1] & 1 << 2) != 0 {
                    basic_propertities.cluster_id = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                }

                Ok(basic_propertities)
            }
        }
        deserializer.deserialize_struct("BasicPropertities", FIELDS, BasicPropertitiesVisitor)
    }
}
