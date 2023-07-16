use leptos::*;
use leptos_meta::*;

mod components;

use crate::{
    api::converse,
    app::components::{chat_area::ChatArea, type_area::TypeArea},
    model::conversation::{Conversation, Message},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            user: true,
            text: new_message.clone(),
        };
        set_conversation.update(move |conversation| {
            conversation.messages.push(user_message);
        });

        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |conversation| {
                conversation.messages.push(model_message);
            });
        }
    });

    // TODO: handle the case when user sends a message while waiting
    // for server response
    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |conversation| {
                conversation.messages.last_mut().unwrap().text = response
            });
        }
    });

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Title text="Gandalf"/>
        <ChatArea conversation />
        <TypeArea send />
    }
}
