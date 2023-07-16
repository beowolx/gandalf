use leptos::{html::Input, *};

#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view! { cx,
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 bg-gray-900">
           <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
           }
           >
                <input class="w-2/3 p-4 placeholder-gray-500 focus: outline-none focus:ring-0 border border-slate-800 bg-slate-400 rounded-full" type="text" placeholder="Scribe your question here for the wizard's counsel..." node_ref=input_ref/>
                <input class="h-full p-4 bg-violet-500 text-white rounded-full cursor-pointer" type="submit" value="Invoke Wizard's Wisdom" />
           </form>
        </div>
    }
}
