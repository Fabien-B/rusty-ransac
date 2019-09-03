
use rand::seq::SliceRandom;
mod regression;
use regression::Point;
use std::f64;
use nalgebra::base::DVector;


pub fn distance_2d(p: &Point, betas: &DVector<f64>) -> f64 {
    (betas[1]*p.x + -p.y + betas[0]).abs() / (betas[1].powi(2)+1.0).sqrt()
}

pub fn ransac_iter_2d(source:&Vec<Point>, num_iter: u32, threshold: f64) -> (Option<DVector<f64>>, Vec<Point>) {
    ransac_iter(source, num_iter, threshold, distance_2d, 2)
}

pub fn ransac_iter<F>(source:&Vec<Point>, num_iter: u32, threshold: f64, distance: F, order: usize) -> (Option<DVector<f64>>, Vec<Point>)
where F: Fn(&Point, &DVector<f64>) -> f64
{
  let mut best_error = f64::MAX;
  let mut best_betas = None;
  let mut best_ensemble = vec![];

  for _k in 0..num_iter {
    let (selected, _others) = split_random(source, order);       //split dataset
    let mut betas = regression::get_betas(&selected, order);    //evaluate model on small dataset part

    let ensemble:Vec<Point> = source.iter().filter(|&p| distance(p, &betas) < threshold).map(|p| p.clone()).collect();

      let d = (source.len() as f64 * 0.3) as usize;

      if ensemble.len() > d {
        betas = regression::get_betas(&ensemble, order);  //reajust model to ensemble
        let error = regression::get_error(&betas, &ensemble);
        if error < best_error {
          //regression::plot_stuff_label(&betas, &source, &k.to_string());
          best_error = error;
          best_betas = Some(betas);
          best_ensemble = ensemble;
        }
      }
  }
    (best_betas, best_ensemble)
}

fn split_random<T: Clone>(source:&Vec<T>, nb:usize) -> (Vec<T>, Vec<T>) {
    //TODO really usefull to split just choose multiple
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
