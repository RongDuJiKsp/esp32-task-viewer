struct BuildToolchain;
fn main() {
    BuildToolchain::espidf();
    BuildToolchain::slint();
}
impl BuildToolchain {
    fn espidf() {
        embuild::espidf::sysenv::output();
    }
    fn slint() {
        let config = slint_build::CompilerConfiguration::new();
        slint_build::compile_with_config("ui/app.slint", config).unwrap();
    }
}
