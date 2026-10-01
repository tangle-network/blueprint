mod ui;

#[cfg(test)]
mod tests {
    #[test]
    fn test_derive_context() {
        let t = trybuild::TestCases::new();
        t.pass("tests/ui/basic.rs");
        t.pass("tests/ui/unnamed_fields.rs");
        t.pass("tests/ui/generic_struct.rs");
        t.compile_fail("tests/ui/missing_config_attr.rs");
        t.compile_fail("tests/ui/not_a_struct.rs");
        t.compile_fail("tests/ui/unit_struct.rs");
        // `custom_crate_path` pins the override to a nonexistent path: it only
        // fails to compile while the derives still honour the attribute.
        t.compile_fail("tests/ui/custom_crate_path.rs");
        t.compile_fail("tests/ui/unknown_crate_key.rs");
        t.compile_fail("tests/ui/duplicate_crate_path.rs");
        t.compile_fail("tests/ui/duplicate_crate_key.rs");
    }
}
