pub struct A;

#[test]
#[should_panic]
fn type_accessed_through_self_referencing_crate_name_are_not_equal_to_itself_in_terms_of_type_id() {
    assert_eq!(
        std::any::TypeId::of::<A>(),
        std::any::TypeId::of::<crate_name_bug_rs::A>()
    )
}
