pub const DVFU_STYLESHEET: &str = r#"
/* ============================================================================
   SELF-HOSTED LOCAL RUSSIAN STEM UNIVERSITY FONT: MANROPE (Variable 400–800)
   Designed by Mikhail Sharanda — Instant 0ms Local WOFF2 Delivery
   ============================================================================ */
@font-face {
  font-family: 'Manrope';
  font-style: normal;
  font-weight: 400 800;
  font-display: swap;
  src: url('/assets/fonts/manrope-latin-ext.woff2') format('woff2');
  unicode-range: U+0100-02BA, U+02BD-02C5, U+02C7-02CC, U+02CE-02D7, U+02DD-02FF, U+0304, U+0308, U+0329, U+1D00-1DBF, U+1E00-1E9F, U+1EF2-1EFF, U+2020, U+20A0-20AB, U+20AD-20C4, U+2113, U+2C60-2C7F, U+A720-A7FF;
}

@font-face {
  font-family: 'Manrope';
  font-style: normal;
  font-weight: 400 800;
  font-display: swap;
  src: url('/assets/fonts/manrope-latin.woff2') format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}

@font-face {
  font-family: 'Caveat';
  font-style: normal;
  font-weight: 400 700;
  font-display: swap;
  src: url('/assets/fonts/caveat-latin-ext.woff2') format('woff2');
  unicode-range: U+0100-02BA, U+02BD-02C5, U+02C7-02CC, U+02CE-02D7, U+02DD-02FF, U+0304, U+0308, U+0329, U+1D00-1DBF, U+1E00-1E9F, U+1EF2-1EFF, U+2020, U+20A0-20AB, U+20AD-20C4, U+2113, U+2C60-2C7F, U+A720-A7FF;
}

@font-face {
  font-family: 'Caveat';
  font-style: normal;
  font-weight: 400 700;
  font-display: swap;
  src: url('/assets/fonts/caveat-latin.woff2') format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}

/* ============================================================================
   UNC EDITORIAL DESIGN SYSTEM (Russian STEM Research University Typography)
   100% Authentic UNC Blue Palette (https://unc.edu.ve) — Zero Purple / Zero ITMO
   Complete Light Theme (#FFFFFF) & Pure-Black (#000000) Dark Theme
   ============================================================================ */

:root,
[data-theme="light"] {
  --unc-navy: #031F80;
  --unc-royal: #0B3C9D;
  --unc-blue: #0072AC;
  --unc-sky: #7BA6D0;
  --unc-gold: #F59E0B;

  --itmoru-bg: #FFFFFF;
  --itmoru-bg-soft: #F3F7FB;
  --itmoru-ink: #0B1936;
  --itmoru-pure-ink: #000000;
  --itmoru-muted: #526480;
  --itmoru-meta: #64748B;
  --itmoru-border: #031F80;
  --itmoru-border-subtle: #D5E0EE;
  --itmoru-dark-band: #031F80;
  --itmoru-subfooter: #021456;
  --itmoru-card-bg: #FFFFFF;
  --itmoru-modal-bg: #FFFFFF;

  /* Hollow Outlined H2 Tokens (Light Surface) */
  --itmoru-outline-fill: #FFFFFF;
  --itmoru-outline-stroke: #031F80;
}

[data-theme="dark"] {
  --unc-navy: #04256E;
  --unc-royal: #1454C4;
  --unc-blue: #0088CC;
  --unc-sky: #8EB8E5;
  --unc-gold: #F59E0B;

  --itmoru-bg: #000000;
  --itmoru-bg-soft: #050914;
  --itmoru-ink: #FFFFFF;
  --itmoru-pure-ink: #FFFFFF;
  --itmoru-muted: #A3B8D7;
  --itmoru-meta: #94A3B8;
  --itmoru-border: #FFFFFF;
  --itmoru-border-subtle: #1E293B;
  --itmoru-dark-band: #031542;
  --itmoru-subfooter: #000000;
  --itmoru-card-bg: #050A14;
  --itmoru-modal-bg: #070E1C;

  /* Hollow Outlined H2 Tokens (Pure-Black Dark Surface) */
  --itmoru-outline-fill: #000000;
  --itmoru-outline-stroke: #FFFFFF;
}

* {
  box-sizing: border-box;
}

html, body {
  margin: 0;
  padding: 0;
  min-width: 300px;
  overflow-x: clip;
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 16.5px;
  font-weight: 400;
  line-height: 1.62;
  background-color: var(--itmoru-bg);
  color: var(--itmoru-ink);
  -webkit-font-smoothing: antialiased;
  -webkit-text-size-adjust: 100%;
  transition: background-color 0.25s ease, color 0.25s ease;
}

h1, h2, h3, .unc-serif {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

a {
  color: var(--itmoru-ink);
  text-decoration: none;
  transition: color 0.2s ease, background-color 0.2s ease, border-color 0.2s ease, transform 0.2s ease;
}

a:hover {
  color: var(--unc-blue);
}

/* Official Mincyt Logo Watermark in Page Background */
.page-mincyt-bg-watermark {
  position: fixed;
  right: 24px;
  bottom: 20px;
  width: 240px;
  height: auto;
  opacity: 0.08;
  pointer-events: none;
  z-index: 0;
}

.watermark-dark-only {
  display: none;
}

[data-theme="dark"] .watermark-light-only {
  display: none;
}

[data-theme="dark"] .watermark-dark-only {
  display: block;
  opacity: 0.14;
}

.itmoru-layout-shell {
  padding-right: 0;
  position: relative;
  z-index: 1;
}

.itmoru-right-rails {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  width: 180px;
  display: flex;
  z-index: 1100;
  box-shadow: -4px 0 24px rgba(4, 37, 110, 0.22);
}

.itmoru-rail-strip {
  width: 90px;
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  padding: 28px 0 36px;
  color: #FFFFFF !important;
  cursor: pointer;
  border: none;
  text-decoration: none;
  position: relative;
  overflow: hidden;
  transition: width 0.25s ease, filter 0.25s ease;
}

.itmoru-rail-strip:hover {
  filter: brightness(1.12);
  color: #FFFFFF !important;
}

.itmoru-rail-strip.rail-news {
  background: linear-gradient(0deg, #04256E 0%, #0072AC 55%, #4CA3D9 100%);
}

.itmoru-rail-strip.rail-menu {
  background: linear-gradient(0deg, #02133B 10%, #04256E 65%, #0B3C9D 100%);
  border-left: 1px solid rgba(255, 255, 255, 0.14);
}

.itmoru-rail-strip.rail-home {
  background: linear-gradient(180deg, #0072AC 0%, #04256E 60%, #02133B 100%);
}

.itmoru-rail-icon {
  width: 44px;
  height: 44px;
  color: #FFFFFF;
}

.itmoru-rail-label {
  font-size: 28px;
  font-weight: 900;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: #FFFFFF;
  writing-mode: vertical-lr;
  transform: rotate(180deg);
  white-space: nowrap;
}

@media (max-width: 1024px) {
  .itmoru-layout-shell {
    padding-right: 0;
  }
  .itmoru-right-rails {
    position: sticky;
    top: 0;
    width: 100%;
    height: 54px;
    flex-direction: row;
  }
  .itmoru-rail-strip {
    width: 50%;
    height: 54px;
    flex-direction: row;
    padding: 0 20px;
    justify-content: center;
    gap: 12px;
  }
  .itmoru-rail-label {
    writing-mode: horizontal-tb;
    transform: none;
    font-size: 16px;
  }
  .itmoru-rail-icon {
    width: 24px;
    height: 24px;
  }
  .page-mincyt-bg-watermark {
    right: 20px;
  }
}

/* ============================================================================
   2. FULL-SCREEN MEGA-MENU OVERLAY (UNC Deep Navy & Pure Black + Dashed Boxes)
   ============================================================================ */
.itmoru-megamenu-overlay {
  display: none;
  position: fixed;
  inset: 0;
  background-color: #02133B;
  color: #FFFFFF;
  z-index: 1200;
  overflow-y: auto;
  padding: 40px 64px 60px 64px;
}

[data-theme="dark"] .itmoru-megamenu-overlay {
  background-color: #000000;
}

.itmoru-megamenu-overlay.open {
  display: block;
}

.itmoru-mega-left-rail {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  width: 90px;
  background: linear-gradient(180deg, #0072AC 0%, #04256E 60%, #02133B 100%);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  padding: 28px 0 36px;
  color: #FFFFFF;
  cursor: pointer;
  border: none;
}

.itmoru-mega-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 20px;
  padding-bottom: 28px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.25);
  margin-bottom: 36px;
}

.itmoru-mega-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 36px;
}

@media (max-width: 900px) {
  .itmoru-mega-grid { grid-template-columns: 1fr; }
  .itmoru-megamenu-overlay { padding: 24px 20px 40px 20px; }
  .itmoru-mega-left-rail { display: none; }
}

.itmoru-mega-col h3 {
  font-size: 26px;
  font-weight: 900;
  text-transform: uppercase;
  color: #7BA6D0;
  margin: 0 0 20px;
  letter-spacing: 0.03em;
}

.itmoru-dashed-links {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.itmoru-dashed-link {
  display: block;
  padding: 12px 18px;
  border: 1px dashed rgba(255, 255, 255, 0.75);
  border-radius: 8px;
  color: #FFFFFF !important;
  font-size: 15px;
  font-weight: 600;
  transition: background-color 0.2s ease, border-color 0.2s ease, transform 0.2s ease;
}

.itmoru-dashed-link:hover {
  background-color: #0072AC;
  border-style: solid;
  border-color: #FFFFFF;
  transform: translateX(4px);
}

/* ============================================================================
   3. CONTAINER, GRID & TOP UTILITY HEADER (EXPANDED 1920x1080 WIDE-SCREEN SCALE)
   ============================================================================ */
.container, .wrapper {
  width: 100%;
  max-width: 1740px;
  margin: 0 auto;
  padding: 0 56px;
}

@media (max-width: 1024px) {
  .container, .wrapper { padding: 0 32px; }
}

@media (max-width: 768px) {
  .container, .wrapper { padding: 0 20px; }
}

.row {
  display: flex;
  flex-wrap: wrap;
  margin-left: -18px;
  margin-right: -18px;
}

.col-3, .col-4, .col-6, .col-8, .col-9, .col-12 {
  padding-left: 18px;
  padding-right: 18px;
  width: 100%;
}

@media (min-width: 768px) {
  .col-sm-6 { width: 50%; }
  .col-sm-4 { width: 33.333333%; }
}

@media (min-width: 992px) {
  .col-md-3 { width: 25%; }
  .col-md-4 { width: 33.333333%; }
  .col-md-5 { width: 41.666667%; }
  .col-md-6 { width: 50%; }
  .col-md-7 { width: 58.333333%; }
  .col-md-8 { width: 66.666667%; }
  .col-md-9 { width: 75%; }
  .col-md-12 { width: 100%; }
}

.itmoru-header {
  padding: 0;
  border-bottom: none;
}

.itmoru-header-tier1-band {
  background-color: #031F80;
  color: #FFFFFF;
  padding: 20px 0;
  border-bottom: 3px solid #0072AC;
}

.itmoru-header-tier1 {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 24px;
}

.itmoru-header-tier2-band {
  background-color: #FFFFFF;
  border-bottom: 1.5px solid var(--itmoru-border-subtle);
  position: sticky;
  top: 0;
  z-index: 900;
  box-shadow: 0 4px 16px rgba(3, 31, 128, 0.05);
}

.itmoru-header-tier2 {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.unc-main-nav {
  display: flex;
  align-items: center;
  gap: 6px;
  list-style: none;
  margin: 0;
  padding: 0;
  flex-wrap: wrap;
}

.unc-nav-item {
  position: relative;
}

.unc-nav-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 16px 18px;
  font-size: 15.5px;
  font-weight: 700;
  color: #0B1936 !important;
  border-bottom: 3px solid transparent;
  transition: color 0.18s ease, border-color 0.18s ease, background-color 0.18s ease;
  white-space: nowrap;
}

.unc-nav-link .caret {
  font-size: 11px;
  opacity: 0.65;
  transition: transform 0.2s ease;
}

.unc-nav-item:hover > .unc-nav-link,
.unc-nav-link.active {
  color: #031F80 !important;
  border-bottom-color: #0072AC;
  background-color: #F4F8FC;
}

.unc-nav-item:hover > .unc-nav-link .caret {
  transform: rotate(180deg);
}

.unc-nav-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  min-width: 290px;
  background-color: #FFFFFF;
  border: 1px solid #D5E0EE;
  border-top: 3px solid #031F80;
  border-radius: 0 0 10px 10px;
  box-shadow: 0 18px 38px rgba(3, 31, 128, 0.14);
  padding: 10px;
  opacity: 0;
  visibility: hidden;
  transform: translateY(6px);
  transition: opacity 0.18s ease, transform 0.18s ease, visibility 0.18s ease;
  z-index: 950;
}

.unc-nav-item:hover .unc-nav-dropdown,
.unc-nav-item:focus-within .unc-nav-dropdown {
  opacity: 1;
  visibility: visible;
  transform: translateY(0);
}

.unc-drop-link {
  display: flex;
  flex-direction: column;
  padding: 10px 14px;
  border-radius: 6px;
  color: #0B1936 !important;
  transition: background-color 0.15s ease, color 0.15s ease;
}

.unc-drop-link strong {
  font-size: 14.5px;
  font-weight: 700;
  color: #031F80;
}

.unc-drop-link span {
  font-size: 12.5px;
  color: #526480;
  margin-top: 2px;
}

.unc-drop-link:hover {
  background-color: #EEF5FC;
}

.unc-nav-menu-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 9px 18px;
  border-radius: 8px;
  border: 1.5px solid #031F80;
  background-color: #031F80;
  color: #FFFFFF !important;
  font-family: inherit;
  font-size: 14.5px;
  font-weight: 700;
  cursor: pointer;
  white-space: nowrap;
  transition: background-color 0.2s ease, border-color 0.2s ease;
}

.unc-nav-menu-btn:hover {
  background-color: #0072AC;
  border-color: #0072AC;
}

/* Official UNC Platform Quick-Access Buttons in Navbar (Tepuy & Savia) */
.unc-platform-links {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}

.unc-platform-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 7px 14px;
  border-radius: 8px;
  border: 1.5px solid #D5E0EE;
  background-color: #F4F8FC;
  color: #031F80 !important;
  font-size: 14.5px;
  font-weight: 800;
  white-space: nowrap;
  transition: background-color 0.2s ease, border-color 0.2s ease, color 0.2s ease, transform 0.15s ease, box-shadow 0.2s ease;
}

.unc-platform-btn img {
  width: 22px;
  height: 22px;
  object-fit: contain;
  display: block;
  flex-shrink: 0;
}

.unc-platform-btn--savia {
  min-height: 39px;
  padding: 7px 15px;
}

.unc-platform-btn img.unc-savia-wordmark {
  width: auto;
  height: 17px;
  transition: filter 0.2s ease;
}

.unc-platform-btn:hover {
  background-color: #031F80;
  border-color: #031F80;
  color: #FFFFFF !important;
  transform: translateY(-1px);
  box-shadow: 0 6px 14px rgba(3, 31, 128, 0.18);
}

.unc-platform-btn:hover img.unc-savia-wordmark {
  filter: brightness(0) invert(1);
}

.itmoru-brand-row {
  display: flex;
  align-items: center;
  gap: 28px;
  flex-wrap: wrap;
}

.itmoru-logo-link {
  display: flex;
  align-items: center;
  gap: 18px;
  text-decoration: none;
}

.itmoru-seal-img,
.itmoru-full-logo-img {
  height: 78px;
  width: auto;
  object-fit: contain;
  display: block;
  background: transparent;
  filter: brightness(0) invert(1);
  transition: transform 0.2s ease;
}

.itmoru-logo-link:hover .itmoru-full-logo-img {
  transform: scale(1.015);
}

.unc-logo-white-on-dark {
  background: transparent !important;
  filter: brightness(0) invert(1);
}

.unc-logo-transparent {
  background: transparent !important;
}

.itmoru-brand-text {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 2px;
}

.itmoru-slash-wordmark {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 42px;
  font-weight: 800;
  letter-spacing: 0.03em;
  line-height: 1;
  color: #FFFFFF;
  display: flex;
  align-items: center;
}

.itmoru-brand-subtitle {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 15px;
  font-weight: 600;
  line-height: 1.25;
  color: #C7DBF0;
  letter-spacing: 0.01em;
}

.itmoru-address-tag {
  margin-left: auto;
  text-align: right;
  padding-right: 20px;
  border-right: 3px solid #7BA6D0;
  line-height: 1.4;
}

.itmoru-address-tag .addr-k {
  display: block;
  font-size: 13.5px;
  color: #C7DBF0;
  font-weight: 600;
}

.itmoru-address-tag .addr-v {
  display: block;
  font-size: 16px;
  font-weight: 800;
  color: #FFFFFF;
}

.itmoru-header-controls {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.itmoru-quick-pills {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  width: 100%;
}

.itmoru-pill-tab {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 9px 22px;
  border-radius: 36px;
  border: 1.5px solid var(--itmoru-border);
  font-size: 15px;
  font-weight: 700;
  color: var(--itmoru-ink) !important;
  transition: background-color 0.2s ease, color 0.2s ease, border-color 0.2s ease, transform 0.15s ease;
}

.itmoru-pill-tab:hover,
.itmoru-pill-tab.active {
  background-color: var(--unc-navy);
  border-color: var(--unc-navy);
  color: #FFFFFF !important;
  transform: translateY(-1px);
}

.itmoru-search-box {
  display: flex;
  align-items: center;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-radius: 10px;
  height: 46px;
  padding: 0 16px;
  background-color: #FFFFFF;
}

.itmoru-search-box input {
  border: none;
  background: transparent;
  color: #031F80;
  font-family: inherit;
  font-size: 15px;
  font-weight: 600;
  width: 280px;
  outline: none;
}

.itmoru-search-box input::placeholder {
  color: #526480;
  font-weight: 500;
}

.itmoru-search-box button {
  border: none;
  background: transparent;
  color: #031F80;
  cursor: pointer;
  display: flex;
  align-items: center;
}

/* ============================================================================
   4. PRESTIGIOUS UNIVERSITY HERO (Stanford / Harvard Academic Editorial Style)
   ============================================================================ */
.itmoru-hero {
  padding: 52px 0 44px;
}

.unc-hero-grid {
  display: grid;
  grid-template-columns: 1.2fr 1fr;
  gap: 56px;
  align-items: center;
  margin-bottom: 56px;
}

@media (max-width: 1080px) {
  .unc-hero-grid {
    grid-template-columns: 1fr;
    gap: 36px;
  }
}

.unc-hero-eyebrow {
  display: inline-block;
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 13px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.14em;
  color: var(--unc-blue);
  margin-bottom: 16px;
  padding-bottom: 6px;
  border-bottom: 2px solid var(--unc-blue);
}

.itmoru-giant-statement {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: clamp(48px, 5.4vw, 84px);
  font-weight: 800;
  line-height: 1.04;
  color: var(--unc-navy);
  letter-spacing: -0.02em;
  margin: 0 0 24px;
}

[data-theme="dark"] .itmoru-giant-statement {
  color: #FFFFFF;
}

.itmoru-giant-statement .unc-hero-brand-pop {
  font-size: clamp(66px, 7.2vw, 108px);
  font-weight: 900;
  letter-spacing: -0.035em;
  color: #031F80;
  line-height: 0.92;
  display: inline-block;
}

.itmoru-giant-statement .cursive-span {
  font-family: 'Caveat', cursive;
  font-size: clamp(62px, 6.6vw, 98px);
  font-weight: 700;
  font-style: normal;
  letter-spacing: 0.01em;
  padding: 0 8px 0 6px;
  color: var(--unc-blue);
  display: inline-block;
  transform: rotate(-3deg);
  vertical-align: 0.04em;
  line-height: 0.85;
}

.itmoru-giant-statement .unc-hero-tagline-line {
  display: inline-block;
  font-size: clamp(34px, 3.85vw, 56px);
  font-weight: 800;
  line-height: 1.08;
  letter-spacing: -0.025em;
  margin-top: 6px;
}

.unc-hero-lead {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 20px;
  line-height: 1.6;
  color: var(--itmoru-muted);
  margin: 0 0 32px;
  max-width: 680px;
}

.unc-hero-actions {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.unc-btn-primary {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 15px 30px;
  border-radius: 6px;
  background-color: var(--unc-navy);
  border: 2px solid var(--unc-navy);
  color: #FFFFFF !important;
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 0.01em;
  transition: background-color 0.2s ease, border-color 0.2s ease, transform 0.15s ease;
}

.unc-btn-primary:hover {
  background-color: var(--unc-blue);
  border-color: var(--unc-blue);
  transform: translateY(-2px);
}

[data-theme="dark"] .unc-btn-primary {
  background-color: var(--unc-blue);
  border-color: var(--unc-blue);
}

[data-theme="dark"] .unc-btn-primary:hover {
  background-color: #1454C4;
  border-color: #1454C4;
}

.unc-btn-secondary {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 15px 28px;
  border-radius: 6px;
  background-color: transparent;
  border: 2px solid var(--itmoru-border);
  color: var(--itmoru-ink) !important;
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 16px;
  font-weight: 700;
  transition: background-color 0.2s ease, color 0.2s ease, border-color 0.2s ease, transform 0.15s ease;
}

.unc-btn-secondary:hover {
  background-color: var(--unc-navy);
  border-color: var(--unc-navy);
  color: #FFFFFF !important;
  transform: translateY(-2px);
}

[data-theme="dark"] .unc-btn-secondary:hover {
  background-color: #FFFFFF;
  border-color: #FFFFFF;
  color: #000000 !important;
}

/* Architectural Campus Showcase Card (Replaces old ITMO retro OS window) */
.unc-campus-showcase {
  border: 1.5px solid var(--itmoru-border-subtle);
  background-color: var(--itmoru-card-bg);
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 18px 42px rgba(4, 37, 110, 0.10);
}

[data-theme="dark"] .unc-campus-showcase {
  border-color: #1E293B;
  box-shadow: 0 18px 42px rgba(0, 114, 172, 0.14);
}

.unc-campus-photo-wrap {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  height: auto;
  overflow: hidden;
  background-color: var(--itmoru-bg-soft);
}

.unc-carousel-slide {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.65s ease-in-out;
}

.unc-carousel-slide.active {
  opacity: 1;
  pointer-events: auto;
  z-index: 1;
}

.unc-carousel-slide img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  display: block;
}

.unc-carousel-btn {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  z-index: 4;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: 1px solid rgba(255, 255, 255, 0.45);
  background: rgba(4, 37, 110, 0.58);
  backdrop-filter: blur(6px);
  color: #FFFFFF;
  font-size: 22px;
  line-height: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  opacity: 0.85;
  transition: background-color 0.2s ease, opacity 0.2s ease, transform 0.2s ease;
}

.unc-carousel-btn:hover {
  background: rgba(0, 114, 172, 0.92);
  opacity: 1;
  transform: translateY(-50%) scale(1.06);
}

.unc-carousel-btn.prev { left: 14px; }
.unc-carousel-btn.next { right: 14px; }

.unc-carousel-dots {
  position: absolute;
  left: 50%;
  bottom: 14px;
  transform: translateX(-50%);
  z-index: 4;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  border-radius: 999px;
  background: rgba(2, 19, 59, 0.48);
  backdrop-filter: blur(6px);
}

.unc-carousel-dot {
  width: 8px;
  height: 8px;
  border-radius: 999px;
  border: none;
  padding: 0;
  background: rgba(255, 255, 255, 0.55);
  cursor: pointer;
  transition: width 0.25s ease, background-color 0.25s ease;
}

.unc-carousel-dot.active {
  width: 24px;
  background: #FFFFFF;
}

.unc-campus-photo-overlay {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, rgba(4, 37, 110, 0.05) 30%, rgba(2, 24, 75, 0.88) 100%);
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  padding: 26px 28px;
  color: #FFFFFF;
}

.unc-campus-photo-overlay .campus-kicker {
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: #8EB8E5;
  margin-bottom: 4px;
}

.unc-campus-photo-overlay .campus-title {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 24px;
  font-weight: 700;
  line-height: 1.25;
  margin: 0;
  color: #FFFFFF;
}

.unc-campus-metrics-bar {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  border-top: 1px solid var(--itmoru-border-subtle);
}

.unc-campus-metric {
  padding: 18px 20px;
  border-right: 1px solid var(--itmoru-border-subtle);
}

.unc-campus-metric:last-child {
  border-right: none;
}

.unc-campus-metric .m-val {
  display: block;
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 26px;
  font-weight: 800;
  color: var(--unc-navy);
  line-height: 1.1;
}

[data-theme="dark"] .unc-campus-metric .m-val {
  color: #FFFFFF;
}

.unc-campus-metric .m-lbl {
  display: block;
  font-size: 13px;
  font-weight: 600;
  color: var(--itmoru-muted);
  margin-top: 4px;
}

.unc-campus-footer-link {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 24px;
  background-color: var(--itmoru-bg-soft);
  border-top: 1px solid var(--itmoru-border-subtle);
  font-size: 14.5px;
  font-weight: 700;
  color: var(--unc-navy) !important;
}

[data-theme="dark"] .unc-campus-footer-link {
  color: #8EB8E5 !important;
}

.unc-campus-footer-link:hover {
  background-color: var(--unc-navy);
  color: #FFFFFF !important;
}

/* ActionItem (Clean Academic Outline Button in UNC Blue) */
.itmoru-action-item {
  display: inline-block;
  border: 2px solid var(--itmoru-border);
  border-radius: 6px;
  padding: 0;
  background-color: var(--itmoru-bg);
  color: var(--itmoru-ink) !important;
  font-weight: 700;
  font-size: 15.5px;
  cursor: pointer;
  transition: transform 0.15s ease, background-color 0.2s ease, border-color 0.2s ease;
}

.itmoru-action-item span {
  display: block;
  border: none;
  padding: 11px 24px;
}

.itmoru-action-item:hover {
  background-color: var(--unc-navy);
  border-color: var(--unc-navy);
  color: #FFFFFF !important;
  transform: translateY(-2px);
}

[data-theme="dark"] .itmoru-action-item:hover {
  background-color: var(--unc-blue);
  border-color: var(--unc-blue);
}

/* ============================================================================
   5. PRESTIGIOUS ACADEMIC SERIF SECTION HEADINGS (Stanford / Harvard Style)
   ============================================================================ */
.itmoru-hollow-h2 {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: clamp(34px, 3.8vw, 56px);
  font-weight: 800;
  text-transform: none;
  line-height: 1.1;
  letter-spacing: -0.015em;
  margin: 0 0 28px;
  color: var(--unc-navy);
  text-shadow: none;
  position: relative;
  padding-bottom: 12px;
}

.itmoru-hollow-h2::after {
  content: "";
  position: absolute;
  left: 0;
  bottom: 0;
  width: 68px;
  height: 4px;
  background-color: var(--unc-blue);
}

[data-theme="dark"] .itmoru-hollow-h2 {
  color: #FFFFFF;
}

.itmoru-dark-band .itmoru-hollow-h2 {
  color: #FFFFFF;
  text-shadow: none;
}

.itmoru-dark-band .itmoru-hollow-h2::after {
  background-color: #7BA6D0;
}

.itmoru-section {
  padding: 72px 0;
  border-top: 2px solid var(--itmoru-border);
}

.itmoru-section-toprow {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 20px;
  margin-bottom: 36px;
}

.itmoru-slide-counter {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 20px;
  font-weight: 900;
}

.itmoru-dot {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background-color: var(--itmoru-border);
  display: inline-block;
}

.itmoru-dot.active {
  background-color: var(--unc-blue);
  transform: scale(1.25);
}

/* ============================================================================
   6. EDITORIAL NEWS GRID (1px solid UNC Navy Hairline Dividers)
   ============================================================================ */
.itmoru-news-top2 {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  border-top: 1.5px solid var(--itmoru-border);
  border-bottom: 1.5px solid var(--itmoru-border);
  margin-bottom: 44px;
}

@media (max-width: 850px) {
  .itmoru-news-top2 { grid-template-columns: 1fr; }
}

.itmoru-news-hero-item {
  padding: 36px;
  border-right: 1.5px solid var(--itmoru-border);
  display: flex;
  flex-direction: column;
}

.itmoru-news-hero-item:last-child {
  border-right: none;
}

.itmoru-news-hero-item .n-date {
  font-size: 14.5px;
  color: var(--itmoru-meta);
  text-transform: lowercase;
  margin-bottom: 14px;
  font-weight: 600;
}

.itmoru-news-hero-item .n-img-wrap {
  width: 100%;
  aspect-ratio: 3 / 2;
  height: auto;
  overflow: hidden;
  margin-bottom: 20px;
  border: 1.5px solid var(--itmoru-border);
  background-color: var(--itmoru-bg-soft);
}

.itmoru-news-hero-item .n-img-wrap img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center 20%;
  transition: transform 0.35s ease;
}

.itmoru-news-hero-item:hover .n-img-wrap img {
  transform: scale(1.04);
}

.itmoru-news-hero-item h3 {
  font-size: 26px;
  font-weight: 800;
  line-height: 1.28;
  margin: 0 0 12px;
  color: var(--itmoru-ink);
}

.itmoru-news-hero-item:hover h3 {
  color: var(--unc-blue);
}

/* Standard 4-Col News Cards (`render_news_card`) */
.post-card {
  border: 1.5px solid var(--itmoru-border);
  padding: 22px;
  margin-bottom: 32px;
  background-color: var(--itmoru-card-bg);
  display: flex;
  flex-direction: column;
  height: calc(100% - 32px);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.post-card:hover {
  transform: translateY(-4px);
  box-shadow: 8px 8px 0 var(--unc-blue);
}

.post-card .date-post {
  font-size: 13.5px;
  color: var(--itmoru-meta);
  text-transform: lowercase;
  margin-bottom: 12px;
  font-weight: 600;
  order: 1;
}

.post-card .thumb-wrap {
  position: relative;
  display: block;
  width: 100%;
  aspect-ratio: 3 / 2;
  height: auto;
  overflow: hidden;
  border: 1px solid var(--itmoru-border);
  background-color: var(--itmoru-bg-soft);
  margin-bottom: 16px;
  order: 2;
}

.post-card .thumb-wrap img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center 20%;
  transition: transform 0.35s ease;
}

.post-card:hover .thumb-wrap img {
  transform: scale(1.05);
}

.post-card .anons-badge {
  position: absolute;
  top: 10px;
  left: 10px;
  background-color: var(--unc-blue);
  color: #FFFFFF;
  font-size: 11px;
  font-weight: 700;
  text-transform: lowercase;
  padding: 4px 12px;
  border-radius: 20px;
}

.post-card .desc-post {
  font-size: 18px;
  font-weight: 700;
  line-height: 1.38;
  color: var(--itmoru-ink);
  order: 3;
}

.post-card:hover .desc-post {
  color: var(--unc-blue);
}

/* ============================================================================
   7. UNC DEEP NAVY BAND ("CIFRAS Y HECHOS" -> #04256E + Cerulean/Sky Arc)
   ============================================================================ */
.itmoru-dark-band {
  background-color: var(--itmoru-dark-band);
  color: #FFFFFF;
  padding: 64px 0;
  position: relative;
  overflow: hidden;
  border-top: 2px solid var(--itmoru-border);
  border-bottom: 2px solid var(--itmoru-border);
}

.itmoru-pink-arc-svg {
  position: absolute;
  top: -40px;
  right: -60px;
  width: 680px;
  height: 680px;
  pointer-events: none;
  opacity: 0.45;
  z-index: 0;
}

.itmoru-dark-band .container {
  position: relative;
  z-index: 2;
}

.itmoru-big-numbers-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 28px;
  margin-bottom: 48px;
}

@media (max-width: 900px) {
  .itmoru-big-numbers-row { grid-template-columns: repeat(2, 1fr); }
}

@media (max-width: 600px) {
  .itmoru-big-numbers-row { grid-template-columns: 1fr; }
}

.itmoru-outlined-stat {
  border: 1.5px solid rgba(255, 255, 255, 0.35);
  padding: 28px 26px;
  background: rgba(255, 255, 255, 0.07);
}

.itmoru-outlined-stat .num-hollow {
  font-family: 'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: clamp(46px, 4.5vw, 68px);
  font-weight: 800;
  font-variant-numeric: tabular-nums;
  line-height: 1.08;
  color: #FFFFFF;
  text-shadow: none;
  display: flex;
  align-items: flex-start;
  height: 1.08em;
  overflow: hidden;
  margin-bottom: 14px;
}

/* Buttery-Smooth 60/120fps GPU-Accelerated Rolling Odometer Digits */
.unc-odo-slot {
  display: block;
  height: 1.08em;
  max-height: 1.08em;
  overflow: hidden;
  flex-shrink: 0;
}

.unc-odo-ribbon {
  display: flex;
  flex-direction: column;
  transform: translate3d(0, 0, 0);
  will-change: transform;
}

.unc-odo-digit {
  display: block;
  height: 1.08em;
  line-height: 1.08em;
  flex-shrink: 0;
  text-align: center;
}

.unc-odo-static {
  display: block;
  height: 1.08em;
  line-height: 1.08em;
  flex-shrink: 0;
}

.itmoru-outlined-stat .num-desc {
  display: block;
  font-size: 16.5px;
  font-weight: 700;
  line-height: 1.4;
  color: #FFFFFF;
}

/* Interactive Processes Accordion with Circular #0072AC '+' Badges */
.itmoru-accordion-list {
  border-top: 1px solid rgba(255, 255, 255, 0.65);
}

.itmoru-acc-row {
  border-bottom: 1px solid rgba(255, 255, 255, 0.65);
  padding: 18px 0;
}

.itmoru-acc-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  cursor: pointer;
}

.itmoru-acc-title {
  font-size: 22px;
  font-weight: 700;
  color: #FFFFFF;
  display: flex;
  align-items: center;
  gap: 16px;
}

.itmoru-acc-plus {
  width: 38px;
  height: 38px;
  border-radius: 50%;
  background-color: var(--unc-blue);
  color: #FFFFFF;
  font-size: 24px;
  font-weight: 700;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  transition: transform 0.25s ease, background-color 0.25s ease;
}

.itmoru-acc-row.open .itmoru-acc-plus {
  transform: rotate(45deg);
  background-color: var(--unc-sky);
  color: #04256E;
}

.itmoru-acc-panel {
  display: none;
  margin-top: 16px;
  padding: 20px 24px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px dashed rgba(255, 255, 255, 0.55);
  border-radius: 10px;
  color: #E6F0FA;
  font-size: 15px;
}

.itmoru-acc-row.open .itmoru-acc-panel {
  display: flex;
  align-items: center;
  gap: 20px;
  flex-wrap: wrap;
}

/* ============================================================================
   8. BENTO GRID & INTERACTIVE #0072AC LEFT-BAR LIST (Building Background Kept, No Career Logos)
   ============================================================================ */
.banners-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 28px;
}

@media (max-width: 1100px) {
  .banners-grid { grid-template-columns: repeat(2, 1fr); }
}

@media (max-width: 600px) {
  .banners-grid { grid-template-columns: 1fr; }
}

.itmoru-bento-card {
  position: relative;
  height: 360px;
  border: 2px solid var(--itmoru-border);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 28px;
  color: #FFFFFF !important;
  background-color: #04256E;
  transition: transform 0.25s ease, box-shadow 0.25s ease;
}

.itmoru-bento-card:hover {
  transform: translateY(-4px);
  box-shadow: 8px 8px 0 var(--unc-blue);
  color: #FFFFFF !important;
}

.itmoru-bento-card img.bento-bg {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center top;
  z-index: 0;
  transition: transform 0.45s ease;
}

.itmoru-bento-card:hover img.bento-bg {
  transform: scale(1.07);
}

/* Authentic UNC Navy & Cerulean Gradient Overlay over Campus Building Photo */
.itmoru-bento-card::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, rgba(4, 37, 110, 0.78) 0%, rgba(0, 114, 172, 0.45) 55%, rgba(2, 24, 75, 0.92) 100%);
  z-index: 1;
  pointer-events: none;
}

.itmoru-bento-top, .itmoru-bento-bottom {
  position: relative;
  z-index: 2;
}

.itmoru-bento-pill {
  display: inline-block;
  background-color: var(--unc-blue);
  color: #FFFFFF;
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  padding: 5px 14px;
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.35);
}

.itmoru-bento-title {
  font-size: 24px;
  font-weight: 900;
  line-height: 1.22;
  color: #FFFFFF;
  margin: 0 0 10px;
  text-transform: uppercase;
}

.itmoru-bento-sub {
  font-size: 15px;
  line-height: 1.45;
  color: rgba(255, 255, 255, 0.94);
  margin: 0;
}

/* Interactive Vertical List with 8px #0072AC Left Bar */
.itmoru-vertical-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.itmoru-vlist-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 24px;
  border: 1.5px solid var(--itmoru-border);
  border-left: 10px solid var(--unc-blue);
  background-color: var(--itmoru-card-bg);
  font-size: 19px;
  font-weight: 700;
  color: var(--itmoru-ink);
  transition: transform 0.2s ease, background-color 0.2s ease;
}

.itmoru-vlist-item:hover {
  transform: translateX(6px);
  border-left-color: var(--unc-navy);
}

/* ============================================================================
   9. COMPATIBILITY CLASSES FOR INNER ROUTES (/carreras, /noticias, /autoridades, etc.)
   ============================================================================ */
.breadcrumbs-bar {
  background-color: var(--itmoru-dark-band);
  color: #FFFFFF;
  padding: 16px 0;
  margin-bottom: 28px;
  border-bottom: 3px solid var(--unc-blue);
}

.breadcrumbs {
  font-size: 13.5px;
  color: #C7DBF0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.breadcrumbs a {
  color: #FFFFFF !important;
  font-weight: 700;
}

hr.blue-line {
  border: 0;
  height: 2px;
  background-color: var(--unc-blue);
  margin: 0 0 26px 0;
}

.button-s {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  background-color: var(--unc-navy);
  color: #FFFFFF !important;
  font-size: 14px;
  font-weight: 700;
  height: 40px;
  padding: 0 22px;
  border-radius: 8px;
  border: 2px solid var(--unc-navy);
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

.button-s:hover {
  background-color: var(--unc-blue);
  border-color: var(--unc-blue);
  color: #FFFFFF !important;
}

.button-outline-dvfu {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  background-color: transparent;
  color: var(--itmoru-ink) !important;
  font-size: 14px;
  font-weight: 700;
  height: 40px;
  padding: 0 20px;
  border-radius: 8px;
  border: 2px solid var(--itmoru-border);
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

.button-outline-dvfu:hover {
  background-color: var(--unc-blue);
  border-color: var(--unc-blue);
  color: #FFFFFF !important;
}

.video-block {
  position: relative;
  padding: 16px;
  border: 1px solid var(--itmoru-border);
  background-color: var(--itmoru-card-bg);
  display: flex;
  flex-direction: column;
}

.white-block-card {
  background-color: var(--itmoru-card-bg);
  border: 1px solid var(--itmoru-border);
  border-left: 8px solid var(--unc-blue);
  padding: 20px;
  display: flex;
  gap: 16px;
  align-items: flex-start;
}

.white-block-card img.pillar-icon {
  width: 54px;
  height: 54px;
  object-fit: contain;
  flex-shrink: 0;
}

.white-block-card h3 {
  font-size: 20px;
  font-weight: 800;
  margin: 0 0 8px;
  color: var(--itmoru-ink);
}

.white-block-card p {
  margin: 0;
  font-size: 16px;
  line-height: 1.6;
  color: var(--itmoru-muted);
}

.pillars-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
}

@media (max-width: 991px) {
  .pillars-grid { grid-template-columns: 1fr; }
}

.tabs-user {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-bottom: 26px;
}

.tab-btn-dvfu {
  background-color: var(--itmoru-card-bg);
  color: var(--itmoru-ink);
  padding: 8px 20px;
  font-size: 14px;
  font-weight: 700;
  border: 1px solid var(--itmoru-border);
  border-radius: 32px;
  cursor: pointer;
}

.tab-btn-dvfu.active,
.tab-btn-dvfu:hover {
  background-color: var(--unc-blue);
  border-color: var(--unc-blue);
  color: #FFFFFF;
}

.authority-card {
  background-color: var(--itmoru-card-bg);
  border: 1px solid var(--itmoru-border);
  margin-bottom: 26px;
  display: flex;
  flex-direction: column;
  height: calc(100% - 26px);
}

.authority-card .auth-photo-wrap {
  width: 100%;
  aspect-ratio: 1 / 1;
  height: auto;
  overflow: hidden;
  background-color: var(--itmoru-bg-soft);
  border-bottom: 3px solid var(--unc-blue);
}

.authority-card .auth-photo-wrap img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center top;
  display: block;
}

.authority-card .auth-body { padding: 18px 20px; }
.authority-card .auth-role {
  font-size: 11px;
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--unc-blue);
  display: block;
  margin-bottom: 4px;
}
.authority-card .auth-name {
  font-size: 19px;
  font-weight: 800;
  margin: 0 0 8px;
  color: var(--itmoru-ink);
}
.authority-card .auth-bio {
  font-size: 13.5px;
  color: var(--itmoru-muted);
  margin: 0;
}

.dvfu-table-wrap {
  overflow-x: auto;
  border: 1px solid var(--itmoru-border);
  background-color: var(--itmoru-card-bg);
}

table.dvfu-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
}

table.dvfu-table th {
  background-color: var(--unc-navy);
  color: #FFFFFF;
  font-weight: 700;
  text-align: left;
  padding: 12px 14px;
  border-bottom: 2px solid var(--unc-blue);
}

table.dvfu-table td {
  padding: 11px 14px;
  border-bottom: 1px dashed var(--itmoru-border-subtle);
  color: var(--itmoru-ink);
}

.dvfu-form-group { margin-bottom: 16px; }
.dvfu-form-group label {
  display: block;
  font-size: 13.5px;
  font-weight: 700;
  margin-bottom: 6px;
}
.dvfu-input, .dvfu-textarea {
  width: 100%;
  padding: 10px 14px;
  font-family: inherit;
  font-size: 14px;
  color: var(--itmoru-ink);
  background-color: var(--itmoru-card-bg);
  border: 2px solid var(--itmoru-border);
  border-radius: 8px;
  outline: none;
}
.dvfu-input:focus, .dvfu-textarea:focus { border-color: var(--unc-blue); }

.banner-tile {
  position: relative;
  height: 225px;
  padding: 20px 22px;
  border: 2px solid var(--itmoru-border);
  color: #FFFFFF;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  overflow: hidden;
  background-color: #04256E;
  background-image:
    linear-gradient(180deg, rgba(4, 37, 110, 0.78) 0%, rgba(0, 114, 172, 0.5) 55%, rgba(2, 24, 75, 0.92) 100%),
    url('/assets/real/campus_unc.jpg');
  background-size: cover;
  background-position: center;
}

.banner-tile .tile-top-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 10px;
}

.banner-tile .tile-tag {
  font-size: 10.5px;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  background: var(--unc-blue);
  border: 1px solid rgba(255,255,255,0.35);
  padding: 4px 11px;
  font-weight: 700;
  border-radius: 20px;
}

/* Hide the old career badge image on inner career pages too */
.banner-tile .portal-featured-badge,
.banner-tile .tile-icon-img {
  display: none !important;
}

.banner-tile .name-banner {
  font-size: 20px;
  line-height: 24px;
  font-weight: 800;
  color: #FFFFFF;
}

.banner-tile .sub-banner {
  font-size: 13px;
  line-height: 18px;
  color: rgba(255,255,255,0.92);
  margin-top: 5px;
}

/* ============================================================================
   9B. DEDICATED VERTICAL VIDEO SPOTLIGHT (Clean 9:16 Split Showcase)
   ============================================================================ */
.unc-video-spotlight-card {
  display: grid;
  grid-template-columns: 1.35fr 0.65fr;
  gap: 48px;
  align-items: center;
  padding: 44px 52px;
  background-color: var(--itmoru-bg-soft);
  border: 1.5px solid var(--itmoru-border-subtle);
  border-left: 8px solid #031F80;
  border-radius: 14px;
}

@media (max-width: 960px) {
  .unc-video-spotlight-card {
    grid-template-columns: 1fr;
    padding: 32px 24px;
    gap: 28px;
  }
}

.unc-video-phone-frame {
  width: 100%;
  max-width: 310px;
  margin: 0 auto;
  aspect-ratio: 9 / 16;
  border-radius: 18px;
  overflow: hidden;
  background-color: #021456;
  border: 4px solid #031F80;
  box-shadow: 0 22px 48px rgba(3, 31, 128, 0.22);
}

.unc-video-phone-frame video {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

/* ============================================================================
   10. FOOTER (#031F80 UNC Navy) & SUB-FOOTER (#021456)
   ============================================================================ */
footer.itmoru-footer {
  background-color: var(--itmoru-dark-band);
  color: #FFFFFF;
  padding: 56px 0 44px;
  border-top: 2px solid var(--itmoru-border);
  position: relative;
  overflow: hidden;
}

.footer-mincyt-bg-watermark {
  position: absolute;
  right: 6%;
  bottom: 16px;
  width: 300px;
  height: auto;
  opacity: 0.12;
  pointer-events: none;
}

.itmoru-footer-topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.35);
  margin-bottom: 36px;
  flex-wrap: wrap;
  gap: 20px;
}

.itmoru-footer-cols {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 28px;
}

@media (max-width: 900px) {
  .itmoru-footer-cols { grid-template-columns: repeat(2, 1fr); }
}

@media (max-width: 575px) {
  .itmoru-footer-cols { grid-template-columns: 1fr; }
}

.itmoru-footer-office h4 {
  font-size: 14px;
  font-weight: 900;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: #7BA6D0;
  margin: 0 0 10px;
}

.itmoru-footer-office p,
.itmoru-footer-office a {
  font-size: 13.5px;
  line-height: 1.55;
  color: #D7E6F7 !important;
  margin: 0 0 8px;
  display: block;
}

.itmoru-footer-office a:hover {
  color: #FFFFFF !important;
  text-decoration: underline;
}

.itmoru-subfooter {
  background-color: var(--itmoru-subfooter);
  color: #93B0D1;
  padding: 28px 0;
  font-size: 13px;
}

.itmoru-subfooter-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 16px;
}

/* ============================================================================
   11. FLAGSHIP MOBILE OPTIMIZATION (Infinix, Tecno, Samsung, iPhone 360px–960px)
   Zero Horizontal Overflow · Touch-Friendly 48px Targets · Collapsible Drawer
   ============================================================================ */
.unc-mobile-nav-toggle {
  display: none;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border-radius: 10px;
  border: 1.5px solid rgba(255, 255, 255, 0.55);
  background: rgba(255, 255, 255, 0.12);
  color: #FFFFFF;
  font-size: 22px;
  font-weight: 700;
  cursor: pointer;
  flex-shrink: 0;
  margin-left: auto;
  -webkit-tap-highlight-color: transparent;
}

.unc-mobile-nav-toggle:active {
  background: rgba(255, 255, 255, 0.28);
}

.dvfu-table-wrap {
  -webkit-overflow-scrolling: touch;
}

@media (max-width: 960px) {
  .unc-mobile-nav-toggle {
    display: inline-flex;
  }

  .itmoru-header-tier1-band {
    padding: 14px 0 16px;
  }

  .itmoru-header-tier1 {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .itmoru-brand-row {
    width: 100%;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
  }

  .itmoru-full-logo-img {
    height: 54px;
    max-width: calc(100vw - 100px);
  }

  .itmoru-address-tag {
    width: 100%;
    margin-left: 0;
    text-align: left;
    padding-left: 12px;
    padding-right: 0;
    border-left: 2px solid #7BA6D0;
    border-right: none;
    order: 3;
  }

  .itmoru-address-tag .addr-k {
    font-size: 11.5px;
  }

  .itmoru-address-tag .addr-v {
    font-size: 13.5px;
  }

  .itmoru-header-controls {
    width: 100%;
  }

  .itmoru-search-box {
    width: 100%;
    height: 44px;
    padding: 0 14px;
  }

  .itmoru-search-box input {
    width: 100%;
    font-size: 16px; /* Prevents iPhone Safari auto-zoom on focus */
  }

  /* Collapsible Mobile Accordion Navigation Drawer */
  .itmoru-header-tier2-band {
    display: none;
    position: static;
    box-shadow: 0 10px 24px rgba(3, 31, 128, 0.12);
  }

  .itmoru-header-tier2-band.mobile-open {
    display: block;
  }

  .unc-main-nav {
    flex-direction: column;
    align-items: stretch;
    width: 100%;
    gap: 0;
    padding: 8px 0 14px;
  }

  .unc-nav-item {
    width: 100%;
    border-bottom: 1px solid var(--itmoru-border-subtle);
  }

  .unc-nav-item:last-child {
    border-bottom: none;
  }

  .unc-nav-link {
    width: 100%;
    justify-content: space-between;
    padding: 14px 8px;
    font-size: 16px;
    border-bottom: none !important;
  }

  .unc-nav-dropdown {
    position: static;
    opacity: 1;
    visibility: visible;
    transform: none;
    display: none;
    min-width: 0;
    width: 100%;
    box-shadow: none;
    border: none;
    border-left: 3px solid #0072AC;
    border-radius: 0;
    background-color: #F4F8FC;
    padding: 6px 8px 10px 12px;
    margin-bottom: 10px;
  }

  .unc-nav-item.mobile-expanded .unc-nav-dropdown {
    display: block;
  }

  .unc-nav-item.mobile-expanded > .unc-nav-link .caret {
    transform: rotate(180deg);
  }

  .unc-drop-link {
    padding: 10px 10px;
  }

  .itmoru-header-tier2 {
    flex-direction: column;
    align-items: stretch;
    gap: 0;
  }

  .unc-platform-links {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    padding: 4px 0 16px;
    width: 100%;
  }

  .unc-platform-btn {
    justify-content: center;
    padding: 11px 12px;
    font-size: 15px;
  }
}

@media (max-width: 640px) {
  .container, .wrapper {
    padding-left: max(16px, env(safe-area-inset-left));
    padding-right: max(16px, env(safe-area-inset-right));
  }

  .row {
    margin-left: -10px;
    margin-right: -10px;
  }

  .col-3, .col-4, .col-6, .col-8, .col-9, .col-12 {
    padding-left: 10px;
    padding-right: 10px;
  }

  /* Hero Mobile Polish */
  .itmoru-hero {
    padding: 28px 0 24px;
  }

  .unc-hero-grid {
    gap: 24px;
    margin-bottom: 32px;
  }

  .unc-hero-eyebrow {
    font-size: 11px;
    letter-spacing: 0.1em;
    margin-bottom: 12px;
  }

  .itmoru-giant-statement {
    font-size: clamp(34px, 9.2vw, 48px);
    line-height: 1.06;
    margin-bottom: 16px;
  }

  .unc-hero-lead {
    font-size: 16.5px;
    line-height: 1.55;
    margin-bottom: 22px;
  }

  .unc-hero-actions {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .unc-btn-primary,
  .unc-btn-secondary,
  .itmoru-action-item {
    width: 100%;
    text-align: center;
    padding: 13px 18px;
    font-size: 15px;
  }

  .itmoru-action-item span {
    padding: 4px 8px;
  }

  /* Carousel Controls Sized for Mobile Thumbs */
  .unc-carousel-btn {
    width: 34px;
    height: 34px;
    font-size: 19px;
  }

  .unc-carousel-btn.prev { left: 8px; }
  .unc-carousel-btn.next { right: 8px; }

  .unc-carousel-dots {
    bottom: 8px;
    gap: 6px;
    padding: 5px 9px;
  }

  .unc-carousel-dot {
    width: 6px;
    height: 6px;
  }

  .unc-carousel-dot.active {
    width: 18px;
  }

  .unc-campus-footer-link {
    padding: 13px 16px;
    font-size: 13.5px;
  }

  /* Sections & Headings */
  .itmoru-section {
    padding: 40px 0;
  }

  .itmoru-section-toprow {
    gap: 14px;
    margin-bottom: 22px;
  }

  .itmoru-hollow-h2 {
    font-size: clamp(28px, 7.6vw, 38px);
    padding-bottom: 10px;
  }

  /* Top 2 Editorial News */
  .itmoru-news-hero-item {
    padding: 20px 0;
    border-right: none;
    border-bottom: 1.5px solid var(--itmoru-border);
  }

  .itmoru-news-hero-item:last-child {
    border-bottom: none;
  }

  .itmoru-news-hero-item h3 {
    font-size: 20px;
  }

  .post-card {
    padding: 16px;
    margin-bottom: 20px;
    height: auto;
  }

  .post-card .desc-post {
    font-size: 16.5px;
  }

  /* Cifras y Hechos & Odometer on 360px–430px Phones */
  .itmoru-dark-band {
    padding: 44px 0;
  }

  .itmoru-big-numbers-row {
    grid-template-columns: 1fr;
    gap: 16px;
    margin-bottom: 32px;
  }

  .itmoru-outlined-stat {
    padding: 20px 18px;
  }

  .itmoru-outlined-stat .num-hollow {
    font-size: 40px;
  }

  .itmoru-outlined-stat .num-desc {
    font-size: 15px;
  }

  .itmoru-acc-title {
    font-size: 17px;
    gap: 12px;
  }

  .itmoru-acc-plus {
    width: 32px;
    height: 32px;
    font-size: 20px;
  }

  .itmoru-acc-panel {
    padding: 16px;
    font-size: 14.5px;
  }

  /* Bento & Career Cards */
  .banners-grid {
    gap: 16px;
  }

  .itmoru-bento-card {
    height: 255px;
    padding: 20px;
  }

  .itmoru-bento-title {
    font-size: 19px;
  }

  .itmoru-vlist-item {
    padding: 14px 16px;
    font-size: 15.5px;
    gap: 10px;
  }

  /* Video Spotlight Mobile */
  .unc-video-spotlight-card {
    padding: 24px 18px;
    border-left-width: 5px;
  }

  .unc-video-phone-frame {
    max-width: 280px;
  }

  /* Footer Mobile */
  footer.itmoru-footer {
    padding: 38px 0 32px;
  }

  .itmoru-footer-topbar {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
    margin-bottom: 24px;
  }

  .itmoru-footer-topbar img {
    height: 52px !important;
  }

  .itmoru-footer-cols {
    grid-template-columns: 1fr;
    gap: 24px;
  }

  .itmoru-footer-office img {
    max-width: 100%;
    height: auto !important;
    max-height: 40px !important;
  }

  .itmoru-subfooter {
    padding: 20px 0;
    font-size: 12.5px;
  }

  /* Prevent iOS auto-zoom on all form inputs */
  .dvfu-input, .dvfu-textarea {
    font-size: 16px;
  }
}

@media (max-width: 400px) {
  .itmoru-full-logo-img {
    height: 46px;
  }
  .itmoru-giant-statement {
    font-size: 32px;
  }
  .itmoru-outlined-stat .num-hollow {
    font-size: 36px;
  }
}
"#;
