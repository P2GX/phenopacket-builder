use crate::{Build, Buildable, Set, Unset};
use phenopackets::schema::v2::core::time_element::Element;
use phenopackets::schema::v2::core::{
    Age, ExternalReference, GestationalAge, OntologyClass, TimeElement,
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

impl Buildable for GestationalAge {
    type Builder = GestationalAgeBuilder;
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
pub struct ExternalReferenceBuilder {
    id: Option<String>,
    reference: Option<String>,
    description: Option<String>,
}

impl ExternalReferenceBuilder {
    pub fn id(mut self, id: impl Into<String>) -> ExternalReferenceBuilder {
        self.id = Some(id.into());
        self
    }

    pub fn reference(mut self, reference: impl Into<String>) -> ExternalReferenceBuilder {
        self.reference = Some(reference.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> ExternalReferenceBuilder {
        self.description = Some(description.into());
        self
    }
}

impl Buildable for ExternalReference {
    type Builder = ExternalReferenceBuilder;
}

impl Build<ExternalReference> for ExternalReferenceBuilder {
    fn build(self) -> ExternalReference {
        ExternalReference {
            id: self.id.unwrap_or_default(),
            reference: self.reference.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
        }
    }
}
