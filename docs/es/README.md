# Documentación en Español

Este directorio contiene la documentación técnica en español para el Sistema de Trazabilidad de Semillas de Cannabis.

## Tabla de Contenidos

- [Archivos de Documentación](#archivos-de-documentación)
- [Inicio Rápido](#inicio-rápido)
- [Resumen de la Documentación](#resumen-de-la-documentación)
- [Recursos Relacionados](#recursos-relacionados)

---

## Archivos de Documentación

### Contratos de Trazabilidad de Cannabis

**Archivo**: [`CONTRATOS_TRAZABILIDAD_CANNABIS.md`](CONTRATOS_TRAZABILIDAD_CANNABIS.md)

Documentación de referencia completa para los contratos inteligentes de trazabilidad de semillas de cannabis, incluyendo:

- Especificaciones detalladas de los contratos
- Referencia de funciones y documentación de API
- Ejemplos de uso y fragmentos de código
- Documentación de flujos de trabajo
- Consideraciones de seguridad
- Patrones de integración

**Secciones Clave**:
- [Contrato de Registro de Semillas](CONTRATOS_TRAZABILIDAD_CANNABIS.md#contrato-de-registro-de-semillas)
- [Contrato NFT de Semillas](CONTRATOS_TRAZABILIDAD_CANNABIS.md#contrato-nft-de-semillas)
- [Gestión del Ciclo de Vida](CONTRATOS_TRAZABILIDAD_CANNABIS.md#gestión-del-ciclo-de-vida)
- [Control de Acceso Basado en Roles](CONTRATOS_TRAZABILIDAD_CANNABIS.md#control-de-acceso-basado-en-roles)

### Documentación de Contratos Soroban

**Archivo**: [`CONTRATOS_SOROBAN_DOCUMENTACION.md`](CONTRATOS_SOROBAN_DOCUMENTACION.md)

Guía completa de desarrollo para trabajar con los contratos inteligentes de Soroban, que cubre:

- Configuración del entorno de desarrollo y requisitos previos
- Instrucciones de compilación y construcción
- Estrategias de prueba
- Despliegue en red local
- Mejores prácticas y convenciones
- Guía de solución de problemas

**Secciones Clave**:
- [Configuración del Entorno de Desarrollo](CONTRATOS_SOROBAN_DOCUMENTACION.md#configuración-del-entorno-de-desarrollo)
- [Compilación de Contratos](CONTRATOS_SOROBAN_DOCUMENTACION.md#compilación-de-contratos)
- [Pruebas](CONTRATOS_SOROBAN_DOCUMENTACION.md#pruebas)
- [Despliegue](CONTRATOS_SOROBAN_DOCUMENTACION.md#despliegue-y-pruebas-en-red-local)

---

## Inicio Rápido

### Para Desarrolladores

1. **Comienza Aquí**: Lee [`CONTRATOS_SOROBAN_DOCUMENTACION.md`](CONTRATOS_SOROBAN_DOCUMENTACION.md) para configurar tu entorno de desarrollo
2. **Entiende los Contratos**: Revisa [`CONTRATOS_TRAZABILIDAD_CANNABIS.md`](CONTRATOS_TRAZABILIDAD_CANNABIS.md) para las especificaciones de los contratos
3. **Compila y Prueba**: Sigue las instrucciones de compilación y pruebas en la documentación de Soroban

### Para Integradores

1. **Referencia de API**: Comienza con [`CONTRATOS_TRAZABILIDAD_CANNABIS.md`](CONTRATOS_TRAZABILIDAD_CANNABIS.md) para la referencia de funciones
2. **Patrones de Integración**: Revisa los ejemplos de integración en la documentación de contratos
3. **Guía de Despliegue**: Consulta las instrucciones de despliegue en [`CONTRATOS_SOROBAN_DOCUMENTACION.md`](CONTRATOS_SOROBAN_DOCUMENTACION.md)

---

## Resumen de la Documentación

### Arquitectura de Contratos

El sistema consta de dos contratos inteligentes principales:

1. **Contrato de Registro de Semillas**: Punto de entrada central para el registro y gestión de semillas
2. **Contrato NFT de Semillas**: Gestiona los estados del ciclo de vida, metadatos y transferencias de NFT

Ambos contratos trabajan juntos para proporcionar trazabilidad completa de extremo a extremo desde el registro de semillas hasta el consumo.

### Conceptos Clave

- **Estados del Ciclo de Vida**: 8 estados distintos que representan el viaje de la semilla
- **Control de Acceso Basado en Roles**: Permisos granulares para diferentes actores
- **Historial Inmutable**: Rastro de auditoría completo de todas las transiciones de estado
- **Compatibilidad con OpenSea**: Los metadatos de NFT siguen los estándares de OpenSea

---

## Recursos Relacionados

### Documentación del Proyecto

- **README Principal**: [`../../README.md`](../../README.md) - Resumen general del proyecto
- **Índice de Documentación**: [`../README.md`](../README.md) - Índice completo de documentación

### Documentación en Inglés

Para documentación en inglés, consulta [`../en/README.md`](../en/README.md)

### Recursos Externos

- **Documentación de Soroban**: https://soroban.stellar.org/docs
- **Referencia del SDK de Soroban**: https://docs.rs/soroban-sdk/
- **Contratos OpenZeppelin Stellar**: https://docs.openzeppelin.com/stellar-contracts
- **Documentación para Desarrolladores de Stellar**: https://developers.stellar.org/

---

## Estructura de la Documentación

```
docs/es/
├── README.md                              # Este archivo
├── CONTRATOS_TRAZABILIDAD_CANNABIS.md     # Especificaciones de contratos
└── CONTRATOS_SOROBAN_DOCUMENTACION.md     # Guía de desarrollo
```

---

## Contribuir

Al actualizar la documentación en español:

1. **Mantén la consistencia**: Mantén la terminología consistente en todos los documentos
2. **Actualiza ambas versiones**: Si es aplicable, actualiza la documentación correspondiente en inglés
3. **Referencias cruzadas**: Enlaza a secciones relacionadas y recursos externos
4. **Mantén actualizado**: Sincroniza la documentación con los cambios en el código

