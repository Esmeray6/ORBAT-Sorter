use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    rolelist: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let role_input_ref = use_node_ref();

    let rolelist = use_state(|| String::new());

    let role_msg = use_state(|| String::new());
    {
        let role_msg = role_msg.clone();
        let rolelist = rolelist.clone();
        let name2 = rolelist.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if rolelist.is_empty() {
                    return;
                }

                let args = to_value(&GreetArgs {
                    rolelist: &*rolelist,
                })
                .unwrap();
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                let new_msg = invoke("greet", args).await.as_string().unwrap();
                role_msg.set(new_msg);
            });

            || {}
        });
    }

    let greet = {
        let rolelist = rolelist.clone();
        let role_input_ref = role_input_ref.clone();
        dbg!(&rolelist, &role_input_ref);
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            rolelist.set(dbg!(role_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()));
        })
    };

    html! {
        <main class="container">
            // <div class="row">
            //     <a href="https://tauri.app" target="_blank">
            //         <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            //     </a>
            //     <a href="https://yew.rs" target="_blank">
            //         <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
            //     </a>
            // </div>

            // <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            // <p>
            //     {"Recommended IDE setup: "}
            //     <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
            //     {" + "}
            //     <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
            //     {" + "}
            //     <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            // </p>

            <form class="row" onsubmit={greet}>
                <textarea type="text" id="greet-input" ref={role_input_ref} placeholder="Enter the list of roles..." />
                <button class="row" id="submit-button" type="submit">{"Greet"}</button>
            </form>

            <textarea class="row" id="role-msg" value={ role_msg.to_string() }></textarea>
        </main>
    }
}
