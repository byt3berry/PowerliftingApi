use maud::{html, Markup};
use strum::IntoEnumIterator;
use types::{Division, Equipment, Sex};

pub mod powerlifters;
pub mod root;

fn css() -> Markup {
    html! {
        style {
            r#"
                table { border: 1px solid black; }
                th { border: 1px solid black; }
                td { border: 1px solid black; }

                html, body {
                    min-height: 100vh;
                    display: flex;
                    flex-direction: column;
                }

                .container {
                    display: flex;
                    flex-direction: column;
                    flex: 1;
                }

                .content {
                    flex: 1;
                }

                .footer {
                    background-color: #f5f5f5;
                    text-align: center;
                    font-size: 1rem;
                    color: #333;
                    padding: 1.5rem;
                }

                .links {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    gap: 3.5rem;
                }

                .icon-link {
                    display: inline-flex;
                    align-items: center;
                    text-decoration: none;
                    gap: 0.5rem;
                    padding: 0.5em
                }

                .icon {
                    width: 1.5em;
                    height: 1.5em;
                    vertical-align: middle;
                    position: relative;
                    top: 0.125em;
                }
            "#

        }
    }
}

fn head() -> Markup {
    html! {
        head {
            script src="https://unpkg.com/htmx.org" { }
            title { "Powerlifting API" }
            (css())
        }
    }
}

fn footer() -> Markup {
    html! {
        footer class="footer" {
            div class="links" {
                a href="https://github.com/byt3berry/PowerliftingApi" class="icon-link" target="_blank" rel="noopener" {
                    img src="https://www.svgrepo.com/show/512317/github-142.svg" alt="GitHub" class="icon" { }
                    "Projet GitHub"
                }
                a href="https://github.com/byt3berry/PowerliftingApi/releases" class="icon-link" target="_blank" rel="noopener" {
                    img src="https://img.shields.io/github/v/release/byt3berry/PowerliftingApi" alt="Latest release" { }
                }
            }
            div class="links" {
                a href="https://github.com/byt3berry/PowerliftingApi" class="icon-link" target="_blank" rel="noopener" {
                    img src="https://www.svgrepo.com/show/448226/gitlab.svg" alt="GitLab" class="icon" { }
                    "Cette page utilise les data du projet OpenPowerlifting"
                }
            }
        }
    }
}

fn body() -> Markup {
    html! {
        body {
            div class="container" {
                main class="content" {
                    (input_div())
                    (result_div())
                }

                (footer())
            }
        }
    }
}

fn input_div() -> Markup {
    html! {
        div {
            form hx-post="/powerlifters" hx-target="#result" {
                div {
                    select id="equipment_choice" name="equipment_choice" {
                        @for value in Equipment::iter() {
                            option value=(value) { (value) }
                        }
                    }

                    select id="sex_choice" name="sex_choice" {
                        @for value in Sex::iter() {
                            option value=(value) { (value) }
                        }
                    }

                    select id="division_choice" name="division_choice" {
                        @for value in Division::iter() {
                            option value=(value) { (value) }
                        }
                    }
                }

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
