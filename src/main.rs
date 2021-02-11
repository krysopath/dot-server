
use tide::log;
//use tide::prelude::*;
use tide::{Response, StatusCode};


mod charts;
mod config;

//use charts::Chart;


#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

    let conf = config::make_config();
    let app = routes(tide::new());

    app.listen(conf.bind_address()).await?;

    Ok(())
}

fn routes(mut app: tide::Server<()>) -> tide::Server<()> {
    app.at("/").get(index);
    app.at("/fsm/new").post(new_fsm);
    app

}

async fn index(_req: tide::Request<()>) -> tide::Result {
    let mut graph = charts::Chart::new(
        "dot-server".into(),
        charts::ChartUnits::KiloBytes,
        false,
        "#8ff0a4".into(),
        15,
    );

    graph.add_point(1.0, 1.0);
    graph.add_point(1.5, 2.0);
    graph.add_point(2.0, 3.0);
    graph.add_point(3.0, 8.5);
    graph.add_point(3.5, 3.0);
    graph.add_point(4.0, 6.0);
    graph.add_point(4.5, 5.0);
    graph.add_point(5.0, 3.0);
    graph.add_point(5.5, 2.0);

    let svg: String = graph.draw_svg(1280, 1024)?;

    let response = Response::builder(StatusCode::Ok)
        .body(svg.to_string())
        .content_type("image/svg+xml")
        .build();

    Ok(response)
}

async fn new_fsm(_req: tide::Request<()>) -> tide::Result {
    Ok(format!("Hello, {}! I've put in an order for {} shoes", "lol", "lol").into())
}
