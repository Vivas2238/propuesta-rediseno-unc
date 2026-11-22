pub struct NewsArticle {
    pub slug: &'static str,
    pub original_url: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub author: &'static str,
    pub category: &'static str,
    pub thumbnail: &'static str,
    pub excerpt: &'static str,
    pub sections: &'static [(&'static str, &'static [&'static str])],
}

pub struct CareerProgram {
    pub slug: &'static str,
    pub category: &'static str,
    pub name: &'static str,
    pub degree_title: &'static str,
    pub duration: &'static str,
    pub semesters: &'static str,
    pub credits: &'static str,
    pub area: &'static str,
    #[allow(dead_code)]
    pub icon_url: &'static str,
    pub pdf_url: &'static str,
    pub tile_color_class: &'static str,
    pub study_fields: &'static [(&'static str, &'static str)],
    pub career_fields: &'static [(&'static str, &'static str)],
}

pub struct Authority {
    pub name: &'static str,
    pub role: &'static str,
    pub photo_url: &'static str,
    pub bio_focus: &'static str,
}

pub struct MagazineIssue {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub cover_url: &'static str,
    pub pdf_url: &'static str,
    pub tile_class: &'static str,
}

pub struct GazetteDoc {
    pub number: &'static str,
    pub title: &'static str,
    pub year: &'static str,
    pub kind: &'static str,
    pub pdf_url: &'static str,
    pub summary: &'static str,
}

pub struct CarnetRecord {
    pub num: &'static str,
    pub cedula: &'static str,
    pub full_name: &'static str,
    pub career: &'static str,
    pub condition: &'static str,
}

pub const NEWS_ARTICLES: &[NewsArticle] = &[
    NewsArticle {
        slug: "dia-nacional-del-estudiante-venezolano-memoria-de-lucha-y-conocimiento-para-construir-el-futuro",
        original_url: "https://unc.edu.ve/index.php/dia-nacional-del-estudiante-venezolano-memoria-de-lucha-y-conocimiento-para-construir-el-futuro/",
        title: "Día Nacional del Estudiante Venezolano, memoria de lucha y conocimiento para construir el futuro",
        date: "21 de noviembre de 2026",
        author: "Prensa UNC – YA",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_1.png",
        excerpt: "El 21 de noviembre se conmemora el Día Nacional del Estudiante Venezolano, una fecha que representa un reconocimiento del rol de la juventud en los procesos históricos, científicos y productivos del país.",
        sections: &[
            ("", &[
                "El 21 de noviembre se conmemora el Día Nacional del Estudiante Venezolano, una fecha que representa un reconocimiento del rol de la juventud en los procesos históricos del país, así como de su capacidad e ímpetu para impulsar transformaciones en los ámbitos político, social, cultural, educativo, científico y productivo.",
                "Desde las luchas independentistas, los jóvenes han encabezado movimientos estudiantiles que han contribuido en la defensa de ideales como libertad, justicia, democracia y transformación social. Su presencia ha estado vinculada a los grandes debates nacionales y a los procesos que han marcado el rumbo de la República.",
                "Uno de los episodios más significativos de esa trayectoria ocurrió el 21 de noviembre de 1957, cuando estudiantes de la Universidad Central de Venezuela (UCV) y de distintos liceos caraqueños protagonizaron una huelga general en rechazo al plebiscito convocado por el gobierno de Marcos Pérez Jiménez. La movilización, en la que participaron estudiantes de instituciones como los liceos Fermín Toro, Andrés Bello, Juan Vicente González, Luis Razetti y Caracas, así como de la escuela Miguel Antonio Caro, representó una manifestación de resistencia cívica frente al autoritarismo.",
                "Las protestas estudiantiles contribuyeron a profundizar el clima de descontento que precedió a los acontecimientos del 23 de enero de 1958 y a la salida del poder de Marcos Pérez Jiménez. En reconocimiento a aquella jornada, la Junta de Gobierno que asumió la conducción del país promulgó el Decreto N.º 436, publicado en la Gaceta Oficial N.º 25.818, mediante el cual se instituyó el 21 de noviembre como Día del Estudiante Universitario. Posteriormente, en noviembre de 2020, la celebración amplió su alcance al incorporar a todos los niveles de formación, bajo la denominación de Día Nacional del Estudiante Venezolano."
            ]),
            ("UNC y el impulso de una nueva generación científica", &[
                "El legado de los movimientos estudiantiles también se proyecta en la responsabilidad de cada generación de interpretar su tiempo, comprender los problemas de su entorno y aportar respuestas desde sus capacidades y conocimientos. En la Venezuela contemporánea, marcada por profundas transformaciones tecnológicas, científicas, económicas y sociales, la preparación de profesionales capaces de investigar, innovar, desarrollar tecnologías y generar soluciones propias constituye una condición necesaria para ampliar las capacidades nacionales y fortalecer la autonomía del país.",
                "En esta relación entre educación, juventud y desarrollo nacional se inscribe la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC), creada para impulsar la formación científica, la investigación y la innovación tecnológica al servicio del bienestar social. La propuesta educativa de la UNC se destaca por la orientación hacia disciplinas científicas y tecnológicas de creciente relevancia, sin desvincular la formación especializada de sus dimensiones humanísticas y éticas.",
                "Más de 900 nuevos ingresos se incorporaron en este mes de noviembre a la comunidad académica para iniciar su formación científica, sumando un total de más de 3 mil jóvenes en esta casa de estudio. Al dirigirse a los jóvenes que ingresan a la institución, la rectora de la UNC, Gabriela Jiménez Ramírez, expresó la importancia de vivir el proceso educativo al señalar que “el mayor deber de ustedes es venir a estudiar y ser felices desde lo afirmativo venezolano, que el conocimiento sea una fiesta para la libertad y la creación de vanguardia”. En esa misma línea, ha definido a la universidad como “una puerta al futuro, una ventana a la libertad que es el conocimiento”."
            ]),
            ("Voces jóvenes", &[
                "Las experiencias de los estudiantes permiten comprender cómo se expresa hoy ese compromiso. Desde la robótica y la automatización hasta la filosofía y la ciberseguridad, sus testimonios muestran distintas maneras de entender la formación, el conocimiento y la responsabilidad de las nuevas generaciones.",
                "Alfredo Ríos, estudiante de Ingeniería en Robótica y Automatización e integrante del equipo ganador de la World Robot Olympiad (WRO) 2024, considera que “El Día del Estudiante sirve para pensar en el futuro y estar tranquilos con la certeza de que hicimos lo mejor que pudimos para apoyar a los que vienen”. Señala que “es necesario seguir renovando esfuerzos y seguir luchando por una mejor educación, para garantizar un futuro brillante”.",
                "Por su parte, Airadmhern Rojas, odontóloga de profesión y estudiante de Licenciatura en Filosofía, estima que su experiencia académica le ha permitido vincular la formación filosófica con la comprensión de la realidad social, la valoración de las raíces de los pueblos originarios y el diálogo entre el pensamiento filosófico y la actividad científica. Para Rojas, ser estudiante supone “crear la disciplina, la moralidad, crear al ciudadano del mañana que necesita conducir su país” y “llevar este mensaje más allá de nuestra ‘casita de cristal azul’ como le decimos a la universidad”.",
                "Otra perspectiva sobre la formación y la responsabilidad de compartir conocimientos la ofrece Valentina Pereira, estudiante de Ingeniería en Ciberseguridad. Su trayectoria académica incluye la participación como tutora en las Olimpiadas de Robótica Creativa, su presencia en las Olimpiadas de Nanotecnología y su preparación para el Hackathon Venezuela Innova. Lo resume asegurando que “logro mis objetivos investigando más allá de lo necesario y enseñando lo que sé a otras personas. Compartir el conocimiento es la mejor forma de aprender”.",
                "En el Día Nacional del Estudiante Venezolano, la Universidad Nacional de las Ciencias saluda a los jóvenes del país y reconoce su preparación, curiosidad intelectual y vocación por el conocimiento. Con su esfuerzo diario, esta generación da continuidad a una tradición de pensamiento e investigación que constituyen la base para consolidar una ciencia orientada al desarrollo nacional, el bienestar colectivo y la transformación sostenible de la sociedad."
            ])
        ]
    },
    NewsArticle {
        slug: "en-la-unc-se-diserta-sobre-que-significa-pensar-en-tiempos-de-inteligencia-artificial",
        original_url: "https://unc.edu.ve/index.php/en-la-unc-se-diserta-sobre-que-significa-pensar-en-tiempos-de-inteligencia-artificial/",
        title: "En la UNC se diserta sobre “Qué significa pensar en tiempos de Inteligencia Artificial”",
        date: "20 de noviembre de 2026",
        author: "Prensa UNC – YA",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_2.jpg",
        excerpt: "En el marco del Día Mundial de la Filosofía, la UNC reunió a autoridades y docentes en el foro ¿Qué significa pensar en tiempos de Inteligencia Artificial? para reflexionar sobre la soberanía cognitiva y la autonomía intelectual.",
        sections: &[
            ("", &[
                "En el marco del Día Mundial de la Filosofía, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) reunió a autoridades y docentes en el foro ¿Qué significa pensar en tiempos de Inteligencia Artificial (IA)?, llevado a cabo el jueves 19 de noviembre en la Sala de Conferencias Dr. Humberto Fernández-Morán, para reflexionar sobre los desafíos que plantea la inteligencia artificial a la autonomía intelectual, la educación y la construcción del conocimiento.",
                "En el encuentro, que contó con la participación de la rectora de la UNC, Gabriela Jiménez Ramírez, del filósofo Freddy Ñáñez y del profesor Antonio Núñez, bajo la moderación de la directora académica, Diana Castillo, se puso en primer plano la necesidad de formar profesionales capaces de utilizar las nuevas tecnologías sin renunciar al juicio crítico ni a su condición humana."
            ]),
            ("Pensar sigue siendo una responsabilidad humana", &[
                "La IA generativa introduce, a diferencia de tecnologías anteriores que fundamentalmente almacenaban, organizaban o facilitaban la consulta de contenidos, sistemas que pueden producir textos, elaborar respuestas, sintetizar información y simular conversaciones mediante el procesamiento de grandes volúmenes de datos.",
                "Durante el encuentro, la rectora Gabriela Jiménez Ramírez hizo énfasis en la dimensión humana del pensamiento y en la imposibilidad de sustituir la experiencia intelectual de las personas por procesos automatizados. Expresó que “lo que nos ha hecho sacros a la humanidad es poder pensar. Para mí es indiscutible, inimaginable que eso se delegue a una máquina. Es como si perdiéramos la condición humana en total”.",
                "La rectora también llamó a examinar las aplicaciones de la IA desde una perspectiva ética y social: «pensar desde las inteligencias usadas como fuerza de guerra, como fuerza de dominación. Pensar desde las inteligencias para superar las brechas humanas, ciudadanas y sociales»."
            ]),
            ("La filosofía frente a la apariencia de certeza", &[
                "Freddy Ñáñez orientó su intervención hacia la naturaleza del acto de pensar y la necesidad de comprenderlo como una capacidad que requiere formación y ejercicio permanente. Apuntó que “aprender a pensar ya es el primer conflicto que pone entre paréntesis si sabemos realmente a qué nos referimos con esto de ser un ser que piensa”.",
                "Pensar implica observar, relacionar ideas, reconocer contradicciones, formular hipótesis y someter las propias conclusiones a revisión. Precisa que la filosofía promueve «preguntas radicales, las preguntas que en definitiva son las que nos motivan y nos entusiasman para seguir en esta carrera de la vida»."
            ]),
            ("Entre el pensamiento industrial, artificial y artesanal", &[
                "En su intervención, el profesor Antonio Núñez describió la coexistencia de tres dinámicas que inciden actualmente en la construcción del saber: el pensamiento industrial, asociado con la mecanización de los procesos de producción intelectual; el pensamiento artificial, sustentado en sistemas automatizados capaces de generar información; y el pensamiento artesanal, relacionado con la lectura directa, la reflexión pausada y el diálogo.",
                "Más que presentar estas modalidades como realidades excluyentes, el profesor explicó que “tenemos que vivir con esos tres tipos de pensamiento” y recalcó que «el manejo de la IA es beneficioso, pero también hay que leer los libros»."
            ]),
            ("Formación universitaria ante los desafíos de la IA", &[
                "Las reflexiones compartidas durante el foro encontraron una expresión concreta en el modelo educativo de la UNC, donde la formación humanística acompaña el desarrollo de las competencias científicas y tecnológicas, incorporando la asignatura Pensamiento Crítico desde el primer semestre y el abordaje en el Curso de Acompañamiento a la Iniciación Universitaria (CAIU).",
                "En definitiva, la soberanía cognitiva, entendida como la capacidad de construir criterios propios, interpretar información y participar conscientemente en la producción del conocimiento, adquiere así una relevancia decisiva frente a la expansión de la inteligencia artificial."
            ])
        ]
    },
    NewsArticle {
        slug: "mas-de-900-nuevos-estudiantes-inician-su-formacion-cientifica-en-la-universidad-nacional-de-las-ciencias",
        original_url: "https://unc.edu.ve/index.php/mas-de-900-nuevos-estudiantes-inician-su-formacion-cientifica-en-la-universidad-nacional-de-las-ciencias/",
        title: "Más de 900 nuevos estudiantes inician su formación científica en la Universidad Nacional de las Ciencias",
        date: "17 de noviembre de 2026",
        author: "Prensa UNC – MG",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_3.jpg",
        excerpt: "Bajo el legado de su epónimo, el Dr. Humberto Fernández-Morán, la UNC dio la bienvenida a más de 900 estudiantes de nuevo ingreso de la cohorte noviembre 2026, alcanzando 3.130 estudiantes regulares.",
        sections: &[
            ("", &[
                "Bajo el legado de su epónimo, el Dr. Humberto Fernández-Morán, quien concebía la ciencia como una herramienta fundamental para la paz y un derecho irrenunciable de la juventud, la Universidad Nacional de las Ciencias (UNC) dio la bienvenida este lunes 16 de noviembre a más de 900 estudiantes de nuevo ingreso correspondientes al primer semestre de la cohorte noviembre 2026.",
                "Con esta incorporación, la institución alcanza una matrícula actual de 3.130 estudiantes regulares entre el primer y segundo semestre, distribuidos en una oferta académica que supera las 15 carreras científicas y tecnológicas, de las cuales 12 son totalmente inéditas en Venezuela.",
                "Durante el acto de bienvenida, la rectora de esta casa de estudios, Gabriela Jiménez Ramírez, destacó que para el año 2031 egresará una generación de profesionales formados con amor por el conocimiento, sentido de identidad y compromiso con el desarrollo del país. «Nuestra universidad está próxima a cumplir dos años de haber sido creada, el pasado 12 de diciembre. Seguimos dándole vida a nuestras aulas de clase y laboratorios, pensando en la ciencia no solo como un territorio en disputa, sino como una garantía de paz y de futuro para las venezolanas y los venezolanos», afirmó la rectora.",
                "Jiménez Ramírez resaltó que el 32 % del cuerpo docente de la UNC está integrado por investigadores del Instituto Venezolano de Investigaciones Científicas (IVIC), institución fundada por el propio Dr. Fernández-Morán. Asimismo, instó a los jóvenes a cultivar el rigor académico: «Ningún estudiante de la UNC puede decir que no le gustan las matemáticas y la física; sería como ir contra la ley de gravedad. No hay modo de hacer ciencia sin ellas, pues son el lenguaje para comprender el mundo»."
            ]),
            ("Talento de los 23 estados del país", &[
                "Por su parte, el director de Atención Estudiantil, Irwing Acevedo, señaló que esta nueva cohorte reúne a jóvenes provenientes de los 23 estados del territorio nacional, quienes representan el pilar de la generación de relevo científico.",
                "«Lo más enriquecedor del ser humano ocurre cuando descubrimos y exploramos nuevas áreas. Hoy ustedes son esa nueva generación. Hagan su proceso de autoevaluación y comiencen a escribir las páginas de su vida definiendo sus sueños, méritos y objetivos en la UNC», expresó Acevedo."
            ]),
            ("Voces de la nueva generación", &[
                "Carlos Leal, estudiante de Ingeniería en Robótica y Automatización señaló que «luego de aprobar el CAIU, hoy estoy aquí con muchas ganas de estudiar una carrera con gran proyección y formarme como científico en los próximos cuatro años. No dudo que tomé la mejor decisión».",
                "Karla Velásquez, estudiante de Inteligencia Artificial: «hoy no solo estoy aquí por mis sueños, sino también por los de mi familia. Inicio la carrera que tanto anhelé; como mujer, quiero demostrar que nosotras también hacemos ciencia de vanguardia e impulsamos la innovación»."
            ])
        ]
    },
    NewsArticle {
        slug: "unc-conversatorio-sobre-fortalecimiento-del-poder-popular-para-la-soberania-popular",
        original_url: "https://unc.edu.ve/index.php/unc-conversatorio-sobre-fortalecimiento-del-poder-popular-para-la-soberania-popular/",
        title: "UNC: Conversatorio sobre Fortalecimiento del Poder Popular para la Soberanía Popular",
        date: "14 de noviembre de 2026",
        author: "Prensa UNC – YA",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_4.jpg",
        excerpt: "Con el propósito de analizar los retos éticos, legales y técnicos que impone la Inteligencia Artificial en la sociedad actual, se realizó el conversatorio sobre Soberanía Comunicacional y protección de datos.",
        sections: &[
            ("", &[
                "Con el propósito de analizar los retos éticos, legales y técnicos que impone la Inteligencia Artificial (IA) en la sociedad actual, este viernes 13 de noviembre se realizó el conversatorio “Fortalecimiento del Poder Popular para la Soberanía Comunicacional” celebrado en el Salón de Conferencias de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC).",
                "Las ponencias estuvieron a cargo de la rectora de la UNC Dra. Gabriela Jiménez Ramírez, la directora de Telecomunicaciones del Ministerio del Poder Popular para Ciencia y Tecnología, MSc. Marglad Bencomo, el Presidente del Observatorio Nacional de Ciencia, Tecnología e Innovación y Secretario de la UNC, Dr. Roberto Betancourt y la Lic. en Ciberseguridad Nerissa Aguilera."
            ]),
            ("Soberanía de los datos y formación científica para el desarrollo nacional", &[
                "La presentación sobre “El derecho humano a la ciencia en la era de la IA: ¿Progreso tecnológico o distopía social?”, estuvo a cargo de la rectora Gabriela Jiménez Ramírez. Como ejemplo, presentó los avances de la UNC y la composición de su matrícula: «El 52% está en inteligencia artificial, ciberseguridad y robótica y automatización acompañado de ciencias de los datos».",
                "Otro de los planteamientos centrales de su intervención estuvo relacionado con la residencia de los datos: «si el dato es venezolano, nace en Venezuela, se queda en Venezuela»."
            ]),
            ("La ética como fundamento del desarrollo tecnológico", &[
                "“La ética desde el código y su aplicación para la IA”, estuvo a cargo de Marglad Bencomo, quien examinó el código de ética, el Plan Nacional de Inteligencia Artificial 2025-2031 y la necesidad de establecer mecanismos de supervisión con enfoque humanista en sectores como la salud, la educación, las finanzas, la agricultura y los hidrocarburos."
            ]),
            ("La inteligencia artificial como poder económico e industria", &[
                "“La IA como frontera entre el progreso tecnológico y desarrollo humano” estuvo a cargo del Dr. Roberto Betancourt, quien examinó los ciclos evolutivos de esta industria, el concepto de diseño dominante y las políticas públicas de investigación y desarrollo para que Venezuela desarrolle competencias propias con valor agregado."
            ]),
            ("Ciberseguridad: proteger datos y anticipar amenazas", &[
                "La seguridad digital fue abordada por la especialista Nerissa Aguilera en su exposición “De la protección del perímetro al extractivismo colonial: redefiniendo la ciberseguridad en la era de la IA”, destacando la necesidad de consolidar una cultura nacional de ciberseguridad en todos los niveles educativos y profesionales."
            ])
        ]
    },
    NewsArticle {
        slug: "la-unc-realizo-primer-foro-de-debate-estudiantil",
        original_url: "https://unc.edu.ve/index.php/la-unc-realizo-primer-foro-de-debate-estudiantil/",
        title: "La UNC realizó Primer Foro de Debate Estudiantil",
        date: "13 de noviembre de 2026",
        author: "Prensa UNC – Guillermo Bentacourt",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_5.jpg",
        excerpt: "En el marco de la conmemoración del II Aniversario de la UNC, estudiantes de ocho carreras debatieron bajo el tema «Ciencia para la Paz y el Desarrollo» y firmaron la Declaración UNC por una ciencia compartida.",
        sections: &[
            ("", &[
                "En el marco de la conmemoración del II Aniversario de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) y del Día del Estudiante Venezolano, este jueves 12 de noviembre se llevó a cabo en el Anfiteatro de la universidad el I Foro de Debate UNC, organizado por el Club de Debate Científico junto a la Dirección Estudiantil.",
                "Bajo el tema «Ciencia para la Paz y el Desarrollo», ocho estudiantes representaron a sus carreras: Inteligencia Artificial, Biotecnología, Física, Biología y Química Computacional, Filosofía, Robótica y Automatización, Biomateriales y Ciencia de Datos. Cada uno aportó la mirada de su disciplina a una misma pregunta: ¿cómo puede el conocimiento científico contribuir a una sociedad más justa, pacífica y desarrollada?",
                "La jornada se desarrolló en tres rondas: un conversatorio inicial, duelos argumentativos uno contra uno y una tercera fase de construcción y consenso en la que negociaron y votaron la Declaración UNC por una ciencia compartida.",
                "Un jurado evaluó cada intervención según criterios de fundamento científico, argumentación, refutación, oratoria, pensamiento crítico, ética, propuesta y negociación. El primer lugar fue para el estudiante Alexander Navas, de Inteligencia Artificial. Le siguieron Stephany Vielma (Biotecnología), César Molleda (Física), Eduardo Contreras (Biología y Química Computacional) y Keiver Mejías (Filosofía).",
                "El Director Estudiantil de la UNC, Dr. Irwing Acevedo, destacó que “este foro demuestra que en la UNC no solo formamos científicos, formamos ciudadanos capaces de pensar, argumentar y escuchar. Ver a estudiantes de carreras tan distintas discutir con respeto, defender sus ideas con evidencia y, al final, ponerse de acuerdo en una declaración común, es la mejor prueba de que la ciencia también se construye en el diálogo”."
            ])
        ]
    },
    NewsArticle {
        slug: "unc-fanb-y-cendit-unen-esfuerzos-para-impulsar-la-formacion-de-alto-nivel-en-telecomunicaciones",
        original_url: "https://unc.edu.ve/index.php/unc-fanb-y-cendit-unen-esfuerzos-para-impulsar-la-formacion-de-alto-nivel-en-telecomunicaciones/",
        title: "UNC, FANB y CENDIT unen esfuerzos para impulsar la formación de alto nivel en telecomunicaciones",
        date: "12 de noviembre de 2026",
        author: "Prensa UNC",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_6.jpg",
        excerpt: "La UNC dio inicio al Curso de Ampliación en Ciencias de las Telecomunicaciones para 26 profesionales en alianza estratégica con la FANB y la Fundación CENDIT.",
        sections: &[
            ("", &[
                "Con el objetivo de fortalecer el talento humano y consolidar la soberanía tecnológica del país, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) dio inicio al Curso de Ampliación en Ciencias de las Telecomunicaciones, fruto de una alianza estratégica junto a la Fuerza Armada Nacional Bolivariana (FANB) y la Fundación Centro Nacional de Desarrollo e Investigación en Telecomunicaciones (CENDIT).",
                "En esta cohorte participan 26 profesionales, quienes inician un trayecto preparatorio diseñado como antesala para cursar estudios avanzados de especialización, maestría y doctorado en el área científica nacional. La malla curricular se estructura en cuatro módulos fundamentales: Identidad y Entornos Virtuales; Investigación y Normas; Redes e Innovación; Proyectos Innovadores Sociotecnológicos y Emprendimiento en Telecomunicaciones.",
                "Siguiendo los lineamientos de la rectora Gabriela Jiménez Ramírez, la profesora Diana Castillo Parra, directora de la Dirección Académica de la UNC, recibió a la Dra. Gloria Carvalho, presidenta del CENDIT; el Almirante Figueroa, presidente de Müröntö; el Vicealmirante Aular Villegas; el General de División Trujillo Cordero y el Contralmirante Vladimir Markovich Vivas.",
                "El rigor académico y el proceso de acreditación del curso cuentan con el respaldo de un equipo docente e interinstitucional integrado por las profesoras Denyz Luz Molina, Xiomara Liberto y Yolimar Sánchez (UNC), junto al profesor Rafael Rivero (UCV-CENDIT)."
            ])
        ]
    },
    NewsArticle {
        slug: "clase-magistral-los-desafios-de-la-nueva-realidad-climatica-en-venezuela",
        original_url: "https://unc.edu.ve/index.php/clase-magistral-los-desafios-de-la-nueva-realidad-climatica-en-venezuela/",
        title: "Clase magistral: Los desafíos de la nueva realidad climática en Venezuela",
        date: "11 de noviembre de 2026",
        author: "Prensa UNC – YA",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_7.jpg",
        excerpt: "En el marco del Día Mundial de la Ciencia para la Paz y el Desarrollo, el Dr. Dirk Thielen (IVIC) dictó la conferencia magistral “La nueva realidad climática: desafíos” sobre la dinámica hidrometeorológica nacional.",
        sections: &[
            ("", &[
                "En el marco de la semana conmemorativa del Día Mundial de la Ciencia para la Paz y el Desarrollo, el Salón de Conferencias de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) fue escenario de la conferencia magistral titulada “La nueva realidad climática: desafíos”.",
                "La ponencia estuvo a cargo del doctor Dirk Thielen, jefe del Laboratorio de Ecología del Paisaje y Clima del Instituto Venezolano de Investigaciones Científicas (IVIC), quien ofreció una perspectiva rigurosa sobre la aceleración climática global y su traducción directa en las dinámicas hidrometeorológicas y productivas de Venezuela."
            ]),
            ("El motor climático nacional", &[
                "Uno de los ejes centrales de la exposición del investigador del IVIC radicó en desmontar la narrativa tradicional que atribuye la variabilidad de las lluvias en Venezuela casi de forma exclusiva al fenómeno de El Niño en el Océano Pacífico. Thielen explicó que el verdadero motor climático del país responde a la dinámica de la fachada del Océano Atlántico Norte Tropical y el Mar Caribe.",
                "A través de ondas de calor oceánicas en estas zonas, se generan pulsos húmedos que guardan una correlación positiva directa con el régimen de precipitaciones en el territorio venezolano, registrando en la última década un incremento en las precipitaciones anuales de entre 100 y 500 milímetros adicionales."
            ]),
            ("El caso crítico del Lago de Valencia", &[
                "Durante su intervención, Thielen expuso los riesgos sectoriales y territoriales más apremiantes para el país, destacando especialmente la situación de la cuenca endorreica del Lago de Valencia, que alberga a más de 300.000 habitantes y concentra cerca del 48% del PIB industrial nacional.",
                "Al tratarse de una cuenca cerrada sin salida natural al mar, los pulsos de lluvias intensas provocan ascensos críticos en el espejo de agua de entre 3,5 y 5 metros en lapsos menores a diez años, afectando más de 11.000 hectáreas de zonas urbanas, productivas e infraestructura vial."
            ]),
            ("Ciencia para la acción y la toma de decisiones", &[
                "Para finalizar, el Dr. Dirk Thielen enfatizó la imperiosa necesidad de cerrar la brecha existente entre la investigación académica y la sociedad, traduciendo los modelos climáticos en herramientas operativas para los tomadores de decisiones, productores agrícolas y la ciudadanía.",
                "La profesora Diana Castillo, directora Académica de la UNC, reflexionó sobre la importancia de la continua investigación en el área climática y la corresponsabilidad ciudadana ante la crisis ambiental global."
            ])
        ]
    },
    NewsArticle {
        slug: "ciencia-y-diplomacia-puente-para-la-paz-y-el-desarrollo-sostenible",
        original_url: "https://unc.edu.ve/index.php/ciencia-y-diplomacia-puente-para-la-paz-y-el-desarrollo-sostenible/",
        title: "Ciencia y diplomacia, puente para la paz y el desarrollo sostenible",
        date: "10 de noviembre de 2026",
        author: "Prensa UNC – YA",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_8.png",
        excerpt: "En el Día Mundial de la Ciencia para la Paz y el Desarrollo, la UNC reafirma su compromiso con la ciencia abierta, la diplomacia científica y la cooperación académica al servicio de la vida.",
        sections: &[
            ("", &[
                "El cambio climático, las enfermedades, la pérdida de biodiversidad, la inseguridad alimentaria y las transformaciones tecnológicas son problemas que trascienden los límites geográficos y políticos, y cuya solución requiere compartir conocimientos, sumar capacidades y establecer mecanismos de cooperación entre las naciones.",
                "Esta dimensión social y universal del conocimiento constituye el fundamento del Día Mundial de la Ciencia para la Paz y el Desarrollo, una efeméride que se celebra cada 10 de noviembre, proclamada por la Organización de las Naciones Unidas para la Educación, la Ciencia y la Cultura (UNESCO) en 2001."
            ]),
            ("Diplomacia científica al servicio de la vida y de los pueblos", &[
                "La celebración de 2026 bajo el lema “Puntos de encuentro a través de la diplomacia científica”, destaca la capacidad de la ciencia para facilitar el diálogo entre países. La diplomacia científica comprende el aprovechamiento de la investigación, el conocimiento especializado y la cooperación entre comunidades académicas para abordar asuntos de interés compartido en salud, energía, protección ambiental, agricultura y recursos naturales."
            ]),
            ("Conocimiento científico y desarrollo sostenible", &[
                "En el marco del Decenio Internacional de las Ciencias para el Desarrollo Sostenible (2024-2033), la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) articula ciencias básicas, naturales, tecnológicas y humanísticas en áreas estratégicas como biotecnología, física, ciencia de datos, nanotecnología, inteligencia artificial, robótica y biomateriales.",
                "Como expresó la rectora Gabriela Jiménez Ramírez en su artículo “Ciencia para una política comunal del agua”: “necesitamos que los territorios hablen y que sus actores se sientan parte de este proceso de transformación”. Promover la ciencia como un bien común significa reconocer su capacidad para tender puentes entre las sociedades y contribuir a un futuro en el que el progreso científico esté al servicio de la vida."
            ])
        ]
    },
    NewsArticle {
        slug: "en-la-unc-la-ciencia-dialoga-con-los-saberes-ancestrales",
        original_url: "https://unc.edu.ve/index.php/en-la-unc-la-ciencia-dialoga-con-los-saberes-ancestrales/",
        title: "En la UNC, la ciencia dialoga con los saberes ancestrales",
        date: "7 de noviembre de 2026",
        author: "Prensa UNC – Yanett Alvarez",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_9.jpg",
        excerpt: "En un ejercicio de convergencia entre el rigor académico y la memoria de los pueblos originarios, la Universidad Nacional de las Ciencias (UNC) Dr. Humberto Fernández-Morán fue escenario del conversatorio Curso de Oralitura Jivi: La...",
        sections: &[
            ("", &[
                "En un ejercicio de convergencia entre el rigor académico y la memoria de los pueblos originarios, la Universidad Nacional de las Ciencias (UNC) Dr. Humberto Fernández-Morán fue escenario del conversatorio Curso de Oralitura Jivi: La ciencia dialoga con los saberes ancestrales, dictado por el profesor Franklin Perozo, adscrito a la cátedra de Metafísica de la Licenciatura en Filosofía.",
                "La actividad, desarrollada en el anfiteatro de la universidad, reunió a autoridades universitarias, docentes de la Dirección Académica y una nutrida representación estudiantil, con la participación de los cursantes de primero y segundo semestre de la mencionada carrera, quienes asumieron la organización de este espacio de intercambio reflexivo.",
            ]),
            ("El pensamiento y la palabra como ejes ontológicos", &[
                "El objetivo central del encuentro fue propiciar un diálogo horizontal sobre la cosmovisión, la filosofía y la oralitura del pueblo Jivi, reconociéndolas no solo como expresiones culturales, sino como formas legítimas de resistencia y preservación de la memoria colectiva.",
                "La programación giró en torno a tres ejes temáticos fundamentales:",
                "Salinei (Origen-Ancestralidad): abordó la palabra como cobija, profundizando en la arquitectura de la cosmovisión Jivi y su compleja carga simbólica.",
                "Navarëcuaicuaijae (Interacción): analizó el idioma vernáculo como la «casa de la existencia» y examinó el pulso constante entre la existencia y la resistencia cultural.",
                "Oralitura y Escritura: contrastó la memoria colectiva viva frente a la rigidez de la página estática, advirtiendo sobre la totalidad corporal de la palabra hablada y el riesgo latente de la folklorización superficial.",
            ]),
            ("Un diálogo necesario entre dos formas de conocimiento", &[
                "La articulación entre los saberes originarios y la educación superior ha dejado de ser una aspiración para convertirse en una necesidad crítica y transformadora. El diálogo entre la filosofía de la ciencia y las cosmovisiones tradicionales interpela los relatos hegemónicos que reducen lo real a lo cuantificable, y revela que la sabiduría ancestral no solo enriquece la producción de conocimiento: ofrece un horizonte ético para la preservación de la vida.",
                "Al respecto, la profesora Denyz Luz Molina, coordinadora de carreras de la UNC, subrayó la trascendencia de incorporar la metafísica dentro de la malla curricular de la Licenciatura en Filosofía, al destacar que esta disciplina “permite examinar las estructuras conceptuales que subyacen a toda comprensión de la realidad, conectando el saber ontológico con las dimensiones históricas y culturales del ser humano”.",
                "Asimismo, la docente enfatizó que, en el marco de una universidad dedicada a las ciencias, “este enfoque dota al pensamiento científico de una perspectiva crítica y reflexiva indispensable para comprender que todo modelo empírico o técnico emerge inevitablemente de una interpretación previa del mundo”.",
                "Con iniciativas como estas, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán consolida su compromiso con la descolonización del pensamiento, abriendo cauces permanentes donde el rigor científico y el patrimonio sabio de los pueblos indígenas convergen en beneficio de la soberanía cognitiva y cultural del país.",
            ]),
        ]
    },
    NewsArticle {
        slug: "estudiantes-de-la-unc-inician-practicas-en-laboratorios-de-intevep-s-a",
        original_url: "https://unc.edu.ve/index.php/estudiantes-de-la-unc-inician-practicas-en-laboratorios-de-intevep-s-a/",
        title: "Estudiantes de la UNC inician prácticas en laboratorios de INTEVEP",
        date: "6 de noviembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_10.jpg",
        excerpt: "La Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) dio inicio a las prácticas de laboratorio de la asignatura Química Integrado, correspondiente al segundo semestre de ocho carreras científicas y tecnológicas....",
        sections: &[
            ("", &[
                "La Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) dio inicio a las prácticas de laboratorio de la asignatura Química Integrado, correspondiente al segundo semestre de ocho carreras científicas y tecnológicas.",
                "Las actividades que se llevan a cabo en las instalaciones de INTEVEP S.A., Centro Tecnológico PDVSA, están establecidas en el convenio marco de cooperación interinstitucional firmado entre ambas instituciones con el propósito de fortalecer los vínculos académicos, científicos y tecnológicos en beneficio del desarrollo nacional.",
                "Dicho acuerdo permitirá impulsar el intercambio de conocimientos, el uso compartido de infraestructura, aulas y laboratorios especializados, así como la participación conjunta de docentes, investigadores y especialistas en proyectos de formación, investigación, desarrollo e innovación.",
                "Esta articulación institucional facilita el desarrollo de experiencias formativas en laboratorios y pone de relieve la importancia de la cooperación para respaldar la enseñanza de las ciencias.",
                "La experiencia permite a los estudiantes entrar en contacto con el trabajo experimental desde una etapa temprana de su formación universitaria, familiarizarse con instrumentos y materiales especializados y comenzar a desarrollar habilidades de observación, medición, análisis y registro de resultados.",
                "Destaca la profesora Lisbeth Mendoza, Coordinadora del Laboratorio de Química Integrado de la UNC que “el inicio de las actividades prácticas es importante ya que para la mayoría de los estudiantes es el primer acercamiento a un laboratorio de química”.",
                "El programa contempla ejercicios de medición y análisis de errores, reacciones químicas, cromatografía en papel, síntesis de carbonato de calcio, preparación de soluciones, determinación de pH y titulaciones ácido-base. Mendoza considera que “con esta experiencia se busca que el estudiante aprenda a ver evidencias y deje plasmadas sus observaciones en una bitácora, así como elaborar documentos de divulgación científica”.",
                "La asignatura Química Integrado se cursa durante el segundo semestre de las carreras que comprende las licenciaturas en Ciencia Molecular, Física, Física Nuclear, Nanotecnología, Biología y Química Computacional y Biotecnología, así como a las ingenierías en Biomateriales y Electromedicina. Esta diversidad de áreas permite apreciar la presencia de la química en campos científicos y tecnológicos con objetos de estudio diferentes, pero relacionados entre sí.",
                "Las actividades, que se desarrollaran durante cinco semanas cuentan con el acompañamiento de profesores y especialistas, así como con espacios adecuados para las actividades de enseñanza. En tal sentido, Mendoza afirma que “docentes de excelente calidad acompañarán a nuestros estudiantes en esta emocionante aventura hacia el aprendizaje de la química”.",
                "“Los estudiantes podrán experimentar y descubrir la magia de la química, su presencia en lo cotidiano y asociarlas a sus respectivas áreas de competencia”, refirió la coordinadora.",
                "Esta experiencia refleja la importancia que tiene el contacto temprano con los laboratorios en la formación científica, al acercar a los futuros profesionales a métodos y procedimientos utilizados en la investigación. Por otra parte, la posibilidad de familiarizarse desde los primeros semestres con entornos experimentales contribuye a construir una sólida base para su futura participación en proyectos científicos y tecnológicos.",
                "Con el inicio de estas actividades, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán fortalece la formación académica de diversas carreras científicas y tecnológicas a través de una primera aproximación al trabajo experimental. El laboratorio se convierte así en un espacio para poner a prueba conocimientos, desarrollar destrezas, documentar evidencias y aprender a comunicar resultados.",
            ]),
        ]
    },
    NewsArticle {
        slug: "el-comic-como-expresion-artistica-y-cultural",
        original_url: "https://unc.edu.ve/index.php/el-comic-como-expresion-artistica-y-cultural/",
        title: "El cómic como expresión artística y cultural",
        date: "5 de noviembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_11.jpg",
        excerpt: "Lejos de ser un simple entretenimiento, el cómic o historieta se ha erigido como un poderoso medio de comunicación, herramienta pedagógica y lenguaje artístico en la era moderna. La antigua relación entre imagen y relato, con el paso...",
        sections: &[
            ("", &[
                "Lejos de ser un simple entretenimiento, el cómic o historieta se ha erigido como un poderoso medio de comunicación, herramienta pedagógica y lenguaje artístico en la era moderna. La antigua relación entre imagen y relato, con el paso del tiempo, se ha consolidado como un medio de expresión social y cultural.",
                "Bajo esta perspectiva, la artista plástica Bárbara Zsymanka ofreció una charla titulada “Comic, noveno arte” en el Salón de Conferencias Dr. Humberto Fernández-Morán de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC). A través de una exposición que sintetiza la historia y evolución del comic, la expositora abarcó desde los primeros desarrollos del género en el siglo XIX hasta sus manifestaciones actuales.",
                "La actividad que contó con la asistencia de autoridades, profesores y estudiantes de la UNC, se inició con las palabras de la rectora de la UNC, Gabriela Jiménez Ramírez, quien destacó la trayectoria de la invitada y su condición de especialista y artista plástica, reconocida con el Premio Nacional de Pintura en Venezuela.",
                "Al referirse a la dimensión cultural y comunicativa del cómic, la rectora señaló que se trata de “una expresión artística y también una ciencia. No cualquier persona hace cómics. No cualquier artista plástico construye historias sobre nuestras semblanzas y experiencias o el manejo de la sátira que le dio al comic ese valor para la denuncia, la denuncia pública con la majestuosidad de la imagen y de las letras”.",
                "La rectora también resaltó la relación entre el conocimiento científico y la creación artística al expresar que “nosotros rescatamos el poder de las letras. En el arte hay mucha ciencia, pues la ciencia también es una expresión del arte”. Aprovechó la oportunidad para invitar a los asistentes a continuar el intercambio con la artista en el marco de las actividades programadas por la universidad para celebrar su segundo aniversario, el próximo 12 de diciembre.",
            ]),
            ("De las primeras historietas a la novela gráfica", &[
                "La presentación de Zsymanka permitió reconocer que la historia del cómic no comienza exclusivamente con la aparición de las publicaciones modernas.",
                "El cómic como lenguaje narrativo moderno tomó forma durante el siglo XIX, en estrecha relación con el desarrollo de la prensa y las nuevas posibilidades de reproducción y circulación de imágenes.",
                "En Estados Unidos, uno de los hitos de esta evolución fue Hogan’s Alley, del dibujante Richard Felton Outcault, publicado en 1895 en el periódico New York World. La serie incorporó recursos que contribuyeron a establecer las características de la historieta moderna, entre ellos la organización secuencial de las escenas y la integración de textos y diálogos en las imágenes. Su personaje más conocido, The Yellow Kid, alcanzó una enorme popularidad y evidenció el potencial comercial de las tiras ilustradas para atraer lectores a los periódicos.",
                "Durante las décadas siguientes, el cómic experimentó una expansión considerable en Estados Unidos. Surgieron personajes, publicaciones y géneros que ampliaron sus posibilidades narrativas, desde las historias humorísticas y de aventuras hasta los relatos de ciencia ficción y superhéroes.",
                "Paralelamente, el medio encontró en Europa otras formas de desarrollo, con propuestas que incorporaron nuevos lenguajes gráficos y abordaron temáticas sociales, históricas y políticas. Esta evolución favoreció el reconocimiento de la novela gráfica como una modalidad narrativa capaz de explorar asuntos complejos y experiencias humanas desde una perspectiva literaria y visual.",
                "Entre las obras que ilustran esa transformación la expositora mencionó “Maus”, de Art Spiegelman, que aborda el Holocausto y la memoria familiar. También destacó el título “Building Stories”, de Chris Ware, una propuesta que explora las posibilidades materiales y narrativas del cómic a través de diferentes formatos impresos y recorridos de lectura.",
                "De esta manera, la presentación de Zsymanka mostró cómo el lenguaje de las viñetas ha ampliado sus recursos y soportes sin perder su capacidad para comunicar ideas, representar realidades y conectar con públicos de distintas generaciones.",
                "Al valorar los distintos períodos de esta manifestación artística, Zsymanka consideró que no existe una época más prolífica o importante que otra, pues cada una ha contribuido, desde sus propias características, a enriquecer el lenguaje del cómic.",
            ]),
            ("Un arte que dialoga con las nuevas tecnologías", &[
                "Más allá de la revisión histórica, la charla propició una reflexión sobre la vigencia del cómic y su relación con las herramientas tecnológicas contemporáneas, particularmente con la inteligencia artificial.",
                "Al ser consultada Zsymanka expresó que acercar a las nuevas generaciones a los orígenes y transformaciones de esta manifestación artística es fundamental para comprender su desarrollo y valorar sus aportes culturales. Al explicar la importancia de ese conocimiento, manifestó que “el cómic es considerado hoy como noveno arte. En tal sentido, es importante que la juventud conozca la historia de este arte”.",
                "En cuanto a la incorporación de la inteligencia artificial (IA) a los procesos creativos, la artista destacó la necesidad de comprender sus posibilidades sin perder de vista el papel de quien crea. Al respecto, señaló que “hay que recordar que la IA es una herramienta, para utilizar y mejorar imágenes, diálogos, textos”.",
                "Su planteamiento pone de relieve que, si bien las actuales tecnologías generativas pueden facilitar determinadas tareas de producción y edición, la concepción de la obra, la intención narrativa y las decisiones artísticas siguen dependiendo de la intervención humana.",
            ]),
            ("La lectura como experiencia de creación", &[
                "A propósito de la charla, cabe citar una reflexión de la rectora Gabriela Jiménez Ramírez sobre la dimensión científica del cómic, publicada en su canal de Telegram, en la que explicó los procesos cognitivos que intervienen durante la lectura de una historieta:",
                "“Desde el punto de vista científico, el cómic resulta especialmente interesante porque exige la participación activa del lector. Las viñetas no suelen mostrar todos los acontecimientos: entre una y otra existen espacios temporales que la mente debe completar. Este proceso, conocido como cierre o inferencia narrativa, permite interpretar movimientos, emociones y acciones que no aparecen representados de manera explícita. Por tanto, leer un cómic no consiste simplemente en observar dibujos, sino en relacionar información visual y verbal, reconocer el orden de la secuencia y construir significados”.",
                "Podemos afirmar entonces que el cómic ofrece un campo de encuentro entre la creación artística, la comunicación y los procesos de percepción y comprensión, aspectos que amplían su interés tanto para la investigación como para la educación.",
                "La presentación de Bárbara Zsymanka en la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán permitió a la comunidad acercarse a una expresión cultural que ha logrado transformarse a lo largo del tiempo, adaptándose a los cambios sociales, tecnológicos y estéticos. Una experiencia que recordó que la ciencia, el arte y la cultura no son ámbitos aislados, sino expresiones complementarias de la capacidad humana para comprender, interpretar y transformar el mundo.",
            ]),
        ]
    },
    NewsArticle {
        slug: "instituto-energetico-de-moscu-dicta-clase-magistral-sobre-ia-y-transicion-energetica",
        original_url: "https://unc.edu.ve/index.php/instituto-energetico-de-moscu-dicta-clase-magistral-sobre-ia-y-transicion-energetica/",
        title: "Instituto Energético de Moscú dicta clase magistral sobre IA y transición energética",
        date: "2 de noviembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_12.jpg",
        excerpt: "En la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) el lunes 2 de noviembre se realizó la clase magistral por videoconferencia “Inteligencia Artificial y la Transición Energética” (Artificial Intelligence and...",
        sections: &[
            ("", &[
                "En la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) el lunes 2 de noviembre se realizó la clase magistral por videoconferencia “Inteligencia Artificial y la Transición Energética” (Artificial Intelligence and the Energy Transition – Trusted AI), dictada por el profesor Andrey Lebedev del Instituto Energético de Moscú (IEM). Actividad que tuvo lugar en el Salón de Conferencias Dr. Humberto Fernández-Morán y contó con la participación de autoridades, profesores y estudiantes de la UNC.",
                "La clase magistral se inscribe en el Acuerdo de Cooperación suscrito entre la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán y el Instituto Energético de Moscú, instrumento que establece un marco para el desarrollo de iniciativas conjuntas y el fortalecimiento de las relaciones académicas entre ambas instituciones firmado en el año 2025.",
                "La rectora de la UNC, Gabriela Jiménez Ramírez, dio inicio a la actividad destacando la fructífera relación con el IEM, al señalar que “a un año del convenio hemos dado inicio a las actividades de cooperación, intercambio y actualización de experiencias”.",
                "En ese sentido, la rectora precisó que “hay una convergencia en las áreas de la ciencia, una convergencia en el análisis de los sistemas energéticos con la incorporación de la inteligencia artificial y la computación cuántica. Moscú tiene un amplio aprendizaje, con referentes importantes para compartir con nosotros, y muy especialmente el Instituto Energético de Moscú, próximo a cumplir en el 2030 su centenario”.",
                "El profesor Andrey Lebedev compartió conocimientos y experiencias sobre el modo en que las tecnologías de inteligencia artificial ofrecen nuevas posibilidades para mejorar la eficiencia de los sistemas, optimizar el uso de los recursos y anticipar fallas en las instalaciones. A su vez, disertó acerca de la confiabilidad, la seguridad y el uso responsable de estas tecnologías.",
            ]),
            ("Inteligencia artificial ante los desafíos del sector energético", &[
                "Andrey Lebedev es profesor asociado del Departamento de Protección por Relés y Automatización de Sistemas Eléctricos en la Universidad Nacional de Investigación «Instituto Energético de Moscú» (MPEI), y ostenta el grado de Candidato en Ciencias Técnicas (equivalente a Ph.D. en Ingeniería). Su trayectoria académica e investigativa ha estado orientada hacia la digitalización y automatización de redes de potencia.",
                "En su intervención, resaltó que la transición energética exige tecnologías avanzadas para gestionar fuentes renovables e integrar sistemas de almacenamiento complejos. Estos avances permiten supervisar la jerarquía operativa, el control industrial y los riesgos de seguridad que, de no atenderse, podrían provocar fallas físicas graves en infraestructuras críticas. Al respecto, sostuvo que garantizar la confiabilidad del sistema requiere combinar modelos analíticos, reglas formales y simulaciones rigurosas.",
                "Asimismo, advirtió que en una red eléctrica un fallo de software puede desencadenar rápidamente un colapso físico o un apagón de gran magnitud. Por ello, enfatizó que no basta con que un sistema de inteligencia artificial responda con fluidez, se requiere una IA de Confianza (Trusted AI), cuyos resultados estén respaldados por pruebas de seguridad y por el cumplimiento estricto de las normas de ingeniería.",
                "Para garantizar dicha seguridad, plantea un modelo híbrido estructurado en tres niveles:",
                "• Generación (Modelos de Lenguaje / LLM): Procesan requerimientos del usuario y formulan un plan de acción.",
                "• Filtro de reglas (Ontologías): Verifican que la propuesta se ajuste estrictamente a las leyes y reglamentos técnicos vigentes.",
                "• Simulación física (Gemelo Digital / DTPS): Modela la red eléctrica en tiempo real mediante un simulador virtual para evaluar el impacto de cada orden antes de ejecutarla en los equipos reales.",
                "Subrayó que la automatización solo genera confianza cuando se fundamenta en evidencia empírica y estándares técnicos rigurosos. En este esquema, la IA no solo ayuda a predecir la generación y el consumo energético, sino que optimiza la gestión eficiente del almacenamiento en baterías.",
                "Para la comunidad de la UNC, la clase magistral representó una oportunidad para acercar a la comunidad universitaria a experiencias internacionales de investigación y desarrollo tecnológico, así como para conocer de cerca importantes avances del IEM.",
            ]),
            ("Programa Dictado Atómico", &[
                "En el marco esta actividad, se hizo entrega de certificados a los estudiantes que representaron a la universidad en la edición 2026 del programa Dictado Atómico, celebrado en la Casa Rusa. Una de sus participantes, la estudiante María Valentina Rojas Peña, valoró la experiencia “como un paso catalizador y fundacional para la consolidación e implementación estratégica de la física nuclear en nuestro país, marcando el inicio de una nueva etapa de vanguardia en el desarrollo científico nacional”.",
                "Asimismo, Rojas Peña hizo un reconocimiento público al rol transformador de la UNC “al propiciar estos espacios de alta exigencia académica. Disfrutar de un entorno formativo integral y estar rodeado de docentes de tan alto calibre es un privilegio que obedece de manera directa a la visión de nuestra rectora”.",
                "Con la realización de actividades de alto nivel académico, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán reafirma el alcance y la pertinencia de los programas de cooperación internacional que impulsa para fortalecer sus procesos de formación e investigación.",
                "De esta manera, en un escenario mundial marcado por la transformación tecnológica y la búsqueda de alternativas energéticas eficientes y sostenibles, el acceso a estos conocimientos permite a los futuros profesionales de la UNC comprender los desafíos científicos e industriales contemporáneos y familiarizarse con herramientas clave para el desarrollo de sus disciplinas.",
            ]),
        ]
    },
    NewsArticle {
        slug: "dia-mundial-de-la-codificacion-aprender-comprender-y-transformar-el-mundo-digital",
        original_url: "https://unc.edu.ve/index.php/dia-mundial-de-la-codificacion-aprender-comprender-y-transformar-el-mundo-digital/",
        title: "Día Mundial de la Codificación: Aprender, comprender y transformar el mundo digital",
        date: "29 de octubre de 2026",
        author: "Prensa UNC – Yanett Alvarez",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_13.png",
        excerpt: "En la sociedad contemporánea, la programación informática es la estructura invisible que sostiene la transformación digital, el desarrollo de la inteligencia artificial y la toma de decisiones complejas. Al pedir un taxi desde el...",
        sections: &[
            ("", &[
                "En la sociedad contemporánea, la programación informática es la estructura invisible que sostiene la transformación digital, el desarrollo de la inteligencia artificial y la toma de decisiones complejas. Al pedir un taxi desde el teléfono, hacer una compra en línea o consultar a un asistente de inteligencia artificial, millones de personas ponen en marcha complejas instrucciones y algoritmos.",
                "Por lo general, estas instrucciones se corresponden con un código de programación. Aprender a escribirlo y a cuestionarlo es la razón por la cual el 29 de octubre se celebra el Día Mundial de la Codificación, una fecha que promueve el pensamiento lógico y analítico capaz de integrar las matemáticas, la creatividad y la resolución de problemas.",
                "Asimismo, expone la necesidad de formar personas capaces de crear soluciones digitales inclusivas, éticas y orientadas al bienestar de la sociedad. Aprender a programar permite comprender mejor la inteligencia artificial y participar en la creación de tecnologías responsables.",
            ]),
            ("La codificación como herramienta para aprender y crear", &[
                "Cada aplicación móvil, plataforma digital o sistema de inteligencia artificial opera mediante secuencias lógicas que procesan información y ejecutan tareas. En la base de este funcionamiento se encuentra la codificación, una disciplina que integra el razonamiento matemático, el pensamiento computacional y la creatividad para transformar ideas en soluciones tecnológicas.",
                "Programar implica definir los comandos necesarios para que un dispositivo procese datos, automatice procesos o desarrolle aplicaciones. La UNESCO vincula estas competencias con áreas como las matemáticas, la ciencia, la ingeniería, la comunicación y las artes, lo que confirma su carácter interdisciplinario.",
                "Es por esto que su aprendizaje ha cobrado un papel central en el fortalecimiento de habilidades transversales que van más allá de lo técnico, tales como la estructuración de ideas, el reconocimiento de patrones, el análisis de problemas y la evaluación crítica de alternativas.",
                "Igualmente, la expansión de tecnologías emergentes como la inteligencia artificial ha reforzado la relevancia de estos conocimientos, dado que los entornos digitales se fundamentan en código, arquitectura de datos y algoritmos para generar resultados.",
                "Este crecimiento exponencial y su adopción masiva demandan una formación tecnológica enfocada en el pensamiento crítico. Resulta indispensable que quienes desarrollan o emplean herramientas digitales aprendan a verificar la calidad de la información, detectar fallas en el software y comprender los alcances y limitaciones de los sistemas con los que interactúan.",
                "Al respecto, la UNESCO enfatiza la necesidad de incorporar principios éticos en la educación tecnológica, una perspectiva fundamental cuando los sistemas automatizados influyen directamente en ámbitos sensibles como la salud, la educación, la economía y la vida cotidiana.",
                "De igual manera, el acceso equitativo a la formación tecnológica constituye un aspecto central, dado que las disparidades en la disponibilidad de dispositivos, conectividad, recursos educativos y acompañamiento docente pueden profundizar la brecha digital y limitar la participación de diversos sectores sociales.",
                "Por esta razón, la enseñanza de la programación exige promover tanto el desarrollo de competencias técnicas como la responsabilidad social. Una solución informática no debe ser evaluada solo por su funcionamiento técnico, sino también por su utilidad, accesibilidad, seguridad e impacto en la sociedad.",
            ]),
            ("Ciencia de Datos en la UNC", &[
                "En Venezuela la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) ofrece la Licenciatura en Ciencia de Datos, un programa académico que integra estadística, programación, inteligencia artificial y análisis avanzado para formar profesionales capaces de transformar información en conocimiento útil.",
                "El plan de estudios contempla fundamentos de estadística, probabilidad, álgebra lineal y optimización, junto con lenguajes de programación, modelos predictivos, bases de datos, macrodatos (“big data”) y computación en la nube. Esta combinación permite comprender la relación entre las matemáticas, la computación y el análisis de información.",
                "La propuesta busca integrar fundamentos teóricos y prácticos para diseñar, implementar y evaluar soluciones innovadoras desde una perspectiva ética y orientada a la sostenibilidad, lo que conecta el aprendizaje de la ciencia de datos con la necesidad de desarrollar capacidades científicas y tecnológicas vinculadas con problemas concretos.",
                "Para la UNC, dominar la programación y el análisis de datos es una competencia fundamental para entender y transformar el entorno, y a través de la Licenciatura en Ciencia de Datos se impulsa la formación de profesionales capaces de liderar la era digital con responsabilidad, innovación y compromiso con el desarrollo nacional.",
            ]),
        ]
    },
    NewsArticle {
        slug: "ministro-yvan-gil-inaugura-espacios-tecnologicos-y-anuncia-nuevas-carreras-en-la-universidad-nacional-de-las-ciencias",
        original_url: "https://unc.edu.ve/index.php/ministro-yvan-gil-inaugura-espacios-tecnologicos-y-anuncia-nuevas-carreras-en-la-universidad-nacional-de-las-ciencias/",
        title: "Ministro Yván Gil inaugura espacios tecnológicos y anuncia nuevas carreras en la Universidad Nacional de las Ciencias",
        date: "28 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_14.jpg",
        excerpt: "El ministro del Poder Popular para Ciencia y Tecnología, Yván Gil, acompañado por la rectora de la Universidad de las Ciencias (UNC) Dr. Humberto Fernández-Morán, Gabriela Jiménez, directivos, autoridades y estudiantes, realizó un...",
        sections: &[
            ("", &[
                "El ministro del Poder Popular para Ciencia y Tecnología, Yván Gil, acompañado por la rectora de la Universidad de las Ciencias (UNC) Dr. Humberto Fernández-Morán, Gabriela Jiménez, directivos, autoridades y estudiantes, realizó un recorrido por el núcleo principal de esta casa de estudios.",
                "Durante la jornada quedaron inaugurados una Sala de Robótica y un Espacio de Innovación, Creación y Optimización (EICO), infraestructuras concebidas para fortalecer la formación práctica, la investigación aplicada y el desarrollo de soluciones tecnológicas desde la universidad.",
                "Asimismo, el titular del Ministerio del Poder Popular para Ciencia y Tecnología (Mincyt) conoció proyectos estudiantiles en áreas como robótica, biotecnología, informática, programación y protección de datos, destacando la preparación del talento joven para la muestra Venezuela Innova, prevista para el mes de diciembre en Caracas y Valencia.",
                "El ministro Yván Gil resaltó el crecimiento acelerado de la institución que, a un año y medio de su creación, cuenta con más de 3.000 estudiantes. «Es la Venezuela que renace, es la Venezuela que queremos, es la Venezuela productiva», afirmó la autoridad ministerial, al tiempo que agregó que «aquí tenemos un montón de proyectos listos para ser presentados, a ser financiados por el Sistema Nacional de Ciencia y Tecnología».",
                "Por su parte, Gabriela Jiménez, rectora de la UNC, destacó que la institución surgió como parte de un proyecto de universidades especializadas y que actualmente registra 18.006 solicitudes de ingreso.",
                "«Nuestro proyecto nació con el punto de cuenta de Alma Mater del Comandante Hugo Chávez, de crear universidades especializadas. De alguna forma, salir de las universidades clásicas, donde la ciencia era una facultad, para convertir la ciencia en una agenda completa de desarrollo situado de conocimiento», indicó Jiménez.",
                "La máxima autoridad universitaria informó además que los investigadores de la Fundación Instituto de Estudios Avanzados (IDEA) representan el 38% de la nómina docente de la UNC, al tiempo que anunció la revisión de los planes de estudio y la incorporación de las carreras de Virología, Geociencias e Ingeniería en Cambio Climático.",
            ]),
            ("Bautizo de dos obras del Fondo Editorial Mincyt", &[
                "En el marco de la visita se efectuó el bautizo de dos obras del Fondo Editorial Mincyt: Una ciencia liberadora para nuestro tiempo y La palestinización del otro, publicaciones que reúnen aportes de autoras y autores de siete países del sur global al pensamiento crítico y descolonial.",
                "La primera obra fue editada por Nerliny Caruci e incluye textos de Rafael Bautista Segales, Katia Colmenares, José Romero Lozada y Gabriela Jiménez Ramírez, mientras que la segunda fue compilada por Nerliny Caruci y Gabriela Jiménez Ramírez, con contribuciones de Ramón Grosfoguel, Irene León, Karina Ochoa Muñoz, Berenice Bento y Juan Cristóbal Altuna.",
                "Al referirse a las publicaciones, el ministro Yván Gil subrayó que el desarrollo científico debe responder a las necesidades del pueblo.",
                "«Cada investigación, cada trabajo, cada conocimiento, cada palabra, cada fórmula, cada ecuación, cada algoritmo que nosotros vemos, resolvemos, trabajamos, tiene un objetivo. Aprender para resolver situaciones, problemas de nuestro pueblo», manifestó.",
            ]),
            ("Anuncio de Ciencia Venezuela", &[
                "Finalmente, el titular del área anunció la creación de la iniciativa de divulgación científica Ciencia Venezuela, política que contempla una publicación en formato impreso y digital, así como una revista científica nacional para investigadores de Venezuela, América Latina y el Caribe, cuyo Consejo Editorial será presentado próximamente.",
                "«La primera función de la universidad es la extensión, es la divulgación, es la formación, es llevar el conocimiento a las comunidades, es llevar a que la gente se apropie del conocimiento», concluyó Gil.",
            ]),
        ]
    },
    NewsArticle {
        slug: "162-anos-del-nacimiento-de-jose-gregorio-hernandez-ciencia-fe-y-vocacion",
        original_url: "https://unc.edu.ve/index.php/162-anos-del-nacimiento-de-jose-gregorio-hernandez-ciencia-fe-y-vocacion/",
        title: "162 años del Nacimiento de José Gregorio Hernández: Ciencia, fe y vocación",
        date: "26 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_15.png",
        excerpt: "San José Gregorio Hernández, nació en Isnotú, estado Trujillo, el 26 de octubre de 1864. Fue médico, investigador, docente y precursor de la medicina experimental, pero también un hombre de profunda fe católica, cuya dedicación a los...",
        sections: &[
            ("", &[
                "San José Gregorio Hernández, nació en Isnotú, estado Trujillo, el 26 de octubre de 1864. Fue médico, investigador, docente y precursor de la medicina experimental, pero también un hombre de profunda fe católica, cuya dedicación a los enfermos, especialmente a los más desposeídos, dio origen a una devoción popular de gran fervor.",
                "Su legado supera los límites de la medicina y la religiosidad nacional. Tras celebrarse su canonización, el 19 de octubre de 2025, se consagró a José Gregorio Hernández como el primer santo de Venezuela.",
                "Antes de su reconocimiento oficial como santo, para millones de personas ya era una figura cercana al consolidarse, primero, como referente académico y profesional de la medicina y, más adelante, como el médico al que se oraba en la enfermedad, el hombre cuya imagen acompañaba los hogares venezolanos y el intercesor al que generaciones enteras atribuyeron favores y sanaciones.",
                "La trayectoria científica de José Gregorio Hernández nos permite comprender por qué su legado conserva particular relevancia para la historia contemporánea de Venezuela.",
                "Se graduó de doctor en Ciencias Médicas en la Universidad Central de Venezuela en 1888. Un año después recibió una beca para continuar su formación en Europa. En París estudió microscopía, histología, bacteriología y fisiología experimental. Posteriormente amplió sus estudios en Berlín.",
                "De regresó al país, en 1891, no se limitó a ejercer la medicina. Trajo consigo conocimientos, métodos e instrumentos que contribuirían a modificar la enseñanza médica venezolana. Fue responsable de los estudios de histología, fisiología experimental y bacteriología y participó en la creación de espacios de formación e investigación como el primer laboratorio científico universitario y la Cátedra de Bacteriología de la UCV, considerada pionera en América.",
                "Su contribución no terminó en la universidad, en 1904 fue miembro fundador de la Academia Nacional de Medicina y publicó, entre otras obras, Elementos de Bacteriología, en 1906, y Elementos de Filosofía, en 1912. De esta manera, enseñar, investigar y atender pacientes fueron su misión de vida.",
            ]),
            ("El médico de los pobres", &[
                "La entrega desinteresada de José Gregorio Hernández al asistir a personas de todas las condiciones sociales, especialmente, a los más necesitados, le valió la admiracióndel pueblo. De esa vocación altruista nació el título con el que se inscribió en la historia de Venezuela: el Médico de los Pobres. La expresión recoge valores asociados a su vida como la solidaridad, la honestidad, la responsabilidad, la dedicación, la constancia y el respeto.",
                "José Gregorio Hernández murió el 29 de junio de 1919, en Caracas, como consecuencia de un accidente de tránsito. Tenía 54 años. Su muerte marcó el inicio de una devoción que ha perdurado a lo largo de generaciones de venezolanos hasta nuestros días.",
                "Durante décadas, numerosos fieles le atribuyeron intercesiones y sanaciones que llevaron a la Iglesia católica a iniciar formalmente su causa de canonización en 1949. En 1972 fue proclamado Siervo de Dios y, en 1986, el papa Juan Pablo II aprobó sus virtudes y lo declaró Venerable.",
                "El paso decisivo hacia la beatificación llegó con el caso de una niña de diez años que había sufrido una herida de bala en la cabeza. La Comisión encargada de estudiar el caso en Venezuela aprobó en 2020 la atribución de la curación a la intercesión de José Gregorio Hernández, y el papa Francisco autorizó posteriormente el decreto que reconoció el milagro. Este hecho abrió el camino a su beatificación y selló el aval de la Iglesia católica a una devoción que ya tenía profundas raíces en el pueblo venezolano.",
                "En febrero de 2025, el papa Francisco autorizó al Dicasterio para las Causas de los Santos promulgar los decretos para la canonización del Beato José Gregorio Hernández.",
                "Posteriormente, luego del fallecimiento del papa Francisco, el papa León XIV convocó el consistorio en el que se estableció su inscripción en el Libro de los Santos para el 19 de octubre de 2025. La ceremonia se llevó a cabo en la Plaza San Pedro del Vaticano, donde también se canonizó a la religiosa María del Carmen Rendiles Martínez.",
            ]),
            ("Ciencia y fe", &[
                "La ejemplar vida de San José Gregorio Hernández muestra a un médico formado en los principales centros científicos de su época que, al mismo tiempo, mantuvo una profunda convicción religiosa. Para él, atender al enfermo era una expresión concreta de sus profundos principios espirituales.",
                "La rectora de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) Gabriela Jiménez Ramírez, ha señalado que la devoción popular hacia San José Gregorio Hernández es el resultado de “testimonios diarios de milagros y bendiciones que provenían de un científico que, en vida, se dedicó a ayudar a los más necesitados y que, al trascender, se convirtió en una presencia sagrada de fe”.",
                "Considera que la vida del santo es expresión de la concepción de “la vida y la ciencia como un acto profundamente humano”.",
                "Para la UNC, la figura de San José Gregorio Hernández es un referente histórico especialmente cercano al permanecer vigente la concepción del conocimiento bajo la perspectiva de la responsabilidad y compromiso social.",
                "Su ejemplo reivindica la dimensión humana de la actividad científica. Su legado se vincula con la filosofía educativa de la UNC que aspira integrar la excelencia académica con el bienestar de las personas y las necesidades de la sociedad.",
                "San José Gregorio Hernández representa al científico que puso su saber al servicio del país, al médico que atendió a los más necesitados, al mentor que formó a nuevas generaciones y al creyente cuya vida fue para millones de venezolanos un modelo de caridad. La rectora ha subrayado su improntaal referirse a él como «un hombre que amó a su patria tanto como amó a su prójimo».",
                "La figura de José Gregorio Hernández, como ha expresado Jiménez Ramírez, “…reconcilia dos mundos que la modernidad insiste en separar: el laboratorio y la capilla, la evidencia y la fe, el saber, la vida en comunidad, el servicio y la caridad”.",
                "El primer santo venezolano, que alguna vez llevó instrumentos y conocimientos científicos para contribuir a modernizar la medicina del país, permanece como figura venerada por la fe del pueblo venezolano y como precursor de una ciencia que entendió el saber no como privilegio, sino como responsabilidad frente a la vida.",
            ]),
        ]
    },
    NewsArticle {
        slug: "dia-internacional-contra-el-cambio-climatico",
        original_url: "https://unc.edu.ve/index.php/dia-internacional-contra-el-cambio-climatico/",
        title: "Día Internacional contra el Cambio Climático",
        date: "24 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_16.jpg",
        excerpt: "Hoy 24 de octubre se conmemora el Día Internacional contra el Cambio Climático. Esta jornada tiene como propósito alertar y generar conciencia a nivel global sobre los impactos del calentamiento global y la necesidad de reducir las...",
        sections: &[
            ("", &[
                "Hoy 24 de octubre se conmemora el Día Internacional contra el Cambio Climático. Esta jornada tiene como propósito alertar y generar conciencia a nivel global sobre los impactos del calentamiento global y la necesidad de reducir las emisiones de gases de efecto invernadero. Asimismo, busca impulsar la reflexión activa dentro de empresas, gobiernos y la ciudadanía sobre la forma en que las acciones individuales y colectivas pueden marcar la diferencia ante este desafío ambiental.",
                "El boletín del 8 de octubre del Centro de Predicción Climática de la agencia estadounidense NOAA estima una probabilidad superior al 83 por ciento de que El Niño persista con intensidad de fuerte a muy fuerte hasta el trimestre enero-marzo de 2027, y una probabilidad similar de que, en el trimestre octubre-diciembre de 2026, el evento supere la magnitud de este fenómeno registrados desde 1950. Para ese mismo trimestre, la Organización Meteorológica Mundial (OMM) prevé que la anomalía de la temperatura de la superficie del mar en el Pacífico tropical central alcance aproximadamente +3,7 grados Celsius, con pico hacia diciembre.",
                "El escenario climático anticipa un periodo especialmente difícil: calor intenso y sequía en algunas regiones, lluvias torrenciales, inundaciones y marejadas en otras.",
                "El calentamiento global es un proceso de largo plazo, impulsado por la acumulación de gases de efecto invernadero en la atmósfera, producto en buena medida de la quema de combustibles fósiles, la deforestación y ciertas prácticas agrícolas y ganaderas. El Grupo Intergubernamental de Expertos sobre el Cambio Climático (IPCC), que desde 1988 evalúa el estado del conocimiento en la materia, ha documentado en seis informes de evaluación el calentamiento del planeta ligado a la actividad humana.",
                "El Niño, en cambio, es un patrón climático natural que se repite de forma irregular cada dos a siete años. Consiste en el calentamiento anómalo de las aguas superficiales del Pacífico tropical, acompañado del debilitamiento de los vientos alisios. Ese calentamiento libera hacia la atmósfera cantidades enormes de calor y humedad, con efectos que se dejan sentir lejos del Pacífico: lluvias desordenadas, sequías donde antes había cosechas seguras, olas de calor más prolongadas.",
                "Los dos fenómenos interactúan. El Niño no causa el cambio climático, pero se despliega sobre un planeta ya calentado, y los especialistas advierten que esa interacción tiende a acentuar sus efectos sobre las precipitaciones y las temperaturas.",
            ]),
            ("El Pacífico sacude al planeta", &[
                "La cobertura internacional del fenómeno, incluido el reportaje «How will El Niño affect the world?» de la periodista Madeleine Cuff en New Scientist, del 29 de septiembre de 2026, advierte que se avecinan condiciones climáticas turbulentas en distintas regiones del planeta. El climatólogo Daniel Swain, de la Universidad de California en Los Ángeles (UCLA), ha señalado que el Pacífico tropical es una parte muy influyente de los océanos del mundo, con una capacidad desproporcionada para afectar el clima global: el fenómeno parte de una enorme liberación de calor y humedad que modifica los patrones de circulación atmosférica a escala planetaria.",
                "Los especialistas señalan que El Niño produce dos impactos principales. El primero es elevar las temperaturas globales del aire, a medida que el calor liberado por el océano se incorpora a la atmósfera. El segundo es la alteración del ciclo hidrológico: al cambiar los patrones de circulación atmosférica, se modifican la ubicación y el momento de las precipitaciones, con lluvias torrenciales e inundaciones repentinas en algunas zonas y sequías severas en otras.",
                "Swain ha explicado que «El Niño es como un breve e intenso impulso de cambio climático que sobrealimenta todos los efectos del calentamiento global».",
                "Por su parte, el doctor Dirk Thielen, jefe del Laboratorio de Ecología del Paisaje y Clima del Instituto Venezolano de Investigaciones Científicas (IVIC), señala que la geografía del país lo expone a condiciones contrastantes. En otras partes de Suramérica, como la selva amazónica, El Niño aumenta la probabilidad de sequías e incendios forestales, mientras que en Venezuela los efectos podrían diferir por regiones.",
                "El IVIC advirtió en agosto que analiza los posibles impactos de la proyección de un «Súper Niño» entre 2026 y parte de 2027, con sequías y estrés hídrico en algunas zonas del territorio y lluvias intensas en otras. El investigador ha vinculado la intensificación de estos eventos con el calentamiento oceánico observado a escala planetaria, y ha señalado que algunas regiones podrían experimentar condiciones más secas, lo que obligaría a los sectores agrícolas a ajustar sus planes de cultivo, mientras otras registrarían un incremento de las precipitaciones.",
                "En el estudio «Respuestas de las precipitaciones en Venezuela al actual calentamiento sin precedentes de las aguas oceánicas globales», publicado en 2024, Thielen y sus colaboradores dividieron a Venezuela en sus ocho regiones naturales más el Territorio Esequibo y documentaron pulsos de sequía extrema alternados con pulsos húmedos extremos, por lo que recomendaron preparar la respuesta para ambos extremos.",
            ]),
            ("Riesgos que aumentan", &[
                "Benjamin Horton, de la Universidad de la Ciudad de Hong Kong, ha advertido de que el evento de El Niño de 2026 podría ser el más fuerte en los últimos 70 años. «A medida que el océano se calienta, cada El Niño llega con más energía y severidad», ha señalado el investigador, al subrayar que El Niño y el calentamiento global se combinan para generar extremos climáticos más frecuentes.",
                "Los efectos se extienden por varios continentes. En la India, uno de los primeros países en sentir el impacto de El Niño, el fenómeno suele alterar la temporada del monzón de junio a septiembre. Los riesgos de calor extremo, sequía y estrés hídrico alcanzan también al sur de Asia, incluyendo Pakistán, Sri Lanka, Bangladesh y Nepal.",
                "En África, la organización humanitaria Save the Children advirtió, en un análisis difundido el 22 de septiembre de 2026, que unos ocho millones de niños de África oriental y meridional podrían enfrentar hambre aguda a principios de 2027, un aumento del 30 por ciento con respecto a los niveles actuales, impulsado por uno de los episodios de El Niño más intensos de los últimos años.",
                "La OMM, por su parte, sitúa la alerta humanitaria más inmediata en el este de África, con mayor riesgo de lluvias intensas e inundaciones entre octubre y diciembre, y en el sur del continente, donde se esperan condiciones más secas a partir de noviembre.",
                "En la costa occidental de Estados Unidos, el cuadro se invierte: el riesgo es el exceso de agua, puesto que se prevé que un El Niño histórico intensifique los ríos atmosféricos que suelen descargar fuertes lluvias sobre esa región.",
                "Frente a la complejidad de este fenómeno, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) ratifica su compromiso institucional como centro de formación científica, innovación tecnológica y educación ciudadana.",
                "La UNC anunció la incorporación de la Licenciatura en Ciencias Ambientales y Climáticas a su oferta académica próximamente. Al respecto, el doctor Jhan Leal, miembro del Comité Curricular, ha señalado que la carrera «nace de la necesidad nacional de contar con expertos que puedan enfrentar los efectos de grandes lluvias, inundaciones o sequías, y apoyar la planificación territorial ante estos escenarios».",
                "La conmemoración del Día Internacional contra el Cambio Climático constituye un marco propicio para recordar que la preservación de los equilibrios ecosistémicos demanda el esfuerzo convergente de la investigación científica, las políticas gubernamentales y la conciencia comunitaria. Desde sus aulas y laboratorios, la UNC reafirma su labor de generar conocimiento riguroso para construir un modelo de desarrollo sostenible.",
            ]),
        ]
    },
    NewsArticle {
        slug: "23-de-octubre-dia-mundial-del-mol",
        original_url: "https://unc.edu.ve/index.php/23-de-octubre-dia-mundial-del-mol/",
        title: "23 de octubre: Día Mundial del Mol",
        date: "23 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_17.jpg",
        excerpt: "Comprender de qué está hecha la materia exige estudiar elementos que no podemos observar directamente. Los átomos, las moléculas y los iones forman parte de las sustancias que nos rodean, desde el agua que bebemos hasta los materiales...",
        sections: &[
            ("", &[
                "Comprender de qué está hecha la materia exige estudiar elementos que no podemos observar directamente. Los átomos, las moléculas y los iones forman parte de las sustancias que nos rodean, desde el agua que bebemos hasta los materiales utilizados en la industria y los componentes que hacen posible la vida. Para cuantificar estas partículas y establecer relaciones entre ellas, la química cuenta con una unidad fundamental: el mol.",
                "El Día del Mol, que se celebra el 23 de octubre, busca acercar la química al público. Aspira que la población se acerque a uno de los conceptos que hacen posible estudiar cuantitativamente la materia y comprender los procesos químicos y biológicos que ocurren a nuestro alrededor. La jornada se extiende desde las 6:02 de la mañana hasta las 6:02 de la tarde, en alusión al número de Avogadro: 6,02 × 1023, es decir, la cantidad de partículas que hay en un mol.",
            ]),
            ("Unidad para contar lo invisible", &[
                "El mol es la unidad de cantidad de sustancia del Sistema Internacional de Unidades. Desde 2019, su definición se basa en un valor fijo al determinar que un mol contiene exactamente 6,02214076 × 1023 entidades elementales que equivalen a unos 602.000 trillones que pueden ser átomos, moléculas, iones, electrones u otras partículas.",
                "El número corresponde a la constante de Avogadro, denominada así en reconocimiento al científico italiano Amedeo Avogadro, quien en 1811 propuso que volúmenes iguales de gases, a igual temperatura y presión, contienen el mismo número de moléculas. Esa idea ayudó a entender la relación entre los gases y las partículas que los componen.",
                "En la práctica, el mol permite relacionar el mundo microscópico de los átomos y las moléculas con cantidades que pueden medirse en el laboratorio. Gracias a esta unidad, los químicos calculan las proporciones entre reactivos y productos, preparan soluciones de concentración conocida y determinan las cantidades necesarias para que una reacción ocurra de acuerdo con las condiciones previstas.",
                "Estos procedimientos son habituales en los laboratorios de investigación, la industria farmacéutica, la producción de alimentos y los análisis ambientales. En todos esos ámbitos, conocer la cantidad de sustancia y su relación con la masa, el volumen y la concentración ayuda a interpretar resultados, planificar experimentos y reproducir procedimientos con precisión.",
                "El concepto de mol permite comprender que, aunque las partículas que constituyen la materia no sean visibles a simple vista, es posible estudiarlas mediante herramientas conceptuales y matemáticas que posibilitan describir su comportamiento y comprender las transformaciones que experimentan.",
            ]),
            ("Ciencia Molecular en la UNC", &[
                "La Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) ofrece la Licenciatura en Ciencia Molecular, una carrera que integra conocimientos de bioquímica, biología molecular, genética, bioinformática y biotecnología. Su enfoque interdisciplinario permite abordar los fenómenos biológicos desde una perspectiva molecular, relacionando los principios de la química con los procesos que ocurren en los organismos vivos.",
                "Esta formación comprende el estudio de las moléculas biológicas, sus propiedades y las interacciones que establecen dentro de las células y los organismos. También permite aproximarse a las herramientas científicas empleadas para investigar esos procesos, interpretar información biológica y comprender cómo las variaciones moleculares pueden influir en el funcionamiento de los sistemas.",
                "La Licenciatura prepara profesionales para participar en investigaciones interdisciplinarias y contribuir a la generación de conocimientos. El desarrollo de estas capacidades demanda una formación científica rigurosa, uso de herramientas de investigación, actualización permanente y cooperación con centros científicos e instituciones dedicadas a la producción de conocimiento.",
                "Estos conocimientos tienen aplicaciones en campos como la investigación biomédica, el diagnóstico de enfermedades, el desarrollo de productos biotecnológicos, la mejora de cultivos y la biorremediación.",
                "A propósito del Día del Mol, la relación entre los principios químicos y los procesos biológicos permite entender la importancia de la Ciencia Molecular como disciplina de formación universitaria.",
                "El Día del Mol es una ocasión para valorar la enseñanza de la química y reconocer a quienes hacen del aprendizaje científico una experiencia cercana y significativa.",
                "Para la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán, comprender lo que ocurre en el mundo molecular es, en definitiva, una manera de ampliar nuestra comprensión de la vida y de reconocer el valor de la investigación científica para responder a los desafíos del presente.",
            ]),
        ]
    },
    NewsArticle {
        slug: "dia-mundial-de-cancer-de-mama-investigacion-cientifica-y-formacion-universitaria-al-servicio-de-la-vida",
        original_url: "https://unc.edu.ve/index.php/dia-mundial-de-cancer-de-mama-investigacion-cientifica-y-formacion-universitaria-al-servicio-de-la-vida/",
        title: "Día Mundial de Cáncer de mama: Investigación científica y formación universitaria al servicio de la vida",
        date: "19 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_18.png",
        excerpt: "El 19 de octubre se conmemora el Día Mundial de la Lucha contra el Cáncer de Mama, la fecha convoca a instituciones sanitarias, investigadores, universidades y comunidades a reforzar la difusión de información sobre esta enfermedad,...",
        sections: &[
            ("", &[
                "El 19 de octubre se conmemora el Día Mundial de la Lucha contra el Cáncer de Mama, la fecha convoca a instituciones sanitarias, investigadores, universidades y comunidades a reforzar la difusión de información sobre esta enfermedad, promover la detección temprana y recordar la importancia de garantizar atención médica oportuna.",
                "La efeméride, es una oportunidad para dar a conocer los avances científicos que buscan mejorar el diagnóstico y los tratamientos, así como el papel de la formación de profesionales capaces de participar en investigaciones.",
                "Según la Organización Mundial de la Salud (OMS), durante 2024 se diagnosticaron aproximadamente 2,4 millones de casos de cáncer de mama en mujeres y se registraron 694.000 fallecimientos en el mundo. Ese año, fue el cáncer más frecuente entre las mujeres en 164 de los 186 países examinados. Aunque la enfermedad afecta principalmente a la población femenina, también puede presentarse en hombres.",
                "Estas cifras reflejan un desafío sanitario que requiere educación, servicios de diagnóstico, tratamientos adecuados y continuidad en la atención.",
            ]),
            ("Detectar a tiempo y garantizar la atención", &[
                "El cáncer de mama se produce cuando determinadas células del tejido mamario comienzan a multiplicarse de manera descontrolada y pueden formar tumores malignos. Su evolución depende de características biológicas del tumor y de la extensión de la enfermedad, factores que influyen en las decisiones clínicas y en la selección del tratamiento.",
                "La detección temprana mejora las posibilidades de recibir tratamiento eficaz. Por esto, la OMS promueve una estrategia integral que comprende educación para reconocer signos de alerta, diagnóstico oportuno y atención coordinada.",
                "Entre los signos que ameritan consulta figuran la aparición de un nódulo o engrosamiento en la mama, cambios en su tamaño o forma, alteraciones de la piel, modificaciones en el pezón y secreciones anormales.",
                "Algunos de los factores de riesgo del cáncer de mama son la predisposición biológica y genética, historia hormonal y reproductivo y factores modificables como obesidad, consumo de alcohol o tabaco, sedentarismo y terapia de reemplazo de hormonal posmenopáusica.",
                "La educación en salud resulta indispensable para reconocer a tiempo los signos de alerta. La autoexploración periódica y la asistencia a consultas médicas regulares continúan siendo las herramientas fundamentales para la prevención primaria.",
                "La Organización Panamericana de la Salud (OPS) advierte que las desigualdades en el acceso al diagnóstico y al tratamiento influyen en resultados de salud no deseados. De ahí que las campañas de sensibilización deban acompañarse de servicios accesibles, personal capacitado y rutas de atención que permitan a la población, sobre todo de menos recursos, la realización de procedimientos, diagnósticos y terapéuticos.",
            ]),
            ("Ciencias que contribuyen a la investigación oncológica", &[
                "La Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) cuenta con programas académicos vinculados con áreas de conocimiento que pueden brindar importantes aportes al estudio de enfermedades complejas como el cáncer de mama.",
                "Física Nuclear, Ciencia Molecular, Biotecnología, Nanotecnología y Biomateriales ofrecen perspectivas distintas y complementarias para comprender los procesos biológicos, explorar nuevas moléculas y desarrollar tecnologías relacionadas con el diagnóstico y el tratamiento.",
                "Estas relaciones entre disciplinas muestran que la investigación oncológica requiere más que conocimientos médicos. El diseño de una molécula, la comprobación de su actividad biológica, el estudio de materiales y la evaluación de la interacción con la radiación plantean problemas científicos diferentes que deben abordarse mediante equipos multidisciplinarios.",
                "Para la UNC, la formación en estas áreas representa una oportunidad de preparar profesionales que, mediante investigación y cooperación institucional, puedan incorporarse a iniciativas científicas vinculadas con las necesidades de salud del país.",
                "Asimismo, el Instituto Venezolano de Investigaciones Científicas (IVIC) desarrolla una línea de investigación que explora una posible combinación de mecanismos de quimioterapia y radioterapia mediante el diseño de una molécula modificada a partir del tamoxifeno, medicamento utilizado en determinados tipos de cáncer de mama. La investigación está dirigida por el doctor en Química Álvaro Álvarez Aular, jefe del Laboratorio de Fisicoquímica Orgánica del Centro de Química del IVIC.",
                "El estudio consiste en modificar la estructura del tamoxifeno e incorporar gadolinio, un elemento químico cuyas propiedades se investigan en distintas aplicaciones. El equipo busca conservar la parte de la molécula responsable de su acción antiestrogénica y modificar otra región para incorporar el componente que permitiría explorar su interacción con la radiación.",
                "El Dr. Alvarez, en nota publicada por Prensa IVIC, el 2 de junio de 2025, explica que “para este estudio hemos tomado como prueba piloto el cáncer de mama y la idea es buscar la forma de combinar en una molécula quimio y radio. Esto se puede lograr tomando el medicamento más usado para tratar el cáncer de mamas llamado Tamoxifen que bloquea la acción del estrógeno en el tejido mamario. Esto puede detener el crecimiento de algunos tumores de mama que necesitan estrógeno para crecer”.",
                "El trabajo del IVIC permite reconocer la importancia de los centros de investigación científica en el desarrollo de conocimiento aplicado a problemas de salud pública. A su vez, la formación que ofrece a Universidad Nacional de las Ciencias puede ampliar la base de profesionales con competencias para participar en proyectos experimentales, comprender sus fundamentos y contribuir a nuevas líneas de investigación.",
                "La respuesta a esta enfermedad depende de importantes y constantes acciones, como reconocer las señales de alerta, acudir a los servicios de salud, facilitar el diagnóstico, garantizar tratamientos adecuados y sostener la investigación científica.",
                "El Día Mundial contra el Cáncer de Mama busca acercar la ciencia a la sociedad es una tarea que conecta educación, investigación y salud pública, dirigida a mejorar la prevención, la atención y la calidad de vida de quienes la enfrentan. Desde la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán reafirmamos nuestro compromiso con la divulgación científica, la promoción de la salud y el respaldo a las iniciativas de investigación que contribuyen al bienestar de la población. La detección temprana, la equidad en el tratamiento y el desarrollo científico representan la vía fundamental para salvar vidas.",
            ]),
        ]
    },
    NewsArticle {
        slug: "12-de-octubre-dia-de-la-resistencia-indigena-memoria-historica-y-compromiso-para-construir-futuro",
        original_url: "https://unc.edu.ve/index.php/12-de-octubre-dia-de-la-resistencia-indigena-memoria-historica-y-compromiso-para-construir-futuro/",
        title: "12 de octubre Día de la Resistencia Indígena: Memoria histórica y compromiso para construir futuro",
        date: "12 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_19.png",
        excerpt: "La conmemoración trasciende un episodio histórico para reivindicar la memoria colectiva y el lugar que ocupan los pueblos indígenas en la construcción del presente y del futuro del país. La Resistencia Indígena no solo es un...",
        sections: &[
            ("", &[
                "La conmemoración trasciende un episodio histórico para reivindicar la memoria colectiva y el lugar que ocupan los pueblos indígenas en la construcción del presente y del futuro del país.",
                "La Resistencia Indígena no solo es un reconocimiento a la lucha frente a la conquista y colonización, sino a la defensa de pueblos que mantienen formas propias de organización, conocimientos, prácticas culturales y relaciones con sus territorios. Esta posición amplía la comprensión de procesos históricos complejos y de la diversidad de actores que han participado en el desarrollo del país.",
            ]),
            ("Una nación multiétnica y pluricultural", &[
                "El reconocimiento a las raíces pluriculturales y la vigencia de los pueblos originarios, consolida en Venezuela la comprensión de la identidad nacional como una plataforma activa para fomentar el diálogo intercultural, garantizar la justicia social y reivindicar el patrimonio multicultural que ha definido a la sociedad venezolana en su devenir histórico.",
                "Este cambio conceptual se corresponde con lo establecido en la Constitución de la República Bolivariana de Venezuela, cuyo Capítulo VIII consagra la existencia de las comunidades originarias, sus formas de organización social, política y económica, así como sus lenguas, costumbres, religiones, hábitat y derechos originarios sobre las tierras que ocupan.",
                "En el ámbito académico y tecnológico, la reivindicación indígena ha abierto nuevas vías de integración. La vinculación entre la investigación científica formal y los saberes tradicionales constituye una prioridad estratégica para abordar desafíos en áreas operativas clave como la salud pública, la seguridad alimentaria, la preservación de cuencas hidrográficas y la gestión ambiental.",
                "En este sentido, la UNESCO ha documentado la importancia de los sistemas de conocimientos indígenas y locales para la conservación de la biodiversidad y la adaptación al cambio climático. El organismo sostiene que la incorporación de estos saberes posiciona a las comunidades como actores activos y determinantes en la investigación y la toma de decisiones, superando la postura que las reducía a simples objetos de estudio. Esta perspectiva resulta especialmente pertinente en ámbitos como la conservación de ecosistemas, la seguridad alimentaria, la gestión del agua y la adaptación a las transformaciones climáticas.",
                "En Venezuela, la articulación entre el conocimiento científico y los saberes comunitarios forma parte de un debate amplio e inclusivo sobre la función social de la ciencia ante las demandas del país. Bajo este paradigma, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) impulsa un modelo académico y de investigación que integra los saberes ancestrales en la búsqueda de soluciones con impacto territorial.",
                "La rectora Gabriela Jiménez Ramírez ha enfatizado en la urgencia de reivindicar los saberes propios, al respecto ha señalado la necesidad de «descolonizar las formas y los procesos de la producción, de cómo hemos sido educados, de cómo se han creado los conocimientos, inclusive de la filosofía del modelo eurocentrista, que quiso de alguna forma invisibilizar la capacidad de creación de nuestros pueblos ancestrales».",
                "De este modo, se reafirma la importancia de consolidar una ciencia nacionalista, comprometida con la historia, la paz y el bienestar, capaz de responder a la diversidad, la espiritualidad y la cultura del pueblo venezolano, integrando plenamente los sistemas de conocimiento de las comunidades indígenas.",
                "Es así como la Resistencia Indígena encuentra en las UNC un espacio de reflexión que conecta historia, educación, ciencia y territorio. La convergencia entre la ciencia formal y las prácticas comunitarias tradicionales permite estructurar un diálogo ético, riguroso y participativo que responde a las necesidades del país y fortalece la soberanía científica.",
                "Este planteamiento sienta las bases de una praxis universitaria arraigada en el contexto nacional y comprometida con el pluralismo cultural. La premisa de investigar con las comunidades resulta determinante para capacitar investigadores que interpreten las dinámicas territoriales, valoren el saber originario y local, y encaucen las demandas sociales hacia respuestas con sustento científico.",
                "El Día de la Resistencia Indígena se convierte en la reivindicación activa de las comunidades que sostienen y enriquecen el tejido plurinacional del país. Frente al reto de repensar nuestra memoria colectiva, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán revalida su responsabilidad social y académica, para la institución, honrar este legado significa cultivar un conocimiento a través de formación, investigación e innovación resultado de un rigor académico y científico que nace del diálogo con las comunidades y coloca la innovación al servicio del desarrollo humano.",
                "Desde el año 2002, se conmemora oficialmente en Venezuela el 12 de octubre como Día de la Resistencia Indígena mediante Decreto Presidencial N.º 2.028, publicado en la Gaceta Oficial N.º 5.605 Extraordinario. Su primer artículo señala expresamente que la fecha está destinada a reconocer la unidad y diversidad cultural y humana, reivindicar a los pueblos indígenas de América y reconocer los aportes de las culturas africanas, asiáticas y europeas en la conformación de la nacionalidad venezolana.",
                "La conmemoración trasciende un episodio histórico para reivindicar la memoria colectiva y el lugar que ocupan los pueblos indígenas en la construcción del presente y del futuro del país.",
                "La Resistencia Indígena no solo es un reconocimiento a la lucha frente a la conquista y colonización, sino a la defensa de pueblos que mantienen formas propias de organización, conocimientos, prácticas culturales y relaciones con sus territorios. Esta posición amplía la comprensión de procesos históricos complejos y de la diversidad de actores que han participado en el desarrollo del país.",
            ]),
            ("Una nación multiétnica y pluricultural", &[
                "El reconocimiento a las raíces pluriculturales y la vigencia de los pueblos originarios, consolida en Venezuela la comprensión de la identidad nacional como una plataforma activa para fomentar el diálogo intercultural, garantizar la justicia social y reivindicar el patrimonio multicultural que ha definido a la sociedad venezolana en su devenir histórico.",
                "Este cambio conceptual se corresponde con lo establecido en la Constitución de la República Bolivariana de Venezuela, cuyo Capítulo VIII consagra la existencia de las comunidades originarias, sus formas de organización social, política y económica, así como sus lenguas, costumbres, religiones, hábitat y derechos originarios sobre las tierras que ocupan.",
                "En el ámbito académico y tecnológico, la reivindicación indígena ha abierto nuevas vías de integración. La vinculación entre la investigación científica formal y los saberes tradicionales constituye una prioridad estratégica para abordar desafíos en áreas operativas clave como la salud pública, la seguridad alimentaria, la preservación de cuencas hidrográficas y la gestión ambiental.",
                "En este sentido, la UNESCO ha documentado la importancia de los sistemas de conocimientos indígenas y locales para la conservación de la biodiversidad y la adaptación al cambio climático. El organismo sostiene que la incorporación de estos saberes posiciona a las comunidades como actores activos y determinantes en la investigación y la toma de decisiones, superando la postura que las reducía a simples objetos de estudio. Esta perspectiva resulta especialmente pertinente en ámbitos como la conservación de ecosistemas, la seguridad alimentaria, la gestión del agua y la adaptación a las transformaciones climáticas.",
                "En Venezuela, la articulación entre el conocimiento científico y los saberes comunitarios forma parte de un debate amplio e inclusivo sobre la función social de la ciencia ante las demandas del país. Bajo este paradigma, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) impulsa un modelo académico y de investigación que integra los saberes ancestrales en la búsqueda de soluciones con impacto territorial.",
                "La rectora Gabriela Jiménez Ramírez ha enfatizado en la urgencia de reivindicar los saberes propios, al respecto ha señalado la necesidad de «descolonizar las formas y los procesos de la producción, de cómo hemos sido educados, de cómo se han creado los conocimientos, inclusive de la filosofía del modelo eurocentrista, que quiso de alguna forma invisibilizar la capacidad de creación de nuestros pueblos ancestrales».",
                "De este modo, se reafirma la importancia de consolidar una ciencia nacionalista, comprometida con la historia, la paz y el bienestar, capaz de responder a la diversidad, la espiritualidad y la cultura del pueblo venezolano, integrando plenamente los sistemas de conocimiento de las comunidades indígenas.",
                "Es así como la Resistencia Indígena encuentra en las UNC un espacio de reflexión que conecta historia, educación, ciencia y territorio. La convergencia entre la ciencia formal y las prácticas comunitarias tradicionales permite estructurar un diálogo ético, riguroso y participativo que responde a las necesidades del país y fortalece la soberanía científica.",
                "Este planteamiento sienta las bases de una praxis universitaria arraigada en el contexto nacional y comprometida con el pluralismo cultural. La premisa de investigar con las comunidades resulta determinante para capacitar investigadores que interpreten las dinámicas territoriales, valoren el saber originario y local, y encaucen las demandas sociales hacia respuestas con sustento científico.",
                "El Día de la Resistencia Indígena se convierte en la reivindicación activa de las comunidades que sostienen y enriquecen el tejido plurinacional del país. Frente al reto de repensar nuestra memoria colectiva, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán revalida su responsabilidad social y académica, para la institución, honrar este legado significa cultivar un conocimiento a través de formación, investigación e innovación resultado de un rigor académico y científico que nace del diálogo con las comunidades y coloca la innovación al servicio del desarrollo humano.",
            ]),
        ]
    },
    NewsArticle {
        slug: "universidad-nacional-de-las-ciencias-celebro-su-1er-encuentro-del-club-de-robotica-360-impulsando-la-innovacion-con-sello-nacional",
        original_url: "https://unc.edu.ve/index.php/universidad-nacional-de-las-ciencias-celebro-su-1er-encuentro-del-club-de-robotica-360-impulsando-la-innovacion-con-sello-nacional/",
        title: "Universidad Nacional de las Ciencias celebró su 1er encuentro del Club de Robótica 360° impulsando la innovación con sello nacional",
        date: "9 de octubre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_20.jpg",
        excerpt: "El 1er Encuentro del Club de Robótica de la Universidad Nacional de las Ciencias (UNC) Dr. Humberto Fernández-Morán 360° reunió a estudiantes y expositores en un espacio donde la tecnología, la innovación y la ciencia se unieron para...",
        sections: &[
            ("", &[
                "El 1er Encuentro del Club de Robótica de la Universidad Nacional de las Ciencias (UNC) Dr. Humberto Fernández-Morán 360° reunió a estudiantes y expositores en un espacio donde la tecnología, la innovación y la ciencia se unieron para impulsar la creatividad de una generación del siglo XXI que construye el futuro desde las raíces del país.",
                "Durante la actividad, la rectora de esta casa de estudio, Gabriela Jiménez Ramírez, destacó que desde la universidad se están abriendo caminos y espacios de libertad para que la ciencia tenga un auténtico sello nacional.",
                "“La Venezuela que renace es una Venezuela que se fundamenta en la capacidad de su juventud. Ver que esos valores, principios y conocimientos hoy forman parte de esta fiesta de intercambio tecnológico, generando nuestra propia cultura científica y de innovación nacional, nos confirma que estamos en el camino correcto”, expresó Jiménez Ramírez.",
                "Por su parte, Irwin Acevedo, Director Estudiantil de la UNC explicó que esta iniciativa busca fomentar la articulación, el trabajo en equipo y, sobre todo, explorar nuevas áreas. “Estamos trabajando con nueve áreas estratégicas, que son: agroindustria, telecomunicaciones, logística, construcción, automatización y, sobre todo, en el área salud y tecnología”.",
                "Además explicó que la temática cuenta con tres áreas fundamentales: las áreas de demostraciones en tres aspectos (la electrónica, la impresión 3D y la arquitectura) y lo que son la estación de prototipos, y los trabajos individuales y grupales.",
                "Finalizó señalando que para la institución fomentar este tipo de iniciativa es fundamental para el trabajo colaborativo. “Nuestro club de robótica persigue crear, innovar, pero sobre todo transformar para la política nacional”, acotó.",
                "Durante la exhibición de sus prototipos, los estudiantes demostraron su capacidad creadora al desarrollar soluciones que van más allá de lo convencional. Ejemplo de ello es el proyecto presentado por el equipo de CROMA, integrado por Eliécer Marcano, Gabriela Rivas, Alfredo Rios, Sebastian Márquez, Bruno Villalobos y Yorgelis Alvarado enfocado en la robótica aplicada a la medicina.",
                "Esta iniciativa busca crear herramientas tecnológicas que ayuden a personas en procesos de recuperación tras sufrir un Accidente Cerebrovascular (ACV) o una lesión grave en las extremidades inferiores, permitiéndoles recuperar su independencia y mejorar su calidad de vida día a día.",
                "La universidad ofrece actualmente 15 carreras, entre las que destaca Ingeniería en Robótica y Automatización, con una duración de 4 años divididos en 8 semestres. Este programa académico abarca áreas estratégicas como:",
                "Robótica: Enmarcada en el diseño, programación y control de robots.",
                "Automatización industrial: Centrada en el control de procesos e instrumentación industrial.",
                "Mecatrónica: Integración de sistemas mecánicos, electrónicos y de control.",
                "Inteligencia artificial: Aplicación de técnicas de IA en sistemas robóticos.",
                "Visión artificial: Procesamiento de imágenes para robots.",
                "Esta carrera ofrece un amplio campo laboral con alta demanda en sectores como la industria manufacturera, la automatización de edificios, la robótica de servicios y la investigación científica.",
                "La Universidad Nacional de las Ciencia busca fomentar la articulación y el trabajo colaborativo de los estudiantes con la ciencia, tecnología y la innovación.Prensa UNC – MG",
            ]),
        ]
    },
    NewsArticle {
        slug: "dia-internacional-de-la-cultura-cientifica-ciencia-para-todos",
        original_url: "https://unc.edu.ve/index.php/dia-internacional-de-la-cultura-cientifica-ciencia-para-todos/",
        title: "Día Internacional de la Cultura Científica: Ciencia para todos",
        date: "28 de septiembre de 2026",
        author: "Prensa UNC – Yanett Alvarez",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_21.png",
        excerpt: "Millones de personas se relacionan diariamente con la ciencia al momento de elegir una fuente de información o de opinión sobre salud o cambio climático, interpretar una cifra, evaluar una afirmación sobre inteligencia artificial, entre...",
        sections: &[
            ("", &[
                "Millones de personas se relacionan diariamente con la ciencia al momento de elegir una fuente de información o de opinión sobre salud o cambio climático, interpretar una cifra, evaluar una afirmación sobre inteligencia artificial, entre otras acciones que requieren herramientas para comprender cómo funciona la ciencia.",
                "El Día Internacional de la Cultura Científica, que se celebra cada 28 de septiembre, busca destacar la importancia de acercar la ciencia a la sociedad y de promover una ciudadanía capaz de comprender, valorar y utilizar el conocimiento científico en su vida diaria. La iniciativa surgió en 2020, impulsada por la Sociedad Mexicana para la Divulgación de la Ciencia y la Técnica (SOMEDICYT) y otras organizaciones de México y Costa Rica.",
                "La elección del 28 de septiembre responde al aniversario de la primera transmisión de Cosmos, la reconocida serie de tv de divulgación científica, creada y conducida por el célebre astrónomo Carl Sagan en 1980. Este día busca acercar los procesos y conocimientos científicos a personas que no pertenecen al ámbito académico para favorecer su apropiación social.",
                "La cultura científica, desde este punto de vista busca promover la difusión y comprensión de la ciencia como parte de la cultura y como un recurso para interpretar nuestro entorno, formular preguntas y participar responsablemente en las decisiones que involucran conocimiento científico y tecnológico.",
            ]),
            ("Ciencia para todos", &[
                "Durante mucho tiempo, la ciencia ha sido presentada mediante grandes descubrimientos o prominentes investigadores que transformaron determinadas áreas del conocimiento. Sin embargo, la cultura científica propone ampliar esa mirada al poner a disposición de todos conocimientos generales, habilidades y criterios que permitan aproximarse a la ciencia, comprender su papel en la sociedad y valorar sus aportes.",
                "La cultura científica busca consolidar una ciudadanía que domine conceptos y hechos básicos, así como que tenga conciencia sobre el papel de las ciencias en ell progreso sociall y en la vida cotidiana. Se sostiene sobre tres pillares fundamentales:",
                "• Pensamiento crítico: fortalece la capacidad de analizar la información, verificar datos e identificar noticias falsas antes de tomar decisiones.",
                "• Curiosidad e indagación: motiva a explorar, experimentar y aprender bajo estándares y criterios validados.",
                "• Participación ciudadana: empodera a la sociedad para debatir y tomar posición frente a temas de alto impacto, como la sostenibilidad ambiental, el uso de la energía o las emergencias sanitarias.",
                "Asimismo, su consolidación y transmisión se apoya en tres vías principales:",
                "• Divulgación científica: traduce e ilustra conceptos complejos para la sociedad mediante publicaciones, documentales y plataformas divulgativas.",
                "• Comunicación institucional: permite que, por ejemplo, universidades y centros de investigación visibilicen el propósito y el impacto social de sus hallazgos.",
                "• Educación formal: promueve vocaciones científicas desde las etapas escolares y forma a las nuevas generaciones en métodos de acceso al conocimiento con actitud crítica y rigor probatorio.",
                "Adquirir conocimientos de las ciencias tiene una significativa importancia a través del potencial de las tecnologías de la Información de la actualidad como Internet y las redes sociales, que han ampliado extraordinariamente el acceso a contenidos científicos.",
                "El proceso que representa todo un desafío en la sociedad contemporánea no consiste simplemente en acumular información, sino en fortalecer un conocimiento crítico al aprender a preguntar de dónde proviene un dato, qué evidencia lo respalda, cuáles son sus límites y qué diferencias existen entre una hipótesis, una conclusión científica y una opinión.",
                "La propia iniciativa del Día Internacional de la Cultura Científica destaca esta necesidad de fortalecer una cultura ciudadana compatible con la ciencia, que permita tomar decisiones fundamentadas y mantener una postura crítica frente a la proliferación de información falsa.",
            ]),
            ("La universidad como promotora de la ciencia", &[
                "La universidad ocupa un lugar particular en este proceso. al acercar a los estudiantes a las formas en que se construye el conocimiento. Una educación basada en cultura científica favorece la indagación y el descubrimiento, permite formular hipótesis, diseñar experimentos y comprender los procedimientos utilizados para validar o refutar ideas. También incorpora dimensiones relacionadas con la ética y la responsabilidad.",
                "En la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC), esta perspectiva tiene una relación directa con su oferta académica. Carreras como Física, Matemáticas, Biotecnología, Ciencia de Datos, Nanotecnología, Ciencia Molecular, Inteligencia Artificial, Robótica y Automatización, Electromedicina, Biomateriales, Oceanología y otras áreas de las ciencias naturales, aplicadas y tecnológicas, sitúan al estudiante frente a problemas que demandan observación, análisis, experimentación y pensamiento crítico.",
                "El conocimiento proporciona herramientas para comprender el mundo, pero la educación también debe preparar para decidir cómo utilizar esas herramientas. Esta relación entre conocimiento y responsabilidad forma parte de la concepción educativa de la UNC.",
                "La cultura científica, desde este punto de vista, incorpora una dimensión ética. Una sociedad científicamente formada debería ser capaz de preguntarse sobre las consecuencias, los beneficios y los riesgos de un avance o conocimiento. En la UNC, este enfoque está presente de manera transversal en la formación de profesionales destinados a trabajar en áreas donde el desarrollo científico y tecnológico puede modificar profundamente la vida de las personas.",
                "Una universidad científica como la UNC tiene la responsabilidad de comunicar y entrar en contacto con una sociedad que necesita comprender cada vez mejor los fenómenos científicos y tecnológicos que afectan su vida. Por eso, la divulgación debe formar parte de un circuito más amplio junto con la educación y la comunicación científica.",
                "Para la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán, el Día Internacional de la Cultura Científica permite reconocer una tarea que comienza en la formación de sus estudiantes y puede proyectarse mucho más allá de sus aulas. Formar científicos, ingenieros y profesionales de las nuevas tecnologías supone también formar ciudadanos capaces de investigar, cuestionar, argumentar y tomar decisiones sustentadas en conocimientos.",
                "Y esa mirada, sustentada en conocimiento, pensamiento crítico y responsabilidad, es la que permite que la ciencia deje de ser únicamente un conjunto de disciplinas para convertirse en parte activa de la cultura y de la vida de la sociedad.",
            ]),
        ]
    },
    NewsArticle {
        slug: "jovenes-talentos-de-la-unc-brillan-en-las-primeras-olimpiadas-venezolanas-de-nanotecnologia-2026",
        original_url: "https://unc.edu.ve/index.php/jovenes-talentos-de-la-unc-brillan-en-las-primeras-olimpiadas-venezolanas-de-nanotecnologia-2026/",
        title: "Jóvenes talentos de la UNC brillan en las Primeras Olimpiadas Venezolanas de Nanotecnología 2026",
        date: "27 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_22.jpg",
        excerpt: "En un hito para el desarrollo científico del país, las Primeras Olimpiadas Venezolanas de Nanotecnología (OVN) 2026 concluyeron con un balance sumamente exitoso, dejando en alto el nombre de la Universidad Nacional de las Ciencias Dr....",
        sections: &[
            ("", &[
                "En un hito para el desarrollo científico del país, las Primeras Olimpiadas Venezolanas de Nanotecnología (OVN) 2026 concluyeron con un balance sumamente exitoso, dejando en alto el nombre de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán. Un destacado grupo de seis estudiantes de esta casa de estudios se alzó con galardones clave, demostrando que la juventud venezolana se consolida como el valor más alto en la construcción de un camino llamado ciencia para la vida.La convocatoria, impulsada por el Ministerio del Poder Popular para Ciencia y Tecnología (Mincyt), superó las expectativas al congregar a más de 3.900 inscritos a nivel nacional. El exigente certamen evaluó propuestas vanguardistas enfocadas en resolver problemas reales mediante el rigor técnico y un fuerte compromiso social, áreas prioritarias para la soberanía tecnológica de la nación.Cuadro de Honor: Talento que revoluciona el futuroEl esfuerzo, la disciplina y la creatividad de los estudiantes de la Universidad Dr. Humberto Fernández-Morán obtuvieron el reconocimiento del jurado calificador en distintas categorías:",
                "Categoría B• Victoria Ladrón De Guevara: 2.° lugar en proyecto y un brillante 2.° lugar absoluto a nivel general.• José Chávez: 3.° lugar en proyecto y 3.° lugar absoluto del certamen.• Gleyber Cárdenas: 2.° lugar en proyecto.• Hans Delgado: 2.° lugar en proyecto.• Miguel Mota: 3.° lugar en proyecto.• Luis Tamayo: 3.° lugar en proyecto.",
                "Categoría A• Yaretzi Arias: Mención HonoríficaEste logro ratifica el papel fundamental de la Universidad Nacional de las Ciencias y las o ropuestas presentadas por los galardonados no solo destacaron por su complejidad técnica en el campo de la nanotecnología, sino también por su viabilidad para transformar sectores estratégicos y mejorar la calidad de vida de las comunidades.Con iniciativas como las OVN 2026, Venezuela reafirma su compromiso con la masificación del conocimiento científico, abriendo espacios para que las nuevas generaciones lideren la transición hacia un modelo de desarrollo basado en la innovación endógena y la independencia tecnológica.",
            ]),
        ]
    },
    NewsArticle {
        slug: "estudiantes-de-la-unc-fortalecen-su-formacion-internacional-en-la-jornada-dictado-atomico-2026",
        original_url: "https://unc.edu.ve/index.php/estudiantes-de-la-unc-fortalecen-su-formacion-internacional-en-la-jornada-dictado-atomico-2026/",
        title: "Estudiantes de la UNC fortalecen su formación internacional en la jornada «Dictado Atómico – 2026»",
        date: "25 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_23.jpg",
        excerpt: "Estudiantes de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán participaron en la jornada educativa internacional denominada “Dictado Atómico – 2026”, celebrada este jueves 24 de junio en la Casa Rusa de Caracas,...",
        sections: &[
            ("", &[
                "Estudiantes de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán participaron en la jornada educativa internacional denominada “Dictado Atómico – 2026”, celebrada este jueves 24 de junio en la Casa Rusa de Caracas, con el valioso apoyo de la Embajada de Rusia en Venezuela y de la Corporación Estatal “Rosatom”.",
                "En la actividad participaron cursantes de las carreras de Física, Física Nuclear, Biología y Química Computacional, Ciencia de Datos, Biotecnología, Matemáticas, Ciberseguridad, Biomateriales, Inteligencia Artificial y Electromedicina, acompañados por autoridades del Consejo Directivo de esta casa de estudios, en representación de la rectora Gabriela Jiménez Ramírez.",
                "La jornada constituyó un espacio propicio para poner a prueba los conocimientos de los jóvenes, profundizar en la historia de la industria nuclear rusa y conocer de cerca los avances científicos y tecnológicos contemporáneos.",
                "Como un punto de encuentro para el intercambio profesional y académico, el “Dictado Atómico” permite a los jóvenes investigadores y estudiantes de distintas naciones compartir saberes, ampliar su perspectiva sobre la ciencia actual y descubrir nuevas áreas de cooperación internacional.",
                "Ante esta iniciativa, la Universidad Nacional de las Ciencias expresa su profundo agradecimiento por el firme respaldo de la Embajada de Rusia al talento científico venezolano, considerándola un puente invaluable para el crecimiento académico y profesional de su comunidad estudiantil.",
                "De esta forma, las autoridades de la UNC continúan consolidando una sólida alianza estratégica en materia de educación y altas tecnologías, y felicita a la Federación de Rusia por permitir a los futuros profesionales nutrice de experiencias internacionales de alto nivel, reafirmando el compromiso institucional de formar a la generación que liderará el desarrollo científico del porvenir.",
                "Por su parte, Embajador de Rusia en Venezuela, Sergey Mélik-Bagdasárov resaltó que los participantes a este taller sean estudiantes vinculados a Física y Química. “La ciencia no conoce, y el intercambio de conocimientos y experiencias entre jóvenes especialistas de Rusia y Venezuela constituye una base sólida para el desarrollo de nuestros vínculos en los ámbitos de la educación, la ciencia y las altas tecnologías”.",
                "La celebración del “Dictado Atómico” en Caracas ratifica así una nueva oportunidad para acercar a la juventud venezolana a los hitos de la ciencia global, fortaleciendo los lazos humanitarios, científicos y educativos entre ambas naciones.",
            ]),
        ]
    },
    NewsArticle {
        slug: "unesco-abre-convocatoria-sobre-etica-de-la-ia",
        original_url: "https://unc.edu.ve/index.php/unesco-abre-convocatoria-sobre-etica-de-la-ia/",
        title: "UNESCO abre convocatoria sobre ética de la IA",
        date: "25 de septiembre de 2026",
        author: "Prensa UNC – Yanett Alvarez",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_24.webp",
        excerpt: "La UNESCO y el AI and Society Institute de la École normale supérieure (ENS-PSL), con sede en París, abrieron una convocatoria internacional de artículos titulada “Advancing research on the Ethics of Artificial Intelligence”, dirigida a...",
        sections: &[
            ("", &[
                "La UNESCO y el AI and Society Institute de la École normale supérieure (ENS-PSL), con sede en París, abrieron una convocatoria internacional de artículos titulada “Advancing research on the Ethics of Artificial Intelligence”, dirigida a investigadores que hayan obtenido el doctorado en los últimos cinco años y cuyo trabajo esté relacionado con aspectos éticos de la inteligencia artificial (IA).",
                "La iniciativa busca promover investigaciones emergentes, favorecer el intercambio interdisciplinario y fortalecer los vínculos entre la producción académica, las políticas públicas y las prácticas relacionadas con la IA.",
                "La misma se encuentra enmarcada en la Recomendación sobre la Ética de la Inteligencia Artificial, adoptada por los Estados Miembros de la UNESCO en 2021. El instrumento constituye el primer marco normativo mundial de este tipo y plantea que el desarrollo y utilización de la IA deben respetar los derechos humanos y la dignidad de las personas.",
            ]),
            ("Una convocatoria abierta a distintas disciplinas", &[
                "La convocatoria está abierta a investigadores de distintas disciplinas y regiones, con especial interés en propuestas interdisciplinarias e intersectoriales. Se aceptan contribuciones teóricas, empíricas, comparativas, interdisciplinarias y orientadas a la práctica. Entre los temas contemplados se encuentran:",
                "Gobernanza y colaboración adaptativa y con múltiples partes interesadas",
                "Equidad, rendición de cuentas y transparencia",
                "Derechos humanos e inteligencia artificial",
                "Sensibilización y alfabetización en ética de la IA",
                "Juventud / Salud mental / Salud y ética de la IA",
                "Inclusión social",
                "Sistemas culturales y de conocimiento",
                "Medio ambiente, sostenibilidad y acción climática",
                "Igualdad de género",
                "Grupos vulnerables o subrepresentados",
                "Dimensiones éticas de la IA en las políticas públicas, la investigación y la innovación",
                "La ética de la IA y la implementación de los Objetivos de Desarrollo Sostenible",
                "Ética de la IA y las ciencias STEM y básicas",
                "Ética de la IA y las tecnologías de vanguardia",
            ]),
            ("¿Quiénes pueden participar?", &[
                "La UNESCO establece como requisito que los investigadores hayan completado un doctorado durante los últimos cinco años y que su trabajo aborde cuestiones relacionadas con la ética de la IA. También permite presentar propuestas en coautoría, siempre que ambos autores cumplan las condiciones de elegibilidad.",
                "La fecha límite para la entrega de los trabajos es el 30 de octubre. Los artículos seleccionados serán presentados en una jornada académica presencial prevista para el 10 de diciembre de 2026 en París, donde investigadores emergentes, especialistas consolidados y representantes de instituciones académicas participarán en debates temáticos y recibirán comentarios de expertos.",
                "El programa contempla presentaciones de las investigaciones seleccionadas, conversaciones con expertos, debates moderados y laboratorios interactivos de ética de la IA, orientados a examinar dilemas concretos de investigación, cuestiones metodológicas y desafíos relacionados con las políticas públicas.",
                "Los trabajos seleccionados serán posteriormente reunidos en las actas de la actividad por UNESCO y el AI and Society Institute. Además, algunos autores podrán ser invitados a adaptar sus investigaciones para su publicación en la Tribune del Conseil de l’intelligence Artificielle Et Du Numérique (CIANum), ampliando su alcance hacia públicos académicos y responsables de políticas.",
                "La convocatoria representa una oportunidad para que docentes e investigadores con doctorado reciente sumen sus investigaciones al debate internacional, abordando aspectos cruciales sobre derechos, responsabilidad y las decisiones que determinarán la integración de innovaciones como la IA en la sociedad.",
                "Información oficial de la convocatoria: https://www.unesco.org/en/articles/call-papers-advancing-research-ethics-artificial-intelligence?utm_source=chatgpt.com",
            ]),
        ]
    },
    NewsArticle {
        slug: "unc-y-funvisis-realizan-jornada-de-formacion-y-prevencion-sismica",
        original_url: "https://unc.edu.ve/index.php/unc-y-funvisis-realizan-jornada-de-formacion-y-prevencion-sismica/",
        title: "UNC y Funvisis realizan jornada de formación y prevención sísmica",
        date: "24 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_25.jpg",
        excerpt: "Los terremotos no solo ponen a prueba la resistencia de las edificaciones y la capacidad de respuesta de los organismos de emergencia. También evidencian cuánto conoce una sociedad sobre los riesgos a los que está expuesta y qué tan...",
        sections: &[
            ("", &[
                "Los terremotos no solo ponen a prueba la resistencia de las edificaciones y la capacidad de respuesta de los organismos de emergencia. También evidencian cuánto conoce una sociedad sobre los riesgos a los que está expuesta y qué tan preparada está para actuar ante ellos.",
                "Con esa perspectiva, este miércoles 23 de septiembre de 2026, se llevó a cabo la Jornada Cultura y Prevención Sísmica, una actividad orientada a fortalecer los conocimientos sobre fenómenos sísmicos y promover comportamientos adecuados antes, durante y después de una emergencia, realizada simultáneamente en el Anfiteatro de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) y en las Aulas de la Biblioteca del Instituto Venezolano de investigaciones Científicas (IVIC) a estudiantes, personal docente y administrativo.",
                "La jornada estuvo a cargo de Daviana Méndez, investigadora e instructora de la Fundación Venezolana de Investigaciones Sismológicas (FUNVISIS) y Antonio Aguilar, profesor e instructor de la referida fundación, quienes abordaron aspectos relacionados con la misión y los pilares de esta institución, los objetivos de la alfabetización sismológica y tsunamigénica, la naturaleza de los sismos, la tectónica de placas y las ondas sísmicas, entre otros tópicos.",
                "El encuentro adquirió particular relevancia después del doble sismo ocurrido en Venezuela, una experiencia que puso de manifiesto la necesidad de contar con información científica e interpretar correctamente estos fenómenos naturales.",
            ]),
            ("Prevención y conocimiento para actuar", &[
                "Explicó Méndez que uno de los propósitos fundamentales de la actividad es “concientizar a la población para estar preparada ante sismos o tsunamis, porque también acá en Venezuela han ocurrido tsunamis”.",
                "La especialista hizo un llamado de atención sobre la responsabilidad que existe en materia de construcción y prevención. Señaló que “hay que tener conciencia sísmica y ética a la hora de construir edificaciones que sean sismos resistentes y ante todo, regirse por las normativas vigentes en la materia”.",
                "La prevención, en este sentido, no comienza cuando ocurre un terremoto, implica conocer las características del entorno, identificar riesgos, establecer mecanismos de actuación y familiarizarse previamente con las medidas que deben adoptarse en una situación de emergencia.",
                "La instructora resumió esta necesidad al señalar que la cultura sísmica se construye “accediendo a la información correcta, a las fuentes del órgano rector que, en este caso, es FUNVISIS”, con la finalidad de conocer qué medidas deben ponerse en práctica y cómo actuar antes, durante y después de un sismo.",
                "La experiencia de los eventos sísmicos ocurridos el 24 de junio pasado, también dejó como consecuencia una amplia circulación de información y contenidos que podían generar confusión. Para los estudiantes que participaron en la jornada, disponer de información científica permitió revisar algunas de esas creencias y distinguir entre recomendaciones sustentadas y prácticas que carecen de respaldo.",
                "Samanta Ríos, estudiante de Ingeniería en Robótica y Automatización, destacó que lo más importante de la jornada “fue desmentir los mitos, sobre todo la falsa información que circuló después del terremoto. Además, la descripción de cuáles son los protocolos correctos a seguir durante un evento sismológico”. Expresó que se “siente un poco más preparada ante un próximo evento”.",
                "Para Jocy Suárez, estudiante de la Licenciatura en Ciencia de Datos, la actividad fue “excelente y llama a la reflexión, porque los venezolanos no tenemos cultura sísmica a pesar del reciente evento sísmico”. La estudiante considera necesario que esta información empiece a difundirse en los colegios, a las nuevas generaciones desde temprana edad.",
                "La necesidad de prepararse también fue destacada por José Pacheco, estudiante de Ingeniería en Robótica y Automatización. A su juicio, la actividad permitió “tener información acerca de lo que se debe hacer, así como identificar o tener más claro algunos aspectos a considerar como medidas preventivas”. El estudiante también señaló la importancia de conocer las características del espacio en el que se desarrolla la vida cotidiana para actuar de la mejor manera ante una eventualidad.",
            ]),
            ("Ciencia, prevención y responsabilidad social", &[
                "FUNVISIS cumple en Venezuela la función de ejecutar y promover investigaciones y estudios sismológicos, además de operar y mantener las estaciones del Servicio Sismológico Nacional, desde donde se registra y monitorea la actividad sísmica del país.",
                "En la jornada Cultura y Prevención Sísmica, FUNVISIS puso el acento en la necesidad de reconocer que una sociedad preparada no es aquella que espera una nueva emergencia para comenzar a informarse, sino aquella que incorpora progresivamente el conocimiento científico a sus prácticas cotidianas. En este sentido, la formación universitaria puede contribuir a que conceptos científicos y recomendaciones de prevención lleguen a familias, comunidades y espacios de trabajo.",
                "La presencia de esta institución en la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán permitió acercar el conocimiento especializado a la población estudiantil como parte complementaria de su formación científica y ciudadana que les permitirá, a su vez, convertirse en agentes multiplicadores de información en sus comunidades.",
                "Estás actividades tendrán continuidad el próximo miércoles 30 de septiembre, en ambos espacios. Y el 7 de octubre se espera el simulacro general.",
            ]),
        ]
    },
    NewsArticle {
        slug: "onu-delcy-rodriguez-por-una-ia-inclusiva-y-responsable",
        original_url: "https://unc.edu.ve/index.php/onu-delcy-rodriguez-por-una-ia-inclusiva-y-responsable/",
        title: "ONU: Delcy Rodríguez: Por una IA inclusiva y responsable",
        date: "24 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_26.jpg",
        excerpt: "La presidenta de la República Bolivariana de Venezuela, Delcy Rodríguez, participó en la 81° Asamblea Ordinaria de la Organización de las Naciones Unidas (ONU) en Nueva York, donde centró su intervención en el impacto global de las...",
        sections: &[
            ("", &[
                "La presidenta de la República Bolivariana de Venezuela, Delcy Rodríguez, participó en la 81° Asamblea Ordinaria de la Organización de las Naciones Unidas (ONU) en Nueva York, donde centró su intervención en el impacto global de las nuevas tecnologías que, a su juicio, deben ser inclusiva y responsable.",
                "Durante su discurso, la Mandataria Nacional destacó que la Inteligencia Artificial (IA) representa una verdadera revolución tecnológica y un hito de transformación para la humanidad. A su juicio, si esta herramienta es empleada correctamente, “significará un desarrollo sin precedentes en la salud, ambiente, los procesos productivos y la vida en general”.",
                "En su alocución, Rodríguez hizo un llamado contundente a la cooperación internacional frente a los retos que impone esta era digital. Resaltó que los gobiernos, el sector tecnológico y la comunidad científica “tienen la inmensa responsabilidad de no desviar el propósito constructivo y vital de la IA para beneficio de los ciudadanos, así como de no profundizar la brecha y la desigualdad digital en el mundo”, explicó.",
                "Finalmente, la Jefa de Estado mostró el compromiso de Venezuela con un modelo de desarrollo tecnológico inclusivo, que priorice la soberanía y el bienestar social por encima de intereses particulares.",
            ]),
        ]
    },
    NewsArticle {
        slug: "ciencia-tecnologia-e-innovacion-para-el-sur",
        original_url: "https://unc.edu.ve/index.php/ciencia-tecnologia-e-innovacion-para-el-sur/",
        title: "Ciencia, tecnología e Innovación para el Sur",
        date: "16 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_27.jpg",
        excerpt: "Las preguntas que orientan una investigación, las tecnologías que se diseñan y las innovaciones que llegan a la sociedad están relacionadas con necesidades concretas, capacidades disponibles y desafíos que cada región o país debe...",
        sections: &[
            ("", &[
                "Las preguntas que orientan una investigación, las tecnologías que se diseñan y las innovaciones que llegan a la sociedad están relacionadas con necesidades concretas, capacidades disponibles y desafíos que cada región o país debe afrontar. En el Sur Global la brecha tecnológica se mantiene amplia debido a la insuficiencia de inversión en investigación y desarrollo (I+D), las deficiencias en los marcos de gobernanza y la limitada difusión de herramientas científicas.",
                "Ese escenario explica la pertinencia del Día Internacional de la Ciencia, la Tecnología y la Innovación para el Sur, establecida por las Naciones Unidas (ONU) y que se celebra cada 16 de septiembre, con la finalidad de reconocer el papel de la ciencia, la tecnología y la innovación en el desarrollo sostenible y promover condiciones que permitan a los países del Sur fortalecer sus propios sistemas científicos y tecnológicos.",
                "Al respecto, la ONU reconoce que las capacidades para producir y aprovechar conocimiento científico y tecnológico continúan distribuyéndose de manera desigual entre países desarrollados y países en desarrollo. En tal sentido plantea la necesidad de construir un entorno abierto, justo, inclusivo y no discriminatorio, donde la ciencia sea motor del bienestar social, la producción y la capacidad de los países para decidir su propio futuro.",
                "En este esfuerzo por fortalecer la cooperación mediante iniciativas de cooperación Sur-Sur, la dimensión universitaria juega un papel fundamental al reunir tres componentes indispensables para el desarrollo científico: formación, investigación y vinculación con la sociedad.",
                "La actividad universitaria se convertirse así en uno de los escenarios ideales para comprender problemas nacionales, formar capacidades y generar soluciones adaptadas a las condiciones sociales, ambientales y productivas de cada territorio.",
                "Frente a este desafío del Sur Global, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) materializa esa visión mediante un modelo formativo innovador, humanista y solidario. Su oferta de carreras estratégicas busca romper la brecha de dependencia tecnológica, a través del impulso de capacidades locales, reduciendo la dependencia tecnológica y formando profesionales aptos para generar soluciones adaptadas a las necesidades del país.",
            ]),
            ("Del conocimiento a la cooperación", &[
                "En este escenario del Sur global actual, el Instituto Internacional de la UNESCO para la Educación Superior en América Latina y el Caribe (IESALC) ha destacado que se requiere la creación de centros regionales de cooperación y plataformas académicas interconectadas.",
                "Las instituciones de educación superior están llamadas a vincular la investigación científica con las demandas concretas de sus comunidades y sectores productivos. En el ámbito universitario, cooperar significa compartir conocimientos, experiencias, metodologías y capacidades humanas. Pueden erigirse como catalizadores primarios para superar el sesgo histórico en la distribución del conocimiento mundial.",
                "Para universidades científicas emergentes como la UNC, esta perspectiva abre un interesante campo de posibilidades. La cooperación puede extenderse hacia proyectos de investigación, innovación tecnológica, formación especializada y resolución conjunta de problemas relacionados con las realidades latinoamericanas y de otros países del Sur.",
                "El fortalecimiento de la ciencia en el Sur Global requiere consolidar ecosistemas donde la universidad, la industria local y la comunidad interactúen dinámicamente. Los desafíos que se enfrentan son diversos en áreas como cambio climático, seguridad alimentaria, enfermedades, transición energética, gestión del agua, transformación digital, desigualdad y protección de la biodiversidad.",
                "Más allá de la producción científica, el debate exige responder a interrogantes de fondo: qué conocimiento se necesita, quién lo genera, con qué fin social y bajo qué principios se aplica. Resolver estas preguntas resulta determinante hoy que la inteligencia artificial, la biotecnología y la digitalización abren la oportunidad de dar un salto tecnológico y omitir etapas tradicionales de industrialización.",
                "En el contexto de este necesario diálogo regional del Sur, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán encuentra uno de sus principales espacios de actuación al contribuir a fortalecer la capacidad de pensar desde nuestras propias realidades, investigar nuestros propios desafíos y construir, mediante la cooperación y el conocimiento, las soluciones que necesitamos para un desarrollo sostenible y para alcanzar la soberanía científico tecnológica.",
            ]),
        ]
    },
    NewsArticle {
        slug: "polo-cientifico-tecnologico-de-venezuela-cinco-anos-de-articulacion-entre-ciencia-y-formacion",
        original_url: "https://unc.edu.ve/index.php/polo-cientifico-tecnologico-de-venezuela-cinco-anos-de-articulacion-entre-ciencia-y-formacion/",
        title: "Polo Científico Tecnológico de Venezuela: Cinco años de articulación entre ciencia y formación",
        date: "15 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_28.png",
        excerpt: "El 15 de septiembre de 2021 fue creado el Polo Científico Tecnológico de Venezuela (Polo CyT) instancia que destaca por sus logros y la amplituden la concepción de la política científica venezolana en un esfuerzo por conectar...",
        sections: &[
            ("", &[
                "El 15 de septiembre de 2021 fue creado el Polo Científico Tecnológico de Venezuela (Polo CyT) instancia que destaca por sus logros y la amplituden la concepción de la política científica venezolana en un esfuerzo por conectar investigación, formación, innovación y producción alrededor de problemas que requieren respuestas desde el conocimiento.",
                "El Polo (CyT) permite establecer conexiones entre quienes generan conocimiento, forman nuevos profesionales y pueden transformar ese conocimiento en tecnologías, procesos o productos.",
                "En este contexto participan universidades, centros de investigación, industrias, instituciones públicas, innovadores y otros actores que pueden aportar conocimientos, infraestructura, experiencia o capacidades productivas a proyectos que requieren la participación de diversas disciplinas.",
            ]),
            ("Ciencia aplicada a necesidades concretas", &[
                "Durante estos cinco años, el propósitodel Polo (CyT) ha estado orientado a consolidar la relación entre ciencia y necesidades sociales, así como promover la autonomía tecnológica. Cabe mencionar, entre otros objetivos, la generación de soluciones para sectores como salud, educación, telecomunicaciones, agricultura, alimentación, industria y transporte.",
                "Esta orientación responde a una característica fundamental de los sistemas contemporáneos de ciencia, tecnología e innovación al involucrar múltiples etapas e instituciones en procesos de investigación, experimentación, prototipado, validación, transferencia tecnológica y producción.",
                "En Venezuela, esta concepción se ha expresado en diversos proyectos. Un ejemplo, es el desarrollo de módems nacionales impulsado desde el Centro Nacional de Desarrollo e Investigación en Telecomunicaciones (Cendit), cuya experiencia incluyó prototipado, perfeccionamiento del diseño y transferencia tecnológica.",
                "Otro ejemplo reciente corresponde al fortalecimiento de las capacidades de microscopía electrónica. En septiembre de 2025 fueron presentados nuevos equipos destinados a investigaciones relacionadas con materiales y minerales, geología, geoquímica y criminalística. En esa ocasión, la rectora de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) Gabriela Jiménez Ramírez expresó que el Polo representa “una alternativa prodigiosa para crear nuevas capacidades, análisis, determinación de productos. Además, la renovación de los ecosistemas de investigación, desarrollo y de innovación tecnológica del país, que vincula los espacios universitarios con los centros de investigación, con la industria nacional y las comunidades”.",
                "Esa misma relación entre infraestructura, investigación y formación cobra relevancia en la actividad académica y de investigación de la Universidad Nacional de las Ciencias. Al articular capacidades científicas, tecnológicas y productivas, la UNC incorpora al sistema del Polo Científico Tecnológico Venezolano un componente fundamental a través de la formación de nuevas generaciones de científicos, ingenieros y profesionales especializados.",
                "Este vínculo permite reducir la distancia entre el aprendizaje conceptual y la práctica de la investigación y ofrece un escenario para que los estudiantes puedan aproximarse progresivamente a la actividad científica. Por otra parte, constituye una de las expresiones más concretas de que las capacidades científicas de un país alcanzan mayor impacto cuando se diseñan espacios permanentes para encontrarse, complementarse y proyectarse hacia el futuro.",
            ]),
        ]
    },
    NewsArticle {
        slug: "de-la-estructura-proteica-a-la-regulacion-genomica",
        original_url: "https://unc.edu.ve/index.php/de-la-estructura-proteica-a-la-regulacion-genomica/",
        title: "De la estructura proteica a la regulación genómica",
        date: "12 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_29.jpg",
        excerpt: "El avance de la biología computacional ha entrado en una era sin precedentes. La evolución natural de este campo se consolida con el lanzamiento de AlphaGenome Atlas por parte de Google DeepMind, un hito que expande radicalmente el...",
        sections: &[
            ("", &[
                "El avance de la biología computacional ha entrado en una era sin precedentes. La evolución natural de este campo se consolida con el lanzamiento de AlphaGenome Atlas por parte de Google DeepMind, un hito que expande radicalmente el horizonte inaugurado por su predecesor, AlphaFold. Mientras que AlphaFold revolucionó la biología molecular al predecir con precisión matemática la estructura tridimensional de más de 200 millones de proteínas a partir de secuencias de aminoácidos —un logro galardonado con el Premio Nobel de Química—, AlphaGenome Atlas traslada esta capacidad predictiva directamente al núcleo del código genético: el ADN.",
            ]),
            ("La Revolución Cuantitativa y Molecular", &[
                "A diferencia de los enfoques tradicionales que se limitaban al estudio de los exones (la región codificante que representa apenas el 2% del genoma), este nuevo modelo procesa e interpreta el 98% del genoma no codificante, una vasta región histórica y erróneamente denominada «ADN basura». El atlas opera sobre un colosal conjunto de datos que alcanza 1 petabyte (1,000 terabytes) de información precalculada.",
                "La potencia cuantitativa del modelo radica en su capacidad para mapear e indexar el impacto regulatorio y molecular de las 9,000 millones de variantes posibles de una sola letra (SNVs) en el genoma humano. Al funcionar como un buscador interactivo no-code (sin código), permite a la comunidad científica internacional consultar de manera inmediata qué procesos biológicos cruciales —tales como la expresión génica, los factores de transcripción o el empalme alternativo de ARN (splicing)— se alteran ante cualquier mutación específica. En términos metodológicos, esto purifica los estudios de asociación del genoma completo (GWAS), reduciendo drásticamente el «ruido estadístico» y acelerando la transición hacia una medicina de precisión personalizada.",
            ]),
            ("Desafíos éticos y la brecha biotecnológica", &[
                "A pesar del acceso público a esta base de datos, la democratización del conocimiento científico se enfrenta a barreras estructurales complejas. Se identifican cinco dilemas críticos que la gobernanza global debe resolver:",
                "La brecha bioinformática: Aunque el petabyte de datos está disponible de forma abierta, solo los laboratorios del norte global e instituciones con infraestructura de supercómputo y especialistas de alto nivel pueden explotar el atlas en su totalidad. Esto amenaza con ampliar la brecha de investigación entre países desarrollados y economías en desarrollo.",
                "El fin del anonimato genético: La precisión milimétrica del modelo incrementa el riesgo de «re-identificación». Un cruce de datos entre una muestra de ADN supuestamente anónima y los perfiles predictivos del atlas permitiría deducir predisposiciones médicas exactas e historiales clínicos de un individuo sin su consentimiento.",
                "Discriminación por impacto de variantes (AVI): El indicador AlphaGenome Variant Impact (AVI) clasifica el riesgo biológico de cada mutación. En naciones con marcos regulatorios laxos, existe el riesgo inminente de que aseguradoras, entidades financieras o empleadores utilicen estos modelos predictivos para denegar servicios o contratos a personas con perfiles AVI desfavorables.",
                "Eurocentrismo genómico: Históricamente, la gran mayoría de los genomas secuenciados en bases de datos globales (como GenBank o The Cancer Genome Atlas) pertenecen a poblaciones de ascendencia europea. Si los datos de entrenamiento arrastran este sesgo, las predicciones del modelo serán significativamente menos precisas para poblaciones latinas, africanas o asiáticas, perpetuando la desigualdad médica.",
                "Dualidad del conocimiento (Optimización vs. Armamento): comprender los mecanismos exactos que encienden o apagan los genes abre la puerta a la medicina regenerativa, pero también presenta un riesgo de seguridad biológica. En un escenario de guerra asimétrica, esta información podría ser malversada para diseñar patógenos modificados o armas biológicas dirigidas a perfiles étnicos o genéticos específicos.",
                "Fuente: artículo de la revista Nature.",
                "Por Gabriela Jiménez Ramírez",
            ]),
        ]
    },
    NewsArticle {
        slug: "la-universidad-nacional-de-las-ciencias-y-el-desarrollo-de-la-investigacion",
        original_url: "https://unc.edu.ve/index.php/la-universidad-nacional-de-las-ciencias-y-el-desarrollo-de-la-investigacion/",
        title: "La Universidad Nacional de las Ciencias y el desarrollo de la investigación",
        date: "11 de septiembre de 2026",
        author: "Prensa UNC – María Guevara",
        category: "Destacados · Noticias",
        thumbnail: "/assets/real/news_30.png",
        excerpt: "Detrás de cada interrogante de una investigación puede existir una necesidad social, una dificultad industrial, una preocupación ambiental o una oportunidad que todavía no ha sido explorada. A partir de allí puede surgir una hipótesis,...",
        sections: &[
            ("", &[
                "Detrás de cada interrogante de una investigación puede existir una necesidad social, una dificultad industrial, una preocupación ambiental o una oportunidad que todavía no ha sido explorada. A partir de allí puede surgir una hipótesis, un proyecto, una tesis y, cuando ese interés se desarrolla de manera sistemática, una línea de investigación.",
                "En el ámbito universitario, una línea de investigación funciona como un eje que organiza y da continuidad a la actividad científica. Permite relacionar diferentes proyectos alrededor de un campo o problemática, integrar el trabajo de estudiantes, profesores e investigadores y establecer una perspectiva de desarrollo que puede extenderse en el tiempo.",
                "Bajo esta perspectiva, la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán (UNC) orienta su actividad académica y científica a través de un amplio mapa de posibilidades de investigación, relacionadas con las carreras que ofrece y, al mismo tiempo, abiertas a nuevos problemas y áreas de conocimiento.",
                "Así lo explica el Dr. David Coll, miembro de la Dirección de Investigación de la UNC y subdirector del Instituto Venezolano de Investigaciones Científicas (IVIC), quien manifiesta que, existe una orientación general que responde a la formación científica que desarrolla la institución.",
                "“Cada carrera tiene entre cuatro y seis líneas de investigación generales”, señala Coll, quien aclara que esta estructura no es limitante ni impide que se puedan desarrollar investigaciones en áreas no contempladas. Como explica, “si la línea no existe, pero se ve que hay algo que se puede desarrollar en cierto ámbito, se crea”.",
                "La posibilidad de incorporar nuevas temáticas responde a una característica propia de la investigación científica. Una línea no constituye un límite rígido, sino un marco que permite organizar problemas, proyectos y esfuerzos alrededor de un área determinada, al tiempo que conserva la capacidad de incorporar nuevas preguntas.",
            ]),
            ("Conocimiento, creatividad y futuro", &[
                "Para Coll, una línea de investigación también surge de la necesidad de encontrar respuestas a situaciones concretas. Lo ejemplifica con una situación relacionada con la energía y el ambiente: “Si tenemos un combustible que tiene muchos contaminantes, se puede diseñar una línea de investigación para generar un catalizador que optimice el proceso”.",
                "La investigación no se limita a atender problemas ya existentes, también puede anticipar necesidades. Sostiene que “no es solo buscar soluciones a problemas actuales, sino hacer una proyección a futuro, como país, como sociedad, como industria nacional y en función de eso formar gente para atender esa situación futura, esa proyección”.",
                "En ese sentido, las líneas de investigación pueden convertirse en un vínculo entre la formación académica y una agenda científica de largo plazo. El estudiante no se prepara únicamente para responder a problemas conocidos, sino también para participar en la construcción de respuestas frente a escenarios que todavía están en desarrollo.",
                "La existencia de líneas de investigación también permite evitar que los trabajos científicos permanezcan como esfuerzos aislados. Una línea puede articular proyectos, problemas, metodologías y actividades de investigación, favoreciendo la interacción entre docentes, estudiantes y profesionales alrededor de un área determinada.",
                "En la UNC la investigación forma parte del proceso de aprendizaje. Una tesis, una pasantía o un proyecto desarrollado durante la carrera pueden constituir el punto de partida para que un estudiante se incorpore progresivamente a un campo de investigación.",
                "Esta posibilidad se relaciona con la amplitud de las disciplinas que forman parte de su oferta académica. Oceanología, Robótica y Automatización, Biología y Química Computacional, Biotecnología, Física, Ciencia de Datos, Nanotecnología y otras áreas abren caminos diferentes para la investigación, con posibilidades de interacción entre disciplinas.",
                "Más que establecer compartimientos independientes, esta diversidad puede favorecer el encuentro entre conocimientos. Un problema ambiental, por ejemplo, puede requerir herramientas de biología, química, computación, análisis de datos y automatización; mientras que una investigación en salud puede incorporar conocimientos provenientes de la biotecnología, los biomateriales, la electromedicina o la inteligencia artificial.",
            ]),
            ("Dimensión ética de la investigación", &[
                "El doctor Coll plantea como prioritario utilizar “la tecnología o la ciencia para simplificar nuestra vida, cómo hacemos todo más óptimo, cómo hacemos para abaratar costos”. No obstante, cualquier resultado científico o tecnológico debe tener un componente adicional, “un gran filtro ético”.",
                "Advierte que “de repente conseguimos algo muy rentable, pero que es sumamente contaminante o que puede generar algunos pasivos ambientales y daños a ciertas comunidades. Entonces, todo eso hay que manejarlo y resolverlo”.",
                "El mismo razonamiento puede aplicarse al desarrollo de medicamentos. Una investigación puede conducir a un producto capaz de resolver un problema de salud, pero si para obtenerlo se produce un daño considerable a una población animal o vegetal, el resultado obliga a considerar otras alternativas.",
                "Por lo anterior, no basta con determinar si algo puede hacerse; también es necesario analizar las consecuencias. Para Coll, “todo lo que hacemos genera una huella, la idea es que esta huella no deje un impacto negativo”.",
                "Esta concepción conecta la investigación con la bioética como una consideración transversal de las decisiones relacionadas con la vida, la salud, el ambiente y el uso del conocimiento.",
            ]),
            ("Formación académica, creatividad e investigación", &[
                "Coll sostiene que “la creatividad es una herramienta indispensable para el investigador, pero desde el punto de vista científico, tiene que tener un fundamento y base formativa”. Esta relación coincide con la concepción abierta de las líneas de investigación al considerar que un punto de partida de un problema durante una investigación puede conducir hacia nuevas preguntas y, eventualmente, hacia nuevas líneas.",
                "Destaca también las posibilidades de articulación con instituciones del sistema científico venezolano, “los muchachos formados por la universidad van a nutrir a este sistema de ciencia, al denominado polo científico”, están destinados a ser los encargados del desarrollo científico del país”. Este compromiso resulta de mucha importancia para una institución que busca formar profesionales en áreas científicas y tecnológicas que requiere el país.",
                "La investigación universitaria se convierte así en un proceso que no termina en la formulación de una línea. Por el contrario, la línea permite ordenar esfuerzos, dar continuidad a las preguntas y crear las condiciones para que aparezcan nuevas investigaciones.",
                "En la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán, ante su compromiso de responder a las necesidades estratégicas del país, esta perspectiva plantea un escenario en el que las líneas de investigación pueden funcionar como un puente entre la formación de los estudiantes y las necesidades científicas y tecnológicas del país, de cara a los desafíos del presente y del futuro.",
            ]),
        ]
    },
];

pub const CAREER_PROGRAMS: &[CareerProgram] = &[
    CareerProgram {
        slug: "ingenieria-en-inteligencia-artificial",
        category: "Ingenierías",
        name: "Ingeniería en Inteligencia Artificial",
        degree_title: "Ingeniero/a en Inteligencia Artificial",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "181 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_ia.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/ING_INTELIGENCIA-ARTIFICIAL-07052025.pdf",
        tile_color_class: "tile-blue",
        study_fields: &[
            ("Aprendizaje automático", "Algoritmos de aprendizaje supervisado, no supervisado y por refuerzo."),
            ("Redes neuronales", "Redes neuronales artificiales, aprendizaje profundo (Deep Learning)."),
            ("Procesamiento del lenguaje natural", "Procesamiento de texto, traducción automática, modelos fundacionales."),
            ("Visión por computadora", "Procesamiento de imágenes, reconocimiento de patrones."),
            ("Robótica", "Inteligencia artificial aplicada a sistemas autónomos y robots.")
        ],
        career_fields: &[
            ("Tecnología", "Desarrollo de software, arquitectura de IA, startups o nuevas empresas de tecnología."),
            ("Industria y Energía", "Automatización de procesos industriales, análisis predictivo de datos."),
            ("Investigación", "Universidades, centros de investigación científica y laboratorios de cómputo avanzado."),
            ("Salud", "Diagnóstico médico asistido por computadora, descubrimiento y desarrollo de fármacos."),
            ("Finanzas", "Análisis cuantitativo de mercado, detección algorítmica de fraudes.")
        ]
    },
    CareerProgram {
        slug: "ingenieria-en-ciberseguridad",
        category: "Ingenierías",
        name: "Ingeniería en Ciberseguridad",
        degree_title: "Ingeniero/a en Ciberseguridad",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "187 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_ciber.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/ING_CIBERSEGURIDAD-07052025.pdf",
        tile_color_class: "tile-mint",
        study_fields: &[
            ("Seguridad de redes", "Protocolos de seguridad, firewalls, arquitecturas zero-trust y VPNs."),
            ("Seguridad de sistemas operativos", "Protección y endurecimiento de sistemas operativos y aplicaciones críticas."),
            ("Criptografía", "Algoritmos de cifrado moderno, gestión de claves y firmas digitales."),
            ("Gestión de riesgos", "Identificación, evaluación y mitigación de vulnerabilidades e infraestructuras."),
            ("Incidentes de seguridad", "Detección, respuesta forense y recuperación ante incidentes cibernéticos."),
            ("Derecho informático", "Legislación, soberanía de datos y normativas en ciberseguridad.")
        ],
        career_fields: &[
            ("Empresas de tecnología", "Desarrollo y auditoría de soluciones de seguridad informática."),
            ("Bancos y financieras", "Protección de datos sensibles e infraestructuras transaccionales."),
            ("Gobierno y Defensa", "Ciberseguridad gubernamental y resguardo de infraestructuras críticas."),
            ("Consultoras", "Asesoramiento estratégico, pentesting y cumplimiento en seguridad informática.")
        ]
    },
    CareerProgram {
        slug: "ingenieria-en-robotica-y-automatizacion",
        category: "Ingenierías",
        name: "Ingeniería en Robótica y Automatización",
        degree_title: "Ingeniero/a en Robótica y Automatización",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "180 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_robotica.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/ING_ROBOTICA-Y-AUTOMATIZACION-07052025.pdf",
        tile_color_class: "tile-coral",
        study_fields: &[
            ("Robótica", "Diseño cinemático, programación y control de robots manipuladores y móviles."),
            ("Automatización industrial", "Control de procesos continuos y discretos, instrumentación industrial."),
            ("Mecatrónica", "Integración de sistemas mecánicos, electrónicos y de control embebido."),
            ("Inteligencia artificial", "Aplicación de técnicas de IA en sistemas robóticos autónomos."),
            ("Visión artificial", "Procesamiento de imágenes en tiempo real para navegación e inspección robótica.")
        ],
        career_fields: &[
            ("Industria manufacturera", "Automatización de líneas y procesos productivos."),
            ("Automatización de edificios", "Control inteligente de sistemas de climatización, energía e iluminación."),
            ("Robótica de servicios", "Robots para asistencia en salud, agricultura y logística."),
            ("Investigación", "Centros de investigación y desarrollo en robótica y automatización.")
        ]
    },
    CareerProgram {
        slug: "ingenieria-en-electromedicina",
        category: "Ingenierías",
        name: "Ingeniería en Electromedicina",
        degree_title: "Ingeniero/a en Electromedicina",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "191 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_electro.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/ING_ELECTROMEDICINA-07052025.pdf",
        tile_color_class: "tile-gold",
        study_fields: &[
            ("Electrónica médica", "Diseño y desarrollo de circuitos y sistemas electrónicos para aplicaciones médicas."),
            ("Biomateriales", "Selección y aplicación de materiales biocompatibles en dispositivos médicos."),
            ("Imagenología médica", "Equipos de rayos X, resonancia magnética, ultrasonido y tomografía computarizada."),
            ("Fisiología médica", "Funcionamiento bioeléctrico del cuerpo humano para el diseño de instrumentación."),
            ("Mantenimiento biomédico", "Gestión hospitalaria, calibración y mantenimiento de equipos médicos.")
        ],
        career_fields: &[
            ("Hospitales y Clínicas", "Mantenimiento de equipos médicos de soporte de vida y gestión tecnológica."),
            ("Industria Biomédica", "Diseño, prototipado y fabricación de equipos e instrumentación médica."),
            ("Centros de investigación", "Desarrollo de nuevas tecnologías médicas y bioinstrumentación."),
            ("Servicios especializados", "Metrología, mantenimiento y calibración certificada de equipos médicos.")
        ]
    },
    CareerProgram {
        slug: "ingenieria-en-biomateriales",
        category: "Ingenierías",
        name: "Ingeniería en Biomateriales",
        degree_title: "Ingeniero/a en Biomateriales",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "184 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_biomat.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/ING_BIOMATERIALES-07052025.pdf",
        tile_color_class: "tile-coral",
        study_fields: &[
            ("Biomateriales", "Propiedades, clasificación y selección de materiales para aplicaciones médicas."),
            ("Ingeniería de tejidos", "Diseño de andamios biocompatibles para regeneración celular y de tejidos."),
            ("Liberación controlada de fármacos", "Diseño de sistemas poliméricos y matrices para liberación de medicamentos."),
            ("Interacción biomaterial-tejido", "Respuesta biológica, inmunológica y hemocompatibilidad."),
            ("Procesamiento de materiales", "Técnicas avanzadas de fabricación, síntesis y caracterización de biomateriales.")
        ],
        career_fields: &[
            ("Industria médica", "Diseño y fabricación de prótesis, implantes y dispositivos médicos."),
            ("Centros de investigación", "Desarrollo de nuevas tecnologías médicas y materiales regenerativos."),
            ("Universidades", "Docencia universitaria e investigación en ciencia de materiales."),
            ("Industrias de biotecnología", "Desarrollo de nuevos tratamientos, apósitos y terapias avanzadas.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-fisica-nuclear",
        category: "Licenciaturas",
        name: "Licenciatura en Física Nuclear",
        degree_title: "Licenciado/a en Física Nuclear",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "181 U.C",
        area: "Ciencias Naturales",
        icon_url: "/assets/real/career_fnuclear.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_FISICA-NUCLEAR-07052025.pdf",
        tile_color_class: "tile-blue",
        study_fields: &[
            ("Núcleo atómico", "Estructura nuclear, propiedades cuánticas y transformaciones."),
            ("Radioactividad", "Tipos de radiación ionizante, decaimiento radioactivo y sus aplicaciones pacíficas."),
            ("Reactores nucleares", "Física de neutrones, funcionamiento, diseño y seguridad radiológica."),
            ("Física de partículas", "Interacciones fundamentales de la materia y modelos teóricos.")
        ],
        career_fields: &[
            ("Centros de investigación", "Institutos de física avanzada y laboratorios nacionales."),
            ("Industria y Salud", "Medicina nuclear, radioterapia, sector energético, farmacéutico y metalúrgico."),
            ("Sector público", "Regulación nuclear, dosimetría y protección radiológica civil."),
            ("Academia", "Docencia e investigación científica en universidades e institutos.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-biologia-y-quimica-computacional",
        category: "Licenciaturas",
        name: "Licenciatura en Biología y Química Computacional",
        degree_title: "Licenciado/a en Biología y Química Computacional",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "185 U.C",
        area: "Ciencias Naturales",
        icon_url: "/assets/real/career_bioquim.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_BIOLOGIA-Y-QUIMICA-COMPUTACIONAL-07052025.pdf",
        tile_color_class: "tile-mint",
        study_fields: &[
            ("Bioinformática", "Análisis de secuencias genómicas, filogenia y modelado de sistemas biológicos."),
            ("Quimioinformática", "Diseño racional de fármacos, dinámica y modelaje molecular, química cuántica."),
            ("Aprendizaje automático", "Aplicación de técnicas de inteligencia artificial a problemas biológicos y químicos."),
            ("Visualización científica", "Representación gráfica tridimensional de macromoléculas y datos químicos.")
        ],
        career_fields: &[
            ("Industria farmacéutica", "Diseño asistido por computadora y cribado virtual de nuevos fármacos."),
            ("Biotecnología", "Desarrollo e ingeniería de proteínas y productos biotecnológicos."),
            ("Agricultura", "Mejora genómica de cultivos y agroquímicos sostenibles."),
            ("Medio ambiente", "Modelado computacional de sistemas ecológicos y contaminantes."),
            ("Centros de investigación", "Institutos de investigación en biología estructural, química y computación.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-biotecnologia",
        category: "Licenciaturas",
        name: "Licenciatura en Biotecnología",
        degree_title: "Licenciado/a en Biotecnología",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "182 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_biotec.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_BIOTECNOLOGIA-07052025.pdf",
        tile_color_class: "tile-mint",
        study_fields: &[
            ("Biología molecular y celular", "Genética molecular, bioquímica estructural y microbiología avanzada."),
            ("Ingeniería genética", "Manipulación y edición de material genético, ADN recombinante."),
            ("Biotecnología industrial", "Diseño de bioprocesos, biorreactores y producción de biomoléculas."),
            ("Biotecnología ambiental", "Biorremediación de suelos y aguas, biocombustibles y bioenergía."),
            ("Biotecnología médica", "Desarrollo de biofármacos, vacunas y terapias génicas.")
        ],
        career_fields: &[
            ("Industria farmacéutica", "Desarrollo y producción de nuevos fármacos y biológicos."),
            ("Agricultura y Soberanía Alimentaria", "Mejora vegetal de cultivos y desarrollo de biofertilizantes."),
            ("Medio ambiente", "Proyectos de biorremediación ecológica y bioenergía."),
            ("Centros de investigación", "Institutos de investigación científica en biotecnología.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-ciencia-molecular",
        category: "Licenciaturas",
        name: "Licenciatura en Ciencia Molecular",
        degree_title: "Licenciado/a en Ciencia Molecular",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "182 U.C",
        area: "Ciencias Médicas y de la Salud",
        icon_url: "/assets/real/career_cmolecular.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/03/CIENCIA-MOLECULAR-LITE.pdf",
        tile_color_class: "tile-coral",
        study_fields: &[
            ("Biología molecular", "Estructura y función de biomoléculas, expresión y regulación génica."),
            ("Genética", "Herencia, variación genética, epigenética e ingeniería genética."),
            ("Bioquímica", "Procesos metabólicos celulares, cinética enzimática y bioenergética."),
            ("Bioinformática", "Análisis computacional de datos ómicos y biología de sistemas."),
            ("Biotecnología", "Aplicaciones traslacionales de la biología molecular en salud e industria.")
        ],
        career_fields: &[
            ("Industria farmacéutica", "Investigación y desarrollo de nuevos fármacos y diagnósticos moleculares."),
            ("Biotecnología", "Desarrollo de productos biotecnológicos de alta precisión."),
            ("Agricultura", "Mejora molecular de cultivos y bioinsumos."),
            ("Medio ambiente", "Monitoreo molecular, biorremediación y bioenergía."),
            ("Centros de investigación", "Laboratorios e institutos de investigación en biología molecular.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-ciencia-de-datos",
        category: "Licenciaturas",
        name: "Licenciatura en Ciencia de Datos",
        degree_title: "Licenciado/a en Ciencia de Datos",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "186 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_cdatos.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/03/CIENCIA-DE-DATOS-LITE_260326_090720.pdf",
        tile_color_class: "tile-blue",
        study_fields: &[
            ("Análisis y visualización de datos", "Exploración estadística, ingeniería de datos y comunicación visual de información compleja."),
            ("Aprendizaje automático y modelos predictivos", "Modelado estadístico, inferencia causal y algoritmos predictivos a gran escala."),
            ("Inteligencia artificial y minería de datos", "Extracción de patrones en grandes volúmenes de datos (Big Data) y optimización.")
        ],
        career_fields: &[
            ("Firmas de tecnología e innovación", "Arquitectura de datos y desarrollo de productos basados en datos."),
            ("Consultoras en análisis de datos", "Inteligencia de negocios y estrategia analítica."),
            ("Sector financiero y energético", "Modelado de riesgo, pronóstico de producción y optimización operativa."),
            ("Ámbito de la salud", "Epidemiología computacional y bioestadística."),
            ("Industria manufacturera y de producción", "Optimización de cadenas de suministro y mantenimiento predictivo.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-fisica",
        category: "Licenciaturas",
        name: "Licenciatura en Física",
        degree_title: "Licenciado/a en Física",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "181 U.C",
        area: "Física, Matemática y Ciencias de la Computación",
        icon_url: "/assets/real/career_fisica.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_FISICA-07052025.pdf",
        tile_color_class: "tile-gold",
        study_fields: &[
            ("Mecánica clásica y cuántica", "Movimiento, fuerzas, energía, sistemas dinámicos y partículas subatómicas."),
            ("Electromagnetismo", "Campos eléctricos y magnéticos, electrodinámica y ondas electromagnéticas."),
            ("Óptica", "Naturaleza cuántica y ondulatoria de la luz, fotónica e instrumentos ópticos."),
            ("Termodinámica y Física Estadística", "Calor, temperatura, entropía y transiciones de fase."),
            ("Física moderna", "Relatividad, física del estado sólido, física nuclear y de partículas."),
            ("Física aplicada", "Física médica, física de materiales, instrumentación y geofísica.")
        ],
        career_fields: &[
            ("Investigación", "Centros de investigación básica y aplicada, observatorios y universidades."),
            ("Industria", "Organizaciones de tecnología, energía, petróleo, óptica y telecomunicaciones."),
            ("Educación", "Docencia superior y formación científica en instituciones educativas."),
            ("Gobierno", "Organismos reguladores, metrología y política científica.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-matematica",
        category: "Licenciaturas",
        name: "Licenciatura en Matemática",
        degree_title: "Licenciado/a en Matemáticas",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "185 U.C",
        area: "Ciencias Naturales y Exactas",
        icon_url: "/assets/real/career_matematica.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_MATEMATICA-07052025.pdf",
        tile_color_class: "tile-blue",
        study_fields: &[
            ("Álgebra, Cálculo y Análisis", "Desarrollo de pensamiento lógico-matemático riguroso desde la teoría pura hasta ecuaciones diferenciales."),
            ("Geometría y Topología", "Estructuras espaciales, variedades y métodos geométricos modernos."),
            ("Estadística y Teoría de Números", "Probabilidad, criptografía matemática, modelado estocástico y análisis de datos."),
            ("Matemática Aplicada y Computacional", "Modelado numérico y resolución de problemas complejos en ciencia, ingeniería y tecnología.")
        ],
        career_fields: &[
            ("Investigación", "Centros de investigación matemática, física teórica y universidades."),
            ("Industria y Tecnología", "Modelado matemático en energía, criptografía, finanzas e inteligencia artificial."),
            ("Educación", "Docencia universitaria y formación de talento matemático."),
            ("Gobierno", "Organismos de planificación, estadística nacional y regulación.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-nanotecnologia",
        category: "Licenciaturas",
        name: "Licenciatura en Nanotecnología",
        degree_title: "Licenciado/a en Nanotecnología",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "182 U.C",
        area: "Ingeniería y Tecnología",
        icon_url: "/assets/real/career_nano.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_NANOTECNOLOGIA-07052025.pdf",
        tile_color_class: "tile-mint",
        study_fields: &[
            ("Síntesis y caracterización de nanomateriales", "Nanotubos de carbono, grafeno, nanopartículas y nanopelículas delgadas."),
            ("Propiedades de la materia a nanoescala", "Fenómenos cuánticos de confinamiento, plasmónica y efectos de superficie."),
            ("Nanobiotecnología", "Interacción entre sistemas biológicos, nanomedicina y biosensores."),
            ("Nanoelectrónica", "Dispositivos semiconductores y espintrónicos a nanoescala."),
            ("Nanomateriales para energía", "Catálisis avanzada, celdas solares de nueva generación y baterías.")
        ],
        career_fields: &[
            ("Industria", "Electrónica avanzada, farmacéutica, catálisis petrolera, automotriz y energía."),
            ("Centros de investigación", "Laboratorios de microscopía electrónica, universidades e institutos de investigación."),
            ("Sector público", "Regulación, metrología a nanoescala y políticas públicas en nanotecnología.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-filosofia",
        category: "Licenciaturas",
        name: "Licenciatura en Filosofía",
        degree_title: "Licenciado/a en Filosofía",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "184 U.C",
        area: "Artes y Humanidades",
        icon_url: "/assets/real/career_filosofia.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_FILOSOFIA-07052025.pdf",
        tile_color_class: "tile-gold",
        study_fields: &[
            ("Historia de la filosofía", "Pensadores, escuelas y corrientes filosóficas a lo largo de la historia universal y latinoamericana."),
            ("Lógica", "Razonamiento formal válido, argumentación y análisis de falacias."),
            ("Teoría del conocimiento y Epistemología", "Naturaleza del conocimiento humano, verdad, justificación y soberanía cognitiva."),
            ("Metafísica y Ontología", "Naturaleza de la realidad, el ser y los problemas fundamentales de la existencia."),
            ("Ética y Filosofía Política", "Moralidad, bioética, ética de la inteligencia artificial, Estado, poder y justicia social."),
            ("Filosofía de la ciencia", "Fundamentos del método científico, paradigmas e implicaciones sociales de la tecnociencia.")
        ],
        career_fields: &[
            ("Educación e Investigación", "Universidades, centros de pensamiento crítico y comités de bioética."),
            ("Medios de comunicación y Cultura", "Periodismo cultural, análisis editorial y divulgación científica."),
            ("Organizaciones e Instituciones", "Ética aplicada, políticas públicas y asesoría humanística."),
            ("Proyectos Sociales", "Desarrollo comunitario y pensamiento descolonial.")
        ]
    },
    CareerProgram {
        slug: "licenciatura-en-oceanologia",
        category: "Licenciaturas",
        name: "Licenciatura en Oceanología",
        degree_title: "Licenciado/a en Oceanología",
        duration: "4 años",
        semesters: "8 semestres",
        credits: "185 U.C",
        area: "Ciencias Agrícolas y del Mar",
        icon_url: "/assets/real/career_oceanologia.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/05/LIC_OCEANOLOGIA-07052025.pdf",
        tile_color_class: "tile-blue",
        study_fields: &[
            ("Oceanografía física", "Dinámica oceánica, interacción océano-atmósfera, olas, mareas y corrientes del Caribe y Atlántico."),
            ("Oceanografía química", "Composición química del agua de mar, acidificación y ciclos biogeoquímicos."),
            ("Oceanografía biológica", "Ecología marina, biodiversidad costera y oceánica, producción primaria."),
            ("Geología marina", "Geomorfología de costas, plataforma continental y sedimentología marina."),
            ("Recursos marinos", "Gestión sostenible de pesca, acuicultura y energías marinas.")
        ],
        career_fields: &[
            ("Investigación", "Institutos oceanográficos, expediciones científicas y universidades."),
            ("Gestión ambiental y Soberanía Marítima", "Ministerios, resguardo de espacios acuáticos y conservación marina."),
            ("Industria Marítima", "Acuicultura, pesquerías sostenibles, turismo costero y energía offshore."),
            ("Consultoría", "Estudios de impacto ambiental marino-costero.")
        ]
    }
];

pub const AUTHORITIES: &[Authority] = &[
    Authority {
        name: "Dra. Gabriela Jiménez Ramírez",
        role: "Rectora (E)",
        photo_url: "/assets/real/auth_1.jpg",
        bio_focus: "Conducción estratégica de la Universidad Nacional de las Ciencias Dr. Humberto Fernández-Morán, articulación con el Sistema Nacional de Ciencia, Tecnología e Innovación y promoción de la ciencia para la vida."
    },
    Authority {
        name: "Dr. Roberto Betancourt",
        role: "Secretario (E)",
        photo_url: "/assets/real/auth_2.jpg",
        bio_focus: "Secretaría General Universitaria, custodia de las Gacetas Universitarias, certificación académica y observatorio de indicadores científicos y tecnológicos."
    },
    Authority {
        name: "Dr. Alberto Quintero",
        role: "Director (E) de Investigación",
        photo_url: "/assets/real/auth_3.jpg",
        bio_focus: "Coordinación de los centros y líneas de investigación científica, vinculación con los laboratorios del IVIC y proyectos de desarrollo tecnológico soberano."
    },
    Authority {
        name: "Dr. Irwing Acevedo",
        role: "Director (E) Estudiantil",
        photo_url: "/assets/real/auth_4.jpg",
        bio_focus: "Atención integral a la comunidad estudiantil proveniente de los 23 estados del país, bienestar universitario, becas, clubes científicos y deportes."
    },
    Authority {
        name: "Profa. Diana Castillo",
        role: "Directora (E) Académica",
        photo_url: "/assets/real/auth_5.jpg",
        bio_focus: "Gestión curricular de las 15 carreras de pregrado, Curso de Acompañamiento a la Iniciación Universitaria (CAIU), programas de ampliación y excelencia docente."
    },
    Authority {
        name: "Lic. Damian López",
        role: "Director (E) Administrativo",
        photo_url: "/assets/real/auth_6.jpg",
        bio_focus: "Administración general, infraestructura del campus en Altos de Pipe, servicios logísticos, transporte universitario y equipamiento de laboratorios."
    },
    Authority {
        name: "Ing. Danmarys Hernández",
        role: "Directora (E) de Registro, Control y Seguimiento",
        photo_url: "/assets/real/auth_7.jpg",
        bio_focus: "Control de estudios, admisión, validación documental, autenticación de carnet estudiantil y seguimiento de rendimiento académico."
    }
];

pub const MAGAZINE_ISSUES: &[MagazineIssue] = &[
    MagazineIssue {
        title: "Noticias de la Ciencia — Noviembre 2026",
        subtitle: "Boletín Especial · 2 de Noviembre de 2026",
        cover_url: "/assets/real/mag_1.jpg",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/11/Noticias-de-la-Ciencia-2-de-NOVIEMBRE-de-2026.pdf",
        tile_class: "tile-blue"
    },
    MagazineIssue {
        title: "Ciencia para la Vida — Edición Octubre",
        subtitle: "Revista Mensual del Mincyt",
        cover_url: "/assets/real/mag_2.png",
        pdf_url: "https://mincyt.gob.ve/wp-content/uploads/2025/11/revista-digital-1.pdf",
        tile_class: "tile-coral"
    },
    MagazineIssue {
        title: "Ciencia para la Vida — Edición Septiembre",
        subtitle: "Revista Mensual del Mincyt",
        cover_url: "/assets/real/mag_3.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/10/Revista-Septiembre-digital.pdf",
        tile_class: "tile-mint"
    },
    MagazineIssue {
        title: "Ciencia para la Vida — Edición Mayo",
        subtitle: "Revista Mensual del Mincyt",
        cover_url: "/assets/real/mag_4.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/06/ciencia_para_la_vida_revista_mayo.pdf",
        tile_class: "tile-gold"
    },
    MagazineIssue {
        title: "Ciencia para la Vida — Edición Abril",
        subtitle: "Revista Mensual del Mincyt",
        cover_url: "/assets/real/mag_5.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/06/Ciencia-para-la-Vida-Abril.pdf",
        tile_class: "tile-blue"
    },
    MagazineIssue {
        title: "Ciencia para la Vida — Edición Marzo",
        subtitle: "Revista Mensual del Mincyt",
        cover_url: "/assets/real/mag_6.png",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2025/06/Ciencia-para-la-Vida-Marzo.pdf",
        tile_class: "tile-mint"
    }
];

pub const GAZETTES: &[GazetteDoc] = &[
    GazetteDoc {
        number: "Gaceta N.º 1",
        title: "Gaceta Universitaria Ordinaria N.º 1",
        year: "2026",
        kind: "Ordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/02/GACETA_1.pdf",
        summary: "Actas fundacionales, estatuto orgánico y disposiciones académicas iniciales de la UNC."
    },
    GazetteDoc {
        number: "Gaceta N.º 2",
        title: "Gaceta Universitaria Ordinaria N.º 2",
        year: "2026",
        kind: "Ordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/06/GACETA2-UNC.pdf",
        summary: "Aprobación de reglamentos de régimen de estudios, evaluación estudiantil y normas de permanencia."
    },
    GazetteDoc {
        number: "Gaceta N.º 3",
        title: "Gaceta Universitaria Ordinaria N.º 3",
        year: "2026",
        kind: "Ordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/04/GACETA__3_.pdf",
        summary: "Designaciones académicas, creación de comisiones curriculares y líneas estratégicas de investigación."
    },
    GazetteDoc {
        number: "Gaceta N.º 4",
        title: "Gaceta Universitaria Ordinaria N.º 4",
        year: "2026",
        kind: "Ordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/04/GACETA_4_.pdf",
        summary: "Normativa de laboratorios docentes, seguridad radiológica, bioética y propiedad intelectual."
    },
    GazetteDoc {
        number: "Gaceta N.º 5",
        title: "Gaceta Universitaria Ordinaria N.º 5",
        year: "2026",
        kind: "Ordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/05/GACETA-5.pdf",
        summary: "Convenios interinstitucionales con el IVIC, CENDIT y centros del Sistema Nacional de Ciencia y Tecnología."
    },
    GazetteDoc {
        number: "Gaceta N.º 6",
        title: "Gaceta Universitaria Ordinaria N.º 6",
        year: "2026",
        kind: "Ordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/10/GACETA-6.pdf",
        summary: "Disposiciones del período lectivo 2026-II, ingreso de nueva cohorte y calendario de actividades."
    },
    GazetteDoc {
        number: "Extraordinaria N.º 1",
        title: "Gaceta Universitaria Extraordinaria N.º 1 (2026)",
        year: "2026",
        kind: "Extraordinaria",
        pdf_url: "https://unc.edu.ve/wp-content/uploads/2026/09/GACETA-EXTRAORDINARIA-N%C2%B01-2026.pdf",
        summary: "Resolución especial del Consejo Universitario de la UNC correspondiente al ejercicio académico 2026."
    }
];

pub const SAMPLE_CARNETS: &[CarnetRecord] = &[
    CarnetRecord { num: "1", cedula: "31.842.109", full_name: "NAVAS PEREZ ALEXANDER JOSE", career: "Ingeniería en Inteligencia Artificial", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "2", cedula: "31.490.221", full_name: "VIELMA ROJAS STEPHANY VALENTINA", career: "Licenciatura en Biotecnología", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "3", cedula: "30.918.445", full_name: "MOLLEDA CASTRO CESAR AUGUSTO", career: "Licenciatura en Física", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "4", cedula: "31.204.883", full_name: "CONTRERAS SILVA EDUARDO LUIS", career: "Licenciatura en Biología y Química Computacional", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "5", cedula: "30.554.712", full_name: "MEJIAS HERNANDEZ KEIVER DANIEL", career: "Licenciatura en Filosofía", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "6", cedula: "31.775.301", full_name: "RIOS MENDOZA ALFREDO ALEJANDRO", career: "Ingeniería en Robótica y Automatización", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "7", cedula: "28.912.640", full_name: "ROJAS GOMEZ AIRADMHERN DEL VALLE", career: "Licenciatura en Filosofía", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "8", cedula: "31.608.914", full_name: "PEREIRA TORRES VALENTINA SOFIA", career: "Ingeniería en Ciberseguridad", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "9", cedula: "32.104.559", full_name: "LEAL MARTINEZ CARLOS EDUARDO", career: "Ingeniería en Robótica y Automatización", condition: "NUEVO INGRESO 2026" },
    CarnetRecord { num: "10", cedula: "32.219.830", full_name: "VELASQUEZ DIAZ KARLA MARIANA", career: "Ingeniería en Inteligencia Artificial", condition: "NUEVO INGRESO 2026" },
    CarnetRecord { num: "11", cedula: "31.339.402", full_name: "SALAZAR QUINTERO GABRIEL ENRIQUE", career: "Ingeniería en Electromedicina", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "12", cedula: "31.501.778", full_name: "MONTOYA RIVAS CAMILA ANDREA", career: "Licenciatura en Ciencia de Datos", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "13", cedula: "31.889.120", full_name: "ZAMBRANO PENA DIEGO ARMANDO", career: "Licenciatura en Física Nuclear", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "14", cedula: "31.945.661", full_name: "FIGUEROA ACUNA MARIANA ISABEL", career: "Licenciatura en Nanotecnología", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "15", cedula: "31.712.004", full_name: "DELGADO MORENO SANTIAGO ANDRES", career: "Licenciatura en Oceanología", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "16", cedula: "31.644.918", full_name: "GUTIERREZ BLANCO SOFIA VICTORIA", career: "Ingeniería en Biomateriales", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "17", cedula: "31.402.315", full_name: "MARQUEZ LINARES ANDRES FELIPE", career: "Licenciatura en Ciencia Molecular", condition: "REGULAR ACTIVO" },
    CarnetRecord { num: "18", cedula: "31.598.207", full_name: "PARRA COLMENARES VALERIA ALEJANDRA", career: "Licenciatura en Matemática", condition: "REGULAR ACTIVO" }
];
