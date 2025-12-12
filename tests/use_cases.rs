/// Examples with Phenopacket Schema v2.
mod v2 {
    use phenopacket_builder::{oc, Build, Buildable};
    use phenopackets::schema::v2::core::time_element::Element;
    use phenopackets::schema::v2::core::vital_status::Status;
    use phenopackets::schema::v2::core::{
        Age, ExternalReference, Individual, KaryotypicSex, MetaData, OntologyClass,
        PhenotypicFeature, Resource, Sex, TimeElement, VitalStatus,
    };
    use phenopackets::schema::v2::Phenopacket;
    use prost_types::Timestamp;

    #[test]
    fn build_a_phenopacket() {
        let _phenopacket: Phenopacket = Phenopacket::builder()
            .id("phenopacket-id")
            .subject(
                Individual::builder()
                    .id("individual-id")
                    .add_alternate_id("alternate-id")
                    .date_of_birth(
                        Timestamp::builder()
                            .iso8601timestamp("2018-03-01")
                            .expect("the timestamp should be well formatted"),
                    )
                    .time_at_last_encounter(TimeElement::builder().age_iso8601duration("P3Y4M"))
                    .alive()
                    .male()
                    .karyotypic_sex_xy()
                    .homo_sapiens(),
            )
            .add_phenotypic_feature(
                PhenotypicFeature::builder()
                    .r#type(
                        oc("HP:0012469", "Infantile spasms")
                    ).add_modifier(
                    oc("HP:0031796", "Recurrent")
                )
                    .onset(
                        TimeElement::builder()
                            .age_iso8601duration("P6M")
                    )
                    .resolution(
                        TimeElement::builder()
                            .age_iso8601duration("P4Y2M")
                    )
            )
            .meta_data(
                MetaData::builder()
                    .created(
                        Timestamp::builder()
                            .iso8601timestamp("2019-07-21T00:25:54.662Z")
                            .unwrap(),
                    )
                    .created_by("Peter R.")
                    .add_resource(Resource::builder().hpo("2018-03-08"))
                    .add_resource(Resource::builder().geno("2018-03-19"))
                    .add_resource(Resource::builder().pmid())
                    .v2()
                    .add_external_reference(
                        ExternalReference::builder()
                            .id("PMID:30808312")
                            .description("Bao M, et al. COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report. BMC Neurol. 2019;19(1):32.")
                    ),
            )
            .build();
    }

    #[test]
    fn build_an_individual() {
        let individual: Individual = Individual::builder()
            .id("individual-id")
            .add_alternate_id("alternate-id")
            .date_of_birth(
                Timestamp::builder()
                    .iso8601timestamp("2018-03-01")
                    .expect("the timestamp should be well formatted"),
            )
            .time_at_last_encounter(TimeElement::builder().age_iso8601duration("P3Y4M"))
            .alive()
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
            Element::Age(Age::builder().iso8601duration("P3Y4M").build())
        );

        assert_eq!(
            individual.vital_status,
            Some(VitalStatus {
                status: Status::Alive.into(),
                time_of_death: None,
                cause_of_death: None,
                survival_time_in_days: 0,
            })
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
    fn build_an_ontology_class() {
        let oc: OntologyClass = OntologyClass::builder()
            .id_label("HP:0001250", "Seizure")
            .build();

        assert_eq!(&oc.id, "HP:0001250");
        assert_eq!(&oc.label, "Seizure");
    }

    #[test]
    fn builder_can_be_stored_as_a_variable() {
        let _builder = OntologyClass::builder();
        let _builder = Individual::builder();
    }
}
