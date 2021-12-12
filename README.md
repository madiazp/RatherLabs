# RatherLabs
Challenge de RatherLabs

Proyecto escrito en Rustlang usando:
- Cargo: cargo 1.52.0-nightly (c68432f1e 2021-03-02)
- rustc: rustc 1.52.0-nightly (234781afe 2021-03-07)
# Idea general
- Se pregunta por los dias del pronóstico.
- Se llama a la API con el día ingresado.
- Se construye un HashMap donde las llaves son los nombres de las provincias, y los valores el índice en que aparecen en la respuesta de la API. Con esto no se muestran repetidas las provincias en la consola y se puede buscar las ciudades que corresponde a la provincia en tiempo constante.
- Se pregunta por el índice de la provincia
- Se traen los índices de las ciudades guardados en el HashMap que corresponden a la provincia seleccionada.
- Se pregunta por el índice relativo (el índice que corresponde a la lista de ciudades de la provincia guardada en el HashMap) 
- Con el índice en la lista del HashMap se obtiene el el índice real de la respuesta de la API y con ello la información de pronóstico de la ciudad.

## Complejidad
  Si n son la cantidad de datos en la respuesta de la API y m la cantidad máxima de ciudades en una provincia, entonces:
  -O(n): creación del mapa.
  -O(1): búsqueda de las ciudades en el hashmap
  -O(m): búsqueda del índice real de laciudad
  -O(1): búsqueda de la ciudad en la respuesta de la api ([Vec get en rust](https://doc.rust-lang.org/std/collections/index.html#sequences))
  Total: O(n+m)
  
# Forma de ejecutar
Ponerce en la raíz y ejecutar:
 `cargo run`
 
# Supuestos
Algunos supuestos para la realización del proyecto fueron:
- El programa finaliza cuando la información del pronóstico de la ciudad es entregada.
- Si los inputs son inválidos (no son números enteros positivos o están fuera de rango), el programa emite un mensaje de error y vuelve a preguntar por un input hasta que el usuario otorgue uno válido o cancele el programa.
- La api siempre funciona (No hay un retry). Si la llamada a la api de pronóstico falla, el programa finalzia con un mensaje de error.
# Notas y cosas que mejorar

- Se usa `.unwrap()` en vez de manejar como corresponde los enums Options o Result, sólo en las veces que se sabe con certeza que el valor existe.
- Se usa un `.clone()` para construir las llaves del HashMap al ser de tipo String. Esto es debido a que tanto la respuesta de la API como el HashMap necesitan los nombres de las provisias simultáneamente, por lo tanto no se puede hacer `borrow` del valor.
## Cosas que mejorar:
- Agregar un retry a la llamada de la API
- Hacer una macro para los retries del stdin
