use std::{collections::HashSet, vec};

fn main() {
    // hola mundo
    /* hola mundo */
    println!("Hello, world!");

    // variables
    // palabra clave mut para poder hacer que la variable pueda ser mutuable en la 
    // en la siguiente asignacion 
      let mut my_string: &str = "estos es una cadena de texto";
      println!("{my_string}");
      my_string = "esto es otro texto";
      println!("{my_string}");

      // my_string = 6; Error por que ya existe el tipo de dato establecido como string 
      

      // a diferencia de la anterior esta te permite nominar de manera especifica el largo 
      // de una cadena de string , la anterior te otorga el maximo posible por defecto 
      let my_string2: String = String::from("Esta es otra cadena de texto"); 
      println!("{my_string2}");

      let mut my_int: i32 = 7;
      my_int = my_int + 4; // resultado de este numero es 11
      println!("{my_int}"); // imprime el numero indicado 11
      println!("{}", my_int - 1); // se le resta un numero pero como numero mut , resultado 10 
      println!("{my_int}"); // este numero nuevamente vuelve a ser 11

      // interpolacion de valores 
      println!("Este es el valor de la variable my_int: {}", my_int);


      let my_int64: i64 = 7;
      println!("{my_int64}");

      // creacion de numeros enteros
      let my_float = 6.5;
      println!("{my_float}");

      // esto no es permitido debido a que es un lenguaje con un tipado super estricto
      // my_float = my_float + my_int;

      let  my_float2: f32 = 6.5;
      println!("{my_float2}");

      // utilizacion de datos de tipo booleano
    
      let mut my_bool = false;
      my_bool = true;
      println!("{my_bool}");

      // constantes 
      // debe definirse en MAYUSCULAS
      // este valor no puede variar 

      const MY_CONST: &str = "Mi propiedad constante";
      println!("{MY_CONST}");

      // Control de flujo 
      // esta estructura de control es la misma que se maneja generalmente
      // en todos los lenguajes de programacion 

      my_int = 11123312;
      my_string = "Hola";


      if my_int == 10 && my_string == "Hola" {
        println!("El valor es 10");
      } else if my_int == 11 || my_string == "Hola" {
        println!("el valor es 11");
      } else {
        println!("el valor no es 10 ni 11");
      }

      // Estructuras : Lista
      // Es necesario agregar un tipo para poder imprimirlo en el mensaje 
      // si es string tiene que ser todo string no puede existir un valor numerico 
      // esto es por que es un lenguaje fuertemente tipado

      let mut my_list: Vec<&str> = vec!["Miguel", "Mendoza", "@Mendoza"];
      my_list.push("Miguel"); // esta es la forma de agregar elementos a un a lista 
      println!("{:?}", my_list);
      println!("{}", my_list[0]); // no es necesario pasarle el tipo de formateo cuando se quiere acceder a un elemento especifico

      // Estructuras : Sets 
      // Este tipo de estructura necesita una importacion 
      // use std::collections::HashSet; 
      // esta importacion se realiza de manera automatica al especificar el tipo de dato 
      // HashSet<>
      // el set es una estructura desordenada y en la cual no se pueden repetir datos 

      let mut my_set: HashSet<&str> = vec!["Miguel", "Mendoza", "@Mendoza"].into_iter().collect();
      my_set.insert("Miguel");
      println!("{:?}", my_set);


}
