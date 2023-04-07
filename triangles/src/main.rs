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

    if args.len() != 4 {
        println!("[ERROR]  - Cantidad invalida de argunmentos");
        return ExitCode::FAILURE;
    }

    let a: f32 = args[1]
        .parse()
        .expect("[ERROR] - Argumento invalido, el argumento debe ser un numero positivo mayor a 0");
    let b: f32 = args[2]
        .parse()
        .expect("[ERROR] - Argumento invalido, el argumento debe ser un numero positivo mayor a 0");
    let c: f32 = args[3]
        .parse()
        .expect("[ERROR] - Argumento invalido, el argumento debe ser un numero positivo mayor a 0");

    println!(
        "args[0] = {} args[1] = {} args[2] = {} args[3] = {}",
        args[0], args[1], args[2], args[3]
    );

    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        println!(
            "[ERROR] - Argumento invalido, el argumento debe ser un numero positivo mayor a 0"
        );
        return ExitCode::FAILURE;
    }

    let mut lados_iguales = 0;

    let data = [[a, b, c], [b, c, a], [c, a, b]];

    for d in &data {
        let dif = ABS!(d[0] - d[1]);

        if d[2] <= dif {
            println!("Lado {0} no cumple con la Propiedad Triangular, debido a que es menor que la diferencia entre {1} y {2} (siendo esta {3})",d[2], d[0], d[1], dif);
            return ExitCode::FAILURE;
        }
        if dif == 0.0 {
            lados_iguales += 1;
        }
    }

    println!("El triangulo cumple con la Propiedad Triangular");

    match lados_iguales {
        0 => println!("Es un triangulo Escaleno"),

        1 => println!("Es un triangulo Isósceles"),

        2 | 3 => println!("Es un triangulo Equilátero"),

        _ => (),
    }

    ExitCode::SUCCESS
}
