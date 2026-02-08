use std::io; //libreria standard entrada y salida por consola

//1. JUEGO DE ADIVINANZAS *********************************

/*
fn main() {
    
   println!("Guess the number!");
// Muestra en pantalla el mensaje inicial del juego.

println!("Please input your guess.");
// Pide al usuario que introduzca su intento.

let mut guess = String::new();
// Crea una variable mutable llamada `guess` y la inicializa como un String vacío.
// Aquí se almacenará lo que escriba el usuario.

io::stdin()
// Accede a la entrada estándar (el teclado).

    .read_line(&mut guess)
// Lee una línea completa de texto introducida por el usuario y la guarda en `guess`.
// Se pasa `&mut guess` porque la función necesita modificar el contenido de la variable.

    .expect("Failed to read line");
// Si ocurre un error al leer la entrada, el programa se detiene y muestra este mensaje.

println!("You guessed: {guess}");
// Imprime el valor que el usuario escribió, usando interpolación de variables.

}

*/

//2. VARIABLES ********************************************************** 
//Escribimos las variables en snake case 
/*
fn main(){
    let life_number = 5 ; 
    const MIN_LIFES = 0; 

 // SHOWDOWNING  o sombriado de la variable 
  //podemos crear una variable con el mismo nombre y cambiar 
  // el tipo o el valor 
    let spaces = "   ";
    let spaces = spaces.len();

  //Pero si uso la misma variable aunque sea mutable 
  // no puedo cambiarle el tipo pero si el valor 
   //esto daria error 
    let mut spaces = "   ";
    spaces = spaces.len(); 
     




}
*/

//3.TIPOS DE DATOS  
/*
//primitivos ..........
fn main(){ 
//decidimos si tipamos o no las variables 

let x = 5 ; //sin tipar 
let y:i32 = 23; //tipado

+--------+---------+----------------------------------------------+
|  Tipo  | Tamaño  |                   Rango                      |
+--------+---------+----------------------------------------------+
|  u8    |  8 bits | 0 a 255                                      |
|  u16   | 16 bits | 0 a 65,535                                   |
|  u32   | 32 bits | 0 a 4,294,967,295                            |
|  u64   | 64 bits | 0 a 18,446,744,073,709,551,615               |
|  u128  |128 bits | 0 a 340,282,366,920,938,463,463,374,607,431… |
| usize  | depende | 0 a 2^N - 1 (N = bits de la arquitectura)    |
+--------+---------+----------------------------------------------+

+--------+---------+--------------------------------------------------------------+
|  Tipo  | Tamaño  |                           Rango                              |
+--------+---------+--------------------------------------------------------------+
|  i8    |  8 bits | -128 a 127                                                   |
|  i16   | 16 bits | -32,768 a 32,767                                             |
|  i32   | 32 bits | -2,147,483,648 a 2,147,483,647                               |
|  i64   | 64 bits | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807       |
|  i128  |128 bits | -170,141,183,460,469,231,731,687,303,715,884,105,728 a ...   |
| isize  | depende | -(2^(N-1)) a (2^(N-1) - 1)                                   |
+--------+---------+--------------------------------------------------------------+

===============================================================
        LITERALES NUMÉRICOS EN RUST — FORMATOS Y EJEMPLOS
===============================================================

+-------------------------+----------------------+-------------------------------+
|      Literal            |      Formato         |            Valor              |
+-------------------------+----------------------+-------------------------------+
| 98_222                  | Decimal              | 98,222                        |
| 0xff                    | Hexadecimal          | 255                           |
| 0o77                    | Octal                | 63                            |
| 0b1111_0000             | Binario              | 240                           |
| b'A'                    | Byte literal (u8)    | 65 (código ASCII de 'A')      |
+-------------------------+----------------------+-------------------------------+

+--------+----------+-----------------------------------------------+
|  Tipo  | Tamaño   |                   Rango                       |
+--------+----------+-----------------------------------------------+
|  f32   | 32 bits  | ~1.175494e−38  a  ~3.402823e+38               |
|        |          | (precisión aproximada de 6–7 decimales)       |
+--------+----------+-----------------------------------------------+
|  f64   | 64 bits  | ~2.225074e−308 a  ~1.797693e+308              |
|        |          | (precisión aproximada de 15–16 decimales)     |
+--------+----------+-----------------------------------------------+


+--------+----------+---------------------------+
|  Tipo  | Tamaño   |          Valores          |
+--------+----------+---------------------------+
|  bool  | 1 byte   | true   /   false          |
+--------+----------+---------------------------+

+--------+----------+-----------------------------------------------+
|  Tipo  | Tamaño   |                   Rango                       |
+--------+----------+-----------------------------------------------+
|  char  | 4 bytes  | Cualquier Unicode: U+0000 a U+10FFFF          |
+--------+----------+-----------------------------------------------+

//Compuestos  

   //TUPLAS (Permiten varios tipos de dato,longitud fija)

   let tup: (i32, f64, u8) = (500, 6.4, 1); 

   //Destructuracion de tupla 

     let tup = (500, 6.4, 1);

     let (x, y, z) = tup;

    println!("The value of y is: {y}");
   
    //Tambien podemos acceder asi : 
      let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;


  //ARRAYS (El mismo tipo , son estaticos,longitud fija)
    
  //puedo no tiparlo 
    let a = [1, 2, 3, 4, 5]; 

 //puedo tiparlo 
 let a: [i32; 5] = [1, 2, 3, 4, 5]; 
  
 //tambien se puede hacer esto 
 let a = [3; 5];
 let a = [3, 3, 3, 3, 3]; 

 //Acceso a elementos de un array  
  let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

//En rust cuando te sales de un array o pides un dato
//Que esta fuera del array , no hay problemas de fugas de 
//memoria como pasa en C++ 


}
*/ 

// 4. FUNCIONES 

fn main(){
 // Se suelen escribir en snake case  hola_mundo() 

 //Ejemplo 
 
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

 //Expresiones 

 let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

//Funciones con retorno 
fn five() -> i32 {
    5
}

let x = five();
 println!("The value of x is: {x}");

}

//Funciones con retorno y paso de parametros 

fn plus_one(x: i32) -> i32 {
    x + 1
}
//Cuando queremos retornar algo no se pone return ni ; 

//5. COMENTARIOS  *************

//De una linea 

/*
De
Varias
*/

//6.BUCLES O CONTROL DE FLUJO 

fn main(){
    //ejemolo
    fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    //otro ejemplo 
        let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //podemos guardar en una variable entera o de cualquier tipo
    //el resultado de una condicion , pero los resultados tienen
    //que ser del mismo tipo en este caso pueden devolverme 
    // 5 0 6 que son del mismo tipo

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //LOOPS 
    loop {
        println!("again!");
    }
    //DEVOLVER VALORES DE BUCLES 
        let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    //Etiquetas en bucles , puedes parar uno independiente del otro 
     let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //  WHILE  
    //PRIMER EJEMPLO

       let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!"); 

    //SEGUNDO EJEMPLO 
     let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

 //FOREACH 
   let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //FOREACH CON RANGO  
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

}
}
}
