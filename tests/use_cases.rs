/// Examples with Phenopacket Schema v2.
use phenopacket_builder::{Build, Buildable};
use phenopackets::schema::v2::core::time_element::Element;
use phenopackets::schema::v2::core::{Individual, KaryotypicSex, OntologyClass, Sex, TimeElement};
use prost_types::Timestamp;

#[test]
fn build_ontology_class() {
    let oc: OntologyClass = OntologyClass::builder()
        .id_label("HP:0001250", "Seizure")
        .build();

    assert_eq!(&oc.id, "HP:0001250");
    assert_eq!(&oc.label, "Seizure");
}

#[test]
fn build_individual() {
    let individual: Individual = Individual::builder()
        .id("individual-id")
        .add_alternate_id("alternate-id")
        .date_of_birth(
            Timestamp::builder()
                .iso8601timestamp("2018-03-01")
                .expect("the timestamp should be well formatted"),
        )
        .time_at_last_encounter(TimeElement::builder().age_iso8601duration("P3Y4M"))
        .male()
        .karyotypic_sex_xy()
        .homo_sapiens()
        .build();

    assert_eq!(&individual.id, "individual-id");
    assert_eq!(&individual.alternate_ids, &["alternate-id"]);

    assert_eq!(
        &individual.date_of_birth.unwrap().to_string(),
        "2018-03-01T00:00:00Z"
    );
    assert_eq!(
        individual.time_at_last_encounter.unwrap().element.unwrap(),
        Element::Timestamp(
            Timestamp::builder()
                .iso8601timestamp("2025-12-03")
                .unwrap()
                .build()
        )
    );

    assert_eq!(&individual.sex, &Sex::Male.into());
    assert_eq!(&individual.karyotypic_sex, &KaryotypicSex::Xy.into());
    assert_eq!(
        &individual.taxonomy,
        &Some(OntologyClass {
            id: "NCBITaxon:9606".into(),
            label: "homo sapiens".into(),
        })
    )
}

#[test]
fn builder_can_be_stored_as_a_variable() {
    let _builder = OntologyClass::builder();
    let _builder = Individual::builder();
}
