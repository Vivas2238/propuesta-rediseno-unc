# UNC — Propuesta de Rediseño del Portal Web Institucional

**Sitio en Vivo (GitHub Pages):** https://vivas2238.github.io/propuesta-rediseno-unc/

> **Aviso:** Este proyecto es una **propuesta independiente de rediseño y modernización** del portal web oficial de la **Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC)** (`https://unc.edu.ve`), desarrollado con fines académicos, de arquitectura web y portafolio de ingeniería.

---

## Características Principales

- **Identidad Visual Oficial UNC**: Paleta institucional en azul universitario (`#031F80`, `#0072AC`, `#7BA6D0`), logotipo oficial UNC en PNG transparente y logotipo oficial del Mincyt con enlace directo a `https://mincyt.gob.ve/`.
- **Tipografía Universitaria Self-Hosted**: Fuente geométrica **Manrope** (WOFF2 local variable `400–800`, 0ms de carga local sin dependencias externas).
- **Navegación Sticky con Menús Desplegables**: 7 secciones principales (`Inicio`, `La Universidad`, `Oferta Académica`, `Noticias`, `Publicaciones`, `Estudiantes`, `Contacto`) optimizadas para pantallas anchas (`1920×1080`, `max-width: 1740px`).
- **Carrusel Automático HD del Campus**: Galería de 9 fotografías reales de la sede Altos de Pipe, aulas interactivas, laboratorios de computación, IVIC y Parque Científico Tecnológico (+Ciencia).
- **Cifras y Hechos con Odómetro GPU**: 6 indicadores verificados directamente de publicaciones oficiales de `unc.edu.ve` con animación fluida a 60/120fps.
- **Archivo Completo de Contenidos**:
  - **30 artículos de noticias** reales con fotografía oficial y paginación interactiva.
  - **15 carreras de pregrado** (5 Ingenierías + 10 Licenciaturas) con páginas individuales y enlaces a mallas curriculares en PDF.
  - **7 autoridades universitarias**, biografía del epónimo **Dr. Humberto Fernández-Morán**, **6 ediciones** de la revista *Ciencia para la Vida*, **Gacetas Universitarias N.º 1–6 + Extraordinaria**, **Calendario Académico 2026–2027**, **Autenticación de Carnet**, **Rutas de Transporte** y video institucional en bucle silencioso.

---

## Arquitectura Técnica (Rust Zero-Dependency Server + Static Site Exporter)

El portal está construido en **Rust puro** (sin frameworks pesados ni dependencias externas):

### Ejecutar servidor local
```bash
cargo run
# Abre http://127.0.0.1:3033
```

### Regenerar exportación estática para GitHub Pages (`docs/`)
```bash
cargo run -- --export-static docs /propuesta-rediseno-unc
```

---

## Autor

**Samuel Vivas** ([@Vivas2238](https://github.com/Vivas2238)) — Estudiante de Ingeniería en Inteligencia Artificial, Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC).
