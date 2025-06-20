use std::{iter, rc::Rc};

use crate::{
    config::Config,
    git,
    items::{self, hash, Item},
    Res,
};
use git2::Repository;
use ratatui::{
    layout::Size,
    text::{Line, Text},
};

use super::Screen;

pub(crate) fn create(
    config: Rc<Config>,
    repo: Rc<Repository>,
    size: Size,
    reference: String,
) -> Res<Screen> {
    Screen::new(
        Rc::clone(&config),
        size,
        Box::new(move || {
            let style = &config.style;
            let commit = git::show_summary(repo.as_ref(), &reference)?;
            let show = git::show(repo.as_ref(), &reference)?;
            let details = Text::from(commit.details).lines;

            Ok(iter::once(Item {
                id: hash(["commit_section", &commit.hash]),
                display: Line::styled(format!("commit {}", commit.hash), &style.section_header),
                section: true,
                depth: 0,
                ..Default::default()
            })
            .chain(details.into_iter().map(|line| Item {
                id: hash(["commit", &commit.hash]),
                display: line,
                depth: 1,
                unselectable: true,
                ..Default::default()
            }))
            .chain([items::blank_line()])
            .chain(items::create_diff_items(
                Rc::clone(&config),
                &Rc::new(show),
                0,
                false,
            ))
            .collect())
        }),
    )
}
