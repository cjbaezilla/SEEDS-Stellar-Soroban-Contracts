# Documentación Técnica: Sistema de Trazabilidad de Semillas de Cannabis

## Tabla de Contenidos

1. [Introducción](#introducción)
2. [Arquitectura del Sistema](#arquitectura-del-sistema)
3. [Contrato de Registro (Seed Registry)](#contrato-de-registro-seed-registry)
4. [Contrato NFT de Semilla (Seed NFT)](#contrato-nft-de-semilla-seed-nft)
5. [Integración entre Contratos](#integración-entre-contratos)
6. [Librerías y Dependencias](#librerías-y-dependencias)
7. [Sistema de Roles y Permisos](#sistema-de-roles-y-permisos)
8. [Ciclo de Vida de las Semillas](#ciclo-de-vida-de-las-semillas)
9. [Tests y Validación](#tests-y-validación)
10. [Flujos de Trabajo](#flujos-de-trabajo)
11. [Eventos y Auditoría](#eventos-y-auditoría)
12. [Consideraciones de Seguridad](#consideraciones-de-seguridad)
13. [Ejemplos de Uso](#ejemplos-de-uso)

---

## Introducción

Este sistema de contratos inteligentes está diseñado para proporcionar trazabilidad completa de semillas de cannabis desde su creación hasta el consumidor final. El sistema está construido sobre la plataforma Soroban de Stellar y utiliza las librerías de OpenZeppelin para garantizar seguridad y cumplimiento de estándares.

### Objetivos del Sistema

- **Trazabilidad Completa**: Rastrear cada semilla desde su registro inicial hasta el consumo final
- **Transparencia**: Proporcionar información verificable sobre el origen, procesamiento y distribución
- **Cumplimiento Regulatorio**: Facilitar el cumplimiento de regulaciones para dispensarios y clubes cannábicos
- **Control de Acceso**: Implementar un sistema robusto de roles y permisos
- **Inmutabilidad**: Garantizar que los datos históricos no puedan ser modificados

### Casos de Uso

- Dispensarios legales que necesitan demostrar la procedencia de sus productos
- Clubes cannábicos que requieren trazabilidad para cumplir con regulaciones
- Reguladores que necesitan auditar la cadena de suministro
- Consumidores que desean verificar la calidad y origen de los productos

---

## Arquitectura del Sistema

El sistema está compuesto por dos contratos inteligentes principales que trabajan en conjunto:

```
┌─────────────────────────────────────────────────────────────┐
│                    Sistema de Trazabilidad                   │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                │                           │
    ┌───────────▼──────────┐    ┌──────────▼──────────┐
    │  Seed Registry       │    │   Seed NFT          │
    │  (Registro)          │    │   (Tokenización)    │
    │                      │    │                     │
    │  - Almacena datos    │    │  - Representa cada  │
    │  - Crea NFTs         │    │    semilla como NFT │
    │  - Consultas         │    │  - Gestiona estados│
    │  - Batch operations  │    │  - Historial        │
    └──────────────────────┘    └─────────────────────┘
```

### Flujo de Datos

1. **Registro Inicial**: El contrato de Registro almacena la información de la semilla y crea automáticamente un NFT
2. **Tokenización**: Cada semilla se representa como un NFT único en el contrato Seed NFT
3. **Actualización de Estados**: A lo largo del ciclo de vida, el NFT se actualiza con nuevos estados y metadata
4. **Transferencias**: Los NFTs solo pueden transferirse entre direcciones autorizadas (whitelist)
5. **Consulta**: Ambos contratos permiten consultar información histórica y actual

---

## Contrato de Registro (Seed Registry)

### Descripción General

El contrato `seed-registry` es el punto central de entrada para todas las semillas. Actúa como un registro maestro que almacena información completa sobre cada semilla y coordina la creación de NFTs correspondientes.

### Estructura del Contrato

```
seed-registry/
├── src/
│   ├── lib.rs          # Contrato principal y funciones públicas
│   ├── registry.rs     # Lógica interna del registro
│   ├── seed_data.rs    # Estructura de datos SeedData
│   ├── roles.rs        # Definición de roles
│   └── test.rs         # Tests unitarios
```

### Estructura de Datos: SeedData

Cada semilla registrada contiene la siguiente información:

```rust
pub struct SeedData {
    pub id: String,                    // ID único de la semilla
    pub created_at: u64,               // Timestamp de creación
    pub creator: Address,               // Dirección que creó el registro
    pub variety: String,                // Variedad/genética (ej: "Indica", "Sativa")
    pub batch: String,                  // Número de lote
    pub origin_country: String,          // País de origen
    pub seed_bank: String,               // Banco de semillas
    pub expected_thc: Option<u32>,      // Porcentaje esperado de THC
    pub expected_cbd: Option<u32>,      // Porcentaje esperado de CBD
    pub organic_certified: bool,         // Certificación orgánica
    pub nft_id: u128,                   // ID del NFT asociado
    pub nft_contract: Address,          // Dirección del contrato NFT
}
```

### Funciones Principales

#### `initialize(env: Env, admin: Address, nft_contract: Address)`

Inicializa el contrato de registro. Solo puede ejecutarse una vez.

**Parámetros:**
- `admin`: Dirección que será asignada como administrador
- `nft_contract`: Dirección del contrato NFT que se utilizará para crear tokens

**Efectos:**
- Establece el contador de semillas en 0
- Almacena la referencia al contrato NFT
- Asigna el rol de ADMIN al administrador
- Inicializa el estado de pausa como `false`

**Ejemplo:**
```rust
let admin = Address::generate(&env);
let nft_contract = Address::generate(&env);
client.initialize(&admin, &nft_contract);
```

#### `register_seed(...) -> u128`

Registra una nueva semilla en el sistema y crea automáticamente su NFT correspondiente.

**Parámetros:**
- `seed_id`: Identificador único de la semilla
- `variety`: Variedad/genética
- `batch`: Número de lote
- `origin_country`: País de origen
- `seed_bank`: Banco de semillas
- `expected_thc`: Porcentaje esperado de THC (opcional)
- `expected_cbd`: Porcentaje esperado de CBD (opcional)
- `organic_certified`: Si tiene certificación orgánica

**Requisitos:**
- El contrato no debe estar pausado
- El invocador debe tener el rol CULTIVATOR
- La semilla no debe existir previamente

**Retorna:**
- `u128`: ID del NFT creado

**Eventos Emitidos:**
- `SeedReg` con campos: `seed_id`, `nft_id`, `creator`, `created`

**Ejemplo:**
```rust
let nft_id = client.register_seed(
    &seed_id,
    &String::from_str(&env, "Indica"),
    &String::from_str(&env, "BATCH-2024-001"),
    &String::from_str(&env, "Colombia"),
    &String::from_str(&env, "Bank-001"),
    Some(20u32),  // 20% THC esperado
    Some(2u32),   // 2% CBD esperado
    true          // Certificado orgánico
);
```

#### `register_seeds_batch(...) -> Vec<u128>`

Registra múltiples semillas en una sola transacción. Útil para registrar lotes completos.

**Parámetros:**
- Vectores de igual longitud con la información de cada semilla

**Límites:**
- Máximo 100 semillas por lote

**Ventajas:**
- Reduce el costo de gas
- Permite registro atómico de lotes completos
- Si una semilla ya existe, se omite sin fallar toda la operación

**Ejemplo:**
```rust
let seed_ids = vec![&env, id1, id2, id3];
let varieties = vec![&env, var1, var2, var3];
// ... otros vectores
let nft_ids = client.register_seeds_batch(
    &seed_ids, &varieties, &batches, 
    &origin_countries, &seed_banks,
    &expected_thcs, &expected_cbds, 
    &organic_flags
);
```

#### Funciones de Consulta

##### `get_seed(env: Env, seed_id: String) -> Option<SeedData>`

Obtiene toda la información de una semilla específica.

##### `get_seed_count(env: Env) -> u64`

Retorna el número total de semillas registradas.

##### `get_all_seed_ids(env: Env) -> Vec<String>`

Retorna una lista de todos los IDs de semillas registradas.

##### `query_seeds_by_variety(env: Env, variety: String) -> Vec<String>`

Busca todas las semillas de una variedad específica.

**Ejemplo:**
```rust
let indica_seeds = client.query_seeds_by_variety(
    &String::from_str(&env, "Indica")
);
```

##### `query_seeds_by_batch(env: Env, batch: String) -> Vec<String>`

Busca todas las semillas de un lote específico.

##### `query_seeds_by_creator(env: Env, creator: Address) -> Vec<String>`

Busca todas las semillas creadas por una dirección específica.

### Storage

El contrato utiliza dos tipos de storage:

#### Instance Storage
- `SEED_CNT`: Contador total de semillas (u64)
- `NFT_CNTRCT`: Dirección del contrato NFT (Address)
- `PAUSED`: Estado de pausa (bool)

#### Persistent Storage
- `(SEED_DATA, seed_id)`: Datos completos de cada semilla (SeedData)
- `(SEED_IDS)`: Lista de todos los IDs de semillas (Vec<String>)
- `(ROLE_KEY, address)`: Mapeo de roles por dirección (bool)

---

## Contrato NFT de Semilla (Seed NFT)

### Descripción General

El contrato `seed-nft` implementa un sistema de tokens no fungibles (NFTs) basado en el estándar ERC-721-like de OpenZeppelin Stellar. Cada NFT representa una semilla única y almacena su estado actual, metadata completa e historial de transiciones.

### Estructura del Contrato

```
seed-nft/
├── src/
│   ├── lib.rs          # Contrato principal
│   ├── nft.rs          # Funciones auxiliares del NFT
│   ├── lifecycle.rs    # Estados del ciclo de vida
│   ├── metadata.rs     # Estructura de metadata
│   ├── history.rs      # Gestión del historial
│   ├── roles.rs        # Definición de roles
│   └── test.rs         # Tests unitarios
```

### Estados del Ciclo de Vida

El sistema define 8 estados principales que representan el ciclo completo de una semilla:

```rust
pub enum LifecycleState {
    Seed = 0,              // Estado inicial: semilla registrada
    Germinated = 1,        // Semilla germinada
    PlantVegetative = 2,   // Planta en fase vegetativa
    PlantFlowering = 3,    // Planta en fase de floración
    PlantHarvested = 4,    // Planta cosechada
    Processed = 5,         // Producto procesado
    Distributed = 6,       // En dispensario/distribución
    Consumed = 7,          // Consumido por el usuario final
}
```

### Reglas de Transición

Las transiciones de estado están estrictamente validadas:

1. **Secuencialidad**: Solo se permiten transiciones al siguiente estado
2. **Sin Retroceso**: No se puede volver a un estado anterior
3. **Sin Saltos**: No se puede saltar estados intermedios
4. **Permisos por Estado**: Cada transición requiere un rol específico

**Matriz de Transiciones Válidas:**
```
Seed → Germinated → PlantVegetative → PlantFlowering 
→ PlantHarvested → Processed → Distributed → Consumed
```

### Estructura de Metadata

Cada NFT almacena metadata completa, incluyendo campos compatibles con el estándar OpenSea:

```rust
pub struct Attribute {
    pub trait_type: String,              // Tipo de atributo (ej: "Variedad", "THC")
    pub value: String,                   // Valor del atributo (ej: "Indica", "20%")
}

pub struct OpenSeaMetadata {
    pub name: Option<String>,            // Nombre del NFT
    pub description: Option<String>,      // Descripción del NFT
    pub image: Option<String>,            // URL de la imagen
    pub external_url: Option<String>,    // URL externa
    pub attributes: Option<Vec<Attribute>>, // Lista de atributos
}

pub struct SeedMetadata {
    pub state: LifecycleState,           // Estado actual
    pub location: Option<String>,         // Ubicación GPS o descripción
    pub temperature: Option<i32>,       // Temperatura en grados Celsius
    pub humidity: Option<u32>,           // Humedad relativa (%)
    pub lab_analysis: Option<String>,    // Análisis de laboratorio (hash o referencia)
    pub processor: Option<Address>,      // Dirección del procesador
    pub distributor: Option<Address>,    // Dirección del distribuidor
    pub consumer: Option<Address>,        // Dirección del consumidor final
    pub updated_at: u64,                 // Timestamp de última actualización
    // Campos compatibles con OpenSea
    pub name: String,                    // Nombre del NFT (requerido por OpenSea)
    pub description: String,             // Descripción del NFT (requerido por OpenSea)
    pub image: String,                   // URL de la imagen (requerido por OpenSea)
    pub external_url: Option<String>,    // URL externa (opcional)
    pub attributes: Vec<Attribute>,       // Lista de atributos/traits (opcional)
}
```

### Funciones Principales

#### `initialize(env: Env, admin: Address, name: String, symbol: String)`

Inicializa el contrato NFT. Configura el token base y asigna el rol de administrador.

**Parámetros:**
- `admin`: Dirección del administrador
- `name`: Nombre del token (ej: "Cannabis Seed NFT")
- `symbol`: Símbolo del token (ej: "CSNFT")

#### `mint(env: Env, to: Address, token_id: u128, name: String, description: String, image: String, external_url: Option<String>, attributes: Vec<Attribute>)`

Crea un nuevo NFT. Esta función es llamada automáticamente por el contrato de Registro.

**Parámetros:**
- `to`: Dirección que recibirá el NFT
- `token_id`: ID único del token
- `name`: Nombre del NFT (requerido por OpenSea)
- `description`: Descripción del NFT (requerido por OpenSea)
- `image`: URL de la imagen del NFT (requerido por OpenSea)
- `external_url`: URL externa opcional (puede ser `None`)
- `attributes`: Lista de atributos/traits del NFT (puede ser un vector vacío)

**Efectos:**
- Crea el NFT con el estado inicial `Seed`
- Inicializa la metadata básica incluyendo los campos OpenSea
- Emite evento `Mint`

**Ejemplo:**
```rust
let name = String::from_str(&env, "Semilla Cannabis Indica #001");
let description = String::from_str(&env, "Semilla certificada de variedad Indica");
let image = String::from_str(&env, "https://example.com/images/seed-001.png");
let external_url = Some(String::from_str(&env, "https://example.com/seeds/001"));
let mut attributes = Vec::new(&env);
attributes.push_back(&Attribute {
    trait_type: String::from_str(&env, "Variedad"),
    value: String::from_str(&env, "Indica"),
});
attributes.push_back(&Attribute {
    trait_type: String::from_str(&env, "THC Esperado"),
    value: String::from_str(&env, "20%"),
});

client.mint(&owner, &token_id, &name, &description, &image, &external_url, &attributes);
```

#### `update_state(...)`

Actualiza el estado del ciclo de vida del NFT. Esta es una de las funciones más importantes del sistema.

**Parámetros:**
- `caller`: Dirección que realiza la actualización (debe autenticarse)
- `token_id`: ID del NFT a actualizar
- `new_state`: Nuevo estado (u32)
- `location`: Ubicación opcional
- `temperature`: Temperatura opcional
- `humidity`: Humedad opcional
- `notes`: Notas adicionales opcionales

**Validaciones:**
1. El contrato no debe estar pausado
2. El caller debe autenticarse
3. El caller debe tener el rol apropiado para el nuevo estado:
   - `Germinated`, `PlantVegetative`, `PlantFlowering`, `PlantHarvested`: Requiere `CULTIVATOR`
   - `Processed`: Requiere `PROCESSOR`
   - `Distributed`: Requiere `DISPENSARY`
   - `Consumed`: Requiere `DISPENSARY`
4. La transición de estado debe ser válida según las reglas

**Efectos:**
- Actualiza el estado en la metadata
- Guarda la transición en el historial
- Actualiza campos específicos según el estado (processor, distributor, consumer)
- Emite evento `StateTrans`

**Ejemplo:**
```rust
client.update_state(
    &cultivator,
    &token_id,
    &(LifecycleState::Germinated as u32),
    &Some(String::from_str(&env, "40.7128,-74.0060")), // GPS
    &Some(25i32),  // 25°C
    &Some(60u32),  // 60% humedad
    &Some(String::from_str(&env, "Germinación exitosa"))
);
```

#### `update_metadata(...)`

Actualiza campos específicos de la metadata sin cambiar el estado. Permite actualizar tanto los campos de trazabilidad como los campos OpenSea.

**Parámetros:**
- `caller`: Dirección autenticada con rol CULTIVATOR
- `token_id`: ID del NFT
- `location`: Nueva ubicación (opcional)
- `temperature`: Nueva temperatura (opcional)
- `humidity`: Nueva humedad (opcional)
- `lab_analysis`: Nuevo análisis de laboratorio (opcional)
- `opensea_metadata`: Estructura opcional con campos OpenSea a actualizar (`Option<OpenSeaMetadata>`)

**Estructura OpenSeaMetadata:**
```rust
pub struct OpenSeaMetadata {
    pub name: Option<String>,            // Nuevo nombre (opcional)
    pub description: Option<String>,      // Nueva descripción (opcional)
    pub image: Option<String>,            // Nueva URL de imagen (opcional)
    pub external_url: Option<String>,    // Nueva URL externa (opcional)
    pub attributes: Option<Vec<Attribute>>, // Nuevos atributos (opcional)
}
```

**Nota:** Solo se actualizan los campos que se proporcionan en `opensea_metadata`. Si un campo es `None`, no se modifica.

**Ejemplo:**
```rust
// Actualizar solo campos de trazabilidad
client.update_metadata(
    &cultivator,
    &token_id,
    &Some(String::from_str(&env, "Nueva ubicación")),
    &Some(22i32),
    &Some(65u32),
    &Some(String::from_str(&env, "THC: 22%, CBD: 3%")),
    &None, // No actualizar campos OpenSea
);

// Actualizar campos OpenSea
let mut opensea = OpenSeaMetadata {
    name: Some(String::from_str(&env, "Semilla Actualizada")),
    description: Some(String::from_str(&env, "Descripción actualizada")),
    image: Some(String::from_str(&env, "https://example.com/new-image.png")),
    external_url: None, // No cambiar
    attributes: None,    // No cambiar
};
client.update_metadata(
    &cultivator,
    &token_id,
    &None, &None, &None, &None, // No cambiar campos de trazabilidad
    &Some(opensea),
);
```

#### `get_metadata(env: Env, token_id: u128) -> Option<SeedMetadata>`

Obtiene la metadata completa de un NFT.

#### `get_history(env: Env, token_id: u128) -> Vec<StateTransition>`

Obtiene el historial completo de transiciones de estado.

**Estructura de StateTransition:**
```rust
pub struct StateTransition {
    pub from_state: LifecycleState,
    pub to_state: LifecycleState,
    pub timestamp: u64,
    pub updated_by: Address,
    pub notes: Option<String>,
}
```

### Funciones NFT Estándar

El contrato implementa todas las funciones estándar de NFT:

- `name()`: Nombre del token
- `symbol()`: Símbolo del token
- `balance_of(owner)`: Balance de NFTs de una dirección
- `owner_of(token_id)`: Propietario de un NFT específico
- `approve(caller, operator, token_id)`: Aprobar transferencia
- `get_approved(token_id)`: Obtener aprobación actual
- `set_approval_for_all(caller, operator, approved)`: Aprobar todos los tokens
- `is_approved_for_all(owner, operator)`: Verificar aprobación global
- `transfer_from(caller, from, to, token_id)`: Transferir NFT

**Nota Importante**: `transfer_from` valida que el destinatario esté en la whitelist antes de permitir la transferencia.

### Compatibilidad con OpenSea

El contrato Seed NFT es compatible con el estándar de metadatos de OpenSea, lo que permite que los tokens sean visualizados correctamente en marketplaces y exploradores que soporten este estándar.

#### Campos Requeridos

Los siguientes campos son requeridos por el estándar OpenSea y deben proporcionarse al crear un NFT:

- **`name`**: Nombre descriptivo del NFT
- **`description`**: Descripción detallada del NFT
- **`image`**: URL de la imagen que representa el NFT (debe ser accesible públicamente)

#### Campos Opcionales

- **`external_url`**: URL externa que apunta a una página web relacionada con el NFT
- **`attributes`**: Lista de atributos o traits que describen características del NFT

#### Estructura de Atributos

Los atributos siguen el formato estándar de OpenSea:

```rust
pub struct Attribute {
    pub trait_type: String,  // Tipo de atributo (ej: "Variedad", "THC", "Estado")
    pub value: String,       // Valor del atributo (ej: "Indica", "20%", "Germinated")
}
```

#### Ejemplo de Metadata OpenSea

Cuando se consulta la metadata de un NFT, los campos OpenSea están disponibles junto con los campos de trazabilidad:

```rust
let metadata = client.get_metadata(&token_id).unwrap();
// metadata.name - Nombre del NFT
// metadata.description - Descripción
// metadata.image - URL de la imagen
// metadata.external_url - URL externa (opcional)
// metadata.attributes - Lista de atributos
```

#### Beneficios

1. **Visualización en Marketplaces**: Los NFTs pueden mostrarse correctamente en plataformas que soporten el estándar OpenSea
2. **Atributos Filtrables**: Los atributos permiten filtrar y buscar NFTs por características específicas
3. **Compatibilidad Universal**: El estándar es ampliamente adoptado, facilitando la integración con herramientas externas
4. **Metadata Rica**: Permite incluir información visual y descriptiva además de los datos de trazabilidad

### Sistema de Whitelist

Para garantizar que solo direcciones autorizadas puedan recibir NFTs, el sistema implementa una whitelist:

#### `add_to_whitelist(env: Env, caller: Address, account: Address)`

Agrega una dirección a la whitelist. Solo el ADMIN puede ejecutar esta función.

#### `remove_from_whitelist(env: Env, caller: Address, account: Address)`

Remueve una dirección de la whitelist.

#### `is_whitelisted(env: Env, account: Address) -> bool`

Verifica si una dirección está en la whitelist.

**Uso:**
```rust
// Solo admin puede agregar direcciones
client.add_to_whitelist(&admin, &authorized_address);

// Las transferencias solo funcionan a direcciones whitelisted
client.transfer_from(&owner, &owner, &authorized_address, &token_id);
```

### Storage

#### Instance Storage
- `TOKEN`: Instancia del NonFungibleToken (NonFungibleToken)
- `PAUSED`: Estado de pausa (bool)

#### Persistent Storage
- `(METADATA, token_id)`: Metadata de cada NFT (SeedMetadata)
- `(HISTORY, token_id)`: Historial de transiciones (Vec<StateTransition>)
- `(WHITELIST, address)`: Whitelist de direcciones (bool)
- `(ROLE_KEY, address)`: Mapeo de roles (bool)

---

## Integración entre Contratos

### Flujo de Integración

```
1. Inicialización
   ├── Seed Registry se inicializa con referencia al NFT contract
   └── Seed NFT se inicializa con nombre y símbolo

2. Registro de Semilla
   ├── Usuario llama register_seed() en Seed Registry
   ├── Registry valida permisos y datos
   ├── Registry genera NFT ID único
   ├── Registry llama mint() en Seed NFT contract
   ├── Seed NFT crea el token con estado Seed
   └── Registry almacena referencia al NFT

3. Actualización de Estado
   ├── Usuario llama update_state() en Seed NFT
   ├── NFT valida transición y permisos
   ├── NFT actualiza metadata y estado
   └── NFT guarda transición en historial

4. Consulta
   ├── Usuario puede consultar datos en Registry
   ├── Usuario puede consultar metadata en NFT
   └── Usuario puede consultar historial en NFT
```

### Referencias Cruzadas

- **Registry → NFT**: El Registry almacena `nft_id` y `nft_contract` para cada semilla
- **NFT → Registry**: El NFT puede validar que fue creado por el Registry (mediante el token_id)

### Sincronización

Los contratos mantienen consistencia mediante:
1. **IDs Únicos**: El Registry genera IDs secuenciales que coinciden con los token_ids del NFT
2. **Eventos**: Ambos contratos emiten eventos que pueden ser monitoreados
3. **Inmutabilidad**: Una vez creado, el NFT no puede ser eliminado

---

## Librerías y Dependencias

### Soroban SDK (v23.1.0)

**Descripción**: SDK oficial de Stellar para desarrollo de contratos inteligentes en Soroban.

**Componentes Utilizados:**
- `contract`, `contractimpl`: Macros para definir contratos
- `contracterror`: Manejo de errores personalizados
- `Env`: Entorno del contrato
- `Address`: Direcciones de contratos y cuentas
- `String`, `Vec`, `Symbol`: Tipos de datos
- `storage`: Sistema de almacenamiento (persistent, instance, temporary)
- `events`: Sistema de eventos

**Documentación**: [Soroban Documentation](https://soroban.stellar.org/docs)

### OpenZeppelin Stellar Contracts (v0.5.1)

#### stellar-access

**Descripción**: Librería de control de acceso para contratos Stellar.

**Uso en el Proyecto:**
- Aunque no se usa directamente, el sistema implementa un patrón similar de roles
- Los roles se almacenan en persistent storage con la estructura `(ROLE_KEY, address)`

**Roles Definidos:**
- `ADMIN`: Control total del contrato
- `CULTIVATOR`: Puede registrar semillas y actualizar estados de cultivo
- `PROCESSOR`: Puede actualizar estado a Processed
- `DISPENSARY`: Puede actualizar estados Distributed y Consumed
- `CONSUMER`: Rol para futuras funcionalidades

#### stellar-tokens

**Descripción**: Implementaciones estándar de tokens para Stellar, similar a ERC-20 y ERC-721.

**Componente Utilizado:**
- `NonFungibleToken`: Implementación base de NFT estándar

**Funcionalidades Heredadas:**
- Minting y burning
- Transferencias
- Aprobaciones (approve, set_approval_for_all)
- Enumeración (balance_of, owner_of)

**Documentación**: [OpenZeppelin Stellar Contracts](https://docs.openzeppelin.com/stellar-contracts)

#### stellar-macros

**Descripción**: Macros procedimentales que reducen boilerplate.

**Uso:**
- Aunque incluido en las dependencias, se usa principalmente para futuras extensiones
- Puede simplificar la definición de eventos y storage

### Versiones

```
soroban-sdk = "23.1.0"
stellar-access = "v0.5.1"
stellar-tokens = "v0.5.1"
stellar-macros = "v0.5.1"
```

---

## Sistema de Roles y Permisos

### Roles Definidos

#### ADMIN
- **Permisos**:
  - Pausar/reanudar contratos
  - Otorgar/revocar cualquier rol
  - Agregar/remover direcciones de whitelist
  - Cambiar contrato NFT asociado (Registry)
- **Asignación**: Automática al inicializar el contrato

#### CULTIVATOR
- **Permisos**:
  - Registrar nuevas semillas
  - Actualizar estados: Germinated, PlantVegetative, PlantFlowering, PlantHarvested
  - Actualizar metadata (ubicación, temperatura, humedad, análisis)
- **Uso**: Cultivadores, granjeros, operadores de invernaderos

#### PROCESSOR
- **Permisos**:
  - Actualizar estado a Processed
  - Actualizar metadata relacionada con procesamiento
- **Uso**: Procesadores, fabricantes de productos derivados

#### DISPENSARY
- **Permisos**:
  - Actualizar estados: Distributed, Consumed
  - Realizar transferencias de NFTs
- **Uso**: Dispensarios, distribuidores, puntos de venta

#### CONSUMER
- **Permisos**:
  - Actualmente limitado, reservado para futuras funcionalidades
- **Uso**: Consumidores finales

### Gestión de Roles

#### Otorgar Rol
```rust
client.grant_role(&admin, &user_address, &ROLE_CULTIVATOR);
```

#### Revocar Rol
```rust
client.revoke_role(&admin, &user_address, &ROLE_CULTIVATOR);
```

#### Verificar Rol
```rust
let has_role = client.has_role(&user_address, &ROLE_CULTIVATOR);
```

### Almacenamiento de Roles

Los roles se almacenan en persistent storage con la clave:
```
(ROLE_KEY, address) → bool
```

Donde `ROLE_KEY` es el símbolo del rol (ej: `CULTIVAT`, `PROCESS`, etc.)

---

## Ciclo de Vida de las Semillas

### Diagrama de Estados

```
┌─────┐
│Seed │ (Estado inicial)
└──┬──┘
   │ [CULTIVATOR]
   ▼
┌──────────┐
│Germinated│
└──┬───────┘
   │ [CULTIVATOR]
   ▼
┌──────────────┐
│PlantVegetative│
└──┬───────────┘
   │ [CULTIVATOR]
   ▼
┌─────────────┐
│PlantFlowering│
└──┬──────────┘
   │ [CULTIVATOR]
   ▼
┌──────────────┐
│PlantHarvested│
└──┬───────────┘
   │ [PROCESSOR]
   ▼
┌──────────┐
│Processed│
└──┬───────┘
   │ [DISPENSARY]
   ▼
┌────────────┐
│Distributed│
└──┬────────┘
   │ [DISPENSARY]
   ▼
┌─────────┐
│Consumed │ (Estado final)
└─────────┘
```

### Descripción de Estados

#### 1. Seed (Semilla)
- **Descripción**: Estado inicial cuando la semilla es registrada
- **Metadata Inicial**: Solo información básica del registro
- **Duración**: Hasta la germinación
- **Responsable**: Sistema (automático al crear NFT)

#### 2. Germinated (Germinada)
- **Descripción**: La semilla ha germinado y comenzado a crecer
- **Metadata Típica**: Ubicación del cultivo, condiciones iniciales
- **Duración**: 1-2 semanas típicamente
- **Responsable**: CULTIVATOR

#### 3. PlantVegetative (Vegetativa)
- **Descripción**: Planta en fase de crecimiento vegetativo
- **Metadata Típica**: Condiciones de cultivo, nutrientes aplicados
- **Duración**: 3-16 semanas dependiendo de la variedad
- **Responsable**: CULTIVATOR

#### 4. PlantFlowering (Floración)
- **Descripción**: Planta en fase de floración
- **Metadata Típica**: Condiciones específicas de floración, fecha estimada de cosecha
- **Duración**: 8-12 semanas típicamente
- **Responsable**: CULTIVATOR

#### 5. PlantHarvested (Cosechada)
- **Descripción**: Planta ha sido cosechada
- **Metadata Típica**: Fecha de cosecha, peso, condiciones de almacenamiento
- **Duración**: Hasta el procesamiento
- **Responsable**: CULTIVATOR

#### 6. Processed (Procesado)
- **Descripción**: Producto ha sido procesado (secado, curado, extracción, etc.)
- **Metadata Típica**: Tipo de procesamiento, análisis de laboratorio
- **Duración**: Hasta la distribución
- **Responsable**: PROCESSOR

#### 7. Distributed (Distribuido)
- **Descripción**: Producto está en dispensario o punto de venta
- **Metadata Típica**: Información del dispensario, fecha de llegada
- **Duración**: Hasta la venta
- **Responsable**: DISPENSARY

#### 8. Consumed (Consumido)
- **Descripción**: Producto ha sido consumido por el usuario final
- **Metadata Típica**: Información del consumidor (si se permite)
- **Duración**: Estado final
- **Responsable**: DISPENSARY

### Validación de Transiciones

El sistema valida cada transición de estado:

```rust
pub fn can_transition_to(self, to: LifecycleState) -> bool {
    match (self, to) {
        (Seed, Germinated) => true,
        (Germinated, PlantVegetative) => true,
        (PlantVegetative, PlantFlowering) => true,
        (PlantFlowering, PlantHarvested) => true,
        (PlantHarvested, Processed) => true,
        (Processed, Distributed) => true,
        (Distributed, Consumed) => true,
        _ => false,  // Cualquier otra transición es inválida
    }
}
```

---

## Tests y Validación

### Estructura de Tests

Los tests están organizados en archivos separados para cada contrato:

- `seed-registry/src/test.rs`: Tests del contrato de registro
- `seed-nft/src/test.rs`: Tests del contrato NFT

### Tests del Contrato de Registro

#### `test_initialize`
Verifica la inicialización correcta del contrato:
- Contador de semillas en 0
- Admin tiene rol ADMIN
- Contrato no está pausado

#### `test_register_seed`
Prueba el registro de una semilla individual:
- Validación de permisos
- Creación de datos
- Incremento del contador

#### `test_roles`
Verifica el sistema de roles:
- Otorgar roles
- Revocar roles
- Verificar roles

#### `test_pause`
Prueba la funcionalidad de pausa:
- Pausar contrato
- Reanudar contrato
- Verificar estado

#### `test_queries`
Valida las funciones de consulta:
- Obtener todas las semillas
- Buscar por variedad
- Buscar por lote
- Buscar por creador

### Tests del Contrato NFT

#### `test_initialize`
Verifica la inicialización:
- Nombre y símbolo correctos
- Admin tiene rol ADMIN
- Estado inicial correcto

#### `test_mint`
Prueba la creación de NFTs:
- Mint exitoso
- Balance correcto
- Owner correcto
- Metadata inicializada

#### `test_state_transitions`
Valida las transiciones de estado:
- Transición válida
- Actualización de metadata
- Guardado en historial
- Validación de permisos

#### `test_whitelist`
Prueba el sistema de whitelist:
- Agregar a whitelist
- Remover de whitelist
- Verificar estado

#### `test_roles`
Similar al test de roles del Registry

#### `test_pause`
Similar al test de pausa del Registry

#### `test_metadata_update`
Valida la actualización de metadata:
- Actualización de campos individuales
- Preservación de otros campos
- Validación de permisos

### Ejecutar Tests

```bash
# Tests del Registry
cd contracts
cargo test --package seed-registry

# Tests del NFT
cargo test --package seed-nft

# Todos los tests
cargo test
```

### Cobertura de Tests

Los tests cubren:
- ✅ Inicialización
- ✅ Operaciones CRUD básicas
- ✅ Sistema de roles
- ✅ Funcionalidad de pausa
- ✅ Validaciones de permisos
- ✅ Transiciones de estado
- ✅ Actualización de metadata
- ✅ Sistema de whitelist
- ⚠️ Integración entre contratos (requiere setup adicional)
- ⚠️ Operaciones en lote (estructura básica)

---

## Flujos de Trabajo

### Flujo 1: Registro Inicial de Semilla

```
1. Admin inicializa Seed Registry
   └── Se establece referencia al NFT contract

2. Admin otorga rol CULTIVATOR a cultivador
   └── Cultivador puede ahora registrar semillas

3. Cultivador registra semilla
   ├── Valida que tiene rol CULTIVATOR
   ├── Valida que semilla no existe
   ├── Crea datos de semilla
   ├── Llama mint() en NFT contract
   ├── NFT crea token con estado Seed
   ├── Registry almacena referencia
   └── Emite eventos

4. Resultado
   ├── Semilla registrada en Registry
   ├── NFT creado en Seed NFT
   └── Ambos contratos sincronizados
```

### Flujo 2: Ciclo de Vida Completo

```
1. Semilla registrada → Estado: Seed

2. Cultivador actualiza a Germinated
   ├── Valida rol CULTIVATOR
   ├── Valida transición válida
   ├── Actualiza estado
   ├── Guarda en historial
   └── Emite evento

3. Cultivador actualiza a PlantVegetative
   └── Similar proceso

4. Cultivador actualiza a PlantFlowering
   └── Similar proceso

5. Cultivador actualiza a PlantHarvested
   └── Similar proceso

6. Processor actualiza a Processed
   ├── Valida rol PROCESSOR
   ├── Actualiza processor en metadata
   └── Guarda análisis de laboratorio

7. Dispensary actualiza a Distributed
   ├── Valida rol DISPENSARY
   ├── Actualiza distributor en metadata
   └── Registra fecha de llegada

8. Dispensary actualiza a Consumed
   ├── Valida rol DISPENSARY
   ├── Actualiza consumer en metadata
   └── Estado final alcanzado
```

### Flujo 3: Transferencia de NFT

```
1. Admin agrega dirección a whitelist
   └── Solo direcciones whitelisted pueden recibir NFTs

2. Owner aprueba transferencia
   ├── approve() o set_approval_for_all()
   └── Autoriza a otra dirección

3. Transferencia ejecutada
   ├── Valida que destinatario está en whitelist
   ├── Valida aprobación
   ├── Transfiere NFT
   └── Emite evento Transfer

4. Resultado
   └── NFT ahora pertenece a nueva dirección
```

### Flujo 4: Consulta de Trazabilidad

```
1. Usuario tiene token_id o seed_id

2. Consulta en Registry
   ├── get_seed(seed_id) → Información inicial
   └── Obtiene nft_id y nft_contract

3. Consulta en NFT
   ├── get_metadata(token_id) → Estado actual y metadata
   └── get_history(token_id) → Historial completo

4. Resultado
   └── Trazabilidad completa desde registro hasta consumo
```

---

## Eventos y Auditoría

### Eventos del Contrato de Registro

#### `SeedReg` (SeedRegistered)
Emitido cuando se registra una nueva semilla.

**Campos:**
- `seed_id`: ID de la semilla
- `nft_id`: ID del NFT creado
- `creator`: Dirección que registró la semilla
- `created`: Timestamp de creación

**Ejemplo de Uso:**
```rust
// Monitorear todos los registros de semillas
env.events().publish(
    (symbol_short!("SeedReg"), symbol_short!("seed_id")),
    seed_id
);
```

#### `RoleGrant` (RoleGranted)
Emitido cuando se otorga un rol.

**Campos:**
- `account`: Dirección que recibió el rol
- `role`: Rol otorgado

#### `RoleRevok` (RoleRevoked)
Emitido cuando se revoca un rol.

**Campos:**
- `account`: Dirección que perdió el rol
- `role`: Rol revocado

#### `Paused` / `Unpaused`
Emitidos cuando el contrato se pausa o reanuda.

**Campos:**
- `account`: Dirección que ejecutó la acción

### Eventos del Contrato NFT

#### `Mint`
Emitido cuando se crea un nuevo NFT.

**Campos:**
- `to`: Dirección que recibe el NFT
- `token_id`: ID del token

#### `StateTrans` (StateTransitioned)
Emitido cuando cambia el estado del ciclo de vida.

**Campos:**
- `token_id`: ID del NFT
- `from_st`: Estado anterior
- `to_st`: Estado nuevo
- `updated`: Dirección que realizó la actualización

#### `MetaUpd` (MetadataUpdated)
Emitido cuando se actualiza la metadata.

**Campos:**
- `token_id`: ID del NFT

#### `Whitelist` (TransferWhitelistUpdated)
Emitido cuando cambia la whitelist.

**Campos:**
- `account`: Dirección afectada
- `added`: true si se agregó, false si se removió

#### `Transfer`
Emitido cuando se transfiere un NFT (estándar ERC-721).

**Campos:**
- `from`: Dirección origen
- `to`: Dirección destino
- `token_id`: ID del token

### Auditoría

El sistema proporciona auditoría completa mediante:

1. **Eventos Inmutables**: Todos los eventos se almacenan en la blockchain
2. **Historial Completo**: Cada transición de estado se guarda con timestamp y responsable
3. **Metadata Preservada**: Toda la metadata histórica se mantiene
4. **Trazabilidad End-to-End**: Desde registro hasta consumo

### Consultas de Auditoría

```rust
// Obtener historial completo de un NFT
let history = client.get_history(&token_id);
for transition in history {
    println!("De {:?} a {:?} por {} en {}", 
        transition.from_state,
        transition.to_state,
        transition.updated_by,
        transition.timestamp
    );
}
```

---

## Consideraciones de Seguridad

### Medidas Implementadas

#### 1. Control de Acceso Basado en Roles
- Solo direcciones con roles apropiados pueden realizar acciones
- Los roles se almacenan en persistent storage
- Verificación en cada operación crítica

#### 2. Validación de Transiciones
- No se pueden saltar estados
- No se puede retroceder en el ciclo de vida
- Cada transición requiere el rol correcto

#### 3. Whitelist para Transferencias
- Solo direcciones autorizadas pueden recibir NFTs
- Previene transferencias no autorizadas
- Controlado por ADMIN

#### 4. Funcionalidad de Pausa
- Permite detener operaciones en caso de emergencia
- Solo ADMIN puede pausar/reanudar
- Útil para responder a vulnerabilidades

#### 5. Autenticación
- Todas las funciones que modifican estado requieren `require_auth()`
- Previene ejecución no autorizada

#### 6. Validación de Datos
- Verificación de existencia antes de crear
- Validación de límites (ej: batch size máximo)
- Validación de tipos y formatos

### Recomendaciones Adicionales

#### Para Producción

1. **Auditoría Externa**: Realizar auditoría de seguridad antes del despliegue
2. **Upgradeability**: Considerar implementar upgradeability para correcciones futuras
3. **Rate Limiting**: Implementar límites de tasa para prevenir spam
4. **Monitoreo**: Configurar monitoreo de eventos y transacciones
5. **Backup**: Mantener backups de datos críticos fuera de la blockchain

#### Mejores Prácticas

1. **Principio de Menor Privilegio**: Otorgar solo los roles necesarios
2. **Rotación de Claves**: Rotar claves de administrador periódicamente
3. **Revisión de Código**: Revisar todos los cambios antes de desplegar
4. **Testing Exhaustivo**: Aumentar cobertura de tests, especialmente integración
5. **Documentación**: Mantener documentación actualizada

---

## Ejemplos de Uso

### Ejemplo 1: Setup Inicial

```rust
let env = Env::default();

// 1. Desplegar contrato NFT
let nft_contract_id = env.register_contract(None, SeedNFTContract);
let nft_client = SeedNFTContractClient::new(&env, &nft_contract_id);
nft_client.initialize(
    &admin,
    &String::from_str(&env, "Cannabis Seed NFT"),
    &String::from_str(&env, "CSNFT")
);

// 2. Desplegar contrato Registry
let registry_contract_id = env.register_contract(None, SeedRegistry);
let registry_client = SeedRegistryClient::new(&env, &registry_contract_id);
registry_client.initialize(&admin, &nft_contract_id);

// 3. Configurar roles
registry_client.grant_role(&admin, &cultivator, &ROLE_CULTIVATOR);
registry_client.grant_role(&admin, &processor, &ROLE_PROCESSOR);
registry_client.grant_role(&admin, &dispensary, &ROLE_DISPENSARY);

// 4. Configurar whitelist
nft_client.add_to_whitelist(&admin, &dispensary);
nft_client.add_to_whitelist(&admin, &processor);
```

### Ejemplo 2: Registro y Ciclo Completo

```rust
// Registrar semilla
let nft_id = registry_client.register_seed(
    &String::from_str(&env, "SEED-2024-001"),
    &String::from_str(&env, "Indica"),
    &String::from_str(&env, "BATCH-2024-001"),
    &String::from_str(&env, "Colombia"),
    &String::from_str(&env, "Premium Seeds Bank"),
    Some(20u32),  // 20% THC
    Some(2u32),   // 2% CBD
    true          // Orgánico
);

// Actualizar a germinada
nft_client.update_state(
    &cultivator,
    &nft_id,
    &(LifecycleState::Germinated as u32),
    &Some(String::from_str(&env, "Invernadero A, Sector 3")),
    &Some(24i32),
    &Some(65u32),
    &None
);

// Continuar ciclo...
// (PlantVegetative, PlantFlowering, etc.)

// Procesar
nft_client.update_state(
    &processor,
    &nft_id,
    &(LifecycleState::Processed as u32),
    &None,
    &None,
    &None,
    &Some(String::from_str(&env, "Secado y curado completado"))
);

// Distribuir
nft_client.update_state(
    &dispensary,
    &nft_id,
    &(LifecycleState::Distributed as u32),
    &Some(String::from_str(&env, "Dispensario Central")),
    &None,
    &None,
    &None
);

// Consumir
nft_client.update_state(
    &dispensary,
    &nft_id,
    &(LifecycleState::Consumed as u32),
    &None,
    &None,
    &None,
    &None
);
```

### Ejemplo 3: Consulta de Trazabilidad

```rust
// Obtener información de la semilla
let seed_data = registry_client.get_seed(
    &String::from_str(&env, "SEED-2024-001")
).unwrap();

println!("Variedad: {}", seed_data.variety);
println!("Lote: {}", seed_data.batch);
println!("NFT ID: {}", seed_data.nft_id);

// Obtener estado actual
let metadata = nft_client.get_metadata(&seed_data.nft_id).unwrap();
println!("Estado actual: {:?}", metadata.state);
println!("Última actualización: {}", metadata.updated_at);

// Obtener historial completo
let history = nft_client.get_history(&seed_data.nft_id);
for (i, transition) in history.iter().enumerate() {
    println!("Transición {}: {:?} -> {:?} por {} en {}",
        i + 1,
        transition.from_state,
        transition.to_state,
        transition.updated_by,
        transition.timestamp
    );
}
```

### Ejemplo 4: Operaciones en Lote

```rust
// Preparar datos
let seed_ids = vec![
    &env,
    String::from_str(&env, "SEED-001"),
    String::from_str(&env, "SEED-002"),
    String::from_str(&env, "SEED-003"),
];

let varieties = vec![
    &env,
    String::from_str(&env, "Indica"),
    String::from_str(&env, "Sativa"),
    String::from_str(&env, "Híbrida"),
];

// ... otros vectores

// Registrar lote completo
let nft_ids = registry_client.register_seeds_batch(
    &seed_ids,
    &varieties,
    &batches,
    &origin_countries,
    &seed_banks,
    &expected_thcs,
    &expected_cbds,
    &organic_flags
);
```

---

## Conclusión

Este sistema de contratos inteligentes proporciona una solución completa para la trazabilidad de semillas de cannabis en la blockchain de Stellar. Con su arquitectura modular, sistema robusto de roles, y validaciones exhaustivas, el sistema está diseñado para cumplir con los requisitos regulatorios mientras proporciona transparencia y confiabilidad.

### Características Clave

✅ Trazabilidad completa end-to-end  
✅ Sistema de roles granular  
✅ Validación estricta de transiciones  
✅ Metadata rica y extensible  
✅ Historial inmutable  
✅ Transferencias controladas  
✅ Eventos para auditoría  
✅ Funcionalidad de pausa para emergencias  

### Próximos Pasos

1. Completar tests de integración entre contratos
2. Implementar mejoras de gas optimization
3. Agregar funcionalidades adicionales según necesidades
4. Realizar auditoría de seguridad
5. Desplegar en testnet para pruebas
6. Preparar documentación para usuarios finales

---

**Versión del Documento**: 1.0  
**Última Actualización**: 2024  
**Autor**: Sistema de Trazabilidad Cannabis  
**Licencia**: Ver archivo LICENSE en el repositorio

