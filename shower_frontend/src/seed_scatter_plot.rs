//use seed::prelude::*;
//
//#[derive(Clone)]
//pub enum PlotGraphMsg{
//    HoverPoint(Option<i32>),
//    HoverPosition((f32,f32))
//}
//
//pub struct PlotPoint{
//    original_x: i32,
//    original_y: i32,
//    x: i32,
//    y: i32,
//    hover_text: String,
//}
//
//pub struct PlotGraph {
//    pub hovered_point: Option<i32>,
//    pub points:  Vec<(i32, i32)>,
//    pub x_label: String,
//    pub y_label: String,
//    x_start: i32,
//    x_end: i32,
//    y_start: i32,
//    y_end: i32
//}
//
//impl Default for PlotGraph{
//    fn default() -> PlotGraph{
//        PlotGraph{
//            hovered_point: None,
//            points: vec![],
//            x_label: "".to_string(),
//            y_label: "".to_string(),
//            x_start: 0,
//            x_end: 1000,
//            y_start: 0,
//            y_end: 600
//        }
//    }
//}
//
//
//impl PlotGraph{
//
//    pub fn update(&mut self, msg: PlotGraphMsg) {
//        match msg {
//            PlotGraphMsg::HoverPoint(point) => {
//                log!("got event!");
//                self.hovered_point = point;
//            },
//            PlotGraphMsg::HoverPosition((pointer_x, pointer_y))  => {
//                for (index, (x, _)) in self.points.iter().enumerate(){
//                    if (pointer_x-(*x as f32)).abs() < 15.0{
//                        self.hovered_point = Some(index as i32);
//                        return;
//                    }
//                }
//            }
//        }
//    }
//
//    pub fn set_points(&mut self, points: Vec<(i32, i32)>){
////        let (x, y) = points.get(0).unwrap();
////        let min_x = 10 as f64;
////        let max_x = 100 as f64;
////        let range_x = self.x_end-self.x_start;
////        let percent_of_total = (x-min_x)/(range_x);
////        let range_y = self.y_end-self.y_start;
////        // remember y down is positive
////        let ajusted_x = 9;
//        self.points = points;
//    }
//
//    pub fn set_x_label(&mut self, label: String){
//        self.x_label = label;
//    }
//
//    pub fn set_y_label(&mut self, label: String){
//        self.y_label = label;
//    }
//
//    fn create_all_points(&self) -> Vec<El<PlotGraphMsg>>{
//        let mut points_vec = Vec::new();
//        for (index, (x, y)) in self.points.iter().enumerate(){
//            points_vec.push(self.create_single_point(*x, *y, index as i32));
//        }
//        points_vec
//    }
//
//    fn create_single_point(&self, x: i32, y: i32, id: i32) -> El<PlotGraphMsg>{
//        let data_style = style!["fill" => "red"; "stroke-width" => "1"; "font-size" => "x-large", "touch-action" => "none"];
//        let hover_text: El<PlotGraphMsg> = if self.hovered_point.map(|curr_id| curr_id==id).unwrap_or(false){
//            text![attrs!["x" => (x+10).to_string(); "y" => (y-10).to_string()], format!("{}, {}", x, y)]
//        }else{
//            seed::empty()
//        };
//
//        g![
//                data_style,
//                circle![attrs!["cx" => x.to_string(); "cy" => y.to_string(); "r" => "5"]],
//                hover_text
//            ]
//    }
//
//    pub fn view(&self) -> El<PlotGraphMsg> {
//        let view_box_x_size = 1000;
//        let view_box_y_size = 700;
//        let graph_x_start = 100;
//        let graph_x_end = 1000;
//        let graph_y_start = 0;
//        let graph_y_end = 600;
//        let axes_line_width = 6;
//        let axes_lines = style!["stroke" => "#000000"; "stroke-width" => axes_line_width];
//
//        let axes_style_vertical = style!["writing-mode" => "tb"; "text-anchor" => "middle"; " font-size" => "xx-large"];
//        let axes_style_horizontal = style!["text-anchor" => "middle"; "font-size" => "xx-large"];
//
//        let touch_in_event = pointer_ev("pointermove", move |event| {
//            use wasm_bindgen::JsCast;
//            let width =  event.client_x();
//            let height =  event.client_y();
//            let plot_svg: web_sys::SvgsvgElement = seed::document()
//                .get_element_by_id("scatter_plot_svg")
//                .unwrap().dyn_into().unwrap();
//            let point = plot_svg.create_svg_point();
//            point.set_x(width as f32);
//            point.set_y(height as f32);
//            let point = point
//                .matrix_transform(&plot_svg.get_screen_ctm()
//                    .unwrap().inverse().unwrap());
//
//            PlotGraphMsg::HoverPosition((point.x(), point.y()))
//        });
//
//        let svg_element:  El<PlotGraphMsg> =
//            svg![ style!["height" => "85vh"; "touch-action" => "none"],
//                class!["w-full"],
//                attrs!["viewBox" => format!("0 0 {} {}", view_box_x_size, view_box_y_size);
//                "version"=>"1.2"; "xmlns"=>"http://www.w3.org/2000/svg";
//                "xmlns:xlink"=>"http://www.w3.org/1999/xlink";
//                "id"=>"scatter_plot_svg";
//                "aria-labelledby"=>"title";
//                "role"=>"img"], touch_in_event,
//            g![ // Y axis
//                // Main line
//                line_![axes_lines.clone(),
//                    attrs!["x1" => graph_x_start;
//                    "x2" => graph_x_start;
//                    "y1" => graph_y_start;
//                    "y2" => graph_y_end]
//                    ],
//                // Y Scales
//                line_![axes_lines.clone(),
//                    attrs!["x1" => graph_x_start-15;
//                    "x2" => graph_x_start;
//                    "y1" => 3*(graph_y_start+graph_y_end)/4;
//                    "y2" => 3*(graph_y_start+graph_y_end)/4]
//            ],
//                line_![axes_lines.clone(), attrs!["x1" => graph_x_start-15; "x2" => graph_x_start; "y1" => (graph_y_start+graph_y_end)/2; "y2" => (graph_y_start+graph_y_end)/2], ],
//                line_![axes_lines.clone(), attrs!["x1" => graph_x_start-15; "x2" => graph_x_start; "y1" => 1*(graph_y_start+graph_y_end)/4; "y2" => 1*(graph_y_start+graph_y_end)/4], ],
//                text![axes_style_vertical, attrs!["x" => graph_x_start/3; "y" => (graph_y_start+graph_y_end)/2], self.y_label]
//            ],
//            g![
//                // X axis
//                // x from 100 to 900
//                line_![axes_lines.clone(), attrs!["x1" => graph_x_start; "x2" => graph_x_end; "y1" => graph_y_end - axes_line_width/2; "y2" => graph_y_end - axes_line_width/2]],
//                line_![axes_lines.clone(), attrs!["x1" => (graph_x_start+graph_x_end)/4; "x2" => (graph_x_start+graph_x_end)/4; "y1" => (graph_y_end - axes_line_width/2); "y2" => (graph_y_end - axes_line_width/2 + 15)]],
//                line_![axes_lines.clone(), attrs!["x1" => 2*(graph_x_start+graph_x_end)/4; "x2" => 2*(graph_x_start+graph_x_end)/4; "y1" => (graph_y_end - axes_line_width/2); "y2" => (graph_y_end - axes_line_width/2 + 15)]],
//                line_![axes_lines.clone(), attrs!["x1" => 3*(graph_x_start+graph_x_end)/4; "x2" => 3*(graph_x_start+graph_x_end)/4; "y1" => (graph_y_end - axes_line_width/2); "y2" => (graph_y_end - axes_line_width/2 + 15)]]
//            ],
//            g![
//                text![axes_style_horizontal, attrs!["x" => (graph_x_start+graph_x_end)/2; "y" => (view_box_y_size+graph_y_end)/2], self.x_label]
//            ],
//            self.create_all_points()
//    ];
//
//        svg_element
//    }
//}
