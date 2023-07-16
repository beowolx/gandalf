use leptos::{html::Div, *};

use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-violet-500 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-gray-200 text-black";

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {cx,
      <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto bg-gray-900 p-5" node_ref=chat_div_ref>
      {move || conversation.get().messages.iter().map(|message| {
        let class_str = if message.user {USER_MESSAGE_CLASS} else {MODEL_MESSAGE_CLASS};
          view! {cx,
              <div class={class_str}>
                { message.text.clone() }
            </div>
          }
        }).collect::<Vec<_>>()
      }
      </div>
    }
}
