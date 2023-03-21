use subxt::PolkadotConfig;
use wasm_bindgen_futures;
use yew::prelude::*;

pub mod polkadot {}

#[function_component(App)]
fn app() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944")
            .await
            .unwrap();

        // todo! with client
    });
    html! {
      <>
          <h1 class="text-3xl font-bold underline bg-slate-100">{"Hello world!"}</h1>
          <div class="-space-x-10">{"wdnmd"}</div>
      </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
