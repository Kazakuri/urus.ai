use askama::Template;

use urusai_lib::models::message::Message;
use urusai_lib::models::short_url::ShortURL;
use urusai_lib::models::user::User;

#[derive(Template)]
#[template(path = "index.html")]
/// Home page that provides ability to shorten a link.
pub struct Index<'a> {
    /// The currently logged in `User`.
    pub user: &'a Option<User>,

    /// An alert message response from the application.
    pub message: Option<&'a Message<'a>>,

    /// The newly created `ShortURL` the user created.
    pub url: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "login.html")]
/// Login page to let users login to their accounts.
pub struct Login<'a> {
    /// The currently logged in `User`.
    pub user: &'a Option<User>,

    /// An alert message response from the application.
    pub message: Option<&'a Message<'a>>,
}

#[derive(Template)]
#[template(path = "register.html")]
/// Register page to let users create accounts on the site.
pub struct Register<'a> {
    /// The currently logged in `User`.
    pub user: &'a Option<User>,

    /// An alert message response from the application.
    pub message: Option<&'a Message<'a>>,
}

#[derive(Template)]
#[template(path = "profile/urls.html")]
/// URL section of the profile of the currently logged in user.
///
/// From here, the user can see the links he has created along with their visit counts.
pub struct ProfileURLs<'a> {
    /// The currently logged in `User`.
    pub user: &'a Option<User>,

    /// An alert message response from the application.
    pub message: Option<&'a Message<'a>>,

    /// A list of ShortURLs associated with the user.
    pub links: &'a Vec<ShortURL>,

    /// The current page number being displayed.
    pub page: &'a i64,

    /// The previous page number to navigate to.
    pub previous_page: &'a Option<i64>,

    /// The next page number to navigate to.
    pub next_page: &'a Option<i64>,

    /// The last page to be shown in the pagination menu.
    pub pages: &'a Vec<Option<i64>>,
}

#[derive(Template)]
#[template(path = "profile/account.html")]
/// Account section of the profile of the currently logged in user.
///
/// From here, the user can update their account information, such as their password.
pub struct ProfileAccount<'a> {
    /// The currently logged in `User`.
    pub user: &'a Option<User>,

    /// An alert message response from the application.
    pub message: Option<&'a Message<'a>>,
}
