fn main() {
    #[cfg(windows)]
    embed_resource::compile("./res/procnuke.rc");
}
