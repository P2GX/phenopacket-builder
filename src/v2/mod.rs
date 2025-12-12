mod core;
mod phenopackets;

use crate::{Build, Buildable, Set, Unset};

use std::marker::PhantomData;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TimestampBuilder<T = Unset> {
    timestamp: Option<prost_types::Timestamp>,
    data: PhantomData<T>,
}

impl TimestampBuilder<Unset> {
    /// Set seconds and nanoseconds to the timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use prost_types::Timestamp;
    /// use phenopacket_builder::{Buildable, Build};
    ///
    /// let ts: Timestamp = Timestamp::builder()
    ///                       .seconds_nanos(125, 11)
    ///                       .build();
    ///
    /// assert_eq!(ts.seconds, 125);
    /// assert_eq!(ts.nanos, 11);
    ///
    /// assert_eq!(&ts.to_string(), "1970-01-01T00:02:05.000000011Z");
    /// ```
    pub fn seconds_nanos(
        self,
        seconds: impl Into<i64>,
        nanos: impl Into<i32>,
    ) -> TimestampBuilder<Set> {
        TimestampBuilder {
            timestamp: Some(prost_types::Timestamp {
                seconds: seconds.into(),
                nanos: nanos.into(),
            }),
            data: PhantomData,
        }
    }

    /// Set ISO8601 timestamp, such as `1970-01-01` or `1970-01-01T00:10:05Z`.
    ///
    /// # Example
    ///
    /// Create a timestamp for a date of birth on Nov 3rd, 2021:
    ///
    /// ```
    /// use prost_types::Timestamp;
    /// use phenopacket_builder::{Buildable, Build};
    ///
    /// let ts: Timestamp = Timestamp::builder()
    ///                       .iso8601timestamp("2021-11-03")
    ///                       .expect("well formatted timestamp")
    ///                       .build();
    ///
    /// assert_eq!(&ts.to_string(), "2021-11-03T00:00:00Z");
    /// ```
    ///
    /// Create a timestamp with resolution in seconds:
    ///
    /// ```
    /// use prost_types::Timestamp;
    /// use phenopacket_builder::{Buildable, Build};
    ///
    /// let ts: Timestamp = Timestamp::builder()
    ///                       .iso8601timestamp("1970-01-01T00:10:05Z")
    ///                       .expect("well formatted timestamp")
    ///                       .build();
    ///
    /// assert_eq!(ts.seconds, 605);
    /// assert_eq!(ts.nanos, 0);
    /// ```
    pub fn iso8601timestamp(
        self,
        timestamp: impl AsRef<str>,
    ) -> Result<TimestampBuilder<Set>, prost_types::TimestampError> {
        Ok(TimestampBuilder {
            timestamp: Some(timestamp.as_ref().parse()?),
            data: PhantomData,
        })
    }
}

impl Buildable for prost_types::Timestamp {
    type Builder = TimestampBuilder;
}

impl Build<prost_types::Timestamp> for TimestampBuilder<Set> {
    fn build(self) -> prost_types::Timestamp {
        self.timestamp.expect("timestamp must have been set")
    }
}
