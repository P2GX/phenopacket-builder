use crate::{Build, Buildable, Set, Unset};
use phenopackets::schema::v2::core::{
    Biosample, Disease, File, Individual, Interpretation, Measurement, MedicalAction, MetaData,
    PhenotypicFeature,
};
use phenopackets::schema::v2::Phenopacket;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PhenopacketBuilder<T = Unset, U = Unset> {
    id: Option<String>,
    subject: Option<Individual>,
    phenotypic_features: Vec<PhenotypicFeature>,
    measurements: Vec<Measurement>,
    biosamples: Vec<Biosample>,
    interpretations: Vec<Interpretation>,
    diseases: Vec<Disease>,
    medical_actions: Vec<MedicalAction>,
    files: Vec<File>,
    meta_data: Option<MetaData>,
    data: PhantomData<(T, U)>,
}

impl<U> PhenopacketBuilder<Unset, U> {
    pub fn id(self, id: impl Into<String>) -> PhenopacketBuilder<Set, U> {
        PhenopacketBuilder {
            id: Some(id.into()),
            subject: self.subject,
            phenotypic_features: self.phenotypic_features,
            measurements: self.measurements,
            biosamples: self.biosamples,
            interpretations: self.interpretations,
            diseases: self.diseases,
            medical_actions: self.medical_actions,
            files: self.files,
            meta_data: self.meta_data,
            data: Default::default(),
        }
    }
}

impl<T> PhenopacketBuilder<T, Unset> {
    pub fn meta_data(self, meta_data: impl Build<MetaData>) -> PhenopacketBuilder<T, Set> {
        PhenopacketBuilder {
            id: self.id,
            subject: self.subject,
            phenotypic_features: self.phenotypic_features,
            measurements: self.measurements,
            biosamples: self.biosamples,
            interpretations: self.interpretations,
            diseases: self.diseases,
            medical_actions: self.medical_actions,
            files: self.files,
            meta_data: Some(meta_data.build()),
            data: Default::default(),
        }
    }
}

impl<T, U> PhenopacketBuilder<T, U> {
    pub fn subject(mut self, subject: impl Build<Individual>) -> Self {
        self.subject = Some(subject.build());
        self
    }

    pub fn add_phenotypic_feature(
        mut self,
        phenotypic_feature: impl Build<PhenotypicFeature>,
    ) -> Self {
        self.phenotypic_features.push(phenotypic_feature.build());
        self
    }

    pub fn extend_phenotypic_features(
        mut self,
        phenotypic_features: impl IntoIterator<Item = impl Build<PhenotypicFeature>>,
    ) -> Self {
        self.phenotypic_features
            .extend(phenotypic_features.into_iter().map(Build::build));
        self
    }

    pub fn clear_phenotypic_features(mut self) -> Self {
        self.phenotypic_features.clear();
        self
    }

    // TODO: add the other fields
}

impl Buildable for Phenopacket {
    type Builder = PhenopacketBuilder;
}

impl Build<Phenopacket> for PhenopacketBuilder<Set, Set> {
    fn build(self) -> Phenopacket {
        Phenopacket {
            id: self.id.expect("id must have been set"),
            subject: self.subject,
            phenotypic_features: self.phenotypic_features,
            measurements: self.measurements,
            biosamples: self.biosamples,
            interpretations: self.interpretations,
            diseases: self.diseases,
            medical_actions: self.medical_actions,
            files: self.files,
            meta_data: self.meta_data,
        }
    }
}
