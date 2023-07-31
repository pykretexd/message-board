use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <nav>
                <Header />
            </nav>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/signup" view=Signup />
                <Route path="/login" view=Login />
            </Routes>
        </Router>
    }
}

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Header"</p>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello world!"</p>
    }
}

#[component]
fn Signup(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello world!"</p>
    }
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello world!"</p>
    }
}
