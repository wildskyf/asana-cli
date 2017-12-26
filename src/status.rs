#[macro_use]
extern crate clap;

pub fn clap() -> clap::App<'static, 'static> {
    clap_app!(status =>
        (about: "show your uncompleted tasks")
        (author: "Wildsky F. <wildsky@moztw.org>")
        (@arg all: -a "show all task assigned to you, completed and uncompleted (with prefix [ ] or [v])"))
}
