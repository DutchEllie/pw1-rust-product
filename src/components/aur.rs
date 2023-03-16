use chrono::prelude::*;
use chrono::NaiveDateTime;
use gloo_net::http::{Request, Response};
use serde::Deserialize;
use yew::prelude::*;

pub enum Msg {
  Update(AurResponse),
}

#[derive(Clone, Default)]
pub struct AurViewer {
  aur_response: Option<AurResponse>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AurPackageProps {
  pub package_name: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct AurPackageDetails {
  #[serde(rename = "ID")]
  id: i32,
  #[serde(rename = "Name")]
  package_name: String,
  #[serde(rename = "PackageBaseID")]
  package_base_id: i32,
  #[serde(rename = "PackageBase")]
  package_base: String,
  #[serde(rename = "Version")]
  version: String,
  #[serde(rename = "Description", default)]
  description: String,
  #[serde(rename = "URL", default)]
  url: String,
  #[serde(rename = "NumVotes")]
  num_votes: i32,
  #[serde(rename = "Popularity")]
  popularity: f64,
  // #[serde(rename = "OutOfDate")]
  // out_of_date: String,
  #[serde(rename = "Maintainer", default)]
  maintainer: String,
  #[serde(rename = "CoMaintainers", default)]
  co_maintainers: Vec<String>,
  #[serde(rename = "Depends", default)]
  depends: Vec<String>,
  #[serde(rename = "MakeDepends", default)]
  make_depends: Option<Vec<String>>,
  #[serde(rename = "License", default)]
  license: Vec<String>,
  #[serde(rename = "Keywords", default)]
  keywords: Vec<String>,
  #[serde(rename = "LastModified", default)]
  last_modified: i32,
  #[serde(rename = "FirstSubmitted", default)]
  first_submitted: i32,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct AurResponse {
  #[serde(rename = "resultcount")]
  result_count: i32,
  #[serde(rename = "results")]
  results: Vec<AurPackageDetails>,
  #[serde(rename = "type")]
  _type: String,
  #[serde(rename = "version")]
  version: i32,
}

impl Component for AurViewer {
  type Message = Msg;
  type Properties = AurPackageProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { aur_response: None }
  }

  fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
    // If not on first render, go away
    // This fetch miiiiight be better to run inside of the update function instead.
    if !_first_render {
      return;
    }

    let name = ctx.props().package_name.clone();

    // I am unsure how to wrap "self" correctly to send it to the thread.
    // let threadsafe_self = Arc::new(Mutex::new(std::mem::take(self)));
    let update_callback = ctx.link().callback(|resp: AurResponse| Msg::Update(resp));
    wasm_bindgen_futures::spawn_local({
      async move {
        let resp: Response = match Request::get(
          format!(
            "https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={}",
            name
          )
          .as_str(),
        )
        .send()
        .await
        {
          Ok(val) => val,
          Err(error) => {
            panic!("{}", error);
          }
        };

        let details: AurResponse = match resp.json().await {
          Ok(val) => val,
          Err(error) => {
            panic!("Error occured: {}", error.to_string());
          }
        };

        update_callback.emit(details.clone());
        ()
      }
    });
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Update(resp) => {
        self.aur_response = Some(resp);
        true
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    // Before the data has been acquired.
    if !self.aur_response.is_some() {
      return html! {
        <section class="aur-viewer">
          <h2>{"AUR packages I (help) maintain"}</h2>
          <article class="aur-package">

          </article>
        </section>
      };
    }

    let aur_response: AurResponse = self.aur_response.clone().unwrap().clone();
    html! {
      <section class="aur-viewer">
        <h2>{"AUR packages I (help) maintain"}</h2>
        <article class="aur-package">
          <h3>{aur_response.results[0].package_name.clone()} {" "} {aur_response.results[0].version.clone()}</h3>
          <table>
            <tbody>
              <tr>
                <th>{"Name:"}</th>
                <td><a href={format!("https://aur.archlinux.org/pkgbase/{}", aur_response.results[0].package_base.clone())}>{aur_response.results[0].package_name.clone()}</a></td>
              </tr>
              <tr>
                <th>{"Description:"}</th>
                <td>{aur_response.results[0].description.clone()}</td>
              </tr>
              <tr>
                <th>{"Version:"}</th>
                <td>{aur_response.results[0].version.clone()}</td>
              </tr>
              <tr>
                <th>{"Upstream:"}</th>
                <td><a href={format!("{}", aur_response.results[0].url.clone())}>{aur_response.results[0].url.clone()}</a></td>
              </tr>
              <tr>
                <th>{"Submitter:"}</th>
                <td>{aur_response.results[0].maintainer.clone()}</td>
              </tr>
              if !aur_response.results[0].co_maintainers.clone().is_empty() {
                <tr>
                  <th>{"Co-maintainers:"}</th>
                  <td>
                    {aur_response.results[0].co_maintainers.clone().iter().map(|item| {
                      html! {
                        {format!("{} ", item)}
                      }
                    }).collect::<Html>()
                    }
                  </td>
                </tr>
              }
              <tr>
                <th>{"Votes:"}</th>
                <td>{aur_response.results[0].num_votes.clone()}</td>
              </tr>
              <tr>
                <th>{"Popularity:"}</th>
                <td>{aur_response.results[0].popularity.clone()}</td>
              </tr>
              <tr>
                <th>{"Last package:"}</th>
                <td>
                  {format!("{}", {
                    let date = NaiveDateTime::from_timestamp_opt(aur_response.results[0].last_modified.clone().into(), 0).unwrap();
                    let converted = date.and_local_timezone(Utc).unwrap();
                    let newtime = converted.with_timezone(&Local);
                    format!("{}", newtime.format("%Y-%m-%d %R (UTC %Z)"))
                  })}
                </td>
              </tr>
              <tr>
                <th>{"First package:"}</th>
                <td>
                  {format!("{}", {
                    let date = NaiveDateTime::from_timestamp_opt(aur_response.results[0].first_submitted.clone().into(), 0).unwrap();
                    let converted = date.and_local_timezone(Utc).unwrap();
                    let newtime = converted.with_timezone(&Local);
                    format!("{}", newtime.format("%Y-%m-%d %R (UTC %Z)"))
                  })}
                </td>
              </tr>
            </tbody>
          </table>
        </article>
      </section>
    }
  }
}
