use crate::data::*;

pub fn render_page_shell(page_title: &str, active_section: &str, body_html: &str) -> String {
    let nav_active = |sec: &str| -> &'static str {
        if active_section == sec { "active" } else { "" }
    };

    let career_dashed_links: String = CAREER_PROGRAMS
        .iter()
        .map(|c| {
            format!(
                r#"<a href="/carreras/{}" class="itmoru-dashed-link">→ {} ({})</a>"#,
                c.slug, c.name, c.credits
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r##"<!DOCTYPE html>
<html lang="es" data-theme="light">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover">
  <meta name="theme-color" content="#031F80">
  <title>{page_title} — UNC | Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán</title>
  <link rel="icon" href="/assets/real/logo_unc.png" sizes="32x32">
  <link rel="apple-touch-icon" href="/assets/real/logo_unc.png">
  <link rel="stylesheet" href="/assets/dvfu.css?v=9">
  <script>
    (function() {{
      try {{ localStorage.removeItem('unc_dvfu_theme'); }} catch (e) {{}}
      document.documentElement.setAttribute('data-theme', 'light');
    }})();
  </script>
</head>
<body>
  <!-- FULL-SCREEN MEGA-MENU OVERLAY (Opened from Top Bar "menú" Button) -->
  <div id="itmoRuMegaMenu" class="itmoru-megamenu-overlay" aria-hidden="true">
    <div class="itmoru-mega-top">
      <div style="display:flex; align-items:center;">
        <img src="/assets/real/logo_unc_full.png" alt="UNC — Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán" class="unc-logo-white-on-dark" style="height:68px; width:auto; display:block;">
      </div>

      <div style="display:flex; align-items:center; gap:14px; flex-wrap:wrap;">
        <button type="button" class="itmoru-dashed-link" style="background:#0072AC; border-style:solid; cursor:pointer;" onclick="toggleItmoRuMenu()">✕ Cerrar Menú</button>
      </div>
    </div>

    <div class="itmoru-mega-grid">
      <!-- Col 1: LA UNC PARA TI -->
      <div class="itmoru-mega-col">
        <h3>LA UNC PARA TI</h3>
        <div class="itmoru-dashed-links">
          <a href="/" class="itmoru-dashed-link">→ Inicio · Portal Principal UNC</a>
          <a href="/carreras-de-pregrado" class="itmoru-dashed-link">→ 15 Carreras de Pregrado (Mallas PDF)</a>
          <a href="/calendario-academico-2026-2027" class="itmoru-dashed-link">→ Calendario Académico 2026 – 2027</a>
          <a href="/validacion-de-documentos" class="itmoru-dashed-link">→ Validación de Documentos</a>
          <a href="/sugerencias" class="itmoru-dashed-link">→ Buzón de Sugerencias y Mapa Altos de Pipe</a>
        </div>

        <h3 style="margin-top:32px;">¿QUIÉNES SOMOS?</h3>
        <div class="itmoru-dashed-links">
          <a href="/quienes-somos" class="itmoru-dashed-link">→ Misión, Visión y 6 Valores Institucionales</a>
          <a href="/autoridades" class="itmoru-dashed-link">→ Autoridades Universitarias (7 Directivos)</a>
          <a href="/nuestro-heroe-eponimo" class="itmoru-dashed-link">→ Epónimo: Dr. Humberto Fernández-Morán</a>
        </div>
      </div>

      <!-- Col 2: ¿QUÉ HACEMOS? (CIENCIA, GACETAS Y REVISTA) -->
      <div class="itmoru-mega-col">
        <h3>¿QUÉ HACEMOS?</h3>
        <div class="itmoru-dashed-links">
          <a href="/noticias" class="itmoru-dashed-link">→ Prensa y Noticias de Interés (377 Artículos)</a>
          <a href="/revista-ciencia-para-la-vida" class="itmoru-dashed-link">→ Revista Mensual «Ciencia para la Vida» (Mincyt)</a>
          <a href="/gaceta-universitaria" class="itmoru-dashed-link">→ Gacetas Universitarias Ordinarias (N.º 1 – N.º 6)</a>
          <a href="/gaceta-extraordinaria" class="itmoru-dashed-link">→ Gaceta Universitaria Extraordinaria (2026)</a>
          <a href="/centros-de-investigacion" class="itmoru-dashed-link">→ Centros de Investigación UNC – IVIC</a>
          <a href="/laboratorios-de-computacion-avanzados" class="itmoru-dashed-link">→ Laboratorios de Computación Avanzados</a>
          <a href="/aulas-interactivas" class="itmoru-dashed-link">→ Aulas Interactivas («La Casita de Cristal Azul»)</a>
        </div>
      </div>

      <!-- Col 3: 15 CARRERAS DE PREGRADO -->
      <div class="itmoru-mega-col">
        <h3>15 CARRERAS UNC</h3>
        <div class="itmoru-dashed-links" style="max-height:480px; overflow-y:auto; padding-right:6px;">
          {career_dashed_links}
        </div>
      </div>
    </div>
  </div>

  <!-- MAIN SCROLLABLE SHELL (Left of the 180px Fixed Vertical Rails) -->
  <div class="itmoru-layout-shell">

    <!-- 3. TWO-TIER INSTITUTIONAL HEADER (Full-Width UNC Logo Blue #031F80 Masthead — Light Mode Only) -->
    <header class="itmoru-header">
      <!-- Tier 1: Full-Width UNC Logo Blue Band (#031F80) -->
      <div class="itmoru-header-tier1-band">
        <div class="container itmoru-header-tier1">
          <div class="itmoru-brand-row">
            <a href="/" class="itmoru-logo-link" aria-label="Inicio — Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán">
              <img class="itmoru-full-logo-img" src="/assets/real/logo_unc_full.png" alt="UNC — Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán">
            </a>

            <button type="button" class="unc-mobile-nav-toggle" id="uncMobileNavToggle" onclick="toggleMobileNav()" aria-label="Abrir menú de navegación" aria-expanded="false">
              <span>☰</span>
            </button>
          </div>

          <div class="itmoru-address-tag">
            <span class="addr-k">Dirección de Atención y Admisión UNC</span>
            <span class="addr-v">Altos de Pipe, Km 11 Panamericana</span>
          </div>
        </div>
      </div>

      <!-- Tier 2: Clean, User-Friendly Sticky University Navigation Bar with Dropdowns -->
      <nav class="itmoru-header-tier2-band" id="uncMainNavBar" aria-label="Navegación Principal UNC">
        <div class="container itmoru-header-tier2">
          <ul class="unc-main-nav">
            <li class="unc-nav-item">
              <a href="/" class="unc-nav-link {nav_home}">Inicio</a>
            </li>

            <li class="unc-nav-item has-dropdown">
              <a href="/quienes-somos" class="unc-nav-link {nav_about} {nav_auth} {nav_eponimo}" onclick="return handleMobileDropdown(event, this)">La Universidad <span class="caret">▾</span></a>
              <div class="unc-nav-dropdown">
                <a href="/quienes-somos" class="unc-drop-link">
                  <strong>¿Quiénes somos?</strong>
                  <span>Misión, Visión y 6 Valores Institucionales</span>
                </a>
                <a href="/autoridades" class="unc-drop-link">
                  <strong>Autoridades Universitarias</strong>
                  <span>Equipo directivo y Rectorado UNC</span>
                </a>
                <a href="/nuestro-heroe-eponimo" class="unc-drop-link">
                  <strong>Dr. Humberto Fernández-Morán</strong>
                  <span>Biografía de nuestro héroe epónimo y libro PDF</span>
                </a>
              </div>
            </li>

            <li class="unc-nav-item has-dropdown">
              <a href="/carreras-de-pregrado" class="unc-nav-link {nav_programs}" onclick="return handleMobileDropdown(event, this)">Oferta Académica <span class="caret">▾</span></a>
              <div class="unc-nav-dropdown">
                <a href="/carreras-de-pregrado" class="unc-drop-link">
                  <strong>15 Carreras de Pregrado</strong>
                  <span>5 Ingenierías · 10 Licenciaturas y Mallas PDF</span>
                </a>
                <a href="/calendario-academico-2026-2027" class="unc-drop-link">
                  <strong>Calendario Académico 2026–2027</strong>
                  <span>Hitos y períodos lectivos oficiales</span>
                </a>
              </div>
            </li>

            <li class="unc-nav-item">
              <a href="/noticias" class="unc-nav-link {nav_news}">Noticias</a>
            </li>

            <li class="unc-nav-item has-dropdown">
              <a href="/revista-ciencia-para-la-vida" class="unc-nav-link {nav_mag} {nav_gazette}" onclick="return handleMobileDropdown(event, this)">Publicaciones <span class="caret">▾</span></a>
              <div class="unc-nav-dropdown">
                <a href="/revista-ciencia-para-la-vida" class="unc-drop-link">
                  <strong>Revista «Ciencia para la Vida»</strong>
                  <span>6 ediciones oficiales Mincyt en PDF</span>
                </a>
                <a href="/gaceta-universitaria" class="unc-drop-link">
                  <strong>Gaceta Universitaria</strong>
                  <span>Gacetas Ordinarias N.º 1 – N.º 6</span>
                </a>
                <a href="/gaceta-extraordinaria" class="unc-drop-link">
                  <strong>Gaceta Extraordinaria</strong>
                  <span>Resoluciones oficiales 2026</span>
                </a>
              </div>
            </li>

            <li class="unc-nav-item has-dropdown">
              <a href="/validacion-de-documentos" class="unc-nav-link" onclick="return handleMobileDropdown(event, this)">Estudiantes <span class="caret">▾</span></a>
              <div class="unc-nav-dropdown">
                <a href="https://tepuy.unc.edu.ve" target="_blank" rel="noopener" class="unc-drop-link">
                  <strong>Tepuy (Gestión Académica) ↗</strong>
                  <span>Notas, inscripciones y expediente estudiantil</span>
                </a>
                <a href="https://savia.unc.edu.ve" target="_blank" rel="noopener" class="unc-drop-link">
                  <strong>SAVIA (Aula Virtual) ↗</strong>
                  <span>Sistema de Aprendizaje Virtual Interactivo Autónomo</span>
                </a>
                <a href="/assets/docs/reglamento-estudiantil-unc.pdf" target="_blank" rel="noopener" class="unc-drop-link">
                  <strong>Reglamento Estudiantil (PDF) ↗</strong>
                  <span>Normas, derechos y deberes del estudiante UNC</span>
                </a>
                <a href="/validacion-de-documentos" class="unc-drop-link">
                  <strong>Validación de Documentos</strong>
                  <span>Verificación documental institucional UNC</span>
                </a>
              </div>
            </li>

            <li class="unc-nav-item">
              <a href="/sugerencias" class="unc-nav-link {nav_contact}">Contacto</a>
            </li>
          </ul>

          <div class="unc-platform-links" aria-label="Plataformas Académicas UNC">
            <a href="https://tepuy.unc.edu.ve" target="_blank" rel="noopener" class="unc-platform-btn" title="Tepuy — Sistema Integral de Gestión Académica Universitaria">
              <img src="/assets/real/logo_tepuy.svg" alt="Logo Tepuy">
              <span>Tepuy</span>
            </a>
            <a href="https://savia.unc.edu.ve" target="_blank" rel="noopener" class="unc-platform-btn unc-platform-btn--savia" title="SAVIA — Sistema de Aprendizaje Virtual Interactivo Autónomo" aria-label="SAVIA — Sistema de Aprendizaje Virtual Interactivo Autónomo">
              <img src="/assets/real/logo_savia.png" alt="SAVIA" class="unc-savia-wordmark">
            </a>
          </div>
        </div>
      </nav>
    </header>

    <!-- MAIN PAGE CONTENT -->
    <main>
      {body_html}
    </main>

    <!-- FOOTER (#031F80 UNC Navy) & SUB-FOOTER -->
    <footer class="itmoru-footer">
      <div class="container">
        <div class="itmoru-footer-topbar">
          <div style="display:flex; align-items:center;">
            <img src="/assets/real/logo_unc_full.png" alt="UNC — Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán" class="unc-logo-white-on-dark" style="height:68px; width:auto; display:block;">
          </div>

          <div style="display:flex; align-items:center; gap:14px; flex-wrap:wrap;">
            <a href="https://www.tiktok.com/@unicienciasvzla?_t=ZM-8u2xdkTqAMB" target="_blank" rel="noopener" class="itmoru-pill-tab" style="border-color:#FFFFFF; color:#FFFFFF !important;">TikTok</a>
            <a href="https://www.instagram.com/unicienciasvzla/?hl=en" target="_blank" rel="noopener" class="itmoru-pill-tab" style="border-color:#FFFFFF; color:#FFFFFF !important;">Instagram</a>
            <a href="https://www.threads.net/@unicienciasvzla" target="_blank" rel="noopener" class="itmoru-pill-tab" style="border-color:#FFFFFF; color:#FFFFFF !important;">Threads</a>
            <a href="/sugerencias" class="itmoru-pill-tab" style="border-color:#0072AC; background:#0072AC; color:#FFFFFF !important;">Sugerencias</a>
          </div>
        </div>

        <div class="itmoru-footer-cols">
          <div class="itmoru-footer-office">
            <h4>DIRECCIÓN DEL CAMPUS</h4>
            <p>Dirección de Atención y Admisión UNC:<br>Altos de Pipe, Km 11 Panamericana, estado Miranda, República Bolivariana de Venezuela.</p>
            <a href="/sugerencias">→ Ver mapa e indicaciones</a>
          </div>

          <div class="itmoru-footer-office">
            <h4>RECTORADO Y SECRETARÍA</h4>
            <p>Rectora (E): Dra. Gabriela Jiménez Ramírez<br>Secretario (E): Dr. Roberto Betancourt</p>
            <a href="/autoridades">→ Ver las 7 Autoridades Universitarias</a>
            <a href="/gaceta-universitaria">→ Gacetas Universitarias N.º 1 – N.º 6</a>
          </div>

          <div class="itmoru-footer-office">
            <h4>OFICINA DE ADMISIÓN Y ESTUDIANTES</h4>
            <a href="/carreras-de-pregrado">→ 15 Carreras de Pregrado y Mallas PDF</a>
            <a href="/calendario-academico-2026-2027">→ Calendario Académico 2026 – 2027</a>
            <a href="/assets/docs/reglamento-estudiantil-unc.pdf" target="_blank" rel="noopener">→ Reglamento Estudiantil UNC (PDF) ↗</a>
            <a href="/validacion-de-documentos">→ Validación de Documentos</a>
          </div>

          <div class="itmoru-footer-office">
            <h4>PRENSA Y DIVULGACIÓN CIENTÍFICA</h4>
            <a href="/noticias">→ Archivo de Noticias UNC (377 Artículos)</a>
            <a href="/revista-ciencia-para-la-vida">→ Revista «Ciencia para la Vida» (Mincyt)</a>
            <a href="/nuestro-heroe-eponimo">→ Biografía Dr. Humberto Fernández-Morán</a>
            <div style="margin-top:20px; display:flex; align-items:center;">
              <a href="https://mincyt.gob.ve/" target="_blank" rel="noopener" title="Ministerio del Poder Popular para Ciencia y Tecnología (Mincyt)" style="display:inline-block;">
                <img src="/assets/real/logo_mincyt_dark.png" alt="Logo Oficial Ministerio del Poder Popular para Ciencia y Tecnología (Mincyt)" class="unc-logo-transparent" style="max-height:46px; width:auto; display:block;">
              </a>
            </div>
          </div>
        </div>
      </div>
    </footer>

    <div class="itmoru-subfooter">
      <div class="container itmoru-subfooter-inner">
        <div>
          © 2026–2027 Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC). Todos los derechos reservados.
        </div>
        <div>
          Desarrollado por el Ministerio del Poder Popular para Ciencia y Tecnología (Mincyt) · Scientia Vitae
        </div>
      </div>
    </div>

  </div><!-- /.itmoru-layout-shell -->

  <script>
    function toggleMobileNav() {{
      var nav = document.getElementById('uncMainNavBar');
      var btn = document.getElementById('uncMobileNavToggle');
      if (!nav || !btn) return;
      var isOpen = nav.classList.toggle('mobile-open');
      btn.setAttribute('aria-expanded', isOpen ? 'true' : 'false');
      btn.innerHTML = isOpen ? '<span>✕</span>' : '<span>☰</span>';
    }}

    function handleMobileDropdown(e, linkEl) {{
      if (window.innerWidth <= 960) {{
        e.preventDefault();
        var item = linkEl.closest('.unc-nav-item');
        if (item) item.classList.toggle('mobile-expanded');
        return false;
      }}
      return true;
    }}

    function toggleItmoRuMenu() {{
      var menu = document.getElementById('itmoRuMegaMenu');
      if (!menu) return;
      menu.classList.toggle('open');
    }}

    function toggleAccRow(el) {{
      var row = el.closest('.itmoru-acc-row');
      if (row) row.classList.toggle('open');
    }}

    function toggleMoreInfo(btnId, blockId) {{
      var block = document.getElementById(blockId);
      var btn = document.getElementById(btnId);
      if (!block || !btn) return;
      if (block.classList.contains('open')) {{
        block.classList.remove('open');
        btn.innerHTML = 'Mostrar más información ▾';
      }} else {{
        block.classList.add('open');
        btn.innerHTML = 'Ocultar información ▴';
      }}
    }}

    function filterCareers(cat, btnEl) {{
      var items = document.querySelectorAll('.career-card-col');
      for (var i = 0; i < items.length; i++) {{
        var itemCat = items[i].getAttribute('data-category');
        if (cat === 'all' || itemCat === cat) {{
          items[i].style.display = '';
        }} else {{
          items[i].style.display = 'none';
        }}
      }}
      var btns = document.querySelectorAll('.tab-btn-dvfu');
      for (var j = 0; j < btns.length; j++) {{
        btns[j].classList.remove('active');
      }}
      if (btnEl) btnEl.classList.add('active');
    }}

    document.addEventListener('keydown', function(e) {{
      if (e.key === 'Escape') {{
        var menu = document.getElementById('itmoRuMegaMenu');
        if (menu && menu.classList.contains('open')) {{
          menu.classList.remove('open');
        }}
      }}
    }});
  </script>
</body>
</html>"##,
        page_title = page_title,
        nav_home = nav_active("home"),
        nav_about = nav_active("about"),
        nav_programs = nav_active("programs"),
        nav_auth = nav_active("authorities"),
        nav_eponimo = nav_active("eponimo"),
        nav_news = nav_active("news"),
        nav_mag = nav_active("magazine"),
        nav_gazette = nav_active("gazette"),
        nav_contact = nav_active("contact"),
        career_dashed_links = career_dashed_links,
        body_html = body_html
    )
}

pub fn render_news_card(article: &NewsArticle) -> String {
    format!(
        r#"<div class="col-12 col-sm-6 col-md-3">
  <article class="post-card">
    <a href="/noticias/{slug}" class="thumb-wrap">
      <img src="{thumb}" alt="{title}" loading="lazy">
      <span class="anons-badge">{category}</span>
    </a>
    <div class="date-post">{date} · {author}</div>
    <a href="/noticias/{slug}" class="desc-post">{title}</a>
  </article>
</div>"#,
        slug = article.slug,
        thumb = article.thumbnail,
        title = article.title,
        category = article.category,
        date = article.date,
        author = article.author
    )
}

pub fn render_home_page() -> String {
    let top_2_hero_news: String = NEWS_ARTICLES
        .iter()
        .take(2)
        .map(|n| {
            format!(
                r#"<a href="/noticias/{slug}" class="itmoru-news-hero-item">
  <span class="n-date">{date} · {author}</span>
  <div class="n-img-wrap">
    <img src="{thumb}" alt="{title}" loading="lazy">
  </div>
  <h3>{title}</h3>
  <p style="margin:0; font-size:14px; color:var(--itmoru-muted);">{excerpt}</p>
</a>"#,
                slug = n.slug,
                date = n.date,
                author = n.author,
                thumb = n.thumbnail,
                title = n.title,
                excerpt = n.excerpt
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let top_4_news: String = NEWS_ARTICLES
        .iter()
        .skip(2)
        .take(4)
        .map(render_news_card)
        .collect::<Vec<_>>()
        .join("\n");

    let next_4_news: String = NEWS_ARTICLES
        .iter()
        .skip(6)
        .take(4)
        .map(render_news_card)
        .collect::<Vec<_>>()
        .join("\n");

    let third_4_news: String = NEWS_ARTICLES
        .iter()
        .skip(10)
        .take(4)
        .map(render_news_card)
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r##"
<!-- 1. PRESTIGIOUS ACADEMIC HERO (Stanford / Harvard Editorial Architecture — Zero Retro Windows) -->
<section class="itmoru-hero">
  <div class="container">
    <div class="unc-hero-grid">
      <!-- Left: Authoritative Institutional Statement & Non-Repetitive CTAs -->
      <div>
        <span class="unc-hero-eyebrow">República Bolivariana de Venezuela · Altos de Pipe</span>
        <h1 class="itmoru-giant-statement">
          <span class="unc-hero-brand-pop">UNC</span><span class="cursive-span">es</span><br>
          <span class="unc-hero-tagline-line">Ciencia para la Vida</span>
        </h1>
        <p class="unc-hero-lead">
          El hogar del conocimiento científico de Venezuela
        </p>
        <div class="unc-hero-actions">
          <a href="/carreras-de-pregrado" class="unc-btn-primary">Explorar 15 Carreras de Pregrado →</a>
          <a href="/nuestro-heroe-eponimo" class="unc-btn-secondary">Dr. Humberto Fernández-Morán →</a>
        </div>
      </div>

      <!-- Right: Automatic 9-Photo Architectural Carousel (UNC, IVIC & Parque Científico Altos de Pipe) -->
      <div class="unc-campus-showcase">
        <div class="unc-campus-photo-wrap" id="uncHeroCarousel">
          <div class="unc-carousel-slide active">
            <img src="/assets/real/hero_campus_unc.jpg" alt="Sede Principal UNC — Edificio de Cristal Azul en Altos de Pipe">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_pct.jpg?v=hq" alt="Parque Científico Tecnológico de Venezuela (+Ciencia) en Altos de Pipe">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_lab_unc.jpg" alt="Sala de Computación e Informática UNC">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_slide_3.jpg" alt="Aulas Interactivas de la Universidad Nacional de las Ciencias">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_clase_unc.jpg?v=hq" alt="Clases y Encuentros Académicos en la UNC" style="object-position: center 20%;">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_slide_4.jpg" alt="Laboratorios de Computación Avanzada UNC">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/ivic_plaza.jpg" alt="Complejo Científico IVIC — Plaza Bolívar y Bello en Altos de Pipe">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_slide_2.jpg" alt="Galería Central y Espacios Académicos UNC">
          </div>
          <div class="unc-carousel-slide">
            <img src="/assets/real/campus_unc.jpg" alt="Fachada Institucional Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán" style="object-position: center 65%;">
          </div>

          <button type="button" class="unc-carousel-btn prev" onclick="stepHeroCarousel(-1)" aria-label="Foto anterior">‹</button>
          <button type="button" class="unc-carousel-btn next" onclick="stepHeroCarousel(1)" aria-label="Foto siguiente">›</button>

          <div class="unc-carousel-dots" role="tablist" aria-label="Galería del Campus UNC e IVIC">
            <button type="button" class="unc-carousel-dot active" onclick="goHeroCarousel(0)" aria-label="Foto 1"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(1)" aria-label="Foto 2"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(2)" aria-label="Foto 3"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(3)" aria-label="Foto 4"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(4)" aria-label="Foto 5"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(5)" aria-label="Foto 6"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(6)" aria-label="Foto 7"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(7)" aria-label="Foto 8"></button>
            <button type="button" class="unc-carousel-dot" onclick="goHeroCarousel(8)" aria-label="Foto 9"></button>
          </div>
        </div>
        <script>
          (function() {{
            var idx = 0;
            var timer = null;
            function renderSlide() {{
              var wrap = document.getElementById('uncHeroCarousel');
              if (!wrap) return;
              var slides = wrap.querySelectorAll('.unc-carousel-slide');
              var dots = wrap.querySelectorAll('.unc-carousel-dot');
              for (var i = 0; i < slides.length; i++) {{
                slides[i].classList.toggle('active', i === idx);
              }}
              for (var j = 0; j < dots.length; j++) {{
                dots[j].classList.toggle('active', j === idx);
              }}
            }}
            function restartAuto() {{
              if (timer) clearInterval(timer);
              timer = setInterval(function() {{
                var wrap = document.getElementById('uncHeroCarousel');
                if (!wrap) return;
                var n = wrap.querySelectorAll('.unc-carousel-slide').length;
                idx = (idx + 1) % n;
                renderSlide();
              }}, 3500);
            }}
            window.stepHeroCarousel = function(delta) {{
              var wrap = document.getElementById('uncHeroCarousel');
              if (!wrap) return;
              var n = wrap.querySelectorAll('.unc-carousel-slide').length;
              idx = (idx + delta + n) % n;
              renderSlide();
              restartAuto();
            }};
            window.goHeroCarousel = function(target) {{
              idx = target;
              renderSlide();
              restartAuto();
            }};
            restartAuto();
          }})();
        </script>
        <a href="/quienes-somos" class="unc-campus-footer-link">
          <span>Descubre la Misión, Visión y Valores de la UNC</span>
          <span>→</span>
        </a>
      </div>
    </div>

    <!-- 2. TOP 2 EDITORIAL FEATURED NEWS STRIP -->
    <div class="itmoru-news-top2">
      {top_2_hero_news}
    </div>
  </div>
</section>

<!-- 3. ITMO.RU HOLLOW OUTLINED NEWS SECTION ("NOTICIAS DE INTERÉS") -->
<section class="itmoru-section">
  <div class="container">
    <div class="itmoru-section-toprow">
      <h2 class="itmoru-hollow-h2" style="margin-bottom:0;">Noticias de Interés</h2>
      <a href="/noticias" class="itmoru-action-item"><span>→ Ver todas las noticias ({total_news})</span></a>
    </div>

    <div class="row">
      {top_4_news}
    </div>
    <div class="row">
      {next_4_news}
    </div>
    <div class="row">
      {third_4_news}
    </div>
    <div style="text-align:center; margin-top:24px;">
      <a href="/noticias" class="itmoru-action-item"><span>→ Explorar el Archivo Completo de Prensa UNC ({total_news} Noticias Verificadas)</span></a>
    </div>
  </div>
</section>

<!-- 4. ITMO.RU DARK CHARCOAL BAND ("ЦИФРЫ И ФАКТЫ" -> "CIFRAS Y HECHOS UNC") -->
<section class="itmoru-dark-band">
  <!-- Decorative UNC Blue/Sky Vector Curve -->
  <svg class="itmoru-pink-arc-svg" viewBox="0 0 600 600" fill="none">
    <path d="M50,550 C140,220 340,380 560,40" stroke="url(#uncArcGrad)" stroke-width="18" stroke-linecap="round"/>
    <circle cx="310" cy="295" r="180" stroke="url(#uncArcGrad)" stroke-width="4" stroke-dasharray="10 10"/>
    <defs>
      <linearGradient id="uncArcGrad" x1="0%" y1="100%" x2="100%" y2="0%">
        <stop offset="0%" stop-color="#0072AC"/>
        <stop offset="50%" stop-color="#4CA3D9"/>
        <stop offset="100%" stop-color="#7BA6D0"/>
      </linearGradient>
    </defs>
  </svg>

  <div class="container">
    <div class="itmoru-section-toprow">
      <h2 class="itmoru-hollow-h2" style="margin-bottom:0;">Cifras y Hechos</h2>
      <div style="display:flex; align-items:center; gap:14px;">
        <img src="/assets/real/logo_unc.png" alt="Escudo UNC" class="unc-logo-white-on-dark" style="height:56px; width:auto;">
        <span class="unc-serif" style="font-size:26px; font-weight:800; color:#FFFFFF;">UNC · Scientia Vitae</span>
      </div>
    </div>

    <div class="itmoru-big-numbers-row" id="uncCifrasGrid">
      <div class="itmoru-outlined-stat">
        <span class="num-hollow unc-odo" data-value="18.006">18.006</span>
        <span class="num-desc">Solicitudes de ingreso registradas a nivel nacional para formarse en la UNC</span>
      </div>
      <div class="itmoru-outlined-stat">
        <span class="num-hollow unc-odo" data-value="3.130">3.130</span>
        <span class="num-desc">Estudiantes regulares activos en formación científica (Semestres I y II)</span>
      </div>
      <div class="itmoru-outlined-stat">
        <span class="num-hollow unc-odo" data-value="900+">900+</span>
        <span class="num-desc">Nuevos ingresos de la cohorte noviembre 2026 provenientes de los 23 estados</span>
      </div>
      <div class="itmoru-outlined-stat">
        <span class="num-hollow unc-odo" data-value="15 + 4">15 + 4</span>
        <span class="num-desc">15 carreras activas (12 inéditas) + 4 nuevas anunciadas: Virología, Geociencias, Ing. en Cambio Climático y Ciencias Ambientales</span>
      </div>
      <div class="itmoru-outlined-stat">
        <span class="num-hollow unc-odo" data-value="38%">38%</span>
        <span class="num-desc">Nómina docente integrada por científicos de la Fundación IDEA + 32% investigadores activos del IVIC</span>
      </div>
      <div class="itmoru-outlined-stat">
        <span class="num-hollow unc-odo" data-value="52%">52%</span>
        <span class="num-desc">Matrícula en Inteligencia Artificial, Ciberseguridad, Robótica y Ciencia de Datos</span>
      </div>
    </div>
    <script>
      (function() {{
        var grid = document.getElementById('uncCifrasGrid');
        if (!grid) return;
        var els = grid.querySelectorAll('.unc-odo');
        var ribbons = [];

        // Build GPU-accelerated vertical rolling strips (0..9 + 0..target) for every digit
        for (var i = 0; i < els.length; i++) {{
          var el = els[i];
          var val = el.getAttribute('data-value') || el.textContent.trim();
          el.textContent = '';
          var digitIndex = 0;
          for (var c = 0; c < val.length; c++) {{
            var ch = val[c];
            if (ch >= '0' && ch <= '9') {{
              var target = parseInt(ch, 10);
              var slot = document.createElement('span');
              slot.className = 'unc-odo-slot';
              var ribbon = document.createElement('span');
              ribbon.className = 'unc-odo-ribbon';
              // One full 0-9 spin + land on exact target digit for silky smooth rolling feel
              var totalSteps = 10 + target;
              for (var d = 0; d <= totalSteps; d++) {{
                var span = document.createElement('span');
                span.className = 'unc-odo-digit';
                span.textContent = String(d % 10);
                ribbon.appendChild(span);
              }}
              var delayMs = (i * 80) + (digitIndex * 75);
              var durMs = 1550 + (digitIndex * 160);
              ribbon.style.transition = 'transform ' + durMs + 'ms cubic-bezier(0.16, 1, 0.3, 1) ' + delayMs + 'ms';
              ribbon.setAttribute('data-steps', String(totalSteps));
              slot.appendChild(ribbon);
              el.appendChild(slot);
              ribbons.push(ribbon);
              digitIndex++;
            }} else {{
              var st = document.createElement('span');
              st.className = 'unc-odo-static';
              st.textContent = ch === ' ' ? '\u00A0' : ch;
              el.appendChild(st);
            }}
          }}
        }}

        var started = false;
        function triggerOdometer() {{
          if (started) return;
          started = true;
          requestAnimationFrame(function() {{
            requestAnimationFrame(function() {{
              for (var k = 0; k < ribbons.length; k++) {{
                var r = ribbons[k];
                var steps = parseInt(r.getAttribute('data-steps') || '0', 10);
                r.style.transform = 'translate3d(0, -' + (steps * 1.08) + 'em, 0)';
              }}
              setTimeout(function() {{
                for (var n = 0; n < els.length; n++) {{
                  els[n].textContent = els[n].getAttribute('data-value') || '';
                }}
              }}, 2900);
            }});
          }});
        }}

        if ('IntersectionObserver' in window) {{
          var obs = new IntersectionObserver(function(entries) {{
            for (var m = 0; m < entries.length; m++) {{
              if (entries[m].isIntersecting) {{
                triggerOdometer();
                obs.disconnect();
                break;
              }}
            }}
          }}, {{ threshold: 0.2 }});
          obs.observe(grid);
        }} else {{
          triggerOdometer();
        }}
      }})();
    </script>

    <!-- Interactive Accordion ("итмо в рейтингах" -> "procesos claves en la formación de la unc") -->
    <h3 style="font-size:28px; font-weight:900; text-transform:uppercase; color:#FFFFFF; margin:0 0 18px;">
      PROCESOS CLAVES EN LA FORMACIÓN DE LA UNC
    </h3>

    <div class="itmoru-accordion-list">
      <div class="itmoru-acc-row open">
        <div class="itmoru-acc-header" onclick="toggleAccRow(this)">
          <div class="itmoru-acc-title">
            <img src="/assets/real/pillar_1.png" alt="Investigar" class="unc-logo-white-on-dark" style="width:42px; height:42px; object-fit:contain;">
            <span>01 · Investigar y Desarrollar</span>
          </div>
          <span class="itmoru-acc-plus">+</span>
        </div>
        <div class="itmoru-acc-panel">
          <div>
            <strong>1. Investigar:</strong> Habilidades para formular preguntas relevantes, diseñar estudios y analizar datos, contribuyendo al avance del saber en sus respectivas disciplinas.<br>
            <strong>2. Desarrollar:</strong> Habilidades para planificar y ejecutar iniciativas que respondan a necesidades en áreas prioritarias para el país, fomentando el crecimiento y la sostenibilidad.
          </div>
        </div>
      </div>

      <div class="itmoru-acc-row">
        <div class="itmoru-acc-header" onclick="toggleAccRow(this)">
          <div class="itmoru-acc-title">
            <img src="/assets/real/pillar_3.png" alt="Innovar" class="unc-logo-white-on-dark" style="width:42px; height:42px; object-fit:contain;">
            <span>02 · Innovar y Producir</span>
          </div>
          <span class="itmoru-acc-plus">+</span>
        </div>
        <div class="itmoru-acc-panel">
          <div>
            <strong>3. Innovar:</strong> Capacidad de pensar de manera creativa y crítica, proponiendo soluciones originales a problemas existentes y adaptándose a un entorno en constante cambio.<br>
            <strong>4. Producir:</strong> Competencias en la creación de obras académicas, investigaciones y productos que reflejen su formación y contribuyan al desarrollo de su campo profesional.
          </div>
        </div>
      </div>

      <div class="itmoru-acc-row">
        <div class="itmoru-acc-header" onclick="toggleAccRow(this)">
          <div class="itmoru-acc-title">
            <img src="/assets/real/pillar_5.png" alt="Aplicar y Transformar" class="unc-logo-white-on-dark" style="width:42px; height:42px; object-fit:contain;">
            <span>03 · Aplicar y Transformar</span>
          </div>
          <span class="itmoru-acc-plus">+</span>
        </div>
        <div class="itmoru-acc-panel">
          <div>
            <strong>5. Aplicar:</strong> Capacidad de transferir teorías y conceptos a la práctica, resolviendo problemas concretos en sus áreas de formación.<br>
            <strong>6. Transformar:</strong> Agentes de cambio, utilizando su formación para impactar positivamente en la sociedad y contribuir a la mejora de la calidad de vida de las comunidades.
          </div>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- 5. ITMO.RU EDUCATION & PORTALS BENTO SECTION ("ОБРАЗОВАНИЕ" -> "PORTALES DESTACADOS") -->
<section class="itmoru-section">
  <div class="container">
    <div class="itmoru-section-toprow">
      <h2 class="itmoru-hollow-h2" style="margin-bottom:0;">Portales Destacados</h2>
      <div class="itmoru-slide-counter">
        <span>1 / 4</span>
        <span class="itmoru-dot active"></span>
        <span class="itmoru-dot"></span>
        <span class="itmoru-dot"></span>
        <span class="itmoru-dot"></span>
      </div>
    </div>

    <div class="banners-grid">
      <a href="/nuestro-heroe-eponimo" class="itmoru-bento-card">
        <img class="bento-bg" src="/assets/real/foto_hfm.png" alt="Dr. Humberto Fernández-Morán">
        <div class="itmoru-bento-top">
          <span class="itmoru-bento-pill">Héroe Epónimo · Panteón Nacional</span>
        </div>
        <div class="itmoru-bento-bottom">
          <h3 class="itmoru-bento-title">Dr. Humberto Fernández-Morán</h3>
          <p class="itmoru-bento-sub">Creador del bisturí de diamante y fundador del IVIC (1924–1999) · Libro PDF</p>
        </div>
      </a>

      <a href="/carreras-de-pregrado" class="itmoru-bento-card">
        <img class="bento-bg" src="/assets/real/news_3.jpg" alt="15 Carreras de Pregrado">
        <div class="itmoru-bento-top">
          <span class="itmoru-bento-pill">5 Ingenierías · 10 Licenciaturas</span>
        </div>
        <div class="itmoru-bento-bottom">
          <h3 class="itmoru-bento-title">15 Carreras de Pregrado</h3>
          <p class="itmoru-bento-sub">12 programas inéditos en Venezuela · IA, Ciberseguridad, Física Nuclear</p>
        </div>
      </a>

      <a href="/revista-ciencia-para-la-vida" class="itmoru-bento-card">
        <img class="bento-bg" src="/assets/real/mag_1.jpg" alt="Revista Ciencia para la Vida">
        <div class="itmoru-bento-top">
          <span class="itmoru-bento-pill">Revista Mensual Mincyt</span>
        </div>
        <div class="itmoru-bento-bottom">
          <h3 class="itmoru-bento-title">Revista Ciencia para la Vida</h3>
          <p class="itmoru-bento-sub">Una ventana a la innovación tecnológica en Venezuela (6 ediciones PDF)</p>
        </div>
      </a>

      <a href="/gaceta-universitaria" class="itmoru-bento-card">
        <img class="bento-bg" src="/assets/real/campus_unc.jpg" alt="Gaceta y Calendario 2026–2027">
        <div class="itmoru-bento-top">
          <span class="itmoru-bento-pill">Documentos Oficiales</span>
        </div>
        <div class="itmoru-bento-bottom">
          <h3 class="itmoru-bento-title">Gaceta y Calendario 2026–2027</h3>
          <p class="itmoru-bento-sub">Gacetas Ordinarias N.º 1–6, Extraordinaria y Calendario Académico</p>
        </div>
      </a>
    </div>
  </div>
</section>

<!-- 6. EDUCACIÓN Y CIENCIA SPLIT -->
<section class="itmoru-section">
  <div class="container">
    <div class="itmoru-section-toprow">
      <h2 class="itmoru-hollow-h2" style="margin-bottom:0;">Educación y Ciencia</h2>
      <a href="/carreras-de-pregrado" class="itmoru-action-item"><span>→ Ver las 15 Carreras y Mallas PDF</span></a>
    </div>

    <div class="row" style="margin-bottom:44px; align-items:center;">
      <div class="col-12 col-md-6" style="margin-bottom:24px;">
        <ul class="itmoru-vertical-list">
          <li><a href="/carreras/ingenieria-en-inteligencia-artificial" class="itmoru-vlist-item"><span>→ Ingeniería en Inteligencia Artificial</span><span>181 U.C</span></a></li>
          <li><a href="/carreras/ingenieria-en-ciberseguridad" class="itmoru-vlist-item"><span>→ Ingeniería en Ciberseguridad</span><span>187 U.C</span></a></li>
          <li><a href="/carreras/ingenieria-en-robotica-y-automatizacion" class="itmoru-vlist-item"><span>→ Ingeniería en Robótica y Automatización</span><span>180 U.C</span></a></li>
          <li><a href="/carreras/ingenieria-en-electromedicina" class="itmoru-vlist-item"><span>→ Ingeniería en Electromedicina</span><span>191 U.C</span></a></li>
          <li><a href="/carreras/licenciatura-en-fisica-nuclear" class="itmoru-vlist-item"><span>→ Licenciatura en Física Nuclear</span><span>181 U.C</span></a></li>
          <li><a href="/carreras/licenciatura-en-biologia-y-quimica-computacional" class="itmoru-vlist-item"><span>→ Lic. en Biología y Química Computacional</span><span>185 U.C</span></a></li>
        </ul>
      </div>

      <div class="col-12 col-md-6">
        <div class="unc-campus-showcase" style="padding:0; overflow:hidden;">
          <img src="/assets/real/news_20.jpg" alt="Formación Científica y Tecnológica en la UNC" style="width:100%; aspect-ratio:3/2; height:auto; object-fit:cover; display:block;">
        </div>
      </div>
    </div>
  </div>
</section>

<!-- 7. UNOBTRUSIVE VERTICAL VIDEO SPOTLIGHT (9:16 Campus Reel Showcase) -->
<section class="itmoru-section" style="padding: 56px 0 72px;">
  <div class="container">
    <div class="unc-video-spotlight-card">
      <div>
        <h2 class="itmoru-hollow-h2" style="margin-bottom:20px;">Vive la experiencia UNC</h2>
        <p style="font-size:18px; line-height:1.68; color:var(--itmoru-muted); margin:0 0 28px; max-width:660px;">
          Conoce de cerca el día a día en el campus de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán en Altos de Pipe: aulas interactivas, laboratorios científicos, formación de vanguardia y comunidad universitaria.
        </p>
        <div style="display:flex; gap:14px; flex-wrap:wrap;">
          <a href="/carreras-de-pregrado" class="unc-btn-primary">Explorar Oferta Académica →</a>
          <a href="https://www.instagram.com/unicienciasvzla/?hl=en" target="_blank" rel="noopener" class="unc-btn-secondary">Ver en Instagram (@unicienciasvzla) ↗</a>
        </div>
      </div>

      <div>
        <div class="unc-video-phone-frame">
          <video autoplay muted loop playsinline preload="auto" poster="/assets/real/unc_video_poster.jpg">
            <source src="/assets/video/unc_video.mp4?v=2" type="video/mp4">
            Tu navegador no soporta la reproducción de video HTML5.
          </video>
        </div>
      </div>
    </div>
  </div>
</section>
"##,
        total_news = NEWS_ARTICLES.len(),
        top_2_hero_news = top_2_hero_news,
        top_4_news = top_4_news,
        next_4_news = next_4_news,
        third_4_news = third_4_news
    );

    render_page_shell("Inicio", "home", &body)
}

pub fn render_news_index_page() -> String {
    let per_page: usize = 12;
    let total_news = NEWS_ARTICLES.len();
    let total_pages = (total_news + per_page - 1) / per_page;

    let all_cards: String = NEWS_ARTICLES
        .iter()
        .enumerate()
        .map(|(idx, article)| {
            let page_group = (idx / per_page) + 1;
            let hidden_attr = if page_group == 1 { "" } else { r#" style="display:none;""# };
            let raw = render_news_card(article);
            let search_haystack = format!(
                "{} {} {}",
                article.title, article.date, article.category
            )
            .to_lowercase()
            .replace('"', "");
            raw.replacen(
                r#"<div class="col-12 col-sm-6 col-md-3">"#,
                &format!(
                    r#"<div class="col-12 col-sm-6 col-md-3 news-archive-col" data-news-page="{}" data-news-search="{}"{}>"#,
                    page_group,
                    search_haystack,
                    hidden_attr
                ),
                1,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r##"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <span>Noticias de Interés</span>
  </div>
</div>
<div class="container" style="padding-bottom:60px;">
  <div class="section-header-row" style="margin-top:0; display:flex; align-items:flex-end; justify-content:space-between; flex-wrap:wrap; gap:16px; margin-bottom:18px;">
    <div>
      <h1 style="margin:0 0 6px; font-size:clamp(32px, 3.6vw, 48px); font-weight:900;">Noticias de Interés — Prensa UNC</h1>
      <span style="font-size:15.5px; color:var(--itmoru-muted);">Archivo histórico completo de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán ({count} artículos desde 2025 hasta hoy)</span>
    </div>

    <div style="display:flex; align-items:center; gap:10px; flex-wrap:wrap; width:100%; max-width:440px;">
      <input type="search" id="newsArchiveSearch" class="dvfu-input" placeholder="Buscar entre las {count} noticias por título, fecha o tema..." oninput="filterNewsArchive()" style="flex:1; min-width:220px;">
    </div>
  </div>
  <hr class="blue-line">

  <div style="display:flex; align-items:center; justify-content:space-between; flex-wrap:wrap; gap:12px; margin-bottom:20px;">
    <div id="newsArchiveStatus" style="font-size:14px; font-weight:600; color:var(--itmoru-muted);">Mostrando página 1 de {total_pages} ({count} noticias totales)</div>
    <div id="newsPaginationTop" class="tabs-user" style="margin-bottom:0; display:flex; flex-wrap:wrap; gap:6px;"></div>
  </div>

  <div class="row" id="newsArchiveGrid">
    {all_cards}
  </div>

  <div id="newsPaginationBottom" class="tabs-user" style="margin-top:28px; display:flex; justify-content:center; flex-wrap:wrap; gap:6px;"></div>

  <script>
    (function() {{
      var PER_PAGE = 12;
      var currentPage = 1;
      var allCols = Array.prototype.slice.call(document.querySelectorAll('.news-archive-col'));
      var filteredCols = allCols.slice();

      function renderControls(totalPages) {{
        var containers = [
          document.getElementById('newsPaginationTop'),
          document.getElementById('newsPaginationBottom')
        ];
        var html = '';
        if (totalPages > 1) {{
          html += '<button type="button" class="tab-btn-dvfu" ' + (currentPage <= 1 ? 'disabled style="opacity:0.45;cursor:not-allowed;"' : '') + ' onclick="goNewsPage(' + (currentPage - 1) + ')">← Anterior</button>';
          var pages = [];
          for (var p = 1; p <= totalPages; p++) {{
            if (p === 1 || p === totalPages || (p >= currentPage - 2 && p <= currentPage + 2)) {{
              pages.push(p);
            }} else if (pages[pages.length - 1] !== '...') {{
              pages.push('...');
            }}
          }}
          for (var i = 0; i < pages.length; i++) {{
            var item = pages[i];
            if (item === '...') {{
              html += '<span style="padding:6px 8px; color:var(--itmoru-muted); font-weight:700;">…</span>';
            }} else {{
              html += '<button type="button" class="tab-btn-dvfu' + (item === currentPage ? ' active' : '') + '" onclick="goNewsPage(' + item + ')">' + item + '</button>';
            }}
          }}
          html += '<button type="button" class="tab-btn-dvfu" ' + (currentPage >= totalPages ? 'disabled style="opacity:0.45;cursor:not-allowed;"' : '') + ' onclick="goNewsPage(' + (currentPage + 1) + ')">Siguiente →</button>';
        }}
        for (var c = 0; c < containers.length; c++) {{
          if (containers[c]) containers[c].innerHTML = html;
        }}
      }}

      function applyPage(scrollTop) {{
        var total = filteredCols.length;
        var totalPages = Math.max(1, Math.ceil(total / PER_PAGE));
        if (currentPage > totalPages) currentPage = totalPages;
        if (currentPage < 1) currentPage = 1;

        for (var i = 0; i < allCols.length; i++) {{
          allCols[i].style.display = 'none';
        }}
        var start = (currentPage - 1) * PER_PAGE;
        var end = Math.min(start + PER_PAGE, total);
        for (var j = start; j < end; j++) {{
          filteredCols[j].style.display = '';
        }}

        var status = document.getElementById('newsArchiveStatus');
        if (status) {{
          if (total === 0) {{
            status.textContent = 'No se encontraron noticias que coincidan con tu búsqueda.';
          }} else {{
            status.textContent = 'Mostrando ' + (start + 1) + '–' + end + ' de ' + total + ' noticias (Página ' + currentPage + ' de ' + totalPages + ')';
          }}
        }}
        renderControls(totalPages);
        if (scrollTop) {{
          var grid = document.getElementById('newsArchiveGrid');
          if (grid) grid.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
        }}
      }}

      window.goNewsPage = function(p) {{
        currentPage = p;
        applyPage(true);
      }};

      window.filterNewsArchive = function() {{
        var q = (document.getElementById('newsArchiveSearch').value || '').trim().toLowerCase();
        if (!q) {{
          filteredCols = allCols.slice();
        }} else {{
          filteredCols = allCols.filter(function(col) {{
            return (col.getAttribute('data-news-search') || '').indexOf(q) !== -1;
          }});
        }}
        currentPage = 1;
        applyPage(false);
      }};

      applyPage(false);
    }})();
  </script>
</div>
"##,
        count = total_news,
        total_pages = total_pages,
        all_cards = all_cards
    );

    render_page_shell("Noticias de Interés", "news", &body)
}

pub fn render_news_detail_page(slug: &str) -> Option<String> {
    let article = NEWS_ARTICLES.iter().find(|a| a.slug == slug)?;

    let sections_html: String = article
        .sections
        .iter()
        .map(|(heading, paragraphs)| {
            let h_html = if heading.is_empty() {
                String::new()
            } else {
                format!(r#"<h3 style="margin-top:26px;">{}</h3>"#, heading)
            };
            let p_html: String = paragraphs
                .iter()
                .map(|p| format!(r#"<p>{}</p>"#, p))
                .collect::<Vec<_>>()
                .join("\n");
            format!("{}\n{}", h_html, p_html)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let sidebar_other_news: String = NEWS_ARTICLES
        .iter()
        .filter(|a| a.slug != slug)
        .take(5)
        .map(|a| {
            format!(
                r#"<div style="padding-bottom:12px; margin-bottom:14px; border-bottom:1px solid var(--itmoru-border);">
  <a href="/noticias/{slug}" style="font-size:14px; line-height:19px; color:var(--itmoru-ink); display:block; font-weight:600; margin-bottom:4px;">{title}</a>
  <span style="font-size:12px; font-style:italic; color:var(--itmoru-muted);">{date} · {author}</span>
</div>"#,
                slug = a.slug,
                title = a.title,
                date = a.date,
                author = a.author
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/noticias">Noticias</a> <span>/</span> <span>{title}</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <div class="row">
    <div class="col-12 col-md-8 content-text-justify">
      <span style="background:var(--unc-blue); color:#FFFFFF; font-size:12px; padding:3px 9px; display:inline-block; margin-bottom:12px;">
        {category}
      </span>
      <h1>{title}</h1>
      <div style="font-size:13px; font-style:italic; color:var(--itmoru-muted); margin-bottom:18px;">
        Publicado el {date} · Redacción: <b>{author}</b> · Fuente original: <a href="{original_url}" target="_blank" rel="noopener">unc.edu.ve</a>
      </div>
      <hr class="blue-line">

      <div class="video-block" style="margin-bottom:24px; padding:10px;">
        <img src="{thumb}" alt="{title}" style="width:100%; height:auto; max-height:680px; object-fit:contain; display:block;">
      </div>

      {sections_html}

      <div style="margin-top:30px; padding-top:18px; border-top:1px solid var(--itmoru-border); display:flex; justify-content:space-between; flex-wrap:wrap; gap:12px;">
        <a href="/noticias" class="button-outline-dvfu">← Volver a todas las noticias</a>
        <a href="{original_url}" target="_blank" rel="noopener" class="button-s">Ver publicación original en unc.edu.ve ↗</a>
      </div>
    </div>

    <div class="col-12 col-md-4">
      <div style="background:var(--itmoru-bg-soft); border:1px solid var(--itmoru-border); padding:20px;">
        <h3 style="margin-top:0; font-size:20px;">Más Noticias de la UNC</h3>
        <hr class="blue-line">
        {sidebar_other_news}
        <a href="/noticias" class="button-s" style="width:100%; margin-top:8px;">Ver archivo de prensa</a>
      </div>
    </div>
  </div>
</div>
"#,
        title = article.title,
        category = article.category,
        date = article.date,
        author = article.author,
        original_url = article.original_url,
        thumb = article.thumbnail,
        sections_html = sections_html,
        sidebar_other_news = sidebar_other_news
    );

    Some(render_page_shell(article.title, "news", &body))
}

pub fn render_careers_index_page() -> String {
    let cards: String = CAREER_PROGRAMS
        .iter()
        .map(|c| {
            let first_fields: String = c
                .study_fields
                .iter()
                .take(3)
                .map(|(k, v)| format!("<li><b>{}:</b> {}</li>", k, v))
                .collect::<Vec<_>>()
                .join("");

            format!(
                r#"<div class="col-12 col-md-4 career-card-col" data-category="{category}" style="margin-bottom:24px;">
  <div style="background:var(--itmoru-card-bg); border:1px solid var(--itmoru-border); height:100%; display:flex; flex-direction:column;">
    <a href="/carreras/{slug}" class="banner-tile {tile_class}" style="height:205px; padding:18px 20px;">
      <div class="tile-top-row">
        <span class="tile-tag">{category} · {credits}</span>
      </div>
      <div>
        <div class="name-banner" style="font-size:22px; line-height:1.25;">{name}</div>
        <div class="sub-banner" style="font-size:14.5px; line-height:1.4;">{degree}</div>
      </div>
    </a>
    <div style="padding:22px 24px; display:flex; flex-direction:column; flex:1;">
      <div style="font-size:15px; line-height:1.55; color:var(--itmoru-muted); margin-bottom:14px;">
        <b>Duración:</b> {duration} ({semesters}) · <b>Créditos:</b> {credits}<br>
        <b>Área:</b> {area}
      </div>
      <ul style="margin:0 0 20px; padding-left:20px; font-size:15.5px; line-height:1.62; color:var(--itmoru-ink);">
        {first_fields}
      </ul>
      <div style="margin-top:auto; display:flex; gap:10px; flex-wrap:wrap;">
        <a href="/carreras/{slug}" class="button-s" style="flex:1;">Ver Programa</a>
        <a href="{pdf}" target="_blank" rel="noopener" class="button-outline-dvfu">Malla PDF ↗</a>
      </div>
    </div>
  </div>
</div>"#,
                category = c.category,
                slug = c.slug,
                tile_class = c.tile_color_class,
                credits = c.credits,
                name = c.name,
                degree = c.degree_title,
                duration = c.duration,
                semesters = c.semesters,
                area = c.area,
                first_fields = first_fields,
                pdf = c.pdf_url
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <span>Programas</span> <span>/</span> <span>Carreras de Pregrado</span>
  </div>
</div>

<div class="container" style="padding-bottom:45px;">
  <div class="section-header-row" style="margin-top:0;">
    <h1>Carreras de Pregrado — Oferta Académica UNC</h1>
    <span style="font-size:13px; color:var(--itmoru-muted);">15 Programas Científicos y Tecnológicos (4 años · 8 semestres)</span>
  </div>
  <hr class="blue-line">

  <!-- DVFU .tabs-user Filter Tabs -->
  <div class="tabs-user">
    <button type="button" class="tab-btn-dvfu active" onclick="filterCareers('all', this)">Todas las Carreras (15)</button>
    <button type="button" class="tab-btn-dvfu" onclick="filterCareers('Ingenierías', this)">Ingenierías (5)</button>
    <button type="button" class="tab-btn-dvfu" onclick="filterCareers('Licenciaturas', this)">Licenciaturas (10)</button>
  </div>

  <div class="row">
    {cards}
  </div>
</div>
"#,
        cards = cards
    );

    render_page_shell("Carreras de Pregrado", "programs", &body)
}

pub fn render_career_detail_page(slug: &str) -> Option<String> {
    let career = CAREER_PROGRAMS.iter().find(|c| c.slug == slug)?;

    let study_rows: String = career
        .study_fields
        .iter()
        .map(|(k, v)| {
            format!(
                r#"<div class="white-block-card" style="margin-bottom:14px;">
  <div>
    <h3 style="font-size:17px; margin-bottom:4px;">{}</h3>
    <p>{}</p>
  </div>
</div>"#,
                k, v
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let career_rows: String = career
        .career_fields
        .iter()
        .map(|(k, v)| {
            format!(
                r#"<div class="white-block-card" style="margin-bottom:14px;">
  <div>
    <h3 style="font-size:17px; margin-bottom:4px;">{}</h3>
    <p>{}</p>
  </div>
</div>"#,
                k, v
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/carreras-de-pregrado">Carreras de Pregrado</a> <span>/</span> <span>{name}</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <div class="row">
    <div class="col-12 col-md-8">
      <span style="background:var(--unc-blue); color:#FFFFFF; font-size:12px; padding:3px 10px; display:inline-block; margin-bottom:12px;">
        {category} · {area}
      </span>
      <h1>{name}</h1>
      <hr class="blue-line">

      <div class="dvfu-table-wrap" style="margin-bottom:28px;">
        <table class="dvfu-table">
          <thead>
            <tr>
              <th>Título que Otorga</th>
              <th>Duración</th>
              <th>Régimen</th>
              <th>Unidades de Crédito</th>
              <th>Área de Conocimiento</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td><b>{degree}</b></td>
              <td>{duration}</td>
              <td>{semesters}</td>
              <td><b>{credits}</b></td>
              <td>{area}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2>Campo de Estudio</h2>
      <hr class="blue-line">
      {study_rows}

      <h2 style="margin-top:32px;">Campo Laboral y Desempeño Profesional</h2>
      <hr class="blue-line">
      {career_rows}
    </div>

    <div class="col-12 col-md-4">
      <div class="video-block" style="margin-bottom:22px;">
        <div class="banner-tile {tile_class}" style="height:240px;">
          <div class="tile-top-row">
            <span class="tile-tag">{credits} · {semesters}</span>
          </div>
          <div>
            <div class="name-banner">{name}</div>
            <div class="sub-banner">{degree}</div>
          </div>
        </div>
      </div>

      <div style="background:var(--itmoru-bg-soft); border:1px solid var(--itmoru-border); padding:22px;">
        <h3 style="margin-top:0; font-size:19px;">Malla Curricular Oficial</h3>
        <p style="font-size:13.5px; line-height:21px;">
          Descarga el documento oficial en PDF con el plan de estudios completo, unidades curriculares por semestre y prelaciones de <b>{name}</b>.
        </p>
        <a href="{pdf}" target="_blank" rel="noopener" class="button-s" style="width:100%; margin-bottom:10px;">
          Descargar Malla Curricular (PDF) ↗
        </a>
        <a href="/carreras-de-pregrado" class="button-outline-dvfu" style="width:100%;">
          ← Ver las 15 Carreras
        </a>
      </div>
    </div>
  </div>
</div>
"#,
        name = career.name,
        category = career.category,
        area = career.area,
        degree = career.degree_title,
        duration = career.duration,
        semesters = career.semesters,
        credits = career.credits,
        tile_class = career.tile_color_class,
        pdf = career.pdf_url,
        study_rows = study_rows,
        career_rows = career_rows
    );

    Some(render_page_shell(career.name, "programs", &body))
}

pub fn render_quienes_somos_page() -> String {
    let body = r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <span>Acerca de</span> <span>/</span> <span>¿Quiénes somos?</span>
  </div>
</div>

<div class="container" style="padding-bottom:45px;">
  <h1>¿Quiénes somos? — Universidad Nacional de las Ciencias</h1>
  <hr class="blue-line">

  <div class="row" style="margin-bottom:32px;">
    <div class="col-12 col-md-7 content-text-justify">
      <p>
        Somos una institución educativa, creada bajo principios humanistas, inclusivos, de cooperación solidaria para garantizar la formación de un profesional innovador, con visión histórica, participativo y comprometido con el fortalecimiento y la consolidación de las capacidades científicas, tecnológicas y productivas del país.
      </p>
      <p>
        <strong>La Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán</strong> es una universidad pública, dedicada a la formación de científicos e investigadores de excelencia y a la promoción del desarrollo científico y tecnológico al servicio de la soberanía nacional y el bienestar de los pueblos.
      </p>

      <div class="white-block-card" style="margin-top:22px; margin-bottom:16px;">
        <div>
          <h3>Misión</h3>
          <p>
            Formar a científicas y científicos que, con ética y liderazgo, transformen el futuro de Venezuela a través de conocimientos profundos, sentido humanista, compromiso con la innovación y la justicia social que promuevan el desarrollo integral de la nación.
          </p>
        </div>
      </div>

      <div class="white-block-card">
        <div>
          <h3>Visión</h3>
          <p>
            Ser la universidad científica del pueblo venezolano, reconocida más allá de cualquier frontera por su excelencia en la formación de profesionales éticos y comprometidos con la transformación del país a través de la ciencia, la tecnología y la innovación.
          </p>
        </div>
      </div>
    </div>

    <div class="col-12 col-md-5">
      <div class="video-block">
        <img src="/assets/real/descubre_unc.jpg" alt="Descubre la UNC" style="width:100%; height:auto; display:block;">
      </div>
    </div>
  </div>

  <h2>Valores que nos Guían</h2>
  <hr class="blue-line">
  <div class="pillars-grid">
    <div class="white-block-card"><div><h3>1. Inclusión y Solidaridad</h3><p>Acceso democrático al conocimiento científico de vanguardia para jóvenes de los 23 estados del país.</p></div></div>
    <div class="white-block-card"><div><h3>2. Innovación y Creatividad</h3><p>Búsqueda permanente de soluciones originales a los desafíos tecnológicos, energéticos, médicos y ambientales.</p></div></div>
    <div class="white-block-card"><div><h3>3. Compromiso Social</h3><p>Ciencia para la vida orientada al bienestar colectivo y a la transformación sostenible de las comunidades.</p></div></div>
    <div class="white-block-card"><div><h3>4. Ética y Responsabilidad</h3><p>Formación humanística transversal y pensamiento crítico ante las tecnologías emergentes y la inteligencia artificial.</p></div></div>
    <div class="white-block-card"><div><h3>5. Soberanía Científica</h3><p>Desarrollo de capacidades propias en investigación, datos, telecomunicaciones, salud, agricultura e industria.</p></div></div>
    <div class="white-block-card"><div><h3>6. Excelencia Académica</h3><p>Rigor matemático, físico y experimental respaldado por investigadores del IVIC y centros científicos nacionales.</p></div></div>
  </div>
</div>
"#;

    render_page_shell("¿Quiénes somos?", "about", body)
}

pub fn render_autoridades_page() -> String {
    let cards: String = AUTHORITIES
        .iter()
        .map(|a| {
            format!(
                r#"<div class="col-12 col-sm-6 col-md-3">
  <div class="authority-card">
    <div class="auth-photo-wrap">
      <img src="{photo}" alt="{name}" loading="lazy">
    </div>
    <div class="auth-body">
      <span class="auth-role">{role}</span>
      <h3 class="auth-name">{name}</h3>
      <p class="auth-bio">{bio}</p>
    </div>
  </div>
</div>"#,
                photo = a.photo_url,
                name = a.name,
                role = a.role,
                bio = a.bio_focus
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/quienes-somos">Acerca de</a> <span>/</span> <span>Autoridades Universitarias</span>
  </div>
</div>

<div class="container" style="padding-bottom:45px;">
  <div class="section-header-row" style="margin-top:0;">
    <h1>Autoridades Universitarias — UNC</h1>
    <span style="font-size:13px; color:var(--itmoru-muted);">Equipo Directivo de la Universidad Nacional de las Ciencias</span>
  </div>
  <hr class="blue-line">
  <div class="row">
    {cards}
  </div>
</div>
"#,
        cards = cards
    );

    render_page_shell("Autoridades Universitarias", "authorities", &body)
}

pub fn render_eponimo_page() -> String {
    let body = r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/quienes-somos">Acerca de</a> <span>/</span> <span>Nuestro Héroe Epónimo</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <h1>Nuestro Epónimo: Dr. Humberto Fernández-Morán (1924 – 1999)</h1>
  <hr class="blue-line">

  <div class="row">
    <div class="col-12 col-md-8 content-text-justify">
      <p>
        Cualquier tentativa de sintetizar la vida y obra de <strong>Humberto Fernández-Morán</strong> se enfrenta a una dificultad común: la imposibilidad de condensar la trayectoria de este gigante; este nuevo intento recuerda que es hijo ilustre de Maracaibo (Venezuela), nació el <strong>18 de febrero de 1924</strong> en el seno de una familia profundamente comprometida con el progreso y la educación. Desde una edad temprana, su insaciable curiosidad y su inclinación por la ciencia lo distinguieron entre sus contemporáneos. En la ribera del lago de Maracaibo, su espíritu inquisitivo lo impulsó a emprender un recorrido académico sin parangón que lo llevó a las universidades más prestigiosas del mundo.
      </p>
      <p>
        Una vez concluidos los estudios de educación secundaria, su rendimiento académico sobresaliente le permitió ingresar en la <strong>Universidad de Múnich (Alemania)</strong>, donde obtuvo la licenciatura en Ciencias Médicas con la máxima calificación de <em>summa cum laude</em>. Posteriormente, dicho título fue revalidado en la Universidad Central de Venezuela, nuevamente con honores. En este contexto, su inclinación hacia la innovación le motivó a desarrollar el <strong>bisturí de diamante</strong>, una innovación que transformó (y sigue transformando) las técnicas microscópicas y neuroquirúrgicas, elevando los estándares de precisión y reduciendo significativamente los riesgos asociados a intervenciones delicadas.
      </p>
      <p>
        Con una visión prospectiva, Fernández-Morán no se limitó a las fronteras del conocimiento establecido. A mediados del siglo pasado, se incorporó al prestigioso <strong>Instituto Tecnológico de Massachusetts (MIT)</strong>, y posteriormente a la <strong>Administración Nacional de Aeronáutica y el Espacio (NASA)</strong>, donde su ingenio contribuyó al desarrollo de tecnologías fundamentales para la exploración espacial y el análisis de muestras lunares del programa Apolo. Su trascendental contribución a la criomicroscopía electrónica y al microscopio electrónico de barrido le valió reconocimiento internacional, consolidando su legado como pionero en este campo.
      </p>
      <p>
        Su retorno a Venezuela supuso el comienzo de una nueva época en el ámbito científico latinoamericano. Su legado más notable fue la fundación del <strong>Instituto Venezolano de Neurología e Investigaciones Cerebrales (IVNIC)</strong>, actualmente denominado <strong>Instituto Venezolano de Investigaciones Científicas (IVIC)</strong>, lo que evidencia su inquebrantable compromiso con el progreso científico de su país natal. Bajo su liderazgo, el instituto se convirtió en un bastión de la investigación neurológica y una fuente de esperanza para muchos jóvenes científicos venezolanos.
      </p>
      <p>
        En reconocimiento a sus contribuciones, Fernández-Morán fue galardonado con numerosos premios y honores, entre los que se encuentran la Orden del Sol del Perú, el título de caballero de la Orden de la Estrella Polar (Suecia), la <strong>medalla John Scott</strong> y el premio Médico del Año otorgado por la Universidad de Cambridge. Falleció el <strong>17 de marzo de 1999</strong> en Estocolmo (Suecia). Posteriormente, el <strong>17 de marzo de 2025</strong>, fue elevado al altar de los héroes y heroínas de la patria, siendo conducido al <strong>Panteón Nacional</strong>.
      </p>
      <p>
        Esta universidad, que lleva su nombre junto a la Gran Misión Ciencia, Tecnología e Innovación, es un reconocimiento y una invitación a seguir su ejemplo, a dedicar todos los esfuerzos a cada proyecto, a cada investigación y a cada solución que contribuya al bien común.
      </p>
    </div>

    <div class="col-12 col-md-4">
      <div class="video-block" style="margin-bottom:22px;">
        <img src="/assets/real/foto_hfm.png" alt="Dr. Humberto Fernández-Morán" style="width:100%; height:auto; background:var(--itmoru-bg-soft);">
      </div>

      <div style="background:var(--itmoru-bg-soft); border:1px solid var(--itmoru-border); padding:22px;">
        <h3 style="margin-top:0; font-size:19px;">Libro Biográfico Oficial (PDF)</h3>
        <p style="font-size:13.5px; line-height:21px;">
          Descubre más sobre nuestro epónimo descargando el libro <strong>«Descubrir lo invisible: Humberto Fernández-Morán, el tecnólogo atómico»</strong>.
        </p>
        <a href="https://mincyt.gob.ve/wp-content/uploads/2025/01/Fernandez-Moran-WEB.pdf" target="_blank" rel="noopener" class="button-s" style="width:100%;">
          Descargar Libro en PDF (Mincyt) ↗
        </a>
      </div>
    </div>
  </div>
</div>
"#;

    render_page_shell("Nuestro Héroe Epónimo — Dr. Humberto Fernández-Morán", "eponimo", body)
}

pub fn render_magazine_page() -> String {
    let cards: String = MAGAZINE_ISSUES
        .iter()
        .map(|m| {
            format!(
                r#"<div class="col-12 col-sm-6 col-md-4" style="margin-bottom:26px;">
  <div class="video-block" style="height:100%; justify-content:space-between;">
    <a href="{pdf}" target="_blank" rel="noopener" style="display:block; overflow:hidden; background:var(--itmoru-bg-soft); aspect-ratio:6/5; height:auto; margin-bottom:14px;">
      <img src="{cover}" alt="{title}" style="width:100%; height:100%; object-fit:contain; display:block;">
    </a>
    <div style="padding:6px 8px 10px;">
      <span class="banner-tile {tile_class}" style="height:auto; padding:3px 9px; font-size:11px; font-weight:700; text-transform:uppercase; display:inline-block; border:none;">{sub}</span>
      <h3 style="font-size:18px; line-height:23px; margin:8px 0 14px;">{title}</h3>
      <a href="{pdf}" target="_blank" rel="noopener" class="button-s" style="width:100%;">Abrir Edición en PDF ↗</a>
    </div>
  </div>
</div>"#,
                pdf = m.pdf_url,
                cover = m.cover_url,
                title = m.title,
                sub = m.subtitle,
                tile_class = m.tile_class
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <span>Revista Ciencia para la Vida</span>
  </div>
</div>

<div class="container" style="padding-bottom:45px;">
  <div class="section-header-row" style="margin-top:0;">
    <h1>Revista «Ciencia para la Vida» — Mincyt & UNC</h1>
    <span style="font-size:13px; color:var(--itmoru-muted);">Una ventana a la innovación tecnológica en Venezuela</span>
  </div>
  <hr class="blue-line">
  <div class="row">
    {cards}
  </div>
</div>
"#,
        cards = cards
    );

    render_page_shell("Revista Ciencia para la Vida", "magazine", &body)
}

pub fn render_gazettes_page(only_extra: bool) -> String {
    let title = if only_extra {
        "Gaceta Universitaria Extraordinaria"
    } else {
        "Gaceta Universitaria (Ordinarias y Extraordinarias)"
    };

    let rows: String = GAZETTES
        .iter()
        .filter(|g| !only_extra || g.kind == "Extraordinaria")
        .map(|g| {
            format!(
                r#"<tr>
  <td><b>{num}</b></td>
  <td>{title}</td>
  <td>{kind}</td>
  <td>{year}</td>
  <td>{summary}</td>
  <td><a href="{pdf}" target="_blank" rel="noopener" class="button-s" style="height:30px; font-size:13px; padding:0 14px;">Descargar PDF ↗</a></td>
</tr>"#,
                num = g.number,
                title = g.title,
                kind = g.kind,
                year = g.year,
                summary = g.summary,
                pdf = g.pdf_url
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/quienes-somos">Acerca de</a> <span>/</span> <span>{title}</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <div class="section-header-row" style="margin-top:0;">
    <h1>{title}</h1>
    <div style="display:flex; gap:10px; flex-wrap:wrap;">
      <a href="/gaceta-universitaria" class="button-outline-dvfu">Todas las Gacetas</a>
      <a href="/gaceta-extraordinaria" class="button-outline-dvfu">Solo Extraordinarias</a>
      <a href="/assets/docs/reglamento-estudiantil-unc.pdf" target="_blank" rel="noopener" class="button-s">Reglamento Estudiantil (PDF) ↗</a>
    </div>
  </div>
  <hr class="blue-line">

  <div class="dvfu-table-wrap">
    <table class="dvfu-table">
      <thead>
        <tr>
          <th>Número</th>
          <th>Documento Oficial</th>
          <th>Tipo</th>
          <th>Año</th>
          <th>Contenido Principal</th>
          <th>Archivo Oficial</th>
        </tr>
      </thead>
      <tbody>
        {rows}
      </tbody>
    </table>
  </div>
</div>
"#,
        title = title,
        rows = rows
    );

    render_page_shell(title, "gazette", &body)
}

pub fn render_calendar_page() -> String {
    let body = r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/quienes-somos">Acerca de</a> <span>/</span> <span>Calendario Académico 2026 – 2027</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <h1>Calendario Académico 2026 – 2027</h1>
  <hr class="blue-line">

  <div class="row">
    <div class="col-12 col-md-7">
      <p style="font-size:14.5px; line-height:25px;">
        Consulta los principales hitos académicos, períodos lectivos e inicio de actividades de la <strong>Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC)</strong> correspondiente al ciclo <strong>2026 – 2027</strong>.
      </p>

      <div class="dvfu-table-wrap" style="margin:22px 0;">
        <table class="dvfu-table">
          <thead>
            <tr>
              <th>Hito Académico</th>
              <th>Período / Fecha</th>
              <th>Condición</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td><b>Bienvenida e Inicio Cohorte Noviembre 2026 (+900 Estudiantes)</b></td>
              <td>16 de noviembre de 2026</td>
              <td>Ejecutado</td>
            </tr>
            <tr>
              <td><b>Día Nacional del Estudiante Venezolano</b></td>
              <td>21 de noviembre de 2026</td>
              <td>Conmemoración Oficial</td>
            </tr>
            <tr>
              <td><b>II Aniversario de Creación de la UNC</b></td>
              <td>12 de diciembre de 2026</td>
              <td>Aniversario Institucional</td>
            </tr>
            <tr>
              <td><b>Período Lectivo Regular 2026 – 2027</b></td>
              <td>Semestres I y II en curso</td>
              <td>Activo (3.130 estudiantes)</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div style="display:flex; gap:12px; flex-wrap:wrap;">
        <a href="/carreras-de-pregrado" class="button-s">Explorar 15 Carreras de Pregrado →</a>
        <a href="/assets/docs/reglamento-estudiantil-unc.pdf" target="_blank" rel="noopener" class="button-outline-dvfu">Reglamento Estudiantil (PDF) ↗</a>
      </div>
    </div>

    <div class="col-12 col-md-5">
      <div class="video-block" style="padding:0; overflow:hidden; border:none; border-radius:14px;">
        <div class="showcase-visual" style="min-height:280px; background:linear-gradient(145deg, #031F80 0%, #0A38A8 100%); padding:28px; display:flex; flex-direction:column; justify-content:space-between; gap:20px;">
          <div>
            <span style="font-size:11.5px; text-transform:uppercase; letter-spacing:0.08em; color:#AED6FF; font-weight:800;">Gestión Académica UNC</span>
            <h3 style="color:#FFFFFF; margin:10px 0 12px; font-size:22px; font-weight:800;">Accesos Estudiantiles</h3>
            <p style="font-size:14.5px; line-height:1.6; color:#E6EFF6; margin:0 0 16px;">
              Consulta tus horarios, inscripciones, calificaciones y aulas virtuales directamente desde las plataformas oficiales de la universidad.
            </p>
            <div style="display:flex; flex-direction:column; gap:10px;">
              <a href="https://tepuy.unc.edu.ve" target="_blank" rel="noopener" class="button-s" style="background:#FFFFFF; color:#031F80 !important; font-weight:800; text-align:center;">
                Ingresar a Tepuy (Notas e Inscripciones) ↗
              </a>
              <a href="https://savia.unc.edu.ve" target="_blank" rel="noopener" class="button-s" style="background:rgba(255,255,255,0.14); color:#FFFFFF !important; border:1.5px solid rgba(255,255,255,0.45); font-weight:800; text-align:center;">
                Ingresar a SAVIA (Aula Virtual) ↗
              </a>
            </div>
          </div>
          <div style="font-size:12.5px; color:#C7DBF0; border-top:1px solid rgba(255,255,255,0.18); padding-top:12px;">
            Atención presencial: Altos de Pipe, Km 11 Carretera Panamericana · Lunes a Viernes.
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
"#;

    render_page_shell("Calendario Académico 2026 – 2027", "calendar", body)
}

pub fn render_campus_page(facility_title: &str) -> String {
    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <a href="/campus-laboratorios">Campus e Investigación</a> <span>/</span> <span>{facility_title}</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <h1>{facility_title} — Campus Altos de Pipe</h1>
  <hr class="blue-line">

  <div class="row">
    <div class="col-12 col-md-7 content-text-justify">
      <p>
        El campus de la <strong>Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC)</strong>, ubicado en el Sector Altos de Pipe (kilómetro 11 de la carretera Panamericana, estado Miranda), integra aulas interactivas de cristal azul, laboratorios de computación avanzada y centros de investigación científica en articulación directa con el <strong>Instituto Venezolano de Investigaciones Científicas (IVIC)</strong>.
      </p>
      <div class="pillars-grid" style="grid-template-columns:1fr; margin-top:20px;">
        <div class="white-block-card">
          <div>
            <h3><a href="/centros-de-investigacion">Centros de Investigación UNC – IVIC</a></h3>
            <p>El 32 % del cuerpo docente de la UNC está conformado por investigadores activos del IVIC en física nuclear, nanotecnología, biomateriales, ciencia molecular, ecología del paisaje y clima, y química computacional.</p>
          </div>
        </div>
        <div class="white-block-card">
          <div>
            <h3><a href="/laboratorios-de-computacion-avanzados">Laboratorios de Computación Avanzados</a></h3>
            <p>Infraestructura especializada para el entrenamiento de modelos de Inteligencia Artificial, simulación cuántica y molecular, ciberseguridad defensiva, ciencia de datos y robótica autónoma.</p>
          </div>
        </div>
        <div class="white-block-card">
          <div>
            <h3><a href="/aulas-interactivas">Aulas Interactivas («La Casita de Cristal Azul»)</a></h3>
            <p>Espacios docentes multimodales diseñados para el aprendizaje colaborativo, el Club de Debate Científico, foros de pensamiento crítico y el Curso de Ampliación en Ciencias de las Telecomunicaciones (UNC–FANB–CENDIT).</p>
          </div>
        </div>
      </div>
    </div>

    <div class="col-12 col-md-5">
      <div class="video-block">
        <img src="/assets/real/campus_unc.jpg" alt="{facility_title}" style="width:100%; max-height:480px; object-fit:cover;">
      </div>
    </div>
  </div>
</div>
"#,
        facility_title = facility_title
    );

    render_page_shell(facility_title, "campus", &body)
}

pub fn render_validacion_page() -> String {
    let body = r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <span>Estudiantes</span> <span>/</span> <span>Validación de Documentos</span>
  </div>
</div>

<div class="container" style="padding: 40px 20px 110px;">
  <h1 style="margin:0 0 12px;">Validación de Documentos</h1>
  <hr class="blue-line">

  <div style="margin-top:36px; padding:56px 28px; text-align:center; background:#F4F8FC; border:1.5px solid #D5E0EE; border-radius:14px; max-width:720px;">
    <span style="display:inline-block; background:#031F80; color:#FFFFFF; font-size:12px; font-weight:800; letter-spacing:0.08em; text-transform:uppercase; padding:5px 14px; border-radius:999px; margin-bottom:16px;">En desarrollo</span>
    <h2 style="margin:0 0 10px; font-size:24px; font-weight:800; color:#031F80;">Sección en desarrollo</h2>
    <p style="margin:0; font-size:15.5px; line-height:1.65; color:#4E6280;">
      El módulo de Validación de Documentos se encuentra actualmente en desarrollo. Próximamente estará disponible para la comunidad estudiantil de la UNC.
    </p>
  </div>
</div>
"#;

    render_page_shell("Validación de Documentos", "services", body)
}

pub fn render_sugerencias_page(submitted: bool) -> String {
    let notice = if submitted {
        r#"<div style="background:var(--unc-blue); color:#FFFFFF; padding:14px 18px; margin-bottom:20px; font-weight:600;">
          ✓ Su sugerencia ha sido registrada satisfactoriamente en el sistema de Atención Ciudadana y Estudiantil de la UNC.
        </div>"#
    } else {
        ""
    };

    let body = format!(
        r#"
<div class="breadcrumbs-bar">
  <div class="container breadcrumbs">
    <a href="/">Inicio</a> <span>/</span> <span>Sugerencias y Ubicación</span>
  </div>
</div>

<div class="container" style="padding-bottom:50px;">
  <h1>Buzón de Sugerencias y Ubicación del Campus</h1>
  <hr class="blue-line">
  {notice}

  <div class="row">
    <div class="col-12 col-md-6">
      <h3>Envíanos tus Sugerencias</h3>
      <form action="/sugerencias" method="GET">
        <input type="hidden" name="enviado" value="1">
        <div class="dvfu-form-group">
          <label>Nombre y Apellidos *</label>
          <input type="text" name="nombre" class="dvfu-input" required placeholder="Ej. Samuel Vivas">
        </div>
        <div class="dvfu-form-group">
          <label>Correo Electrónico *</label>
          <input type="email" name="correo" class="dvfu-input" required placeholder="usuario@correo.com">
        </div>
        <div class="dvfu-form-group">
          <label>Número de Teléfono *</label>
          <input type="text" name="telefono" class="dvfu-input" required placeholder="+58 412 0000000">
        </div>
        <div class="dvfu-form-group">
          <label>Sugerencias / Mensaje *</label>
          <textarea name="mensaje" rows="5" class="dvfu-textarea" required placeholder="Escribe tu consulta o sugerencia para la UNC..."></textarea>
        </div>
        <button type="submit" class="button-s">Enviar Sugerencia</button>
      </form>
    </div>

    <div class="col-12 col-md-6">
      <h3>Ubicación Geográfica — Altos de Pipe</h3>
      <div class="video-block">
        <iframe
          src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3924.427025744987!2d-66.96833322690968!3d10.38761836624896!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x8c2af500676561e3%3A0xe37e7076ff445089!2sUniversidad%20de%20las%20Ciencias%20%E2%80%9CDr%20Humberto%20Fern%C3%A1ndez%20Mor%C3%A1n%E2%80%9D!5e0!3m2!1ses-419!2sve!4v1763933659405!5m2!1ses-419!2sve"
          width="100%"
          height="360"
          style="border:0;"
          allowfullscreen=""
          loading="lazy"
          referrerpolicy="no-referrer-when-downgrade"
          title="Mapa Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán">
        </iframe>
      </div>
      <p style="font-size:13.5px; color:var(--itmoru-muted); margin-top:12px;">
        <b>Dirección Oficial:</b> Sector Altos de Pipe, kilómetro 11 de la carretera Panamericana, estado Miranda, Venezuela.
      </p>
    </div>
  </div>
</div>
"#,
        notice = notice
    );

    render_page_shell("Sugerencias y Contacto", "contact", &body)
}
