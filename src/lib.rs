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
    title: Cow<'static, str>,
    brand: Cow<'static, str>,
    current_page: Page,
    heading: Cow<'static, str>,
    description: Cow<'static, str>,
    boosted: bool,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    title: Cow<'static, str>,
    brand: Cow<'static, str>,
    current_page: Page,
    heading: Cow<'static, str>,
    description: Cow<'static, str>,
    boosted: bool,
}

#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate {
    title: Cow<'static, str>,
    brand: Cow<'static, str>,
    current_page: Page,
    heading: Cow<'static, str>,
    description: Cow<'static, str>,
    boosted: bool,
}

async fn index(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let boosted = is_boosted(&req);
    let index_template = IndexTemplate {
        title: Cow::Borrowed("Meowcake - A simple web framework for Cloudflare Workers"),
        brand: Cow::Borrowed("Meowcake"),
        current_page: Page::Home,
        heading: Cow::Borrowed("Welcome to Meowcake!"),
        description: Cow::Borrowed("A simple web framework for Cloudflare Workers."),
        boosted,
    };

    render_template(index_template)
}

async fn about(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let boosted = is_boosted(&req);
    let about_template = AboutTemplate {
        title: Cow::Borrowed("About Meowcake"),
        brand: Cow::Borrowed("Meowcake"),
        current_page: Page::About,
        heading: Cow::Borrowed("About"),
        description: Cow::Borrowed(
            "Meowcake is a minimalist web framework for Cloudflare Workers.",
        ),
        boosted,
    };

    render_template(about_template)
}

async fn contact(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let boosted = is_boosted(&req);
    let contact_template = ContactTemplate {
        title: Cow::Borrowed("Contact"),
        brand: Cow::Borrowed("Meowcake"),
        current_page: Page::Contact,
        heading: Cow::Borrowed("Contact"),
        description: Cow::Borrowed("Say hello at hello@meowcake.dev."),
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
