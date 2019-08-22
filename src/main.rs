
use std::env;
use rand::seq::SliceRandom;
mod regression;
use regression::Point;
use std::f64;

static THRESHOLD: f64 = 3.0;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let points = match regression::parse_points(filename) {
    Ok(points) => points,
    Err(err) => {
      println!("Problem reading file {}: {}", filename, err.to_string());
      std::process::exit(1)
    }
  };

  ransac_iter(&points);

}

fn ransac_iter(source:&Vec<Point>) {
  let mut best_error = f64::MAX;
  let mut best_betas = regression::get_betas(&source, 2); //tODO use Option
  let mut best_ensemble = vec![];

  for k in 0..10 {
    let (selected, _others) = split_random(source, 2);       //split dataset
    let mut betas = regression::get_betas(&selected, 2);    //evaluate model on small dataset part
    let distance = |p: &Point| (betas[1]*p.x + -p.y + betas[0]).abs() / (betas[1].powi(2)+1.0).sqrt(); //closure giving distance to model

    let ensemble:Vec<Point> = source.iter().filter(|&p| distance(p) < THRESHOLD).map(|p| p.clone()).collect();

      let d = (source.len() as f64 * 0.6) as usize;

      if ensemble.len() > d {
        betas = regression::get_betas(&ensemble, 2);  //reajust model to ensemble
        let error = regression::get_error(&betas, &ensemble);
        if error < best_error {
          regression::plot_stuff_label(&betas, &source, &k.to_string());
          best_error = error;
          best_betas = betas;
          best_ensemble = ensemble;
        }
      }
  }

    regression::plot_stuff_label(&best_betas, &source, "best");
}

fn split_random<T: Clone>(source:&Vec<T>, nb:usize) -> (Vec<T>, Vec<T>) {
    let range = (0..source.len()).collect::<Vec<_>>();
    let nbs:Vec<_> = range.choose_multiple(&mut rand::thread_rng(), nb).collect();
    
    let mut selected = vec![];
    let mut others = vec![];
    
    for (i, elt) in source.iter().enumerate() {
      if nbs.iter().any(|&x| *x==i) {
        selected.push(elt.clone())
      } else {
        others.push(elt.clone())
      }
    }
    (selected, others)
}
