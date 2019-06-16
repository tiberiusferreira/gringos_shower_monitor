#[macro_use]
extern crate seed;
use seed::prelude::*;

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Model

struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
    }
}



fn view(model: &Model) -> El<Msg> {
    let svg = format!(r#"
<svg  viewBox="0 0 750 450" version="1.2" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" class="graph" aria-labelledby="title" role="img">
  <title id="title">A line chart showing some information</title>
<g class="grid x-grid" id="xGrid">
  <line x1="90" x2="90" y1="5" y2="371"></line>
</g>
<g class="grid y-grid" id="yGrid">
  <line x1="90" x2="705" y1="370" y2="370"></line>
</g>
  <g class="labels x-labels">
  <text x="100" y="400">2008</text>
  <text x="246" y="400">2009</text>
  <text x="392" y="400">2010</text>
  <text x="538" y="400">2011</text>
  <text x="684" y="400">2012</text>
  <text x="400" y="440" class="label-title">Horário</text>
</g>
<g class="labels y-labels">
  <text x="80" y="15">60</text>
  <text x="80" y="131">40</text>
  <text x="80" y="248">20</text>
  <text x="80" y="373">0</text>
  <text text-anchor="middle" writing-mode="tb-rl" x="50" y="200" class="label-title">Duração [minutos]</text>
</g>
<g class="data" data-setname="Our first data set">
  <circle cx="90" cy="192" r="4"></circle>
  <circle cx="240" cy="141" r="4"></circle>
  <circle cx="388" cy="179" r="4"></circle>
  <circle cx="531" cy="200" r="4"></circle>
  <circle cx="677" cy="104" r="4"></circle>
</g>
</svg>
"#);

    div![
    class!["h-screen", "w-screen", "flex", "flex-col"], // root container
        div![class!["flex", "w-full"],                  // header container
            h1![class!["pb-8", "text-center", "flex-grow-1", "text-4xl", "md:text-6xl", "tracking-wider"], "Gringos Shower Monitor"],
        ],
        div![class!["w-full"], style!["height" => "40rem"], // chart container
            div![class!["flex", "flex-col", "w-full", "h-full"], // vertical flex container
                div![class!["w-full", "bg-blue-800"], p![class!["text-center"], "Title"]], // title

                div![class!["w-full", "flex"], // chart
                    El::from_html(&svg)
                ],
//    line![
//  ]]
//  <g class="grid y-grid">
//    <line x1="90" x2="705" y1="370" y2="370"></line>
//  </g>
                div![class!["w-full", "bg-purple-800"], p![class!["text-center"], "Subtitles"]], // subtitle
            ]
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();

}