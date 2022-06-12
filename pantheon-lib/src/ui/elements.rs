use seed::{prelude::*, *};

#[derive(Clone)]
pub struct UiError<M: 'static> {
    pub title: String,
    pub body: String,
    pub message: Box<M>,
}

impl<M: Clone + 'static> IntoNodes<M> for UiError<M> {
    fn into_nodes(self) -> Vec<Node<M>> {
        vec![article! {
            C!("message", "is-danger"),
            div! {
                C!("message-header"),
                p!(self.title),
                button! {
                    C!("delete"),
                    attrs! {
                        At::AriaLabel => "delete"
                    },
                    ev(Ev::Click, |_| *self.message)
                }
            },
            div! {
                C!("message-body"),
                self.body
            }
        }]
    }
}

pub struct MenuBar<M: Clone + 'static>(pub Vec<(String, M)>);

impl<M: Clone + 'static> IntoNodes<M> for MenuBar<M> {
    fn into_nodes(self) -> Vec<Node<M>> {
        vec![nav! {
            C!("navbar"),
            style! {
                St::Padding => rem(1),
            },
            attrs! {
                At::from("role") => "navigation",
                At::AriaLabel => "main navigation"
            },
            div! {
                C!("navbar-brand"),
                a! {
                    attrs! {
                        At::Href => format!(
                            "{}/",
                            window().location().origin().unwrap(),
                        )
                    },
                    img! {
                        C!("navbar-item"),
                        attrs! {
                            At::Src => "/favicon.ico",
                            At::Width => 60,
                        }
                    },
                },
                a! {
                    attrs! {
                        At::Href => format!(
                            "{}/",
                            window().location().origin().unwrap(),
                        )
                    },
                    div! {
                        C!("navbar-item"),
                        h1! {
                            C!("title"),
                            "Pantheon"
                        }
                    }
                },
                a! {
                    C!("navbar-burger"),
                    attrs! {
                        At::from("role") => "button",
                        At::AriaLabel => "menu",
                        At::AriaExpanded => false,
                        At::from("data-target") => "navbar"
                    },
                    span! { attrs! { At::from("aria-hidden") => true }},
                    span! { attrs! { At::from("aria-hidden") => true }},
                    span! { attrs! { At::from("aria-hidden") => true }}
                }
            },
            div! {
                C!("navbar-menu"),
                id!("navbar"),
                div! {
                    C!("navbar-start"),
                    self.0.into_iter().map(|(name, message)| {
                        a! {
                            C!("navbar-item"),
                            ev(Ev::Click, |_| message),
                            name
                        }
                    })
                },
                div! {
                    C!("navbar-end"),
                    a! {
                        C!("navbar-item"),
                        attrs!{At::Href => "https://github.com/JoelCourtney/pantheon"},
                        "Github"
                    }
                }
            }
        }]
    }
}
