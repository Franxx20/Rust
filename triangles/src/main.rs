use std::env;
use std::process::ExitCode;

macro_rules! ABS {
    ($a:expr) => {
        if $a >= 0.0 {
            $a
        } else {
            $a * -1.0
        }
    };
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 4 {
        println!("[ERROR]  - Cantidad invalida de argunmentos");
        return ExitCode::FAILURE;
    }

    let a: f32 = args[1].clone().parse().unwrap();
    let b: f32 = args[2].clone().parse().unwrap();
    let c: f32 = args[3].clone().parse().unwrap();

    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        println!(
            "[ERROR] - Argumento invalido, el argumento debe ser un numero positivo mayor a 0"
        );
        return ExitCode::FAILURE;
    }

    let mut lados_iguales = 0;

    let data = [[a, b, c], [b, c, a], [c, a, b]];

    for i in 0..3 {
        let dif = ABS!(data[i][0] - data[i][1]);

        if data[i][2] <= dif {
            println!("Lado {0} no cumple con la Propiedad Triangular, debido a que es menor que la diferencia entre {1} y {2} (siendo esta {3})",data[i][2], data[i][0], data[i][1], dif);
            return ExitCode::FAILURE;
        }

        if dif == 0.0 {
            lados_iguales += 1;
        }
    }

    println!("El triangulo cumple con la Propiedad Triangular");

    match lados_iguales {
        0 => {
            println!("Es un triangulo Escaleno");
        }
        2 | 3 => {
            println!("Es un triangulo Isósceles");
        }
        _ => {}
    }

    ExitCode::SUCCESS
}
