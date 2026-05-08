use askama::Template;
use std::borrow::Cow;
use worker::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    About,
    Contact,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    brand: Cow<'static, str>,
    current_page: Page,
    boosted: bool,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    brand: Cow<'static, str>,
    current_page: Page,
    boosted: bool,
}

#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate {
    brand: Cow<'static, str>,
    current_page: Page,
    boosted: bool,
}

async fn index(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let boosted = is_boosted(&req);
    let index_template = IndexTemplate {
        brand: Cow::Borrowed("Meowcake"),
        current_page: Page::Home,
        boosted,
    };

    render_template(index_template)
}

async fn about(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let boosted = is_boosted(&req);
    let about_template = AboutTemplate {
        brand: Cow::Borrowed("Meowcake"),
        current_page: Page::About,
        boosted,
    };

    render_template(about_template)
}

async fn contact(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let boosted = is_boosted(&req);
    let contact_template = ContactTemplate {
        brand: Cow::Borrowed("Meowcake"),
        current_page: Page::Contact,
        boosted,
    };

    render_template(contact_template)
}

fn render_template<T: Template>(template: T) -> Result<Response> {
    let rendered = template
        .render()
        .map_err(|err| Error::RustError(err.to_string()))?;

    Response::from_html(rendered)
}

fn is_boosted(req: &Request) -> bool {
    req.headers()
        .get("HX-Boosted")
        .ok()
        .flatten()
        .is_some_and(|value| value == "true")
}

fn router() -> Router<'static, ()> {
    Router::new()
        .get_async("/", index)
        .get_async("/about", about)
        .get_async("/contact", contact)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    router().run(req, env).await
}
