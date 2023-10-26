use anyhow::Result;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

use common::structs::Direction::Neutral;
use common::structs::Post;

use crate::pages::vote::vote_buttons;

pub async fn post_details(post: &Post, focused: bool, pool: &SqlitePool) -> Result<Markup> {
    let top_note = db::get_top_note(post.id, pool).await?;
    let top_note_id = top_note.clone().map(|post| post.id);

    Ok(html! {
        div data-postid=(post.id) class="post mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
            div  {
                @if !focused {
                    a href=(format!("/view_post/{}", post.id)) {
                        (post.content)
                    }
                } @else {
                    (post.content)
                }
            }
            div {
                @match top_note.clone() {
                    Some(note) => {
                        a href=(format!("/view_post/{}", note.id)) {
                            div data-postid=(note.id) class="post mt-4 mb-5 p-5 rounded-lg shadow bg-gray-100 dark:bg-slate-600" {
                                p { (note.content) }
                            }
                        }
                    },
                    None => div {},
                }
            }
            div class="w-full flex flex-col mt-4" {
                (vote_form(post.id, top_note_id))
                // (vote_state(post_id, note_id))
                @if focused {
                    (tag_form(post.id))
                    (reply_form(post.id))
                }
           }
        }
    })
}

pub fn create_post_form() -> Markup {
    html! {
        div class="bg-white rounded-lg shadow-lg w-120 h-30 p-5 mb-10 flex dark:bg-slate-700" {
            form hx-post="/create_post" {
                div class="w-full flex" {
                    div class="mr-1" {
                        textarea
                            name="post_content"
                            // class="p-10 resize-none w-full text-black"
                            class="block p-2.5 text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" cols="100" rows="1"
                            style="width: 100%"
                            placeholder="New Post" {}
                    }
                    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded float-right tx-sm" {
                        "Post"
                    }
                }
            }
        }
    }
}

pub fn vote_form(post_id: i64, note_id: Option<i64>) -> Markup {
    html! {
        div class="flex nowrap vote-form" {
            form id="form" hx-post="/vote" hx-trigger="click queue:last" {
                (vote_buttons(post_id, note_id, Neutral))
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

pub fn tag_form(post_id: i64) -> Markup {
    html! {
        div class="flex nowrap tag-form" {
            form hx-post=(format!("/add_tag?redirect=/view_post/{}", post_id)) {
                div
                    class="w-full flex"
                {
                    input
                        type="hidden"
                        name="post_id"
                        value=(format!("{}", post_id)) {}
                    div class="mr-1" {
                        textarea
                            name="tag"
                            class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" cols="100" rows="1"
                            placeholder="Enter your tag" {}
                    }
                    div {
                        button
                            class="bg-blue-500 hover:bg-blue-700 text-base text-white font-bold py-2 px-4 rounded float-right"
                        {
                            "#"
                        }
                    }
                }
            }
        }
    }
}
