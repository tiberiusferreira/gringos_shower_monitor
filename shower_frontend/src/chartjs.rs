use wasm_bindgen::prelude::*;
use js_sys::*;
use seed::prelude::*;
use web_sys::window;
use serde::{Serialize, Deserializer};

#[wasm_bindgen]
extern "C" {
    pub type Chart;

    #[wasm_bindgen(constructor)]
    fn new(ctx: web_sys::CanvasRenderingContext2d, config: JsValue) -> Chart;

}

#[derive(Serialize, Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32
}

impl From<Point> for PointMod{
    fn from(ori: Point) -> Self {
        let y_js_value = JsValue::from_f64(ori.y as f64);
        PointMod{
            t: (ori.x as f64)*1000.0,
            y: (((ori.y as f64)/60.0)*100.0).round()/100.0
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PointMod{
    pub t: f64,
    pub y: f64
}

#[derive(Serialize, Debug)]
struct Points(Vec<Point>);

#[derive(Serialize, Debug)]
struct HoverConfig{
    mode: String,
    intersect: bool
}

#[derive(Serialize, Debug)]
struct TicksConfig{
    source: String,
    autoSkip: bool,
}

#[derive(Serialize, Debug)]
struct TimeConfig{
//    displayFormats: TimeDisplayConfig
    unit: String
}

//
//#[derive(Serialize, Debug)]
//struct TimeDisplayConfig{
//    displayFormats: TimeDisplayConfig
//}

#[derive(Serialize, Debug)]
struct Axes{
    r#type: String,
    distribution: String,
    ticks: TicksConfig,
    time: TimeConfig,
}
#[derive(Serialize, Debug)]
struct Scales{
    xAxes: Vec<Axes>
}

#[derive(Serialize, Debug)]
struct PluginsConf {
    zoom: ZoomConf,
    pan: ZoomConf
}

#[derive(Serialize, Debug)]
struct ZoomConf{
    enabled: bool,
    drag: bool,
    mode: String,
//    rangeMin: Range,
//    rangeMax: Range,
    speed: Option<f64>,
}
#[derive(Serialize, Debug, Default)]
struct Range{
    x: Option<f64>,
    y: Option<f64>
}


#[derive(Serialize, Debug)]
struct Options{
    hover: HoverConfig,
    tooltips: HoverConfig,
    scales: Scales,
    zoom: ZoomConf,
    pan: ZoomConf,
}
#[derive(Serialize, Debug)]
#[serde(rename = "data")]
struct DataSets{
    label: String,
    data: Vec<PointMod>,
    backgroundColor: String
}

#[derive(Serialize, Debug)]
#[serde(rename = "data")]
struct Data{
    datasets: Vec<DataSets>,
}
#[derive(Serialize, Debug)]
struct ChartDataConfig {
    pub r#type: String,
    pub data: Data,
    options: Options

}

impl Chart{
    pub fn load_js_lib(context: web_sys::CanvasRenderingContext2d, points: Vec<PointMod>){
//        let my_str = include_str!("chartsjs/Chart.bundle.min.js");
//        let js_fun: js_sys::Function = js_sys::Function::new_no_args(my_str);
//        let window = window().unwrap();
//        js_fun.call0(&window).unwrap();
        let data_ = Data{
            datasets: vec![DataSets{
                label: "Tempo de Banho [min]".to_string(),
                data: points,//, Point{x: 12, y: 15}])
                backgroundColor: "red".to_string()
            }]
        };
        let conf = ChartDataConfig{
            r#type: "scatter".to_string(),
            data: data_,
            options: Options {
                hover: HoverConfig {
                    mode: "index".to_string(),
                    intersect: false
                },
                tooltips: HoverConfig {
                    mode: "index".to_string(),
                    intersect: false
                },
                scales: Scales {
                    xAxes: vec![Axes {
                        r#type: "time".to_string(),
                        distribution: "linear".to_string(),
                        ticks: TicksConfig {
                            source: "ticks".to_string(),
                            autoSkip: true
                        },
                        time: TimeConfig {
                            unit: "day".to_string()
                        }
                    }]
                },
//                plugins: PluginsConf {
                    zoom: ZoomConf {
                        enabled: true,
                        drag: false,
                        mode: "x".to_string(),
                        speed: Some(0.01),//"0.1".to_string(),
                    },
                    pan: ZoomConf {
                        enabled: true,
                        drag: false,
                        mode: "xy".to_string(),
//                        rangeMin: Default::default(),
//                        rangeMax: Default::default(),
                        speed: Some(20.0),//"0.1".to_string()
//                        rangeMin: Default::default(),
//                        rangeMax: Default::default()
                    }
//                }
            }
        };

        log!(JsValue::from_serde(&conf).unwrap());
        Chart::new(context, JsValue::from_serde(&conf).unwrap());
    }
}
