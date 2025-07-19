use maud::{html, Markup};

pub mod powerlifters;
pub mod root;
pub mod styles;

fn head() -> Markup {
    html! {
        head {
            script src="https://unpkg.com/htmx.org" { }
            title { "Powerlifting API" }
            link rel="stylesheet" href="styles.css" { }
        }
    }
}

fn input_div() -> Markup {
    html! {
        div {
            form hx-post="/powerlifters" hx-target="#result" {
                label for="powerlifters" { "Powerlifters:" }
                br;
                textarea name="powerlifters" id="powerlifters" rows="5" cols="40" { }
                br;
                button type="submit" { "Send" }
            }
        }
    }
}

fn result_div() -> Markup {
    html! {
        div id="result" { }
    }
}
