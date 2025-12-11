use crate::{Build, Buildable, Set, Unset};
use phenopackets::schema::v2::core::time_element::Element;
use phenopackets::schema::v2::core::vital_status::Status;
use phenopackets::schema::v2::core::{
    Age, GestationalAge, Individual, KaryotypicSex, OntologyClass, Sex, TimeElement, VitalStatus,
};
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct OntologyClassBuilder<T = Unset> {
    id: Option<String>,
    label: Option<String>,
    data: PhantomData<T>,
}

impl OntologyClassBuilder<Unset> {
    pub fn id_label(
        self,
        id: impl Into<String>,
        label: impl Into<String>,
    ) -> OntologyClassBuilder<Set> {
        OntologyClassBuilder {
            id: Some(id.into()),
            label: Some(label.into()),
            data: PhantomData,
        }
    }
}

impl Buildable for OntologyClass {
    type Builder = OntologyClassBuilder;
}

impl Build<OntologyClass> for OntologyClassBuilder<Set> {
    fn build(self) -> OntologyClass {
        OntologyClass {
            id: self.id.expect("id must have been set"),
            label: self.label.expect("label must have been set"),
        }
    }
}

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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IndividualBuilder<T = Unset> {
    id: Option<String>,
    alternate_ids: Vec<String>,
    date_of_birth: Option<prost_types::Timestamp>,
    time_at_last_encounter: Option<TimeElement>,
    vital_status: Option<VitalStatus>,
    sex: Sex,
    karyotypic_sex: KaryotypicSex,
    gender: Option<OntologyClass>,
    taxonomy: Option<OntologyClass>,
    data: PhantomData<T>,
}

impl<T> IndividualBuilder<T> {
    pub fn add_alternate_id(mut self, id: impl Into<String>) -> Self {
        self.alternate_ids.push(id.into());
        self
    }

    pub fn extend_alternate_ids(
        mut self,
        ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.alternate_ids.extend(ids.into_iter().map(Into::into));
        self
    }

    pub fn clear_alternate_ids(mut self) -> Self {
        self.alternate_ids.clear();
        self
    }

    pub fn date_of_birth(mut self, date: impl Build<prost_types::Timestamp>) -> Self {
        self.date_of_birth = Some(date.build());
        self
    }

    pub fn time_at_last_encounter(
        mut self,
        time_at_last_encounter: impl Build<TimeElement>,
    ) -> Self {
        self.time_at_last_encounter = Some(time_at_last_encounter.build());
        self
    }

    pub fn vital_status(mut self, vital_status: impl Build<VitalStatus>) -> Self {
        self.vital_status = Some(vital_status.build());
        self
    }

    pub fn deceased(mut self) -> Self {
        self.vital_status = Some(VitalStatus::builder().deceased().build());
        self
    }

    pub fn deceased_at(mut self, time_of_death: impl Into<TimeElement>) -> Self {
        self.vital_status = Some(
            VitalStatus::builder()
                .deceased()
                .time_of_death(time_of_death)
                .build(),
        );
        self
    }

    pub fn alive(mut self) -> Self {
        self.vital_status = Some(VitalStatus::builder().alive().build());
        self
    }

    pub fn sex(mut self, sex: impl Into<Sex>) -> Self {
        self.sex = sex.into();
        self
    }

    pub fn male(self) -> Self {
        self.sex(Sex::Male)
    }

    pub fn female(self) -> Self {
        self.sex(Sex::Female)
    }

    pub fn other_sex(self) -> Self {
        self.sex(Sex::OtherSex)
    }

    pub fn karyotypic_sex(mut self, karyotypic_sex: impl Into<KaryotypicSex>) -> Self {
        self.karyotypic_sex = karyotypic_sex.into();
        self
    }

    pub fn karyotypic_sex_xx(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xx)
    }

    pub fn karyotypic_sex_xy(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xy)
    }

    pub fn karyotypic_sex_xo(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xo)
    }

    pub fn karyotypic_sex_xxy(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xxy)
    }

    pub fn karyotypic_sex_xxyy(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xxyy)
    }

    pub fn karyotypic_sex_xxxy(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xxxy)
    }

    pub fn karyotypic_sex_xxxx(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xxxx)
    }

    pub fn karyotypic_sex_xyy(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::Xyy)
    }

    pub fn karyotypic_sex_other(self) -> Self {
        self.karyotypic_sex(KaryotypicSex::OtherKaryotype)
    }

    pub fn gender(mut self, gender: impl Build<OntologyClass>) -> Self {
        self.gender = Some(gender.build());
        self
    }

    pub fn taxonomy(mut self, taxonomy: impl Build<OntologyClass>) -> Self {
        self.taxonomy = Some(taxonomy.build());
        self
    }

    pub fn homo_sapiens(self) -> Self {
        self.taxonomy(OntologyClass::builder().id_label("NCBITaxon:9606", "homo sapiens"))
    }
}

impl IndividualBuilder<Unset> {
    pub fn id(self, id: impl Into<String>) -> IndividualBuilder<Set> {
        IndividualBuilder {
            id: Some(id.into()),
            alternate_ids: self.alternate_ids,
            date_of_birth: self.date_of_birth,
            time_at_last_encounter: self.time_at_last_encounter,
            vital_status: self.vital_status,
            sex: self.sex,
            karyotypic_sex: self.karyotypic_sex,
            gender: self.gender,
            taxonomy: self.taxonomy,
            data: PhantomData,
        }
    }
}
impl Buildable for Individual {
    type Builder = IndividualBuilder;
}

impl Build<Individual> for IndividualBuilder<Set> {
    fn build(self) -> Individual {
        Individual {
            id: self.id.expect("id must have been set"),
            alternate_ids: self.alternate_ids,
            date_of_birth: self.date_of_birth,
            time_at_last_encounter: self.time_at_last_encounter,
            vital_status: self.vital_status,
            sex: self.sex.into(),
            karyotypic_sex: self.karyotypic_sex.into(),
            gender: self.gender,
            taxonomy: self.taxonomy,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TimeElementBuilder<T = Unset> {
    element: Option<Element>,
    data: PhantomData<T>,
}

impl<T> TimeElementBuilder<T> {
    /// Set the gestational age.
    ///
    /// # Examples
    ///
    /// Create the gestational age, for instance corresponding to 25 weeks and 5 days:
    ///
    /// ```
    /// use phenopacket_builder::{Build, Buildable};
    /// use phenopackets::schema::v2::core::{GestationalAge, TimeElement};
    /// use phenopackets::schema::v2::core::time_element::Element;
    ///
    /// let te: TimeElement = TimeElement::builder()
    ///                         .gestational_age(GestationalAge::builder()
    ///                             .weeks(25)
    ///                             .days(5)
    ///                         ).build();
    ///
    /// assert_eq!(
    ///     te.element,
    ///     Some(Element::GestationalAge(
    ///         GestationalAge {
    ///             weeks: 25,
    ///             days: 5,
    ///         }
    ///     ))
    /// );
    pub fn gestational_age(
        self,
        gestational_age: impl Build<GestationalAge>,
    ) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::GestationalAge(gestational_age.build())),
            data: PhantomData,
        }
    }

    pub fn gestational_age_weeks(self, weeks: impl Into<i32>) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::GestationalAge(GestationalAge {
                weeks: weeks.into(),
                days: Default::default(),
            })),
            data: PhantomData,
        }
    }

    pub fn gestational_age_weeks_days(
        self,
        weeks: impl Into<i32>,
        days: impl Into<i32>,
    ) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::GestationalAge(GestationalAge {
                weeks: weeks.into(),
                days: days.into(),
            })),
            data: PhantomData,
        }
    }

    /// Set the age.
    ///
    /// # Examples
    ///
    /// Use ISO8601 duration, such as `P1Y2D`:
    ///
    /// ```
    /// use phenopacket_builder::{Build, Buildable};
    /// use phenopackets::schema::v2::core::{Age, TimeElement};
    /// use phenopackets::schema::v2::core::time_element::Element;
    ///
    /// let te: TimeElement = TimeElement::builder()
    ///                         .age(Age::builder()
    ///                             .iso8601duration("P1Y2D")
    ///                         ).build();
    ///
    /// assert_eq!(
    ///     te.element,
    ///     Some(Element::Age(
    ///         Age {
    ///             iso8601duration: "P1Y2D".to_string()
    ///         }
    ///     ))
    /// );
    /// ```
    pub fn age(self, age: impl Build<Age>) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::Age(age.build())),
            data: PhantomData,
        }
    }

    /// Set the age with a corresponding ISO8601 duration.
    ///
    /// # Examples
    ///
    /// Create age corresponding to 1 year and 2 days:
    ///
    /// ```
    /// use phenopacket_builder::{Build, Buildable};
    /// use phenopackets::schema::v2::core::{Age, TimeElement};
    /// use phenopackets::schema::v2::core::time_element::Element;
    ///
    /// let te: TimeElement = TimeElement::builder()
    ///                         .age_iso8601duration("P1Y2D")
    ///                         .build();
    ///
    /// assert_eq!(
    ///     te.element,
    ///     Some(Element::Age(
    ///         Age {
    ///             iso8601duration: "P1Y2D".to_string()
    ///         }
    ///     ))
    /// );
    /// ```
    pub fn age_iso8601duration(
        self,
        iso8601duration: impl Into<String>,
    ) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::Age(Age {
                iso8601duration: iso8601duration.into(),
            })),
            data: PhantomData,
        }
    }

    pub fn ontology_class(self, oc: impl Build<OntologyClass>) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::OntologyClass(oc.build())),
            data: PhantomData,
        }
    }

    pub fn timestamp(self, ts: impl Build<prost_types::Timestamp>) -> TimeElementBuilder<Set> {
        TimeElementBuilder {
            element: Some(Element::Timestamp(ts.build())),
            data: PhantomData,
        }
    }

    // TODO: add support for timestamp and interval
}

impl Buildable for TimeElement {
    type Builder = TimeElementBuilder;
}

impl Build<TimeElement> for TimeElementBuilder<Set> {
    fn build(self) -> TimeElement {
        self.element
            .map(|e| TimeElement { element: Some(e) })
            .expect("element must have been set")
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AgeBuilder<T = Unset> {
    iso8601duration: Option<String>,
    data: PhantomData<T>,
}

impl Buildable for Age {
    type Builder = AgeBuilder;
}

impl AgeBuilder<Unset> {
    /// Set the age value as ISO8601 duration.
    ///
    /// Note: the string is *not* checked to constitute a valid ISO8601 duration.
    ///
    /// # Example
    ///
    /// ```
    /// use phenopacket_builder::{Build, Buildable};
    /// use phenopackets::schema::v2::core::Age;
    ///
    /// let age: Age = Age::builder()
    ///                 .iso8601duration("P1Y2M4D")
    ///                 .build();
    ///
    /// assert_eq!(
    ///     age,
    ///     Age {
    ///         iso8601duration: "P1Y2M4D".to_string(),
    ///     }
    /// )
    /// ```
    pub fn iso8601duration(self, iso8601duration: impl Into<String>) -> AgeBuilder<Set> {
        AgeBuilder {
            iso8601duration: Some(iso8601duration.into()),
            data: PhantomData,
        }
    }
}

impl Build<Age> for AgeBuilder<Set> {
    fn build(self) -> Age {
        Age {
            iso8601duration: self
                .iso8601duration
                .expect("iso8601duration must have been set"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GestationalAgeBuilder<T = Unset> {
    weeks: Option<i32>,
    days: Option<i32>,
    data: PhantomData<T>,
}

impl Buildable for GestationalAge {
    type Builder = GestationalAgeBuilder;
}

impl GestationalAgeBuilder<Unset> {
    pub fn weeks(self, weeks: impl Into<i32>) -> GestationalAgeBuilder<Set> {
        GestationalAgeBuilder {
            weeks: Some(weeks.into()),
            days: self.days,
            data: PhantomData,
        }
    }
}

impl<T> GestationalAgeBuilder<T> {
    pub fn days(mut self, days: impl Into<i32>) -> GestationalAgeBuilder<T> {
        self.days = Some(days.into());
        self
    }
}

impl Build<GestationalAge> for GestationalAgeBuilder<Set> {
    fn build(self) -> GestationalAge {
        GestationalAge {
            weeks: self.weeks.expect("weeks must have been set"),
            days: self.days.unwrap_or(0),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VitalStatusBuilder<T = Unset> {
    status: Option<Status>,
    time_of_death: Option<TimeElement>,
    cause_of_death: Option<OntologyClass>,
    survival_time_in_days: Option<u32>,
    data: PhantomData<T>,
}

impl<T> VitalStatusBuilder<T> {
    pub fn status(self, status: impl Into<Status>) -> VitalStatusBuilder<Set> {
        VitalStatusBuilder {
            status: Some(status.into()),
            time_of_death: self.time_of_death,
            cause_of_death: self.cause_of_death,
            survival_time_in_days: self.survival_time_in_days,
            data: PhantomData,
        }
    }

    pub fn alive(self) -> VitalStatusBuilder<Set> {
        self.status(Status::Alive)
    }

    pub fn deceased(self) -> VitalStatusBuilder<Set> {
        self.status(Status::Deceased)
    }

    pub fn time_of_death(mut self, time_of_death: impl Into<TimeElement>) -> VitalStatusBuilder<T> {
        self.time_of_death = Some(time_of_death.into());
        self
    }

    pub fn time_of_death_at_age(mut self, age: impl Into<Age>) -> VitalStatusBuilder<T> {
        self.time_of_death = Some(TimeElement {
            element: Some(Element::Age(age.into())),
        });
        self
    }

    pub fn time_of_death_at_gestational_age(
        mut self,
        gestational_age: impl Into<GestationalAge>,
    ) -> VitalStatusBuilder<T> {
        self.time_of_death = Some(TimeElement {
            element: Some(Element::GestationalAge(gestational_age.into())),
        });
        self
    }

    pub fn cause_of_death(
        mut self,
        cause_of_death: impl Into<OntologyClass>,
    ) -> VitalStatusBuilder<T> {
        self.cause_of_death = Some(cause_of_death.into());
        self
    }

    pub fn survival_time_in_days(
        mut self,
        survival_time_in_days: impl Into<u32>,
    ) -> VitalStatusBuilder<T> {
        self.survival_time_in_days = Some(survival_time_in_days.into());
        self
    }
}

impl Buildable for VitalStatus {
    type Builder = VitalStatusBuilder;
}

impl Build<VitalStatus> for VitalStatusBuilder<Set> {
    fn build(self) -> VitalStatus {
        VitalStatus {
            status: self.status.expect("status must have been set").into(),
            time_of_death: self.time_of_death,
            cause_of_death: self.cause_of_death,
            survival_time_in_days: self.survival_time_in_days.unwrap_or_default(),
        }
    }
}
