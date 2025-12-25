mod navbar;
mod home;
mod projects;
mod about;

pub use navbar::NavBar;
pub use home::HomePage;
pub use projects::{ProjectsPage, ProjectCard};
pub use about::{AboutPage, ExperienceCard};
pub mod resume;
pub use resume::Resume;
