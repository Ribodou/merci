use plotters::prelude::*;

fn snowflake_iter(nb_points: &usize, a: &f64) -> Vec<(f64, f64)> {
    let mut ret = vec![];
    let deuxpi = 2.0 * std::f64::consts::PI;
    let delta_theta = deuxpi / *nb_points as f64;
    for i in 0..*nb_points {
        let theta = delta_theta * i as f64;
        let y = -a * f64::cos(theta) * (1.0 + f64::cos(theta));
        let x = a * f64::sin(theta) * (1.0 + f64::cos(theta));
        let x = x / 1.2; // écrase l'axe des x pour rendre un peu mieux
        ret.push((x, y));
    }
    ret
}

const OUT_FILE_NAME: &str = "cadeau.gif";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(OUT_FILE_NAME, (800, 600), 1_0)?.into_drawing_area();
    let nb_points = 100; // nombre de points dans le cadeau
    let nb_iter = 70; // nombre de fois ou on montre le cadeau

    // début et fin du paramètre a de l'équation du cadeau
    let start_a = 0.6;
    let end_a = 1.2;
    let pas_a = (end_a - start_a) / nb_iter as f64;

    for i in 0..(nb_iter * 2) {
        // calcul du paramètre a de l'équation du cadeau
        let a = if i < nb_iter {
            i as f64 * pas_a + start_a
        } else {
            (2 * nb_iter - i) as f64 * pas_a + start_a
        };

        // le cadeau
        let mut snowflake_vertices = snowflake_iter(&nb_points, &a);

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Merci !", ("sans-serif", 50))
            .build_cartesian_2d(-2.0..2.0, -2.5..0.5)?;

        chart.draw_series(std::iter::once(Polygon::new(
            snowflake_vertices.clone(),
            RED.mix(0.2),
        )))?;

        snowflake_vertices.push(snowflake_vertices[0]);
        chart.draw_series(std::iter::once(PathElement::new(snowflake_vertices, RED)))?;

        root.present()?;
    }

    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
