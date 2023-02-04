#[derive(std::fmt::Debug)]
pub enum BookmarkCommand {
    BookmarkPage { url: String, title: String },
    UpdateTitle { title: String },
    Delete,
}
