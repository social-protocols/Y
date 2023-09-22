use anyhow::Result;
use maud::{html, Markup, PreEscaped};
use sqlx::SqlitePool;

use crate::db;

use crate::structs::Direction;
use crate::structs::Post;

use crate::pages::vote::vote_buttons;


pub async fn post_details(post_id: i64, user_id: Option<i64>, focused: bool, pool: &SqlitePool) -> Result<Markup> {
    let post = db::get_post(post_id, pool).await?;
    let top_note = db::get_top_note(post_id, pool).await?;
    let top_note_id = top_note.clone().map(|post| post.id);
    let current_vote = match user_id {
        None => Direction::None,
        Some(user_id) => db::get_current_vote(post_id, user_id, pool).await?
    };

    let note_current_vote = match user_id {
        None => Direction::None,
        Some(uid) =>  match top_note_id {
            None => Direction::None,
            Some(nid) => db::get_current_vote(nid, uid, pool).await?
        }
    };

    Ok(html! {
        div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
            a href=(if !focused { format!("/view_post/{}", post.id) } else { format!("") }  ) {
                div class="mb-5" {
                        (post.content)
                }
            }
            (note(top_note, note_current_vote))
            @if focused {
                (post_actions_focused(post_id, top_note_id, current_vote))
            } @else {
                (post_actions(post_id, top_note_id, current_vote))
            }
        }
    })
}

pub fn note(maybe_note: Option<Post>, current_vote: Direction) -> Markup {

    match maybe_note {
        None => html! { div {} },
        Some(note) => {
            html! {
                div {
                    a href=(format!("/view_post/{}", note.id)) {
                        div class="mb-5 p-5 rounded-lg shadow bg-blue-50 dark:bg-slate-800" { 
                            div { (note.content) } 
                            span { (show_current_vote(current_vote)) }
                        }
                    }
                }
            }
        },
    }
}

pub fn show_current_vote(current_vote: Direction) -> Markup {
    match current_vote {
        Direction::Up => { html! {  span class="text-green-500 text-smi" { (format!("(upvoted)")) } } } ,
        Direction::Down => { html! {  span class="text-red-500 text-smi" { (format!("(downvoted)")) } } },
        _ => {html! {}},
    }
}

pub fn post_actions_focused(post_id: i64, maybe_note_id: Option<i64>, current_vote: Direction) -> Markup {

    html! {
        div class="flex w-full" {
            (
                vote_form(post_id, maybe_note_id, current_vote)
            )
            (
                reply_form(post_id)
            )
        }
    }
}


pub fn post_actions(post_id: i64, maybe_note_id: Option<i64>, current_vote: Direction) -> Markup {

    html! {
        a href=(format!("/view_post/{}", post_id)) {
            div class="flex w-full " {
                button class="mr-1" {
                    (PreEscaped(r#"
                        <svg xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 32 32" width="24px" height="24px"><path d="M 16 3 C 12.210938 3 8.765625 4.113281 6.21875 5.976563 C 3.667969 7.835938 2 10.507813 2 13.5 C 2 17.128906 4.472656 20.199219 8 22.050781 L 8 29 L 14.746094 23.9375 C 15.15625 23.96875 15.570313 24 16 24 C 19.789063 24 23.234375 22.886719 25.78125 21.027344 C 28.332031 19.164063 30 16.492188 30 13.5 C 30 10.507813 28.332031 7.835938 25.78125 5.976563 C 23.234375 4.113281 19.789063 3 16 3 Z M 16 5 C 19.390625 5 22.445313 6.015625 24.601563 7.589844 C 26.757813 9.164063 28 11.246094 28 13.5 C 28 15.753906 26.757813 17.835938 24.601563 19.410156 C 22.445313 20.984375 19.390625 22 16 22 C 15.507813 22 15.015625 21.972656 14.523438 21.925781 L 14.140625 21.894531 L 10 25 L 10 20.859375 L 9.421875 20.59375 C 6.070313 19.019531 4 16.386719 4 13.5 C 4 11.246094 5.242188 9.164063 7.398438 7.589844 C 9.554688 6.015625 12.609375 5 16 5 Z"/></svg>
                    "#))
                }
                (show_current_vote(current_vote))
            }
        }
    }
}

fn vote_form(post_id: i64, note_id: Option<i64>, current_vote: Direction) -> Markup {
    html! {
        form id="vote-form" hx-post="/vote" hx-trigger="click" {
            (vote_buttons(post_id, note_id, current_vote))
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
                        "Post"
                    }
                }
            }
        }
    }
}

