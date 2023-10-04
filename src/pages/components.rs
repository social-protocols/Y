use anyhow::Result;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

use common::structs::{Direction, Post};
use common::structs::Direction::Up;
use common::structs::Direction::Down;
use common::structs::Direction::Neutral;

use crate::pages::vote::vote_buttons;

pub async fn post_details(post: &Post, user_id: Option<i64>, focused: bool, pool: &SqlitePool) -> Result<Markup> {
    let top_note = db::get_top_note(post.id, pool).await?;
    let top_note_id = top_note.clone().map(|post| post.id);
    let current_vote = match user_id {
        None => Neutral,
        Some(user_id) => db::get_current_vote(post.id, user_id, pool).await?,
    };


    let top_note_vote = match user_id {
        None => Neutral,
        Some(user_id) => match top_note_id {
            None => Neutral,
            Some(note_id) => db::get_current_vote(note_id, user_id, pool).await?,
        }
    };

    let post_vote_class = vote_class(current_vote);

    let top_note_vote_class = vote_class(top_note_vote);
    // match top_note_vote {
    //     None => "",
    //     Some(note_id) => 
    // };

    // let top_note_class = match top_note_vote {
    //     Neutral => "",
    //     Up => "bg-green-50",
    //     Down => "bg-red-50",
    // };

    Ok(html! {
        div class=(format!("mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700 {}", post_vote_class)) {
            div class="mb-5" {
                (post.content)
            }
            div {
                @match top_note.clone() {
                    Some(note) => {
                        a href=(format!("/view_post/{}", note.id)) {
                            div class=(format!("mb-5 p-5 rounded-lg shadow bg-gray-100 dark:bg-slate-600 {}", top_note_vote_class)) { 
                                p { (note.content) }
                                // (show_vote(top_note_vote))
                            }
                        }
                    },
                    None => div {},
                }
            }
            div class="w-full flex" {
                (vote_form(post.id, top_note_id, current_vote))
                @if focused { 
                    (reply_form(post.id))
                }
           }
        }
    })
}

pub fn show_vote(vote: Direction) -> Markup {

    html! {
        
            @match vote {
                Neutral => "",
                Up => div {"you upvoted"},
                Down => div {"you downvoted"},
            }
    }
}

pub fn vote_form(post_id: i64, note_id: Option<i64>, current_vote: Direction) -> Markup {
    // Todo: initial state from DB if this user has voted

    html! {
        div class="flex nowrap" {
            form id="form" hx-post="/vote" hx-trigger="click" {
                (vote_buttons(post_id, note_id, current_vote))
            }
        }
    }
}

pub fn reply_form(parent_id: i64) -> Markup {
    html! {

        form hx-post=(format!("/create_post?redirect=/view_post/{}", parent_id)) {
            div class="w-full flex" {
                input
                    type="hidden"
                    name="post_parent_id"
                    value=(format!("{}", parent_id)) {}
                div class="mr-1" {
                    textarea
                        name="post_content"
                        class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" cols="100" rows="1"

                        placeholder="Enter your reply" {}
                }
                div class="justify-end" {
                    button class="bg-blue-500 hover:bg-blue-700 text-base text-white font-bold py-2 px-4 rounded float-right" {
                        "Reply"
                    }
                }
            }
        }
    }
}

pub fn vote_class(current_vote: Direction) -> &'static str {

    match current_vote {
        Neutral => "border-r-4 border-transparent",
        Up => "border-r-4 border-green-600",
        Down => "border-r-4 border-red-600",
    }
}


