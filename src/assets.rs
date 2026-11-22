pub fn render_svg_asset(name: &str) -> (&'static str, String) {
    let svg = match name {
        "unc-seal.svg" => r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 240 240">
  <defs>
    <linearGradient id="g1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#0067B1"/>
      <stop offset="100%" stop-color="#003D6B"/>
    </linearGradient>
    <linearGradient id="g2" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="#27D1A7"/>
      <stop offset="100%" stop-color="#0093D1"/>
    </linearGradient>
  </defs>
  <circle cx="120" cy="120" r="112" fill="url(#g1)" stroke="#AED6FF" stroke-width="5"/>
  <circle cx="120" cy="120" r="95" fill="none" stroke="rgba(255,255,255,0.3)" stroke-width="2" stroke-dasharray="4 4"/>
  <!-- Atom orbits -->
  <ellipse cx="120" cy="115" rx="68" ry="26" fill="none" stroke="#AED6FF" stroke-width="3" transform="rotate(-30 120 115)"/>
  <ellipse cx="120" cy="115" rx="68" ry="26" fill="none" stroke="#27D1A7" stroke-width="3" transform="rotate(30 120 115)"/>
  <ellipse cx="120" cy="115" rx="68" ry="26" fill="none" stroke="#F5C742" stroke-width="3" transform="rotate(90 120 115)"/>
  <circle cx="120" cy="115" r="12" fill="#FFFFFF"/>
  <text x="120" y="192" text-anchor="middle" fill="#FFFFFF" font-family="Open Sans, sans-serif" font-weight="800" font-size="24" letter-spacing="2">UNC</text>
  <text x="120" y="212" text-anchor="middle" fill="#AED6FF" font-family="Open Sans, sans-serif" font-weight="600" font-size="11" letter-spacing="3">SCIENTIA VITAE</text>
</svg>"##.to_string(),

        "logos-header.svg" => r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 380 64">
  <rect x="2" y="6" width="52" height="52" rx="8" fill="#0067B1"/>
  <circle cx="28" cy="32" r="17" fill="none" stroke="#AED6FF" stroke-width="2.5"/>
  <circle cx="28" cy="32" r="5" fill="#27D1A7"/>
  <text x="66" y="26" fill="#0067B1" font-family="Open Sans, sans-serif" font-weight="800" font-size="16">UNC · SCIENTIA VITAE</text>
  <text x="66" y="43" fill="#464C4F" font-family="Open Sans, sans-serif" font-weight="600" font-size="12">Gran Misión Ciencia, Tecnología e Innovación</text>
  <text x="66" y="57" fill="#879BB3" font-family="Open Sans, sans-serif" font-weight="600" font-size="10">Dr. Humberto Fernández-Morán · Mincyt</text>
</svg>"##.to_string(),

        "fernandez-moran.svg" => r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 420 480">
  <defs>
    <linearGradient id="hfm" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#003E6B"/>
      <stop offset="55%" stop-color="#0067B1"/>
      <stop offset="100%" stop-color="#08243E"/>
    </linearGradient>
  </defs>
  <rect width="420" height="480" fill="url(#hfm)"/>
  <!-- Low poly background facets -->
  <polygon points="0,0 180,0 90,140" fill="rgba(255,255,255,0.06)"/>
  <polygon points="420,0 240,0 350,170" fill="rgba(255,255,255,0.05)"/>
  <polygon points="0,480 180,340 290,480" fill="rgba(39,209,167,0.12)"/>
  <!-- Diamond knife & electron microscope motif -->
  <circle cx="210" cy="185" r="95" fill="rgba(255,255,255,0.08)" stroke="#AED6FF" stroke-width="2.5"/>
  <polygon points="210,105 265,185 210,265 155,185" fill="none" stroke="#27D1A7" stroke-width="3"/>
  <circle cx="210" cy="185" r="32" fill="#FFFFFF" fill-opacity="0.18" stroke="#FFFFFF" stroke-width="2"/>
  <path d="M180,185 L240,185 M210,155 L210,215" stroke="#FFFFFF" stroke-width="2.5"/>
  <text x="210" y="340" text-anchor="middle" fill="#FFFFFF" font-family="Open Sans, sans-serif" font-weight="700" font-size="22">Dr. Humberto Fernández-Morán</text>
  <text x="210" y="368" text-anchor="middle" fill="#AED6FF" font-family="Open Sans, sans-serif" font-weight="600" font-size="14">Maracaibo (1924) – Estocolmo (1999) · Panteón Nacional (2025)</text>
  <text x="210" y="405" text-anchor="middle" fill="#27D1A7" font-family="Open Sans, sans-serif" font-weight="700" font-size="13" letter-spacing="1.5">BISTURÍ DE DIAMANTE · CRIOMICROSCOPÍA · FUNDADOR DEL IVIC</text>
</svg>"##.to_string(),

        "campus-pipe.svg" => r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 420">
  <defs>
    <linearGradient id="sky" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#004477"/>
      <stop offset="50%" stop-color="#0067B1"/>
      <stop offset="100%" stop-color="#1A8C9C"/>
    </linearGradient>
  </defs>
  <rect width="640" height="420" fill="url(#sky)"/>
  <!-- Altos de Pipe mountains -->
  <polygon points="0,320 140,190 290,320" fill="rgba(255,255,255,0.1)"/>
  <polygon points="180,320 370,160 550,320" fill="rgba(255,255,255,0.14)"/>
  <polygon points="410,320 560,200 640,280 640,320" fill="rgba(255,255,255,0.09)"/>
  <!-- Blue glass building ("Casita de Cristal Azul") -->
  <rect x="130" y="170" width="380" height="150" fill="rgba(174,214,255,0.22)" stroke="#FFFFFF" stroke-width="2.5"/>
  <line x1="225" y1="170" x2="225" y2="320" stroke="#AED6FF" stroke-width="1.5"/>
  <line x1="320" y1="170" x2="320" y2="320" stroke="#AED6FF" stroke-width="1.5"/>
  <line x1="415" y1="170" x2="415" y2="320" stroke="#AED6FF" stroke-width="1.5"/>
  <line x1="130" y1="220" x2="510" y2="220" stroke="#AED6FF" stroke-width="1.5"/>
  <line x1="130" y1="270" x2="510" y2="270" stroke="#AED6FF" stroke-width="1.5"/>
  <text x="320" y="365" text-anchor="middle" fill="#FFFFFF" font-family="Open Sans, sans-serif" font-weight="700" font-size="20">CAMPUS UNC · ALTOS DE PIPE</text>
  <text x="320" y="390" text-anchor="middle" fill="#AED6FF" font-family="Open Sans, sans-serif" font-weight="600" font-size="13">Km 11 Carretera Panamericana · Estado Miranda · Scientia Vitae</text>
</svg>"##.to_string(),

        _ => {
            // Dynamic generator for news, authorities, magazines, pillars, and career icons
            let (bg1, bg2, accent, tag, title, subtitle) = classify_svg_banner(name);
            format!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 360">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{bg1}"/>
      <stop offset="100%" stop-color="{bg2}"/>
    </linearGradient>
  </defs>
  <rect width="600" height="360" fill="url(#bg)"/>
  <!-- DVFU Low-Poly Geometric Facets -->
  <polygon points="0,0 240,0 110,190" fill="rgba(255,255,255,0.08)"/>
  <polygon points="600,0 380,0 510,220" fill="rgba(255,255,255,0.06)"/>
  <polygon points="600,360 350,360 490,150" fill="rgba(0,0,0,0.16)"/>
  <polygon points="0,360 220,360 80,200" fill="rgba(255,255,255,0.05)"/>
  <circle cx="495" cy="110" r="64" fill="none" stroke="{accent}" stroke-width="2.5" stroke-opacity="0.55"/>
  <circle cx="495" cy="110" r="36" fill="none" stroke="#FFFFFF" stroke-width="1.5" stroke-opacity="0.45"/>
  <polygon points="495,62 537,134 453,134" fill="none" stroke="{accent}" stroke-width="2" stroke-opacity="0.65"/>
  <rect x="34" y="34" width="210" height="28" rx="2" fill="rgba(0,0,0,0.28)"/>
  <text x="46" y="53" fill="{accent}" font-family="Open Sans, sans-serif" font-weight="700" font-size="12" letter-spacing="1.2">{tag}</text>
  <text x="36" y="255" fill="#FFFFFF" font-family="Open Sans, sans-serif" font-weight="700" font-size="24">{title}</text>
  <text x="36" y="288" fill="#E6EFF6" font-family="Open Sans, sans-serif" font-weight="600" font-size="14">{subtitle}</text>
  <line x1="36" y1="312" x2="240" y2="312" stroke="{accent}" stroke-width="3"/>
</svg>"##,
                bg1 = bg1,
                bg2 = bg2,
                accent = accent,
                tag = tag,
                title = title,
                subtitle = subtitle
            )
        }
    };

    ("image/svg+xml; charset=utf-8", svg)
}

fn classify_svg_banner(name: &str) -> (&'static str, &'static str, &'static str, &'static str, &'static str, &'static str) {
    match name {
        "news-1.svg" => ("#005698", "#002B4D", "#27D1A7", "21 NOV 2026 · PRENSA UNC", "Día Nacional del Estudiante Venezolano", "Memoria de lucha y conocimiento · +3.130 estudiantes UNC"),
        "news-2.svg" => ("#1F4E79", "#0D263E", "#F5C742", "20 NOV 2026 · FORO FILOSOFÍA", "Pensar en Tiempos de Inteligencia Artificial", "Soberanía cognitiva, ética y autonomía intelectual en la UNC"),
        "news-3.svg" => ("#0067B1", "#003B66", "#AED6FF", "17 NOV 2026 · COHORTE 2026", "+900 Nuevos Estudiantes Inician en la UNC", "23 estados representados · 15 carreras · 32% docentes IVIC"),
        "news-4.svg" => ("#165B4C", "#0B332A", "#27D1A7", "14 NOV 2026 · SOBERANÍA DATOS", "Fortalecimiento del Poder Popular e IA", "Plan Nacional de IA 2025–2031 · Ciberseguridad y ética"),
        "news-5.svg" => ("#8A4329", "#4D2212", "#F5A282", "13 NOV 2026 · DEBATE UNC", "I Foro de Debate Estudiantil Científico", "Ciencia para la Paz y el Desarrollo · 8 carreras en diálogo"),
        "news-6.svg" => ("#004A80", "#002642", "#38AEED", "12 NOV 2026 · ALIANZA CENDIT", "UNC, FANB y CENDIT · Telecomunicaciones", "Curso de Ampliación para 26 profesionales estratégicos"),
        "news-7.svg" => ("#145A68", "#092D36", "#27D1A7", "11 NOV 2026 · CLASE MAGISTRAL", "Desafíos de la Nueva Realidad Climática", "Dr. Dirk Thielen (IVIC) · Dinámica del Atlántico y Lago de Valencia"),
        "news-8.svg" => ("#2B4C7E", "#152642", "#F5C742", "10 NOV 2026 · UNESCO &amp; UNC", "Ciencia y Diplomacia para la Paz", "Puntos de encuentro y desarrollo sostenible 2024–2033"),

        "auth-1.svg" => ("#0067B1", "#003761", "#AED6FF", "RECTORADO UNC", "Dra. Gabriela Jiménez Ramírez", "Rectora (E) · Universidad Nacional de las Ciencias"),
        "auth-2.svg" => ("#005698", "#002E52", "#27D1A7", "SECRETARÍA GENERAL", "Dr. Roberto Betancourt", "Secretario (E) · Universidad Nacional de las Ciencias"),
        "auth-3.svg" => ("#1B5E7A", "#0D3140", "#F5C742", "DIRECCIÓN DE INVESTIGACIÓN", "Dr. Alberto Quintero", "Director (E) de Investigación · Articulación UNC – IVIC"),
        "auth-4.svg" => ("#166956", "#0B382D", "#27D1A7", "DIRECCIÓN ESTUDIANTIL", "Dr. Irwing Acevedo", "Director (E) Estudiantil · Bienestar y Comunidad UNC"),
        "auth-5.svg" => ("#2E5B94", "#172F4F", "#AED6FF", "DIRECCIÓN ACADÉMICA", "Profa. Diana Castillo", "Directora (E) Académica · 15 Programas de Pregrado"),
        "auth-6.svg" => ("#3A536B", "#1E2C3A", "#F5A282", "DIRECCIÓN ADMINISTRATIVA", "Lic. Damian López", "Director (E) Administrativo · Campus Altos de Pipe"),
        "auth-7.svg" => ("#0067B1", "#00355C", "#F5C742", "REGISTRO Y CONTROL", "Ing. Danmarys Hernández", "Directora (E) de Registro, Control y Seguimiento"),

        "mag-1.svg" => ("#0067B1", "#003459", "#AED6FF", "BOLETÍN ESPECIAL · NOV 2026", "Noticias de la Ciencia", "Edición 2 de Noviembre de 2026 · Mincyt &amp; UNC"),
        "mag-2.svg" => ("#B85635", "#6E2E18", "#F5A282", "REVISTA MENSUAL MINCYT", "Ciencia para la Vida · Octubre", "Innovación tecnológica y soberanía científica"),
        "mag-3.svg" => ("#168265", "#0B4736", "#27D1A7", "REVISTA MENSUAL MINCYT", "Ciencia para la Vida · Septiembre", "Investigación aplicada y formación de relevo"),
        "mag-4.svg" => ("#B58916", "#664C08", "#F5C742", "REVISTA MENSUAL MINCYT", "Ciencia para la Vida · Mayo", "Biotecnología, salud y soberanía agroalimentaria"),
        "mag-5.svg" => ("#2368A2", "#123859", "#AED6FF", "REVISTA MENSUAL MINCYT", "Ciencia para la Vida · Abril", "Inteligencia Artificial, robótica y telecomunicaciones"),
        "mag-6.svg" => ("#187A62", "#0B3D30", "#27D1A7", "REVISTA MENSUAL MINCYT", "Ciencia para la Vida · Marzo", "Homenaje al Dr. Humberto Fernández-Morán"),

        _ => ("#0067B1", "#003963", "#AED6FF", "SCIENTIA VITAE · UNC", "Universidad Nacional de las Ciencias", "Dr. Humberto Fernández-Morán · Somos Ciencia para la Vida"),
    }
}

pub fn render_icon_svg(name: &str) -> (&'static str, String) {
    let inner = match name {
        "pillar-1.svg" => r##"<circle cx="32" cy="28" r="14" stroke="#0067B1" stroke-width="3" fill="rgba(0,103,177,0.12)"/><line x1="42" y1="38" x2="54" y2="50" stroke="#27D1A7" stroke-width="4" stroke-linecap="round"/><circle cx="32" cy="28" r="5" fill="#0067B1"/>"##,
        "pillar-2.svg" => r##"<polygon points="32,8 54,20 54,44 32,56 10,44 10,20" stroke="#0067B1" stroke-width="3" fill="rgba(39,209,167,0.14)"/><polyline points="10,20 32,32 54,20" stroke="#0067B1" stroke-width="2.5" fill="none"/><line x1="32" y1="32" x2="32" y2="56" stroke="#0067B1" stroke-width="2.5"/>"##,
        "pillar-3.svg" => r##"<path d="M24,46h16M26,52h12M32,10a14,14 0 0,1 8,25.5V41H24v-5.5A14,14 0 0,1 32,10z" stroke="#0067B1" stroke-width="3" fill="rgba(242,192,55,0.2)"/>"##,
        "pillar-4.svg" => r##"<rect x="12" y="14" width="40" height="36" rx="4" stroke="#0067B1" stroke-width="3" fill="rgba(0,103,177,0.1)"/><path d="M22,26h20M22,34h14M22,42h10" stroke="#27D1A7" stroke-width="3" stroke-linecap="round"/>"##,
        "pillar-5.svg" => r##"<circle cx="32" cy="32" r="20" stroke="#0067B1" stroke-width="3" fill="rgba(39,209,167,0.12)"/><polyline points="23,32 30,39 43,25" stroke="#0067B1" stroke-width="3.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"##,
        "pillar-6.svg" => r##"<path d="M12,44 L26,28 L36,36 L52,18" stroke="#0067B1" stroke-width="3.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/><polyline points="40,18 52,18 52,30" stroke="#27D1A7" stroke-width="3.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"##,
        _ => r##"<circle cx="32" cy="32" r="22" stroke="#FFFFFF" stroke-width="2.5" fill="rgba(255,255,255,0.16)"/><polygon points="32,14 48,42 16,42" stroke="#FFFFFF" stroke-width="2.2" fill="none"/><circle cx="32" cy="33" r="5" fill="#FFFFFF"/>"##,
    };

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">{}</svg>"##,
        inner
    );
    ("image/svg+xml; charset=utf-8", svg)
}
