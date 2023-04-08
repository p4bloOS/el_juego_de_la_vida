use crate::Util;
use crate::Juego;

/// Derivamos Deserialize/Serialize para poder persistir el estado de la app al cerrarse
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // si agregamos nuevos campos, daremos valores predeterminados al deserializar el estado anterior
pub struct TemplateApp {

    // Atributos para mantener el estado de nuestra aplicación
    #[serde(skip)]
    complejidad: u32,

    // Así es como se excluye de la serialización un miembro
    #[serde(skip)]
    tam_fuente: f32,

    #[serde(skip)]
    juego: Juego,

    #[serde(skip)]
    en_marcha: bool,

}

impl Default for TemplateApp {

    fn default() -> Self {

        // Complejidad por defecto = 8
        let complj = 8;
        Self {
            complejidad: complj,
            tam_fuente: 10.0, // no recomendado usar este valor
            juego: Juego::new(complj as usize, complj as usize),
            en_marcha: false,
        }
    }
}

impl TemplateApp {

    /// <<Inicialización>> (invocada una vez antes del primer fotograma)
    /// ** Retorna el estado inicial de la aplicación
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // Personaliza los colores
        let visuales = Util::visuales();
        cc.egui_ctx.set_visuals(visuales);

        // Obtiene el tamaño de fuente y personaliza la fuente
        let tam_fuente = Util::tamano_fuente_adecuado(cc);
        Util::cambiar_estilo(cc, tam_fuente);

        println!("Tamaño obtenido: {}", tam_fuente);

        // Carga el estado previo de la aplicación (si lo hubiese)
        // Advierte que se debe habilitar la característica `persistence` para que funcione
        if let Some(storage) = cc.storage {
            let mut retorno: TemplateApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            retorno.tam_fuente = tam_fuente;
            return retorno
        }

        Self {
            tam_fuente: tam_fuente,
            ..Default::default()
        }
    }
}

/*Tenemos que implementar la característica App para escribir aplicaciones que puedan
compilarse tanto para web como para nativo usando eframe*/
impl eframe::App for TemplateApp {


    /// <<Guardado>> (se invoca para guardar el estado después de cerrar)
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// <<Refresco>> (se invoca cuando la IU necesita redibujarse, lo cual podría ocurrir múltiples veces
    ///  por segundo. ctx es el contexto de la interfaz egui; nos permite manejarla.)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Recuperamos el estado de la aplicación
        let Self { complejidad, tam_fuente, juego, en_marcha } = self;

        // EA partir de aquí añadimos los contenedores y widgets de nuestra IGU
        egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
        ui.vertical_centered(|ui| {

            // Título
            ui.heading("EL JUEGO DE LA VIDA");
            ui.add_space(*tam_fuente * 2.0);

            // Juego
            let tam_celda = *tam_fuente * 2.0;
            let anchura =  tam_celda * *complejidad as f32;
            let dims_tablero = egui::Vec2::new(anchura, 1.0);
            let dims_deslizador = egui::Vec2::new(*tam_fuente * 16.0,0.0);
            let disposicion = egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true);

            // Selector de complejidad y botón EMPEZAR
            ui.allocate_ui_with_layout(dims_deslizador,disposicion, |ui| {

                // Selector de complejidad
                let deslizador = egui::Slider::new( complejidad, 0..=100).prefix("Selecciona la complejidad: ");
                let mut interaccion_deslizador = ui.add(deslizador);
                interaccion_deslizador = interaccion_deslizador.on_hover_text("Arrástrame!");
                if interaccion_deslizador.changed() {
                    *juego = Juego::new(*complejidad as usize, *complejidad as usize);
                    *en_marcha = false;
                }
            });

            ui.add_space(*tam_fuente * 2.0);

            // Botón de empezar
            let boton = ui.button("EMPEZAR");
            if boton.clicked(){
                println!("EMPEZANDO");
                *en_marcha = true;
            }

            ui.add_space(*tam_fuente * 2.0);

            // Graficación del tablero
            juego.pintar(tam_celda, dims_tablero, disposicion, ui);
            
            ui.add_space(tam_celda);

            // Gestión temporal del juego
            let tiempo = ui.input(|i| i.time);
            if *en_marcha && juego.momento_de_progreso(tiempo) {
                juego.progresar();
            }

            // Si el juego está en marcha la IGU debe refrescarse más a menudo
            if *en_marcha
                { ctx.request_repaint_after(std::time::Duration::from_secs_f32(0.25)); }

            // Créditos
            ui.label("Basado en el célebre autómata celular diseñado por John Horton Conway. ");
            ui.hyperlink_to("( https://es.wikipedia.org/wiki/Juego_de_la_vida) ","https://es.wikipedia.org/wiki/Juego_de_la_vida");
            ui.label(">> Selecciona la complejidad, escribe el patrón y pulsa empezar ");

            // Avisa si no es una release
            egui::warn_if_debug_build(ui);


        });
        });
        });
    }


}
