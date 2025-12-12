use crate::{Build, Buildable, Set, Unset};
use phenopackets::schema::v2::core::{Evidence, OntologyClass, PhenotypicFeature, TimeElement};
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PhenotypicFeatureBuilder<T = Unset> {
    description: Option<String>,
    r#type: Option<OntologyClass>,
    excluded: bool,
    severity: Option<OntologyClass>,
    modifiers: Vec<OntologyClass>,
    onset: Option<TimeElement>,
    resolution: Option<TimeElement>,
    evidence: Vec<Evidence>,
    data: PhantomData<T>,
}

impl PhenotypicFeatureBuilder<Unset> {
    pub fn r#type(self, r#type: impl Build<OntologyClass>) -> PhenotypicFeatureBuilder<Set> {
        PhenotypicFeatureBuilder {
            description: self.description,
            r#type: Some(r#type.build()),
            excluded: self.excluded,
            severity: self.severity,
            modifiers: self.modifiers,
            onset: self.onset,
            resolution: self.resolution,
            evidence: self.evidence,
            data: Default::default(),
        }
    }
}

impl<T> PhenotypicFeatureBuilder<T> {
    pub fn description(mut self, description: impl Build<String>) -> PhenotypicFeatureBuilder<T> {
        self.description = Some(description.build());
        self
    }

    pub fn observed(mut self) -> PhenotypicFeatureBuilder<T> {
        self.excluded = false;
        self
    }

    pub fn excluded(mut self) -> PhenotypicFeatureBuilder<T> {
        self.excluded = true;
        self
    }

    pub fn severity(mut self, severity: impl Build<OntologyClass>) -> PhenotypicFeatureBuilder<T> {
        self.severity = Some(severity.build());
        self
    }

    pub fn add_modifier(
        mut self,
        modifier: impl Build<OntologyClass>,
    ) -> PhenotypicFeatureBuilder<T> {
        self.modifiers.push(modifier.build());
        self
    }

    pub fn extend_modifiers(
        mut self,
        modifiers: impl IntoIterator<Item = impl Build<OntologyClass>>,
    ) -> PhenotypicFeatureBuilder<T> {
        self.modifiers
            .extend(modifiers.into_iter().map(Build::build));
        self
    }

    pub fn clear_modifiers(mut self) -> PhenotypicFeatureBuilder<T> {
        self.modifiers.clear();
        self
    }

    pub fn onset(mut self, onset: impl Build<TimeElement>) -> PhenotypicFeatureBuilder<T> {
        self.onset = Some(onset.build());
        self
    }

    pub fn resolution(
        mut self,
        resolution: impl Build<TimeElement>,
    ) -> PhenotypicFeatureBuilder<T> {
        self.resolution = Some(resolution.build());
        self
    }

    pub fn add_evidence(mut self, evidence: impl Build<Evidence>) -> PhenotypicFeatureBuilder<T> {
        self.evidence.push(evidence.build());
        self
    }

    pub fn extend_evidence(
        mut self,
        evidence: impl IntoIterator<Item = impl Build<Evidence>>,
    ) -> PhenotypicFeatureBuilder<T> {
        self.evidence.extend(evidence.into_iter().map(Build::build));
        self
    }

    pub fn clear_evidence(mut self) -> PhenotypicFeatureBuilder<T> {
        self.evidence.clear();
        self
    }
}

impl Buildable for PhenotypicFeature {
    type Builder = PhenotypicFeatureBuilder;
}

impl Build<PhenotypicFeature> for PhenotypicFeatureBuilder<Set> {
    fn build(self) -> PhenotypicFeature {
        PhenotypicFeature {
            description: self.description.unwrap_or_default(),
            r#type: self.r#type,
            excluded: self.excluded,
            severity: self.severity,
            modifiers: self.modifiers,
            onset: self.onset,
            resolution: self.resolution,
            evidence: self.evidence,
        }
    }
}
