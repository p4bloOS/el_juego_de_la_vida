// Todo lo que escribamos aquí puede ser invocado en app.rs

#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

struct Util;

impl Util {

    /// Obtención de un aspecto visual personalizado
    fn visuales() -> egui::Visuals {
        let mut visual = egui::Visuals::dark();
        visual.panel_fill = egui::Color32::from_rgb(32, 33, 36);
        visual.override_text_color = Some(egui::Color32::from_rgb(5,213,255));
        //visual.override_text_color = Some(egui::Color32::WHITE);
        visual
    }

    /// Obtención de un tamaño de letra que tiene en cuenta la resolución del monitor
    fn tamano_fuente_adecuado(_cc: &eframe::CreationContext<'_>) -> f32 {
        
        let puntos_fuente;
        
        #[cfg(target_arch = "wasm32")]
        {
        puntos_fuente = 20.0; // Tamaño de letra en versión web (provisional)
                            // Los puntos no parecen igual de grandes que en nativo
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
        let resolucion = _cc.integration_info.window_info.monitor_size.unwrap();
        //let resolucion = [1024.0, 768.0];
        let pixeles_por_punto = _cc.integration_info.native_pixels_per_point.unwrap();
        let pixeles_fuente_con_monitor_1080 = 20.0;
        let diagonal_monitor = ((resolucion[0] as f32).powi(2)  + (resolucion[1] as f32).powi(2)).sqrt();
        let diagonal_monitor_1080 = ((1920 as f32).powi(2)  + (1080 as f32).powi(2)).sqrt();
        let pixeles_fuente = (
            pixeles_fuente_con_monitor_1080 / diagonal_monitor_1080) * diagonal_monitor;
        puntos_fuente = pixeles_fuente / pixeles_por_punto;

        println!("Resolución de monitor: {}x{}", resolucion[0], resolucion[1]);
        println!("Píxeles por punto de forma nativa: {}", pixeles_por_punto);
        println!("Diagonal del monitor en píxeles: {}", diagonal_monitor);
        println!("Tamaño de fuente en píxeles: {}", pixeles_fuente);
        println!("Tamaño de fuente en puntos: {}", puntos_fuente);
        }

        puntos_fuente
    }
    
    // Cambio del estilo (texto y el espaciado)
    fn cambiar_estilo(cc: &eframe::CreationContext<'_>, puntos_fuente: f32) {

        // Instalamos nuestra propia fuente (.ttf and .otf files supported)
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("fuente_1".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/Cantarell-VF.otf")),);
        fonts.font_data.insert("fuente_2".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/DroidSansMono-enMp.ttf")),);

        // Damos la máxima prioridad a nuestra fuente_1 para el texto "Proportional":
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "fuente_1".to_owned());

        // Damos la máxima prioridad a nuestra fuente_2 para el texto "Monospace":
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "fuente_2".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut style = (*cc.egui_ctx.style()).clone();
        let fuente = egui::FontFamily::Proportional;

        style.text_styles = [
            (egui::style::TextStyle::Heading, egui::FontId::new(puntos_fuente*1.5, fuente.clone())),
            (egui::style::TextStyle::Body, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Monospace, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Button, egui::FontId::new(puntos_fuente, fuente.clone())),
            (egui::style::TextStyle::Small, egui::FontId::new(puntos_fuente, fuente)),
        ].into();

        style.spacing.item_spacing.y = puntos_fuente / 8.0;
        style.spacing.item_spacing.x = puntos_fuente / 8.0;

        cc.egui_ctx.set_style(style);

    }

}


struct Juego {
    matriz: Vec<Vec<bool>>, // matriz de celdas -> true = vivas, false = muertas
    ultima_progresion: f64, //tiempo de la última progresión del juego si está en marcha
}

impl Juego {

    /// Crea el tablero con longitudes x e y
    fn new(x: usize, y: usize) -> Self {

        println!("Dimensionando matriz");

        let mut matriz: Vec<Vec<bool>> = Vec::with_capacity(x);

        for i in 0..x {
            matriz.push(Vec::with_capacity(x));
            for _j in 0..y {
                matriz[i].push(false);
            }
        }

        Self {matriz, ultima_progresion:-1.0}
    }

    /// Graficación el tablero
    fn pintar(& mut self, tam_celda: f32, dims_totales: egui::Vec2, disposicion: egui::Layout, ui: & mut egui::Ui) {

        // los márgenes establecidos
        let margen = ui.style().spacing.item_spacing.x;

        // Contenedor
        ui.allocate_ui_with_layout(dims_totales,disposicion, |ui| {

            // Limitación de las dimensiones para que cuadren se coloquen las celdas en cuadrícula perfecta
            ui.set_max_width( (tam_celda + margen) * self.matriz.len() as f32 );
            ui.set_max_height(tam_celda + margen);

            // Bucle de pintado
            for i in 0..self.matriz.len() {
                for j in 0..self.matriz[i].len() {

                    let (response, painter) = ui.allocate_painter(egui::Vec2::splat(tam_celda), egui::Sense::click());
                    let rectangulo = response.rect;
                    let color_muertas = egui::Color32::from_gray(128);
                    let color_vivas = egui::Color32::from_rgb(5,213,255);

                    if response.hovered()
                        { painter.rect_filled(rectangulo, egui::Rounding::none(), egui::Color32::WHITE);}

                    else if !self.matriz[i][j] 
                        { painter.rect_filled(rectangulo, egui::Rounding::none(), color_muertas);}

                    else
                        { painter.rect_filled(rectangulo, egui::Rounding::none(), color_vivas); }

                    if response.clicked() { self.modificar(i, j);}
                }
            }
        });


    }

    /// Modificación de una celda (inversión de su estado)
    fn modificar(& mut self, i: usize, j: usize) {
        self.matriz[i][j] = ! self.matriz[i][j];
    }

    /// ¿Es momento de ejecutar una progresión?
    fn momento_de_progreso(& mut self, tiempo_actual: f64) -> bool
    {
        // Si han pasado más de 0.49seg desde la última progresión, hay que hacer otra
        if tiempo_actual-self.ultima_progresion > 0.49
            { self.ultima_progresion = tiempo_actual;
            return true;
            }
        else
            {return false; }
    }

    /// Ejecucuión de un paso o turno del autómata
    fn progresar(& mut self) {

        self.matriz[0][0] = !self.matriz[0][0];

        // Invertiremos las celdas que indexemos en lista_progresion
        let mut lista_progresion: Vec<(usize,usize)> = Vec::new();

        for i in 0..self.matriz.len() {
            for j in 0..self.matriz[i].len() {

                println!("{}",self.matriz[i][j]);

                let vv = self.vecinas_vivas(i,j);

                // Nacimiento
                if !self.matriz[i][j] {
                    if vv == 3
                        {lista_progresion.push((i,j)); }
                }

                // Muerte
                else {
                    println!("hola");
                    if vv>3 || vv<2
                        { lista_progresion.push((i,j)) ;}
                }
            }
        }
        println!("--------");

        for (i, j) in lista_progresion
            {println!("En la lista: {},{}",i,j);
            self.matriz[i][j] = !self.matriz[i][j];}
    }

    /// Cálculo del núm. de celdas vecinas vivas
    fn vecinas_vivas(& mut self, i: usize, j: usize)-> u32
    {
        let mut retorno = 0;
        let mut vecinas = [false; 8];
        let anchura = self.matriz.len();
        let altura = self.matriz[0].len();
        let i_menos_uno;
        if i==0
            {i_menos_uno = anchura-1; }
        else
            {i_menos_uno = i - 1 ; }
        let j_menos_uno;
        if j==0
        {j_menos_uno = altura-1; }
        else
        {j_menos_uno = j - 1 ; }

        vecinas[0] = self.matriz[i][ j_menos_uno ];
        vecinas[1] = self.matriz[i][ (j+1) % altura ];
        vecinas[2] = self.matriz[ i_menos_uno ][j];
        vecinas[4] = self.matriz[ i_menos_uno ][ j_menos_uno ];
        vecinas[5] = self.matriz[ i_menos_uno ][ (j+1) % altura ];
        vecinas[3] = self.matriz[ (i+1) % anchura ][j];
        vecinas[6] = self.matriz[ (i+1) % anchura ][ j_menos_uno ];
        vecinas[7] = self.matriz[ (i+1) % anchura ][ (j+1) % altura ];

        for v in vecinas
            {if v { retorno+=1;} }

        retorno
    }

}