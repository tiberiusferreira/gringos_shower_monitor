#[macro_use]
extern crate seed;
use seed::prelude::*;
mod chartjs;
//mod seed_scatter_plot;
//use seed_scatter_plot::*;
use seed::{document, Request, fetch};
use futures::Future;
use serde::{Serialize, Deserialize};
use crate::chartjs::{Point, PointMod};

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;





//#[wasm_bindgen]
//extern "C" {
////    #[wasm_bindgen(js_namespace = console, js_name=log)]
////    fn log_str(s: &str);
////    #[wasm_bindgen(js_namespace = console, js_name=log)]
////    fn log(s: &JsValue);
////    #[wasm_bindgen(js_namespace = console, js_name=log)]
////    fn log_event(s: &MouseEvent);
////    #[wasm_bindgen(js_namespace = console, js_name=log)]
////    fn log_key(s: &KeyboardEvent);
////    #[wasm_bindgen(js_namespace = console, js_name=log)]
////    fn log_wheel(s: &WheelEvent);
////    #[wasm_bindgen(js_namespace = console, js_name=log)]
////    fn log_target(s: &EventTarget);
//
//    pub type Editor;
//    fn ext_rec() -> Editor;
//    #[wasm_bindgen(method, js_name=calcArea)]
//    fn calcArea(s: &Editor) -> f64;
//
//    fn set_canvas_dim(w: i32, h: i32);
//}

struct Model {
//    plot_state: PlotGraph,
    test: i32,
    data: Option<Vec<ShowerData>>
}



impl Default for Model {
    fn default() -> Self {
        log!("got event!");
//        let mut scatter_plot = PlotGraph::default();
//        scatter_plot.set_points(vec![(200,200), (400,200), (500,150), (600,250)]);
//        scatter_plot.set_y_label("Tempo [minutos]".to_string());
//        scatter_plot.set_x_label("Data".to_string());
        Self {
//            plot_state: scatter_plot,
            test: 0,
            data: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowerData {
    pub start_unix_date: i64,
    pub duration: i32,
}

impl From<ShowerData> for Point{
    fn from(data: ShowerData) -> Self {
        Self{
            x: data.start_unix_date as i32,
            y: data.duration
        }
    }
}

// Update

#[derive(Clone)]
enum Msg {
//    PlotMsg(PlotGraphMsg),
    Clicked,
    FetchData,
    DataFetched(fetch::FetchObject<Vec<ShowerData>>),
    OnFetchError {
        label: &'static str,
        fail_reason: fetch::FailReason<Vec<ShowerData>>,
    }
}

fn fetch_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://192.168.15.22:8000/shower_data";
    Request::new(url).fetch_json(Msg::DataFetched)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Clicked => {
            model.test = model.test+1;
        },
        Msg::FetchData=> {
            orders
                .skip()
                .perform_cmd(fetch_data());
        },
        Msg::DataFetched(data) => {
            model.data = Some(data.result.unwrap().data.unwrap());
            use wasm_bindgen::JsCast;

            let doc:web_sys::HtmlCanvasElement = document().get_element_by_id("myChart").unwrap().dyn_into().unwrap();
            let context:web_sys::CanvasRenderingContext2d = doc.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
            let points : Vec<PointMod> = model.data.clone().unwrap().into_iter().map(|data| Point::from(data).into() ).collect();
            chartjs::Chart::load_js_lib(context, points);
        }
        Msg::OnFetchError { label: _, fail_reason: _ } => {}
    }
}



fn view(model: &Model) -> impl View<Msg> {


//    let scatter_plot = model.plot_state.view().map_message(|orig_msg|{
//        Msg::PlotMsg(orig_msg)
//    });

    div![
    class!["w-screen", "flex", "flex-col"], // root container
        div![class!["flex", "w-full"],                  // header container
            h1![class!["pb-4", "text-center", "flex-grow-1", "text-4xl", "md:text-6xl", "tracking-wider"],
            "Gringos Shower Monitor"],
        ],
        div![class!["w-11/12", "md:w-4/5", "m-auto"], // chart container
                div![class!["w-full", "flex", "justify-center"], // chart
//                     scatter_plot
//    <canvas id="myChart" width="200" height="200"></canvas>
                   canvas![attrs![At::Id => "myChart"]]
                ]
        ]
//        button![ simple_ev("click", Msg::Clicked), "Click me!" ],
//                    h3![ model.test.to_string() ]
    ]
}

#[wasm_bindgen]
pub fn render() {
//    let a = ext_rec();

//    let some = a.calcArea();
//    log!("got event! {}", some);
//    log!("got event!");
    let app = seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();

    app.update(Msg::FetchData);



}