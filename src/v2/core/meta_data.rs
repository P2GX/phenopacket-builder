use crate::{Build, Buildable, Set, Unset};
use phenopackets::schema::v2::core::{ExternalReference, MetaData, Resource, Update};
use prost_types::Timestamp;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MetaDataBuilder<T = Unset, U = Unset, V = Unset> {
    created: Option<Timestamp>,
    created_by: Option<String>,
    submitted_by: Option<String>,
    resources: Vec<Resource>,
    updates: Vec<Update>,
    phenopacket_schema_version: Option<String>,
    external_references: Vec<ExternalReference>,
    data: PhantomData<(T, U, V)>,
}

impl<U, V> MetaDataBuilder<Unset, U, V> {
    pub fn created(self, created: impl Build<Timestamp>) -> MetaDataBuilder<Set, U, V> {
        MetaDataBuilder {
            created: Some(created.build()),
            created_by: self.created_by,
            submitted_by: self.submitted_by,
            resources: self.resources,
            updates: self.updates,
            phenopacket_schema_version: self.phenopacket_schema_version,
            external_references: self.external_references,
            data: Default::default(),
        }
    }
}

impl<T, V> MetaDataBuilder<T, Unset, V> {
    pub fn created_by(self, created_by: impl Into<String>) -> MetaDataBuilder<T, Set, V> {
        MetaDataBuilder {
            created: self.created,
            created_by: Some(created_by.into()),
            submitted_by: self.submitted_by,
            resources: self.resources,
            updates: self.updates,
            phenopacket_schema_version: self.phenopacket_schema_version,
            external_references: self.external_references,
            data: Default::default(),
        }
    }
}

impl<T, U> MetaDataBuilder<T, U, Unset> {
    pub fn phenopacket_schema_version(
        self,
        version: impl Into<String>,
    ) -> MetaDataBuilder<T, U, Set> {
        MetaDataBuilder {
            created: self.created,
            created_by: self.created_by,
            submitted_by: self.submitted_by,
            resources: self.resources,
            updates: self.updates,
            phenopacket_schema_version: Some(version.into()),
            external_references: self.external_references,
            data: Default::default(),
        }
    }

    pub fn v2(self) -> MetaDataBuilder<T, U, Set> {
        self.phenopacket_schema_version("2.0.0")
    }

    pub fn v2_0_2(self) -> MetaDataBuilder<T, U, Set> {
        self.phenopacket_schema_version("2.0.2")
    }
}

impl<T, U, V> MetaDataBuilder<T, U, V> {
    pub fn submitted_by(mut self, submitted_by: impl Into<String>) -> Self {
        self.submitted_by = Some(submitted_by.into());
        self
    }

    pub fn add_resource(mut self, resource: impl Build<Resource>) -> Self {
        self.resources.push(resource.build());
        self
    }

    pub fn extend_resources(
        mut self,
        resources: impl IntoIterator<Item = impl Build<Resource>>,
    ) -> Self {
        self.resources
            .extend(resources.into_iter().map(Build::build));
        self
    }

    pub fn clear_resources(mut self) -> Self {
        self.resources.clear();
        self
    }

    pub fn add_update(mut self, update: impl Build<Update>) -> Self {
        self.updates.push(update.build());
        self
    }

    pub fn extend_updates(mut self, updates: impl IntoIterator<Item = impl Build<Update>>) -> Self {
        self.updates.extend(updates.into_iter().map(Build::build));
        self
    }

    pub fn clear_updates(mut self) -> Self {
        self.updates.clear();
        self
    }

    pub fn add_external_reference(
        mut self,
        external_reference: impl Build<ExternalReference>,
    ) -> Self {
        self.external_references.push(external_reference.build());
        self
    }

    pub fn extend_external_references(
        mut self,
        external_references: impl IntoIterator<Item = impl Build<ExternalReference>>,
    ) -> Self {
        self.external_references
            .extend(external_references.into_iter().map(Build::build));
        self
    }

    pub fn clear_external_references(mut self) -> Self {
        self.external_references.clear();
        self
    }
}

impl Buildable for MetaData {
    type Builder = MetaDataBuilder;
}

impl Build<MetaData> for MetaDataBuilder<Set, Set, Set> {
    fn build(self) -> MetaData {
        MetaData {
            created: self.created,
            created_by: self.created_by.expect("created_by must have been set"),
            submitted_by: self.submitted_by.unwrap_or_default(),
            resources: self.resources,
            updates: self.updates,
            phenopacket_schema_version: self
                .phenopacket_schema_version
                .expect("phenopacket schema must have been set"),
            external_references: self.external_references,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ResourceBuilder<T = Unset, U = Unset, V = Unset, X = Unset, Y = Unset, Z = Unset> {
    id: Option<String>,
    name: Option<String>,
    namespace_prefix: Option<String>,
    url: Option<String>,
    version: Option<String>,
    iri_prefix: Option<String>,
    data: PhantomData<(T, U, V, X, Y, Z)>,
}

impl<U, V, X, Y, Z> ResourceBuilder<Unset, U, V, X, Y, Z> {
    pub fn id(self, id: impl Into<String>) -> ResourceBuilder<Set, U, V, X, Y, Z> {
        ResourceBuilder {
            id: Some(id.into()),
            name: self.name,
            namespace_prefix: self.namespace_prefix,
            url: self.url,
            version: self.version,
            iri_prefix: self.iri_prefix,
            data: Default::default(),
        }
    }
}

impl<T, V, X, Y, Z> ResourceBuilder<T, Unset, V, X, Y, Z> {
    pub fn name(self, name: impl Into<String>) -> ResourceBuilder<T, Set, V, X, Y, Z> {
        ResourceBuilder {
            id: self.id,
            name: Some(name.into()),
            namespace_prefix: self.namespace_prefix,
            url: self.url,
            version: self.version,
            iri_prefix: self.iri_prefix,
            data: Default::default(),
        }
    }
}

impl<T, U, X, Y, Z> ResourceBuilder<T, U, Unset, X, Y, Z> {
    pub fn namespace_prefix(
        self,
        namespace_prefix: impl Into<String>,
    ) -> ResourceBuilder<T, U, Set, X, Y, Z> {
        ResourceBuilder {
            id: self.id,
            name: self.name,
            namespace_prefix: Some(namespace_prefix.into()),
            url: self.url,
            version: self.version,
            iri_prefix: self.iri_prefix,
            data: Default::default(),
        }
    }
}

impl<T, U, V, Y, Z> ResourceBuilder<T, U, V, Unset, Y, Z> {
    pub fn url(self, url: impl Into<String>) -> ResourceBuilder<T, U, V, Set, Y, Z> {
        ResourceBuilder {
            id: self.id,
            name: self.name,
            namespace_prefix: self.namespace_prefix,
            url: Some(url.into()),
            version: self.version,
            iri_prefix: self.iri_prefix,
            data: Default::default(),
        }
    }
}

impl<T, U, V, X, Z> ResourceBuilder<T, U, V, X, Unset, Z> {
    pub fn version(self, version: impl Into<String>) -> ResourceBuilder<T, U, V, X, Set, Z> {
        ResourceBuilder {
            id: self.id,
            name: self.name,
            namespace_prefix: self.namespace_prefix,
            url: self.url,
            version: Some(version.into()),
            iri_prefix: self.iri_prefix,
            data: Default::default(),
        }
    }
}

impl<T, U, V, X, Y> ResourceBuilder<T, U, V, X, Y, Unset> {
    pub fn iri_prefix(self, iri_prefix: impl Into<String>) -> ResourceBuilder<T, U, V, X, Y, Set> {
        ResourceBuilder {
            id: self.id,
            name: self.name,
            namespace_prefix: self.namespace_prefix,
            url: self.url,
            version: self.version,
            iri_prefix: Some(iri_prefix.into()),
            data: Default::default(),
        }
    }
}

impl<T, U, V, X, Y, Z> ResourceBuilder<T, U, V, X, Y, Z> {
    pub fn hpo(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("hp".into()),
            name: Some("human phenotype ontology".into()),
            namespace_prefix: Some("HP".into()),
            url: Some("https://purl.obolibrary.org/obo/hp.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/HP_".into()),
            data: Default::default(),
        }
    }

    pub fn geno(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("geno".into()),
            name: Some("genotype ontology".into()),
            namespace_prefix: Some("GENO".into()),
            url: Some("https://purl.obolibrary.org/obo/geno.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/GENO_".into()),
            data: Default::default(),
        }
    }

    pub fn ncit(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("ncit".into()),
            name: Some("NCI Thesaurus".into()),
            namespace_prefix: Some("NCIT".into()),
            url: Some("https://purl.obolibrary.org/obo/geno.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/NCIT_".into()),
            data: Default::default(),
        }
    }

    pub fn mondo(
        self,
        version: impl Into<String>,
    ) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("mondo".into()),
            name: Some("Mondo Disease Ontology".into()),
            namespace_prefix: Some("MONDO".into()),
            url: Some("https://purl.obolibrary.org/obo/mondo.obo".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/MONDO_".into()),
            data: Default::default(),
        }
    }

    pub fn uberon(
        self,
        version: impl Into<String>,
    ) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("uberon".into()),
            name: Some("Uber-anatomy ontology".into()),
            namespace_prefix: Some("UBERON".into()),
            url: Some("https://purl.obolibrary.org/obo/uberon.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/UBERON_".into()),
            data: Default::default(),
        }
    }

    pub fn ncbi_taxon(
        self,
        version: impl Into<String>,
    ) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("ncbitaxon".into()),
            name: Some("NCBI organismal classification".into()),
            namespace_prefix: Some("NCBITaxon".into()),
            url: Some("https://purl.obolibrary.org/obo/ncbitaxon.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/NCBITaxon_".into()),
            data: Default::default(),
        }
    }

    pub fn so(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("so".into()),
            name: Some("Sequence types and features ontology".into()),
            namespace_prefix: Some("SO".into()),
            url: Some("https://purl.obolibrary.org/obo/so.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/SO_".into()),
            data: Default::default(),
        }
    }

    pub fn ucum(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("ucum".into()),
            name: Some("Unified Code for Units of Measure".into()),
            namespace_prefix: Some("UCUM".into()),
            url: Some("https://ucum.org".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://units-of-measurement.org/".into()),
            data: Default::default(),
        }
    }

    pub fn uo(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("uo".into()),
            name: Some("Units of measurement ontology".into()),
            namespace_prefix: Some("UO".into()),
            url: Some("https://purl.obolibrary.org/obo/uo.owl".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://purl.obolibrary.org/obo/UO_".into()),
            data: Default::default(),
        }
    }

    pub fn loinc(
        self,
        version: impl Into<String>,
    ) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("loinc".into()),
            name: Some("Logical Observation Identifiers Names and Codes".into()),
            namespace_prefix: Some("LOINC".into()),
            url: Some("https://loinc.org".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://loinc.org/".into()),
            data: Default::default(),
        }
    }

    pub fn omim(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("omim".into()),
            name: Some("An Online Catalog of Human Genes and Genetic Disorders".into()),
            namespace_prefix: Some("OMIM".into()),
            url: Some("https://www.omim.org".into()),
            version: Some(version.into()),
            iri_prefix: Some("https://www.omim.org/entry/".into()),
            data: Default::default(),
        }
    }

    pub fn hgnc(self, version: impl Into<String>) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("hgnc".into()),
            name: Some("HUGO Gene Nomenclature Committee".into()),
            namespace_prefix: Some("HGNC".into()),
            url: Some("https://www.genenames.org".into()),
            version: Some(version.into()),
            iri_prefix: Some(
                "https://www.genenames.org/data/gene-symbol-report/#!/hgnc_id/".into(),
            ),
            data: Default::default(),
        }
    }

    pub fn pmid(self) -> ResourceBuilder<Set, Set, Set, Set, Set, Set> {
        ResourceBuilder {
            id: Some("pmid".into()),
            name: Some("PubMed".into()),
            namespace_prefix: Some("PMID".into()),
            url: Some("https://pubmed.ncbi.nlm.nih.gov".into()),
            version: Some("".into()),
            iri_prefix: Some("https://pubmed.ncbi.nlm.nih.gov/".into()),
            data: Default::default(),
        }
    }
}

impl Buildable for Resource {
    type Builder = ResourceBuilder;
}

impl Build<Resource> for ResourceBuilder<Set, Set, Set, Set, Set, Set> {
    fn build(self) -> Resource {
        Resource {
            id: self.id.expect("id must have been set"),
            name: self.name.expect("name must have been set"),
            url: self.url.expect("url must have been set"),
            version: self.version.expect("version must have been set"),
            namespace_prefix: self
                .namespace_prefix
                .expect("namespace prefix must have been set"),
            iri_prefix: self.iri_prefix.expect("iri prefix must have been set"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UpdateBuilder<T = Unset> {
    timestamp: Option<Timestamp>,
    updated_by: Option<String>,
    comment: Option<String>,

    data: PhantomData<T>,
}

impl UpdateBuilder<Unset> {
    pub fn timestamp(self, timestamp: impl Build<Timestamp>) -> UpdateBuilder<Set> {
        UpdateBuilder {
            timestamp: Some(timestamp.build()),
            updated_by: self.updated_by,
            comment: self.comment,
            data: Default::default(),
        }
    }
}

impl<T> UpdateBuilder<T> {
    pub fn updated_by(self, updated_by: impl Into<String>) -> UpdateBuilder<T> {
        UpdateBuilder {
            timestamp: self.timestamp,
            updated_by: Some(updated_by.into()),
            comment: self.comment,
            data: Default::default(),
        }
    }

    pub fn comment(self, comment: impl Into<String>) -> UpdateBuilder<T> {
        UpdateBuilder {
            timestamp: self.timestamp,
            updated_by: self.updated_by,
            comment: Some(comment.into()),
            data: Default::default(),
        }
    }
}

impl Buildable for Update {
    type Builder = UpdateBuilder;
}

impl Build<Update> for UpdateBuilder<Set> {
    fn build(self) -> Update {
        Update {
            timestamp: self.timestamp,
            updated_by: self.updated_by.unwrap_or_default(),
            comment: self.comment.unwrap_or_default(),
        }
    }
}
