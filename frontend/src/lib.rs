use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    return html! {
        <>
        <div>
            <h1>
                {"Blog post title"}
            </h1>
            <p>{"Date: 12.3.2000"} </p>
            <h3 >
                {"Author: Lorem ipsum"}
            </h3>
            <p> {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
            Cras id mi a quam malesuada gravida. Morbi lorem libero, porta quis ultrices ac, 
            molestie quis est. Donec fringilla, ipsum a consequat maximus, 
            dui purus posuere tortor, et iaculis enim est at velit. Pellentesque 
            habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. 
            Donec sit amet sem eleifend, tristique lorem et, vulputate eros. 
            Proin ac viverra justo, ac mattis leo. Aliquam mollis erat id massa sodales, 
            at venenatis velit varius. Cras sed molestie neque, non sodales massa. "}
            </p>

            <p> {"Mauris eleifend egestas finibus. Vestibulum hendrerit maximus turpis. 
            Donec molestie, dui ullamcorper tempor tincidunt, arcu erat pulvinar massa, 
            sit amet mattis dolor orci ac arcu. Suspendisse tempor lacus nisl, in feugiat arcu mollis sit amet.
             Mauris tincidunt orci eros, a tempor urna elementum vitae. Curabitur augue sem, 
             tempus at quam sit amet, rutrum porttitor risus. Vestibulum eget facilisis nibh, 
             vel dapibus massa. Maecenas blandit, eros sit amet volutpat semper, 
             lorem tellus ornare tellus, a vehicula lacus nulla id odio. Sed dapibus 
             enim sit amet erat iaculis, at sollicitudin risus mattis. "}
            </p>

        </div>
        </>
    };
}
