use gloo_net::http::{Request, Response};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use weblog::{self, console_log};
use yew::prelude::*;
// use wasm_bindgen::prelude::*;

pub enum Msg {
    Update(AurResponse),
}

#[derive(Clone)]
pub struct AurViewer {
    aur_response: Arc<Mutex<Option<AurResponse>>>,
    aur2: Option<AurResponse>,
    ur_good: bool,
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
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "NumVotes")]
    num_votes: i32,
    #[serde(rename = "Popularity")]
    popularity: f64,
    // #[serde(rename = "OutOfDate")]
    // out_of_date: String,
    #[serde(rename = "Maintainer")]
    maintainer: String,
    #[serde(rename = "CoMaintainers")]
    co_maintainers: Vec<String>,
    #[serde(rename = "Depends")]
    depends: Vec<String>,
    #[serde(rename = "MakeDepends")]
    make_depends: Option<Vec<String>>,
    #[serde(rename = "License")]
    license: Vec<String>,
    #[serde(rename = "Keywords")]
    keywords: Vec<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct AurResponse {
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
        Self {
            aur_response: Arc::new(Mutex::new(None)),
            aur2: None,
            ur_good: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if !_first_render {
            return;
        }
        let name = ctx.props().package_name.clone();
        // let threadsafe_self = Arc::new(Mutex::new(self.clone()));
        let pain = ctx.link().callback(|resp: AurResponse| Msg::Update(resp));
        // let aur_response = Arc::clone(&self.aur_response);
        wasm_bindgen_futures::spawn_local(async move {
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
                Ok(val) => {
                    console_log!(val.as_raw());
                    val
                }
                Err(error) => {
                    panic!("{}", error);
                }
            };

            let details: AurResponse = match resp.json().await {
                Ok(val) => {
                    // console_log!(val.results[0].package_name);
                    val
                }
                Err(error) => {
                    panic!("Error occured: {}", error.to_string());
                }
            };
            // aur_response.lock().unwrap().insert(details);
            // let threadsafe_self = &mut threadsafe_self.lock().unwrap();
            // ctx.link().callback(|_: ()| Msg::Update).emit(());
            // threadsafe_self.aur2 = Some(details.clone());
            // threadsafe_self.ur_good = true;

            pain.emit(details.clone());
			// console_log!("Fuckkdkkkkc");
            ()
        });
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(resp) => {
				self.aur2 = Some(resp);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
		console_log!(self.aur2.is_some());
		console_log!(self.ur_good);
        if !self.aur2.is_some() {
			console_log!("Pain");
            return html! {
                <div class="aur-viewer">
        			{self.ur_good}
                    {"hi"}
                </div>
            }
        }

		console_log!("Pain2");
        let aur_response = self.aur2.clone().unwrap().clone();
        html! {
            <div class="aur-viewer">
                {"hi2"}
                {aur_response.results[0].package_base.clone()}
				{self.ur_good}
            </div>
        }
    }
}
