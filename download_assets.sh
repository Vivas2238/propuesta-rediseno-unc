#!/usr/bin/env bash
set -e
DEST="/home/sam/.gemini/antigravity/scratch/unc-dvfu-portal/static/real"
mkdir -p "$DEST"

UA="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"

fetch() {
  local url="$1"
  local out="$2"
  echo "Downloading $out ..."
  curl -sSL -A "$UA" -H "Referer: https://unc.edu.ve/" "$url" -o "$DEST/$out"
}

# Logos & Mincyt
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/website_UNC_logoUNC_transparente_04.png" "logo_unc.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/11/logos_header_unc.png" "logos_header_unc.png"
curl -sSL -A "$UA" -H "Referer: https://mincyt.gob.ve/" "https://mincyt.gob.ve/wp-content/themes/mincyt-custom/assets/img/logo.webp" -o "$DEST/logo_mincyt.webp"

# All 8 Real News Photos
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/dia-del-estudiante-venezolano.png" "news_1.png"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/0K8A7105-scaled.jpg" "news_2.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/photo_5176954928231026007_y.jpg" "news_3.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/0K8A4806-scaled.jpg" "news_4.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/photo_5165859855179386028_y.jpg" "news_5.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/WhatsApp-Image-2026-11-11-at-5.13.41-PM.jpeg" "news_6.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/0K8A4467-scaled.jpg" "news_7.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/paz-y-desarrollo.png" "news_8.png"

# Institutional & Epónimo & Campus Photos
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/FotoHFM_05.png" "foto_hfm.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/descubre_UNC_logo_12.jpg" "descubre_unc.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/03_ZC_L9718_P-986x1024-1.jpg" "campus_unc.jpg"

# 6 Pillars Icons
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/Investigar_04.png" "pillar_1.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/desarrollar_04.png" "pillar_2.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/Innovar_04.png" "pillar_3.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/Producir_04.png" "pillar_4.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/iconoApplyNew_04.png" "pillar_5.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/iconoTransformNew_04.png" "pillar_6.png"

# 7 Authorities Real Photos
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/1743431577162.jpg" "auth_1.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/WhatsApp-Image-2025-03-30-at-11.29.45-AM.jpeg" "auth_2.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/03/alberto.jpg" "auth_3.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/WhatsApp-Image-2025-03-30-at-11.29.44-AM-1.jpeg" "auth_4.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/WhatsApp-Image-2025-03-30-at-11.29.33-AM.jpeg" "auth_5.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/WhatsApp-Image-2025-03-30-at-11.29.33-AM-1.jpeg" "auth_6.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/03/adamarys.jpg" "auth_7.jpg"

# 6 Magazine Covers
fetch "https://unc.edu.ve/wp-content/uploads/2026/11/Banner_Noticias_de_la_Ciencia.jpg" "mag_1.jpg"
fetch "https://unc.edu.ve/wp-content/uploads/2026/06/Banner_Octubre.png" "mag_2.png"
fetch "https://unc.edu.ve/wp-content/uploads/2026/06/Banner_Septiembre.png" "mag_3.png"
fetch "https://unc.edu.ve/wp-content/uploads/2026/06/Banner_Mayo.png" "mag_4.png"
fetch "https://unc.edu.ve/wp-content/uploads/2026/06/Banner_Abril.png" "mag_5.png"
fetch "https://unc.edu.ve/wp-content/uploads/2026/06/Banner_Marzo.png" "mag_6.png"

# Career Icons
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_14_03.png" "career_ia.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_15_03.png" "career_ciber.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_iconos_RobotAutomat_05.png" "career_robotica.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_11_03.png" "career_electro.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_13_03.png" "career_biomat.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/Entrada_FisicaNuclearMy_05.png" "career_fnuclear.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_02_03.png" "career_bioquim.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_03_03.png" "career_biotec.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_04_03.png" "career_cmolecular.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_05_03.png" "career_cdatos.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_06_03.png" "career_fisica.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/03/matematicasolo_09.png" "career_matematica.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_08_03.png" "career_nano.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_09_03.png" "career_filosofia.png"
fetch "https://unc.edu.ve/wp-content/uploads/2025/02/progamas_carrera_10_03.png" "career_oceanologia.png"

echo "All assets downloaded successfully."
ls -lh "$DEST"
