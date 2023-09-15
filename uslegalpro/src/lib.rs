//! # USLEGALPRO
//!
//! `uslegalpro` is a simple library to help with interacting with **uslegalpro**'s JSON API.
//!
//! ## On Endpoints
//!
//! Currently there are a few endpoints available. I hope to add the rest as needed. Feel free to create a pull request to add more.
//!
//! - Authentication **Authenticate a user**
//! - Case Previews **Query cases by case_number & jurisdictions**
//! - Detail **Get details of a case**
//!
//! ## On URLs
//!
//! You can specify what url to use as the endpoint, you can also use the prebuilt on for Texas right now, I plan to add more states later on.
//!
//! It's presumed the endpoints above have the following routes
//!
//! > POST /authenticate
//! > GET /search_case
//! > GET /case/{CASE_TRACKING_ID}
//!
//! ## Requests & Typing
//!
//! The only way to make an authenticated request is IFF (if and only if) you have successfully authenticated. It is impossible to construct a client (authed or not) outside of the methods provided by the library. You can still modify and view properties used on the clients however.
//!
//! ## Example Usage
//!
//! ### Authentication
//!
//! ```ignore
//! use uslegalpro::{client::NoAuthClient, state::State, auth::{authenticate::{authenticate,  User, Auth}}};
//! use reqwest::Client;
//!
//! async fn auth() {
//!     let client = Client::new();
//!
//!     let texas = State::Texas;
//!     let client = NoAuthClient::new(client, "MY_CLIENT_TOKEN", texas.endpoint());
//!
//!     let user = User {
//!         username: "MY_USERNAME",
//!         password: "MY_PASSWORD",
//!     };
//!
//!     let auth = Auth {
//!         client: client.clone(),
//!         user,
//!     };
//!
//!     let authtoken = authenticate(auth).await.unwrap();
//!
//!     println!("AUTHED = {:?}", authtoken);
//!
//!     let client = client.into_authed_client(&authtoken);
//!
//!     // client can now make authed requests
//!     }
//! ```
//!
//! ### Case Preview
//!
//! ```ignore
//! use uslegalpro::{search::query::{query, Query}};
//!
//! async fn pre() {
//!     // client from last part of authentication example
//!     let client = ();
//!
//!     let case_query = Query {
//!     client,
//!     case_number: "MY_CASE_#",
//!     jurisdiction: "county:court",
//!     };
//!
//!     let previews = query(case_query).await.unwrap();
//!
//!     println!("my vector of previews = {:?}", previews);
//! }
//! ```

pub mod auth;
pub mod case;
pub mod client;
pub mod item;
pub mod search;
pub mod state;
