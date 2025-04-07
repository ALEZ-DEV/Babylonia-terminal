use glib_build_tools::compile_resources;

fn main() {
    compile_resources(&["assets"], "assets/resources.xml", "resources.gresource");
}
