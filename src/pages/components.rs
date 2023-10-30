use crate::{db, pages::vote::vote_buttons};
use anyhow::Result;
use common::structs::{Direction::Neutral, Post};
use maud::{html, Markup};
use sqlx::SqlitePool;

pub async fn post_details(
    tag: &str,
    post: &Post,
    focused: bool,
    pool: &SqlitePool,
) -> Result<Markup> {
    let top_note = db::get_top_note(tag, post.id, pool).await?;
    let top_note_id = top_note.clone().map(|post| post.id);

    Ok(html! {
        div data-postid=(post.id) class="post mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
            div  {
                @if !focused {
                    a href=(format!("/y/{}/post/{}", tag, post.id)) {
                        (post.content)
                    }
                } @else {
                    (post.content)
                }
            }
            div {
                @match top_note.clone() {
                    Some(note) => {
                        a href=(format!("/y/{}/post/{}", tag, note.id)) {
                            div data-postid=(note.id) class="post mt-4 mb-5 p-5 rounded-lg shadow bg-gray-100 dark:bg-slate-600" {
                                p { (note.content) }
                            }
                        }
                    },
                    None => div {},
                }
            }
            div class="w-full flex flex-col mt-4" {
                (vote_form(tag, post.id, top_note_id))
                // (vote_state(post_id, note_id))
                @if focused {
                    (tag_form(post.id, top_note_id))
                    (reply_form(tag, post.id))
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
                            class=r#"
                                block p-2.5 text-gray-900 bg-gray-50 rounded-lg border border-gray-300
                                focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600
                                dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500
                            "#
                            cols="100"
                            rows="1"
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

pub fn vote_form(tag: &str, post_id: i64, note_id: Option<i64>) -> Markup {
    html! {
        div class="flex nowrap vote-form" {
            form id="form" hx-post="/vote" hx-trigger="click queue:last" {
                (vote_buttons(tag, post_id, note_id, Neutral))
            }
        }
    }
}

pub fn reply_form(tag: &str, parent_id: i64) -> Markup {
    html! {
        form hx-post=(format!("/create_post?redirect=/y/{}/post/{}", tag, parent_id)) {
            div class="w-full flex" {
                input
                    type="hidden"
                    name="post_parent_id"
                    value=(format!("{}", parent_id)) {}
                input
                    type="hidden"
                    name="tag"
                    value=(tag)
                div class="mr-1" {
                    textarea
                        name="post_content"
                        class=r#"
                            block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300
                            focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600
                            dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500
                        "#
                        cols="100"
                        rows="1"
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

pub fn tag_form(post_id: i64, note_id: Option<i64>) -> Markup {
    html! {
        div class="flex nowrap tag-form" {
            form hx-post=(format!("/tag")) {
                div
                    class="w-full flex"
                {
                    input
                        type="hidden"
                        name="post_id"
                        value=(post_id) {}

                    @if let Some(nid) = note_id {
                        input type="hidden" value=(nid) name="note_id" {}
                    }

                    div class="mr-1" {
                        textarea
                            name="tags"
                            class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" cols="100" rows="1"
                            placeholder="Enter your tags" {}
                    }
                    div {
                        button
                            class="bg-blue-500 hover:bg-blue-700 text-base text-white font-bold py-2 px-4 rounded float-right"
                        {
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}

pub async fn post_feed(tag: &str, posts: Vec<Post>, pool: &SqlitePool) -> Result<Markup> {
    Ok(html! {
        div {
            @for post in posts.iter() {
                div { (post_details(tag, post, false, pool).await?) }
            }
        }
    })
}
