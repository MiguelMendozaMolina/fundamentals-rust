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
}
