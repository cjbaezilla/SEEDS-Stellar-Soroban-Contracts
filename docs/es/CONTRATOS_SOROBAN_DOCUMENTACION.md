# Documentación Técnica: Workspace de Contratos Soroban

## Tabla de Contenidos

1. [Introducción](#introducción)
2. [Arquitectura y Estructura del Proyecto](#arquitectura-y-estructura-del-proyecto)
3. [Configuración del Workspace](#configuración-del-workspace)
4. [Librerías y Dependencias](#librerías-y-dependencias)
5. [Guía de Desarrollo](#guía-de-desarrollo)
6. [Compilación y Testing](#compilación-y-testing)
7. [Despliegue y Pruebas en Red Local](#despliegue-y-pruebas-en-red-local)
8. [Ejemplos de Código](#ejemplos-de-código)
9. [Mejores Prácticas](#mejores-prácticas)
10. [Solución de Problemas](#solución-de-problemas)
11. [Referencias y Recursos](#referencias-y-recursos)

---

## Introducción

Este documento proporciona una guía técnica completa para el workspace de contratos inteligentes de Stellar Soroban ubicado en `/home/carlos/dispensario_digital_sc/contracts`. El workspace está configurado como un proyecto Cargo autónomo que permite desarrollar, compilar y probar múltiples contratos inteligentes de Soroban de manera eficiente.

### ¿Qué es Soroban?

Soroban es la plataforma de contratos inteligentes de Stellar, diseñada para ser segura, escalable y eficiente. Los contratos se escriben en Rust y se compilan a WebAssembly (WASM), permitiendo ejecución determinista y segura en la red Stellar.

### Objetivos del Workspace

- **Modularidad**: Cada contrato es un proyecto independiente dentro del workspace
- **Reutilización**: Dependencias compartidas entre contratos
- **Optimización**: Configuración optimizada para tamaño mínimo de binarios WASM
- **Desarrollo Eficiente**: Herramientas y configuración listas para desarrollo

---

## Arquitectura y Estructura del Proyecto

### Estructura de Directorios

```
contracts/
├── Cargo.toml                 # Configuración del workspace
├── Cargo.lock                 # Lock file de dependencias (generado)
├── rust-toolchain.toml        # Configuración de Rust toolchain
├── target/                    # Directorio de compilación (generado)
│   └── wasm32v1-none/
│       └── release/
│           └── *.wasm         # Binarios compilados
└── hello-world/               # Contrato de ejemplo
    ├── Cargo.toml             # Configuración del contrato
    └── src/
        ├── lib.rs             # Código fuente del contrato
        └── test.rs            # Tests unitarios
```

### Componentes Principales

#### 1. Cargo.toml (Workspace Root)

Archivo de configuración principal del workspace que define:
- Miembros del workspace (contratos)
- Dependencias compartidas
- Perfiles de compilación optimizados

#### 2. rust-toolchain.toml

Especifica la versión exacta de Rust y los targets necesarios para compilación WASM.

#### 3. Contratos Individuales

Cada contrato es un proyecto Cargo independiente con su propio `Cargo.toml` y código fuente.

---

## Configuración del Workspace

### Cargo.toml del Workspace

```toml
[workspace]
members = ["hello-world"]
resolver = "2"
exclude = ["target"]
```

**Explicación de la configuración:**

- **`members`**: Lista de directorios que contienen contratos. Agregar nuevos contratos aquí.
- **`resolver = "2"`**: Usa el resolver v2 de Cargo para mejor resolución de dependencias.
- **`exclude`**: Directorios a excluir del workspace (como `target/` de compilación).

### Dependencias del Workspace

Las dependencias compartidas se definen en `[workspace.dependencies]`:

```toml
[workspace.dependencies]
soroban-sdk = "23.1.0"
stellar-access = { git = "https://github.com/OpenZeppelin/stellar-contracts", tag = "v0.5.1", package = "stellar-access" }
stellar-macros = { git = "https://github.com/OpenZeppelin/stellar-contracts", tag = "v0.5.1", package = "stellar-macros" }
stellar-tokens = { git = "https://github.com/OpenZeppelin/stellar-contracts", tag = "v0.5.1", package = "stellar-tokens" }
```

**Ventajas:**
- Versiones consistentes en todos los contratos
- Actualización centralizada
- Menor duplicación de código

### Perfil de Compilación Release

```toml
[profile.release]
opt-level = "z"              # Optimización para tamaño mínimo
debug = false                # Sin información de debug
lto = true                   # Link-Time Optimization
debug-assertions = false     # Sin assertions de debug
codegen-units = 1           # Una sola unidad de código para mejor optimización
panic = "abort"             # Abortar en panic (reduce tamaño)
overflow-checks = true      # Verificar overflow (seguridad)
strip = true                # Eliminar símbolos de debug
```

**¿Por qué estas optimizaciones?**

En contratos WASM, el tamaño del binario afecta directamente el costo de despliegue y ejecución. Estas configuraciones minimizan el tamaño mientras mantienen la seguridad.

### Rust Toolchain Configuration

```toml
[toolchain]
channel = "1.89.0"
targets = ["wasm32v1-none"]
```

**Especificaciones:**
- **Rust 1.89.0**: Versión específica requerida para compatibilidad con Soroban SDK
- **wasm32v1-none**: Target para compilación WASM compatible con Soroban v1

---

## Librerías y Dependencias

### 1. soroban-sdk (v23.1.0)

**Descripción:**
El SDK oficial de Soroban proporciona todas las APIs necesarias para interactuar con el entorno de ejecución de Soroban.

**Componentes Principales:**

#### Env (Entorno de Ejecución)
- **Persistent Storage**: Almacenamiento persistente de datos
- **Temporary Storage**: Almacenamiento temporal (dura solo la transacción)
- **Events**: Sistema de eventos para logging
- **Crypto**: Funciones criptográficas
- **Ledger Info**: Información del ledger (tiempo, secuencia, etc.)

#### Tipos de Datos
- **Address**: Direcciones de contratos y cuentas
- **BytesN**: Arrays de bytes de tamaño fijo
- **Symbol**: Símbolos (strings cortos optimizados)
- **String**: Strings dinámicos
- **Vec**: Vectores dinámicos
- **Map**: Mapas clave-valor
- **I128, I256, U128, U256**: Enteros de precisión arbitraria

#### Macros
- **`#[contract]`**: Marca una estructura como contrato
- **`#[contractimpl]`**: Implementa funciones del contrato
- **`#[contracterror]`**: Define errores personalizados

**Ejemplo de Uso:**

```rust
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn store(env: Env, key: Symbol, value: Address) {
        env.storage().persistent().set(&key, &value);
    }
    
    pub fn retrieve(env: Env, key: Symbol) -> Address {
        env.storage().persistent().get(&key).unwrap()
    }
}
```

**Características Especiales:**
- **Testutils**: Feature para testing que proporciona `Env::default()` y funciones de mock
- **Determinismo**: Todas las operaciones son deterministas
- **Gas Accounting**: Todas las operaciones tienen costo de gas asociado

### 2. stellar-access (v0.5.1)

**Descripción:**
Librería de OpenZeppelin para control de acceso en contratos Stellar. Proporciona patrones comunes de autorización.

**Componentes:**

#### Ownable
Patrón de propiedad donde un contrato tiene un único dueño.

```rust
use stellar_access::Ownable;

#[contract]
pub struct OwnableContract {
    ownable: Ownable,
}

#[contractimpl]
impl OwnableContract {
    pub fn only_owner(env: Env) {
        ownable.require_owner(&env);
        // Solo el dueño puede ejecutar esto
    }
}
```

#### Roles
Sistema de roles para control de acceso granular.

**Casos de Uso:**
- Administradores
- Operadores
- Roles personalizados

**Ventajas:**
- Reutilización de código probado
- Seguridad auditada
- Patrones estándar de la industria

### 3. stellar-macros (v0.5.1)

**Descripción:**
Macros procedimentales que reducen boilerplate en contratos Stellar.

**Macros Disponibles:**
- Macros para eventos
- Macros para storage
- Macros para validación

**Ejemplo:**

```rust
use stellar_macros::*;

// Simplifica la definición de eventos y storage
```

**Beneficios:**
- Menos código repetitivo
- Menor probabilidad de errores
- Código más legible

### 4. stellar-tokens (v0.5.1)

**Descripción:**
Implementaciones estándar de tokens para Stellar, similar a ERC-20 y ERC-721 de Ethereum.

**Tipos de Tokens:**

#### Fungible Tokens (ERC-20-like)
- Transferencia de tokens
- Aprobaciones y allowances
- Minting y burning
- Allowlists

#### Non-Fungible Tokens (ERC-721-like)
- Tokenización de activos únicos
- Enumeración de tokens
- Metadata

**Ejemplo de Uso:**

```rust
use stellar_tokens::FungibleToken;

#[contract]
pub struct MyToken {
    token: FungibleToken,
}

#[contractimpl]
impl MyToken {
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        self.token.transfer(&env, &from, &to, amount);
    }
}
```

**Características:**
- Implementación completa y probada
- Compatible con estándares
- Optimizado para gas

---

## Guía de Desarrollo

### Crear un Nuevo Contrato

#### Paso 1: Crear la Estructura

```bash
cd /home/carlos/dispensario_digital_sc/contracts
mkdir mi-nuevo-contrato
cd mi-nuevo-contrato
mkdir src
```

#### Paso 2: Crear Cargo.toml

```toml
[package]
name = "mi-nuevo-contrato"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }

[package.metadata.stellar]
cargo_inherit = true
```

**Explicación:**
- **`crate-type = ["cdylib"]`**: Necesario para compilar como librería dinámica (WASM)
- **`cargo_inherit = true`**: Hereda configuración del workspace

#### Paso 3: Crear lib.rs

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MiContrato;

#[contractimpl]
impl MiContrato {
    pub fn saludar(env: Env) -> String {
        String::from_str(&env, "Hola desde Soroban!")
    }
}

#[cfg(test)]
mod test;
```

**Puntos Importantes:**
- **`#![no_std]`**: Requerido - Soroban no usa stdlib de Rust
- **`#[contract]`**: Marca la estructura como contrato
- **`#[contractimpl]`**: Implementa las funciones públicas del contrato

#### Paso 4: Agregar al Workspace

Editar `contracts/Cargo.toml`:

```toml
[workspace]
members = ["hello-world", "mi-nuevo-contrato"]
```

### Estructura de un Contrato

#### Componentes Básicos

1. **Imports**: Importar tipos y funciones del SDK
2. **Struct del Contrato**: Estructura marcada con `#[contract]`
3. **Implementación**: Funciones marcadas con `#[contractimpl]`
4. **Tests**: Módulo de tests (opcional pero recomendado)

#### Ejemplo Completo

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Symbol, String, Vec,
    Address,
    storage::Storage,
};

#[contract]
pub struct ContratoEjemplo {
    // Storage puede ser definido aquí si se usa
}

#[contractimpl]
impl ContratoEjemplo {
    // Función pública simple
    pub fn obtener_mensaje(env: Env) -> String {
        String::from_str(&env, "Mensaje desde el contrato")
    }
    
    // Función con parámetros
    pub fn saludar(env: Env, nombre: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hola"), nombre]
    }
    
    // Función con storage
    pub fn guardar_valor(env: Env, key: Symbol, value: i128) {
        env.storage().persistent().set(&key, &value);
    }
    
    pub fn obtener_valor(env: Env, key: Symbol) -> Option<i128> {
        env.storage().persistent().get(&key)
    }
    
    // Función con autenticación
    pub fn solo_admin(env: Env, admin: Address) {
        // Verificar que el invocador es el admin
        admin.require_auth();
        // Lógica del admin
    }
}
```

### Storage en Soroban

#### Tipos de Storage

1. **Persistent Storage**: Persiste entre transacciones
2. **Temporary Storage**: Solo dura la transacción actual
3. **Instance Storage**: Almacenamiento a nivel de instancia del contrato

#### Ejemplos de Storage

```rust
// Persistent Storage
env.storage().persistent().set(&key, &value);
let value: Option<Type> = env.storage().persistent().get(&key);
env.storage().persistent().remove(&key);

// Temporary Storage
env.storage().temporary().set(&key, &value);
let value: Option<Type> = env.storage().temporary().get(&key);

// Instance Storage
env.storage().instance().set(&key, &value);
let value: Option<Type> = env.storage().instance().get(&key);
```

**Consideraciones:**
- Persistent storage tiene mayor costo de gas
- Temporary storage es más barato pero se pierde
- Instance storage es para datos de configuración del contrato

### Eventos y Logging

```rust
use soroban_sdk::{contract, contractimpl, Env, symbol_short, log};

#[contract]
pub struct ContratoConEventos;

#[contractimpl]
impl ContratoConEventos {
    pub fn emitir_evento(env: Env, mensaje: Symbol) {
        log!(&env, "Evento emitido: {}", mensaje);
        
        // Eventos estructurados
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("from")),
            address_from,
        );
    }
}
```

### Manejo de Errores

```rust
use soroban_sdk::{contracterror, contract, contractimpl, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    DivisionPorCero = 1,
    ValorInvalido = 2,
}

#[contract]
pub struct ContratoConErrores;

#[contractimpl]
impl ContratoConErrores {
    pub fn dividir(env: Env, a: i128, b: i128) -> Result<i128, Error> {
        if b == 0 {
            return Err(Error::DivisionPorCero);
        }
        Ok(a / b)
    }
}
```

---

## Compilación y Testing

### Compilar un Contrato

#### Compilación Individual

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release --package nombre-del-contrato
```

**Resultado:**
- Binario WASM en `target/wasm32v1-none/release/nombre-del-contrato.wasm`

#### Compilar Todos los Contratos

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release
```

### Ejecutar Tests

#### Tests de un Contrato Específico

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo test --package nombre-del-contrato
```

#### Todos los Tests

```bash
cargo test
```

#### Tests con Output Detallado

```bash
cargo test --package nombre-del-contrato -- --nocapture
```

### Estructura de Tests

```rust
use soroban_sdk::{symbol_short, vec, Env, Address};

use crate::{MiContrato, MiContratoClient};

#[test]
fn test_funcion_basica() {
    // Crear entorno de prueba
    let env = Env::default();
    
    // Registrar el contrato
    let contract_id = env.register(MiContrato, ());
    
    // Crear cliente para interactuar
    let client = MiContratoClient::new(&env, &contract_id);
    
    // Ejecutar función y verificar resultado
    let resultado = client.funcion_a_probar();
    assert_eq!(resultado, valor_esperado);
}

#[test]
fn test_con_storage() {
    let env = Env::default();
    let contract_id = env.register(MiContrato, ());
    let client = MiContratoClient::new(&env, &contract_id);
    
    // Guardar valor
    client.guardar_valor(&symbol_short!("key"), &100);
    
    // Recuperar y verificar
    let valor = client.obtener_valor(&symbol_short!("key"));
    assert_eq!(valor, Some(100));
}

#[test]
#[should_panic(expected = "Error esperado")]
fn test_manejo_de_errores() {
    let env = Env::default();
    let contract_id = env.register(MiContrato, ());
    let client = MiContratoClient::new(&env, &contract_id);
    
    // Esto debería fallar
    client.funcion_que_falla();
}
```

### Verificar el Binario WASM

```bash
# Ver tamaño del binario
ls -lh target/wasm32v1-none/release/*.wasm

# Ver información del WASM
wasm-objdump -h target/wasm32v1-none/release/contrato.wasm
```

---

## Despliegue y Pruebas en Red Local

### Configuración de Red Local

Para probar contratos en una red local de Stellar, necesitas ejecutar Stellar Quickstart, que es un contenedor Docker que proporciona una red Stellar completa y funcional en tu máquina local.

#### Requisitos Previos

- **Docker**: Debe estar instalado y corriendo
- **Stellar CLI**: Herramienta de línea de comandos de Stellar

#### Iniciar la Red Local

**Opción 1: Usando Stellar Quickstart (Recomendado)**

```bash
# Iniciar el contenedor de Stellar Quickstart
docker run --rm -it \
  -p 8000:8000 \
  -p 8001:8001 \
  --name stellar \
  stellar/quickstart:testing \
  --standalone \
  --enable-soroban-rpc

# O en modo detached (en segundo plano)
docker run -d \
  -p 8000:8000 \
  -p 8001:8001 \
  --name stellar \
  stellar/quickstart:testing \
  --standalone \
  --enable-soroban-rpc
```

**Puertos:**
- **8000**: Horizon API (HTTP)
- **8001**: Soroban RPC (HTTP)

**Verificar que está corriendo:**

```bash
# Verificar contenedor
docker ps | grep stellar

# Verificar que responde
curl http://localhost:8000
curl http://localhost:8001
```

**Opción 2: Usando Stellar CLI**

Si tienes `stellar-cli` configurado, puedes usar:

```bash
# Configurar red local
stellar network use local

# Esto automáticamente iniciará un contenedor Docker si no está corriendo
```

#### Detener la Red Local

```bash
# Detener el contenedor
docker stop stellar

# Eliminar el contenedor (opcional)
docker rm stellar
```

### Configurar Stellar CLI para Red Local

#### Configurar la Red

```bash
# Usar red local
stellar network use local

# Verificar configuración actual
stellar network show
```

#### Crear Identidades (Cuentas)

```bash
# Generar una nueva identidad
stellar keys generate alice

# Ver la clave pública
stellar keys address alice

# Ver la clave secreta (¡manejar con cuidado!)
stellar keys secret alice
```

#### Fondear Cuentas en Red Local

En red local, puedes fondear cuentas usando friendbot:

```bash
# Fondear una cuenta usando friendbot
curl "http://localhost:8000/friendbot?addr=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

# O usando la CLI (si está configurado)
stellar keys fund alice
```

**Nota:** En red local, friendbot está disponible en `http://localhost:8000/friendbot`. Solo necesitas pasar la dirección pública de la cuenta como parámetro `addr`.

### Desplegar un Contrato en Red Local

#### Paso 1: Compilar el Contrato

```bash
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release --package hello-world
```

#### Paso 2: Desplegar el Contrato

```bash
# Asegúrate de estar usando la red local
stellar network use local

# Usar una identidad con fondos
stellar keys use alice

# Desplegar el contrato
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --alias hello-world
```

**Salida esperada:**
```
Contract deployed with ID: CBB65ZLBQBZL5IYHDHEEPCVUUMFOQUZSQKAJFV36R7TZETCLWGFTRLOQ
```

#### Paso 3: Verificar el Despliegue

```bash
# Ver información del contrato
stellar contract read --id hello-world

# O usando el ID directamente
stellar contract read --id CBB65ZLBQBZL5IYHDHEEPCVUUMFOQUZSQKAJFV36R7TZETCLWGFTRLOQ
```

### Invocar Funciones del Contrato

#### Invocar una Función Simple

```bash
# Invocar la función hello con un parámetro
stellar contract invoke \
  --id hello-world \
  -- hello \
  --to World
```

**Ejemplo con múltiples parámetros:**

```bash
stellar contract invoke \
  --id hello-world \
  -- funcion_con_parametros \
  --param1 valor1 \
  --param2 valor2
```

#### Invocar con Autenticación

Si la función requiere autenticación:

```bash
# Usar una identidad específica
stellar keys use alice

# Invocar la función
stellar contract invoke \
  --id hello-world \
  -- funcion_autenticada \
  --admin alice
```

### Leer Storage del Contrato

#### Leer Storage Persistente

```bash
# Leer un valor del storage persistente
stellar contract read \
  --id hello-world \
  --durability persistent \
  --key COUNTER
```

#### Leer Storage Temporal

```bash
# Leer un valor del storage temporal
stellar contract read \
  --id hello-world \
  --durability temporary \
  --key TEMP_KEY
```

#### Leer Storage de Instancia

```bash
# Leer un valor del storage de instancia
stellar contract read \
  --id hello-world \
  --durability instance \
  --key ADMIN
```

### Flujo Completo de Pruebas en Red Local

#### Ejemplo: Contrato Completo

```bash
# 1. Iniciar red local (en otra terminal)
docker run -d -p 8000:8000 -p 8001:8001 --name stellar \
  stellar/quickstart:testing --standalone --enable-soroban-rpc

# 2. Configurar red local
stellar network use local

# 3. Crear y fondear cuenta
stellar keys generate alice
curl "http://localhost:8000/friendbot?addr=$(stellar keys address alice)"
stellar keys use alice

# 4. Compilar contrato
cd /home/carlos/dispensario_digital_sc/contracts
cargo build --target wasm32v1-none --release --package hello-world

# 5. Desplegar contrato
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --alias hello-world

# 6. Invocar función
stellar contract invoke --id hello-world -- hello --to Developer

# 7. Leer storage (si aplica)
stellar contract read --id hello-world --durability persistent --key COUNTER

# 8. Limpiar (opcional)
docker stop stellar
docker rm stellar
```

### Verificar Estado de la Red

#### Ver Información del Ledger

```bash
# Ver información del ledger actual
curl http://localhost:8000/ledgers?order=desc&limit=1

# Ver información de una cuenta
curl http://localhost:8000/accounts/GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

#### Ver Transacciones

```bash
# Ver transacciones recientes
curl http://localhost:8000/transactions?order=desc&limit=10
```

### Solución de Problemas en Red Local

#### Error: "Network connection failed"

**Causa:** El contenedor Docker no está corriendo.

**Solución:**
```bash
# Verificar que el contenedor está corriendo
docker ps | grep stellar

# Si no está, iniciarlo
docker start stellar

# O crear uno nuevo
docker run -d -p 8000:8000 -p 8001:8001 --name stellar \
  stellar/quickstart:testing --standalone --enable-soroban-rpc
```

#### Error: "Account not found"

**Causa:** La cuenta no tiene fondos o no existe.

**Solución:**
```bash
# Fondear la cuenta usando friendbot
curl "http://localhost:8000/friendbot?addr=$(stellar keys address alice)"

# Verificar balance
stellar keys balance alice
```

#### Error: "Insufficient balance"

**Causa:** La cuenta no tiene suficientes XLM para pagar las fees.

**Solución:**
```bash
# Fondear más la cuenta
curl "http://localhost:8000/friendbot?addr=$(stellar keys address alice)"
```

#### Error: "Contract not found"

**Causa:** El contrato no está desplegado o el ID es incorrecto.

**Solución:**
```bash
# Verificar que el contrato está desplegado
stellar contract read --id hello-world

# Si no existe, desplegarlo de nuevo
stellar contract deploy --wasm path/to/contract.wasm --alias hello-world
```

### Ventajas de Usar Red Local

1. **Gratis**: No hay costos de transacción
2. **Rápido**: Transacciones instantáneas
3. **Control Total**: Puedes resetear el estado cuando quieras
4. **Privado**: Todo queda en tu máquina
5. **Ideal para Desarrollo**: Pruebas sin riesgo

### Comparación: Red Local vs Testnet

| Característica | Red Local | Testnet |
|----------------|-----------|---------|
| Costo | Gratis | Gratis (pero requiere fondos) |
| Velocidad | Instantánea | ~5 segundos |
| Persistencia | Se pierde al detener | Persistente |
| Friendbot | Manual (curl) | Automático (`stellar keys fund`) |
| Uso | Desarrollo/Testing | Testing más realista |

---

## Ejemplos de Código

### Ejemplo 1: Contrato Simple con Storage

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Symbol, i128,
};

#[contract]
pub struct Contador;

#[contractimpl]
impl Contador {
    pub fn incrementar(env: Env) -> i128 {
        let key = symbol_short!("contador");
        let mut valor: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);
        valor += 1;
        env.storage().persistent().set(&key, &valor);
        valor
    }
    
    pub fn obtener(env: Env) -> i128 {
        let key = symbol_short!("contador");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(0)
    }
    
    pub fn resetear(env: Env) {
        let key = symbol_short!("contador");
        env.storage().persistent().remove(&key);
    }
}
```

### Ejemplo 2: Contrato con Autenticación

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Address, Symbol,
};

#[contract]
pub struct ContratoSeguro {
    // El admin se almacena en instance storage
}

#[contractimpl]
impl ContratoSeguro {
    pub fn inicializar(env: Env, admin: Address) {
        let key = symbol_short!("admin");
        env.storage().instance().set(&key, &admin);
    }
    
    pub fn solo_admin(env: Env) {
        let key = symbol_short!("admin");
        let admin: Address = env.storage().instance().get(&key).unwrap();
        admin.require_auth();
        // Solo el admin puede ejecutar esto
    }
    
    pub fn funcion_publica(env: Env) -> Symbol {
        symbol_short!("publico")
    }
}
```

### Ejemplo 3: Contrato con Eventos

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    symbol_short,
    Env, Address, Symbol,
    log,
};

#[contract]
pub struct ContratoConEventos;

#[contractimpl]
impl ContratoConEventos {
    pub fn transferir(env: Env, from: Address, to: Address, cantidad: i128) {
        from.require_auth();
        
        // Lógica de transferencia...
        
        // Emitir evento
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("from")),
            from.clone(),
        );
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("to")),
            to,
        );
        env.events().publish(
            (symbol_short!("transfer"), symbol_short!("amount")),
            cantidad,
        );
        
        // Log simple
        log!(&env, "Transferencia completada: {} unidades", cantidad);
    }
}
```

### Ejemplo 4: Contrato con Manejo de Errores

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror,
    Env, Address, i128,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    SaldoInsuficiente = 1,
    CantidadInvalida = 2,
    NoAutorizado = 3,
}

#[contract]
pub struct Banco;

#[contractimpl]
impl Banco {
    pub fn retirar(env: Env, usuario: Address, cantidad: i128) -> Result<i128, Error> {
        usuario.require_auth();
        
        if cantidad <= 0 {
            return Err(Error::CantidadInvalida);
        }
        
        let saldo_key = symbol_short!("saldo");
        let saldo_actual: i128 = env
            .storage()
            .persistent()
            .get(&saldo_key)
            .unwrap_or(0);
        
        if saldo_actual < cantidad {
            return Err(Error::SaldoInsuficiente);
        }
        
        let nuevo_saldo = saldo_actual - cantidad;
        env.storage().persistent().set(&saldo_key, &nuevo_saldo);
        
        Ok(nuevo_saldo)
    }
}
```

---

## Mejores Prácticas

### 1. Seguridad

#### Validación de Inputs
```rust
pub fn funcion_segura(env: Env, valor: i128) {
    // SIEMPRE validar inputs
    if valor < 0 {
        panic!("Valor negativo no permitido");
    }
    // ...
}
```

#### Autenticación
```rust
pub fn funcion_privada(env: Env, admin: Address) {
    // Verificar autenticación
    admin.require_auth();
    // ...
}
```

#### Prevención de Reentrancy
- Usar checks-effects-interactions pattern
- Considerar locks cuando sea necesario

### 2. Optimización de Gas

#### Usar Storage Eficientemente
- Preferir temporary storage cuando sea posible
- Agrupar datos relacionados
- Usar tipos compactos (Symbol vs String)

#### Minimizar Cálculos
- Cachear valores calculados
- Evitar loops innecesarios
- Usar operaciones nativas cuando sea posible

### 3. Código Limpio

#### Nombres Descriptivos
```rust
// Malo
pub fn fn1(env: Env, a: i128, b: i128) -> i128

// Bueno
pub fn calcular_balance_total(env: Env, cuenta: Address, periodo: i128) -> i128
```

#### Comentarios Útiles
```rust
/// Calcula el balance total de una cuenta para un período específico.
/// 
/// # Argumentos
/// * `cuenta` - Dirección de la cuenta
/// * `periodo` - Período de tiempo en días
/// 
/// # Retorna
/// El balance total como i128
pub fn calcular_balance_total(env: Env, cuenta: Address, periodo: i128) -> i128 {
    // ...
}
```

#### Modularización
- Separar lógica compleja en funciones privadas
- Usar módulos para organizar código grande

### 4. Testing

#### Cobertura Completa
- Tests para casos felices
- Tests para casos de error
- Tests de edge cases
- Tests de integración

#### Tests Legibles
```rust
#[test]
fn test_retiro_exitoso_cuando_hay_saldo_suficiente() {
    // Arrange
    let env = Env::default();
    let contract_id = env.register(Banco, ());
    let client = BancoClient::new(&env, &contract_id);
    
    // Act
    let resultado = client.retirar(&usuario, &100);
    
    // Assert
    assert_eq!(resultado, Ok(900));
}
```

### 5. Versionado

- Usar versiones semánticas en Cargo.toml
- Documentar cambios breaking
- Mantener changelog

---

## Solución de Problemas

### Error: "target wasm32v1-none not found"

**Solución:**
```bash
rustup target add wasm32v1-none
```

### Error: "failed to load manifest for workspace member"

**Causa:** El workspace está intentando incluir directorios que no son contratos.

**Solución:** Verificar que `exclude = ["target"]` esté en `Cargo.toml` y que solo se listen contratos válidos en `members`.

### Error: "use of undeclared crate or module `std`"

**Causa:** Falta `#![no_std]` al inicio del archivo.

**Solución:** Agregar `#![no_std]` como primera línea del archivo.

### Error de Compilación: "cannot find type X in this scope"

**Causa:** Falta importar el tipo del SDK.

**Solución:** Verificar imports:
```rust
use soroban_sdk::{Env, Address, Symbol, /* otros tipos necesarios */};
```

### Binario WASM Muy Grande

**Soluciones:**
1. Verificar que se compile en modo `--release`
2. Revisar que el perfil `release` tenga `opt-level = "z"`
3. Eliminar código no usado
4. Usar tipos más compactos (Symbol en lugar de String cuando sea posible)

### Tests No Compilan

**Causa Común:** Falta feature `testutils` en dev-dependencies.

**Solución:**
```toml
[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

### Error: "method `register_contract` is deprecated"

**Solución:** Usar `env.register(Contract, ())` en lugar de `env.register_contract(None, Contract)`.

---

## Referencias y Recursos

### Documentación Oficial

- **Soroban Documentation**: https://soroban.stellar.org/docs
- **Soroban SDK Reference**: https://docs.rs/soroban-sdk/
- **Stellar Documentation**: https://developers.stellar.org/

### Repositorios

- **Soroban SDK**: https://github.com/stellar/rs-soroban-sdk
- **OpenZeppelin Stellar Contracts**: https://github.com/OpenZeppelin/stellar-contracts
- **Stellar Examples**: https://github.com/stellar/soroban-examples

### Herramientas

- **Stellar CLI**: https://github.com/stellar/stellar-cli
- **Soroban Tools**: https://github.com/stellar/soroban-tools

### Comunidad

- **Stellar Discord**: https://discord.gg/stellar
- **Stellar Stack Exchange**: https://stellar.stackexchange.com/

### Tutoriales

- **Soroban Book**: https://soroban.stellar.org/docs/category/getting-started
- **Smart Contract Development Guide**: https://soroban.stellar.org/docs/category/smart-contracts

### Especificaciones

- **WASM Specification**: https://webassembly.org/spec/
- **Stellar Protocol**: https://developers.stellar.org/docs/encyclopedia/protocol

---

## Conclusión

Este workspace proporciona una base sólida para el desarrollo de contratos inteligentes en Soroban. Con las herramientas y configuraciones adecuadas, puedes desarrollar, probar y desplegar contratos de manera eficiente y segura.

Para preguntas o problemas, consulta la documentación oficial o la comunidad de Stellar.

---

**Última actualización:** Noviembre 2024  
**Versión del SDK:** 23.1.0  
**Versión de Rust:** 1.89.0

