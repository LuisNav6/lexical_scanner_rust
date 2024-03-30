### 🚀 Descripción

Este repositorio contiene un programa escrito en Rust para realizar el análisis léxico de un código fuente en un lenguaje de programación simulado. El análisis léxico es la primera fase del proceso de compilación, donde se convierte una secuencia de caracteres en una secuencia de tokens, que son unidades léxicas con significado en el lenguaje de programación.

El programa consta de las siguientes partes principales:

1. **Enums TokenType y StateType**: Definen los tipos de los tokens y los estados del autómata finito determinista (DFA, por sus siglas en inglés) utilizado para reconocer los tokens en el código fuente.

2. **Funciones de análisis léxico**:
   - `get_next_char`: Obtiene el siguiente carácter no en blanco de la línea actual.
   - `unget_next_char`: Retrocede un carácter en la línea actual.
   - `reserved_lookup`: Busca palabras reservadas y devuelve su TokenType correspondiente.
   - `get_token`: Realiza el análisis léxico y devuelve una lista de tokens junto con su tipo, lexema, número de línea y número de columna.

3. **Función main**: Contiene un ejemplo de código fuente simulado y llama a la función `get_token` para realizar el análisis léxico. Luego imprime los tokens resultantes junto con su información de ubicación en el código fuente.

Este programa se ha desarrollado con el objetivo de entender y demostrar el proceso de análisis léxico en un compilador, utilizando el lenguaje de programación Rust. 📝
Puedes probar este programa en la siguiente URL de un compilador web de Rust: https://www.programiz.com/rust/online-compiler/ 🌐
