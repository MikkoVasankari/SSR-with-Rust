use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| Model { value: 1 });

    let onclick = {
        if state.value == 1 {
            let state = state.clone();
            Callback::from(move |_| state.set(Number { value: 2 }))
        } else if state.value == 2 {
            let state = state.clone();
            Callback::from(move |_| state.set(Number { value: 1 }))
        } else {
            let state = state.clone();
            Callback::from(move |_| state.set(Number { value: 0 }))
        }
    };
    if state.value == 1{
        return html! {
            <>
            <div>
    
                <h1>
                    {"Blog post title "} { state.value }
                </h1>
    
                <button {onclick}>
                    { "Next post"}
                </button>
    
    
                <p>{ state.value }</p>
    
                <p>{"Date: 12.3.2000"} </p>
                <h2 >
                    {"Author: Lorem ipsum"}
                </h2>
                <p> {"Proin id sapien eget neque mollis dapibus. 
                Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; 
                Nulla placerat dui vitae cursus sodales. 
                Morbi imperdiet dolor felis, in lobortis magna viverra sollicitudin. 
                Nunc consectetur rhoncus dolor, eu ornare leo accumsan eget. 
                Ut et odio risus. Suspendisse at purus odio. "}
                </p>
    
                <p> {"Curabitur diam justo, fermentum sed libero id, bibendum cursus eros. 
                Maecenas ut consectetur eros, ac finibus mauris. Etiam nisi dui, porttitor et vehicula sed, 
                vulputate vel ligula. Suspendisse leo metus, vehicula non efficitur vitae, condimentum congue est. 
                Praesent fermentum faucibus erat. Cras fermentum ante ut dui vehicula, nec bibendum diam bibendum. 
                Mauris eget mi eu urna aliquam mollis. "}
                </p>
    
            </div>
            </>
        };
    }else {
    return html! {
        <>
        <div>

            <h1>
                {"Blog post title "} { state.value }
            </h1>

            <button {onclick}>
                { "Next post"}
            </button>
            <p>{ state.value }</p>

            <p>{"Date: 12.3.2018"} </p>
            <h2 >
                {"Author: Ipsum Lorem"}
            </h2>
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
}
