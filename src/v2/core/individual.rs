use crate::{Build, Buildable, Set, Unset};
use phenopackets::schema::v2::core::time_element::Element;
use phenopackets::schema::v2::core::vital_status::Status;
use phenopackets::schema::v2::core::{
    Age, GestationalAge, Individual, KaryotypicSex, OntologyClass, Sex, TimeElement, VitalStatus,
};
use std::marker::PhantomData;

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
