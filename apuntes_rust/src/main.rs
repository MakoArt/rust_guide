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
/*
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

/*
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
*/
//7. PROPIEDAD 
//Permite a Rust garantizar la seguridad de la memoria sin Necesito un basurero 
/*
Rust utiliza un tercer enfoque: 
la memoria se gestiona a través 
de un sistema de propiedad con un 
conjunto de reglas que el compilador comprueba. 
Si Si se incumple alguna norma, el programa no se compila. Ninguna de las características La propiedad 
ralentizará tu programa mientras está en marcha. 

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

*/ 

*/
//8. PROPIEDAD 
// fn main(){
//     //esto da error no se puede hacer para evitar la doble liberacion
//      let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!"); 
  
//     //Por medio de el metodo clone si podemos hacerlo hacemos una copia completa
//  let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
     

//     //En este caso si podemos porque es asignado a la pila 
//     // no hay que hacer copias profundas , tienen tamaño 
//     //determinado 
//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");

/* 



/fn main(){ 
    //dos maneras de declarar e inicializar un string 
    // let s = String::from("hello");
    // let s2:&str ="Hello"; 

    // println!("{s},{s2}");

    // //mutabilidad de los strings 

    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() appends a literal to a String

    // println!("{s}"); // this will print `hello, world!`
    
    // //esto se puede hacer   
    // //los datos primitivos simples puedo igualar una variable a otra 
    // //de esta manera porque me crea una copia automaticamente
    // let x = 5;
    // let y = x;

    // //Con un string la cosa cambia  esto daria un error por move :
    // //el lenguage no copia un string y tendriamos un dangling pointer 
    // //o puntero colgante 

    // let s1 = String::from("hello");
    // let s2 = s1;
    
    // //Con un string podemos clonar la variable 
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {s1}, s2 = {s2}");

    // Funciones Ejemplo 1 
    
//     //paso parametros por referencia y me deja prestar a una funcion 
//     //new_string sin perder la propiedad y es valido. 
//     let new_string = String::from("Sambo");
//     paso_parametros(&new_string); 
//     println!("Me devuelve {}",new_string);
   

// }

// fn paso_parametros(texto:&String){
//     println!("{}",texto);
//     //texto.clone()
//     //texto
// }

//EJEMPLO 2 
// tambien se puede hacer esto , la propiedad sigue siendo 
// de new_string aunque hagamos devolucion y por referencia
fn main(){
     let new_string = String::from("Sambo");
     let new_string2=paso_parametros(&new_string); 
    println!("Me devuelve {}",new_string); 
    println!("Me devuelve {}",new_string2);
}


fn paso_parametros(texto:&str)->&str{
    println!("{}",texto);
    texto
   
}


//9.CORTES O SLICES

fn main(){

    //1. una referencia a la parte continua de una colleccion , no es dueño de los datos solo apunta a referencia  

    

//     let mut s = String::from("hello world"); 
//     let word = first_word(&s); 
//     println!("{}",word);
//     s.clear();



// }
// // Problema que intenta resolver
// // Imagina que quieres escribir una función que devuelva la primera palabra de un String.

// //Sin slices, podrías devolver el índice donde termina la palabra: 

// //nos referencia a todo 

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// //Cortes de cuerda  

//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];

// //cartes de cuerda , 

// let s = String::from("hello");

// let len = s.len();

// let slice = &s[3..len];
// let slice = &s[3..];


// //cortes con cuerdas  

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

//Cortes en arrays   

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);


//10. ESTRUCTURAS . 

//Declaración de una estructura 

/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
 //si los valores van a ser mutables , la isntancia user 
 //debe ser mutable , los valores independientemente 
 //no se pueden hacer mutables. 

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

     user1.email = String::from("anotheremail@example.com");
}


10. METODOS DE LOS STRUCTS 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//EJEMPLO CON ESTRUCTURA WARRIOR  


struct Warrior {
    name: String,
    lives: u8,
    strength: u8,
}
impl Warrior{
    fn atacar(&self){
        println!("Estoy atacando y tengo {} vidas", self.lives);
    }
}

fn main() {
    let conan = Warrior {
        name: String::from("Conan"),
        lives: 5,
        strength: 9,
    };

println!("{} tiene {} vidas y {} de fuerza", conan.name, conan.lives, conan.strength);

conan.atacar();

}
// ┌──────────────────────────────────────────────────────────────┐
// │                 ⚠️  CUADRO DE MALAS PRÁCTICAS  ⚠️             │
// ├──────────────────────────────────────────────────────────────┤
// │ 1. `mut` innecesario                                          │
// │    Declaras `let mut conan`, pero nunca lo modificas.         │
// │    → Warning: “variable does not need to be mutable”.         │
// ├──────────────────────────────────────────────────────────────┤
// │ 2. Estilo de formato                                          │
// │    Falta espacio tras los `:` y las llaves no están alineadas.│
// │    → No es error, pero rompe las convenciones de Rust.        │
// ├──────────────────────────────────────────────────────────────┤
// │ 3. Nombre incorrecto del campo `lifes`                        │
// │    En inglés correcto sería `lives`.                          │
// │    → No afecta al compilador, pero sí a la claridad.          │
// ├──────────────────────────────────────────────────────────────┤
// │ 4. Campo `strength` no utilizado                              │
// │    El struct tiene un campo que no se usa en ningún sitio.    │
// │    → No da warning, pero es mala práctica.                    │
// ├──────────────────────────────────────────────────────────────┤
// │ 5. Falta de métodos (`impl`)                                  │
// │    Acceder a los campos está bien, pero es más idiomático     │
// │    crear métodos para operaciones comunes.                    │
// ├──────────────────────────────────────────────────────────────┤
// │ 6. Falta de `#[derive(Debug)]`                                │
// │    Útil para imprimir structs fácilmente durante depuración.  │
// │    → No es obligatorio, pero sí recomendable.                 │
// └──────────────────────────────────────────────────────────────┘


//11. ENUMERABLES Y COINCIDENCIAS : 


enum IpAddrKind {
    V4,
    V6,
}
//PODEMOS ASIGNARLOS A VARIABLES
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

//PODEMOS PASARLO POR PARAMETROS 
    fn route(ip_kind: IpAddrKind) {} 
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

        enum IpAddrKind {
        V4,
        V6,
    }


 //USAR EL ENUM DENTRO DE ESTRUCTURAS 
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
//ASOCIARLE OTROS TIPOS DE DATOS 



        enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
 

//USAR ESTRUCTURAS DENTRO DE ENUMERABLES 

    struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//FUNCIONES EN ENUMERABLES 
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //ENUMERABLES 

    enum Option<T> {
    None,
    Some(T),
}

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

//COINCIDENCIAS 
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
//PODEMOS PASAR UN VALOR O PASARLE NADA Y ME SUMARA O NO ME DEVOLVERA NADA 
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
//SEGUN SALGA EL RESULTADO DE UN DADO PUEDEN PASAR COSAS DISTINTAS 

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // MAS COINCIDENCIAS 

    impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }


    //mas comprobaciones 
    enum Option<T> {
    Some(T),
    None,
} 
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
//format! en Rust es una macro que construye un String usando interpolación de variables, muy parecido a println!, pero en lugar de imprimirlo, lo devuelve como texto.

En tu código aparece así:
Some(format!("{state:?} is pretty old, for America!"))


}


12.PACKAGES 
-------------
//creamos una libreria 
//cargo new restaurant --lib  
//main.rs y lib.rs son los crate raices 

//    // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();


// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

//  En Rust, todos los elementos 
//  (funciones, métodos, estructuras, 
//     enums, módulos y constantes) 
//     son privados de los módulos 
//     padres por defecto. 
//     Si quieres Para hacer que 
//     un objeto como una función 
//     o estructura sea privado, 
//     lo pones en un 
//     módulo.hostinghostingadd_to_waitlist


 mod front_of_house {
  pub mod hosting {
       pub fn add_to_waitlist() 
       {
           println!("Todo va bien");

       }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Podemos construir caminos 
// relativos que comienzan 
// en el módulo padre, 
// en lugar de el módulo 
// actual o la raíz de caja, 
// usando al inicio de la Camino. 
// Esto es como iniciar una ruta de 
// sistema de archivos con la sintaxis q
// ue significa para ir al directorio principal. 
// Usar nos permite referenciar un objeto que sabemos 
// que está en el módulo padre, lo que puede hacer
//  que reorganizar el módulo El árbol es más fácil
//   cuando el módulo está estrechamente relacionado 
//   con el padre pero el padre Quizá algún día se 
//   traslade a otro lugar del árbol de módulos.super..super 

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

//Podemos poner use y no tener que llamar a toda la ruta completa 

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

//Para no repetir rutas   

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}



mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//para usar en el ambito como publico poner pub 
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//packete(aplicacion) formado por cajas binarias o de libreria 
//una caja de libreria tiene un modulo , un submodulo y dentro la logica 
//una cana binaria es donde esta el main.rs y genera el ejecutable 
//cuando llamamos a una libreria es una caja de libreria externa 
//para llamar a la libreria standard de rust y un pack es : 
// use std::collections::HashMap;
//use std::io;
//use std::io::Write;
//use std::collections::*;


//TRABAJAR EN VARIOS ARCHIVOS : 
//LA LIBRERIA RESTAURANT QUEDARIA DE ESTA MANERA: 

en el lib.rs :  

mod front_of_house; //tenemos que incluir el archivo a usar

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

en el front_of_house.rs  : 

pub mod hosting {
    pub fn add_to_waitlist() {}
}

la estructura de carpetas es : 

my-project : 
   -restaurant : libreria 
     -src 
       : 
       front_of_house.rs 
       lib.rs  

   -src : 
    main.rs 


13.COLECCIONES 

// VECTORES  
//DECLARACIÓN  SIN VECTOR SIN DATOS 
    let v: Vec<i32> = Vec::new();
//INICIALIZACION SIN DECLARACION 
       let v = vec![1, 2, 3]; 

   
  //ACTUALIZACION DEL VECTOR 
      let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //hacer referencia a un elemento del array 
        let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
  // Con la primera declaracion da error si no existe un indice 
  //con la segunda no da error 
      let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    //EJEMPLO 

        let mut v = vec![1, 2, 3, 4, 5]; //DECLARO ARRAY INICIALIZAO

    let first = &v[0]; //HAGO REFERENCIA AL PRIMER ELEMENTO

    v.push(6);//PUSH , SE GENERA NUEVO VECTOR HASTA 6 Y DESAPARECE EL ANTERIOR

    println!("The first element is: {first}");//COMO LA REFERENCIA ES AL VECTOR ANTERIOR Y NO EXISTE DA ERROR
  
    //RECORRER UN VECTOR  
        let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //ITERANDO SOBRE REFERENCIAS MUTABLES 
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

  //PÀRA GUARDAR ELEMENTOS DE DISTINTO TIPO ENGAÑANDO AL 
  //LENGUAJE 
      enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //CADENAS DE STRINGS 
      //declaracion inmutable  
          let new_s:&str=34;
     //declaracion de un string mutable 
        let mut s = String::new(); 
        // o 
         let s = String::from("initial contents");
   
        //otra forma de hacerlo mutable 
        let data = "initial contents";

       let s = data.to_string();

    // ya podemos trabajar con el metodo asi 
    let s = "initial contents".to_string();

   // multiples lenguajes 
       let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

  //otro ejemplo 
  //(usamos push para añadir char '' al final , y 
  // push_str para cadenas mas largas al final )
   let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}"); 

    //OTRO EJEMPLO 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    //PODEMOS RECORRER CADA ELEMENTO CONVIRTIENDOLO EN CHAR 
    for c in "Зд".chars() {
    println!("{c}");
   } 
   //O RECORRER NUMERCIAMENTE POR BYTES 
   for b in "Зд".bytes() {
    println!("{b}");
}

//HASH MAP  

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

//ACCEDEMOS A UN VALOR DEL HASH MAP 

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

 // -------------------------------------------------------------
// EXPLICACIÓN DE:
// let score = scores.get(&team_name).copied().unwrap_or(0);
// -------------------------------------------------------------

// scores.get(&team_name)
// Busca la clave `team_name` dentro del HashMap `scores`.
// Devuelve un Option<&i32>:
//   - Some(&valor) si la clave existe
//   - None si no existe

// .copied()
// Convierte Option<&i32> en Option<i32> copiando el valor.
// Es decir, pasa de referencia (&i32) a valor (i32).

// .unwrap_or(0)
// Toma el Option<i32> y:
//   - Si es Some(valor), devuelve ese valor
//   - Si es None, devuelve 0 como valor por defecto

// Resultado final:
// Obtiene el puntaje del equipo si existe; si no, usa 0.
//nOS DEVUELVE LOS DATOS EN ORDEN ALEATORIO 
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

   //PODEMOS ACTUALIZAR LOS DATOS SOBREESCRIBIENDOLOS 

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

// -------------------------------------------------------------
// EXPLICACIÓN DE:
// scores.entry(String::from("Yellow")).or_insert(50);
// -------------------------------------------------------------

// scores.entry(String::from("Yellow"))
// Accede a la "entrada" del HashMap para la clave "Yellow".
// Devuelve un Entry, que puede ser:
//   - OccupiedEntry  → la clave ya existe
//   - VacantEntry    → la clave no existe

// .or_insert(50)
// Si la clave NO existe, inserta el valor 50 y devuelve
// una referencia mutable a ese valor recién insertado.
// Si la clave YA existe, no cambia nada y devuelve una
// referencia al valor existente.

// Resultado final:
// Garantiza que la clave "Yellow" tenga un valor en el HashMap.
// Si no existía, se crea con 50. Si ya existía, se mantiene.



 //OTRO EJEMPLO  

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}"); 

    // -------------------------------------------------------------
// EXPLICACIÓN DEL CÓDIGO COMPLETO
// -------------------------------------------------------------

use std::collections::HashMap;
// Importa HashMap desde la librería estándar para poder usarlo.

let text = "hello world wonderful world";
// Cadena de texto que vamos a analizar palabra por palabra.

let mut map = HashMap::new();
// Crea un HashMap vacío donde la clave será una palabra (&str)
// y el valor será un contador (i32). Es mutable porque lo modificaremos.

for word in text.split_whitespace() {
// split_whitespace() divide el texto en palabras usando espacios.
// El bucle recorre cada palabra encontrada.
    
    let count = map.entry(word).or_insert(0);
    // map.entry(word):
    //   Accede a la entrada del HashMap para esa palabra.
    //   Si existe → devuelve su entrada.
    //   Si no existe → crea una entrada vacía.
    //
    // .or_insert(0):
    //   Si la palabra NO estaba en el mapa, inserta 0.
    //   Si ya estaba, no cambia nada.
    // Devuelve una referencia mutable al valor asociado a la palabra.

    *count += 1;
    // count es &mut i32, una referencia mutable.
    // *count desreferencia para obtener el valor.
    // Incrementa el contador de esa palabra.
}

println!("{map:?}");
// Imprime el HashMap usando formato de depuración.
// Resultado esperado:
// {"hello": 1, "world": 2, "wonderful": 1}



*/

