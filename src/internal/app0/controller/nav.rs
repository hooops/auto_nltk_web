use util::read;
pub async fn index_nav() -> &'static str {
    read::read_file_and_return_static_str("")
}