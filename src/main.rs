use log::info;
use rand::prelude::*;
use std::iter::zip;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum SphereMessage {
    SetUnits(String),
    SetDiameter(Option<f64>),
    SetStitchesPerUnit(Option<f64>),
    SetRowsPerUnit(Option<f64>),
}

pub struct SphereComponent {
    units: String,
    diameter: Option<f64>,
    stitches_per_unit: Option<f64>,
    rows_per_unit: Option<f64>,
}

impl Component for SphereComponent {
    type Message = SphereMessage;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            units: String::from("in"),
            diameter: None,
            stitches_per_unit: None,
            rows_per_unit: None,
        }
    }
    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let units = input_el.value();
            SphereMessage::SetUnits(units)
        });

        let on_diam_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let diameter = input_el.value().parse().ok();
            SphereMessage::SetDiameter(diameter)
        });
        let on_st_per_u_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let st_per_u = input_el.value().parse().ok();
            SphereMessage::SetStitchesPerUnit(st_per_u)
        });
        let on_row_per_u_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let row_per_u = input_el.value().parse().ok();
            SphereMessage::SetRowsPerUnit(row_per_u)
        });

        let mut instructions = Vec::<Html>::new();
        let pattern = if let (Some(diameter), Some(stitches_per_unit), Some(rows_per_unit)) =
            (&self.diameter, &self.stitches_per_unit, &self.rows_per_unit)
        {
            generate_instructions_for_sphere(
                diameter,
                rows_per_unit,
                stitches_per_unit,
                &mut instructions,
            );
            html! {
                <div>
                    <h1>{"Pattern"}</h1>
                    <ul>{instructions}</ul>
                </div>
            }
        } else {
            html!{<div/>}
        };

        html! {
        <div>
            <div>
                <span>
                    <h3>{"Sphere Size"}</h3>
                    <span>
                        <label>{"Units: "}</label>
                        <input type="text" placeholder="Units (in, cm)" oninput={on_input} value={self.units.clone()}/>
                    </span>
                    <span>
                        <label>{"Diameter: "}</label>
                        <input type="number" placeholder="Diameter of sphere" oninput={on_diam_input}/>
                    </span>
                </span>
                <span>
                    <h3>{"Gauge"}</h3>
                    <span>
                        <label>{format!("Stitches/{}: ", &self.units)}</label>
                        <input type="number" placeholder="Stitch count" oninput={on_st_per_u_input}/>
                    </span>
                    <span>
                        <label>{format!("Rows/{}: ", &self.units)}</label>
                        <input type="number" placeholder="Row count" oninput={on_row_per_u_input}/>
                    </span>
                </span>
            </div>

            { pattern }
        </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SphereMessage::SetUnits(val) => {
                self.units = val;
                true
            }
            SphereMessage::SetDiameter(val) => {
                self.diameter = val;
                true
            }
            SphereMessage::SetStitchesPerUnit(val) => {
                self.stitches_per_unit = val;
                true
            }
            SphereMessage::SetRowsPerUnit(val) => {
                self.rows_per_unit = val;
                true
            }
        }
    }
}

fn generate_instructions_for_sphere(
    diameter: &f64,
    rows_per_unit: &f64,
    stitches_per_unit: &f64,
    instructions: &mut Vec<Html>,
) {
    let r = diameter / 2.0;
    let pi = std::f64::consts::PI;
    let mut rng = StdRng::seed_from_u64(123);

    let circle_dist = 2.0 * pi * r / 4.0;
    let rough_rows_in_hemisphere = circle_dist * rows_per_unit;
    let row_pairs_in_hemisphere = (rough_rows_in_hemisphere / 2.0).ceil() as i32;

    let rows = 1..=row_pairs_in_hemisphere;
    let per_row_pair_angle = (pi / 2.0) / f64::from(row_pairs_in_hemisphere);

    let angles: Vec<f64> = rows.map(|x| f64::from(x) * per_row_pair_angle).collect();
    let radius_of_row: Vec<f64> = angles.iter().map(|a| r * f64::sin(*a)).collect();
    let row_length: Vec<f64> = radius_of_row.iter().map(|r| 2.0 * pi * r).collect();
    let stitch_count: Vec<f64> = row_length.iter().map(|rl| stitches_per_unit * rl).collect();
    let stitch_count_int: Vec<i32> = stitch_count.iter().map(|c| c.round() as i32).collect();

    // Copy the sequence and delete one element to shift:
    let d1 = stitch_count_int.clone();
    let mut d2 = stitch_count_int.clone();
    d2.remove(0);
    // diff will be x_i - x_{i-1}. Start it with None since first element has no diff:
    let mut diff: Vec<Option<i32>> = zip(d1, d2).map(|(x, y)| Some(y - x)).collect();
    diff.insert(0, None);

    for (i, (count, inc_by)) in zip(stitch_count_int, diff).enumerate() {
        match inc_by {
            None => {
                instructions
                    .push(html! {<div>{format!("Row 1: Cast on {} stitches", count)}</div>});
                instructions.push(html! {<div>{format!("Row 2: k{}", count)}</div>});
            }
            Some(inc) => {
                instructions.push(generate_row_instruction(inc, count, &mut rng, i));
                instructions.push(html! {<div>{format!("Row {}: k{}", 2*i, count)}</div>});
            }
        }
    }
}

fn generate_row_instruction(inc: i32, count: i32, rng: &mut StdRng, i: usize) -> Html {
    if inc + inc == count {
        return html! {<div>{format!("Row {}: *k1,inc rep from * to end (total of {} inc, {} st total)", 2*i+1, inc, count)}</div>};
    } else if inc > 1 {
        // Row with increases
        // Divide in to roughly even blocks of knitting which will have increases between them:
        let blocks = inc + 1;
        // Figure out how many stitches in each block *before* the increases happen:
        let block_sizes = f64::floor((f64::from(count - inc)) / f64::from(blocks)) as i32;
        // Since we use floor, we rounded down so we may have a few stitches left after the blocks:
        let rem = count - (blocks * block_sizes + inc);
        // We don't want to start everything inc at the same place or we end up with too much of a pattern
        // so pick a random amount to put at the beginning:
        let before_st = rng.gen_range(0..(rem + block_sizes));
        // Figure out how many stitches that leaves at the end:
        let after_st = rem + block_sizes - before_st - 1;
        let instruction = format!("Row {}: k{} st, inc, * k{}, inc, rep from * {} times, k{} (total of {} inc, {} st total)",
                                              2*i+1,before_st,     block_sizes,        blocks-1,  after_st,    inc,    count);
        info!(
            "{} --- block_sizes={}, rem={}, before_st={}, blocks={}, count={}, sum={}",
            instruction,
            block_sizes,
            rem,
            before_st,
            blocks,
            count,
            before_st + 1 + (block_sizes + 1) * (blocks - 1) + after_st
        );
        return html! {<div>{instruction}</div>};
    } else if inc == 1 {
        // Row without significant increases
        return html! {<div>{format!("Row {}: Knit, inc. by total of {} st for total of {} st in row", 2*i+1, inc, count)}</div>};
    } else {
        return html! {<div>{format!("Row {}: k{}", 2*i+1, count)}</div>};
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Sphere Pattern Generator" }</h1>
            <p><SphereComponent /></p>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
