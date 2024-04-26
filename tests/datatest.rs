fn func(_: &std::path::Path) -> datatest_stable::Result<()> {
    Ok(())
}

datatest_stable::harness!(func, "testfiles", r".*",);
