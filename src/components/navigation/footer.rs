use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {

    html! {
        <footer class="footer">
            <div class="footer__center dynamic-padding">
                <div class="footer__column">
                    <h1>{"Digrapher"}</h1>
                    <p>{"Math visually."}</p> 
                    <p>{ "\u{00A9}" } { " 2026 Digrapher" }</p> 
                    <p>{"App created by Zane Lindquist"}</p> 
                </div>
                <div class="footer__column">
                    <h1>{"Author"}</h1>
                    <p>{"Zane Lindquist"}</p> 
                    <p><a href="https://github.com/zanelindquist" target="_blank">{"GitHub"}</a></p>
                    <p><a href="https://www.linkedin.com/in/zane-lindquist-99212b407/" target="_blank">{"LinkedIn"}</a></p>
                    <p><a href="https://x.com/ZaneLindquist_" target="_blank">{"X"}</a></p>
                </div>
                <div class="footer__column">
                    <h1>{"Code"}</h1>
                    <p><a href="https://github.com/zanelindquist/digrapher" target="_blank">{"View source code"}</a></p>
                    <p><a href="https://github.com/zanelindquist/digrapher/issues/new/choose" target="_blank">{"Submit an issue"}</a></p>
                </div>
            </div>
        </footer>
    }
}