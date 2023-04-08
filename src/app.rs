use egui::Vec2;

use crate::Util;
use crate::Juego;

/// Derivamos Deserialize/Serialize para poder persistir el estado de la app al cerrarse
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // si agregamos nuevos campos, daremos valores predeterminados al deserializar el estado anterior
pub struct TemplateApp {
    // Atributos de ejemplo:
    texto_introducido: String,

    // Así es como se excluye de la serialización un miembro
    #[serde(skip)]
    replicar: bool,

    complejidad: u32,

    tam_fuente: f32,

    #[serde(skip)]
    juego: Juego,

}

impl Default for TemplateApp {
    fn default() -> Self {
        let complj = 8;
        Self {
            // Atributos de ejemplo:
            texto_introducido: "Hola mundo!".to_owned(),
            replicar: false,
            complejidad: complj,
            tam_fuente: 10.0, // no recomendado usar este valor
            juego: Juego::new(complj as usize, complj as usize),
        }
    }
}

impl TemplateApp {
    /// Llamada una vez antes del primer fotograma
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Aquí se puede personalizar la apariencia de la IGU usando
        // `cc.egui_ctx.set_visuals` y `cc.egui_ctx.set_fonts`.

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
    /// Llamada por el framework para guardar el estado después de cerrar.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Llamada cuando la IU necesita redibujarse, lo cual podría ocurrir múltiples veces por segundo.
    /// ctx es el contexto de la interfaz egui; nos permite manejarla.
    /// Pon tus widgets dentro de `SidePanel`, `TopPanel`, `CentralPanel`, `Window` o `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let Self { texto_introducido, replicar, complejidad, .. } = self;

        // Ejemplos de cómo crear algunos paneles y widgets.
        // Consejo: una buena elección por defecto es simplemente dejar el `CentralPanel`.
        // Para más ejemplos e inspiración ir a https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {

                // Título
                ui.heading("EL JUEGO DE LA VIDA");
                ui.add_space(self.tam_fuente * 2.0);

                // Juego
                

                let tam_celda = egui::Vec2::splat(self.tam_fuente * 2.0);
                let tam_total = egui::Vec2::new(tam_celda.x * 10.0, 1.0);
                let esquema = egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true);

                ui.allocate_ui_with_layout(tam_total,esquema, |ui| {

                    //let (response2, painter2) = ui.allocate_painter(tam_total, egui::Sense::click());
                    //let rect = response2.rect;
                    //painter2.rect_filled(rect, egui::Rounding::none(), egui::Color32::WHITE);
                    let margen = ui.style().spacing.item_spacing.x;
                    ui.set_max_width( (tam_celda.x + margen) * self.juego.matriz.len() as f32 );
                    ui.set_max_height(tam_celda.y + margen);

                    for i in 0..self.juego.matriz.len() {

                        for j in 0..self.juego.matriz[i].len() {

                            let (response, painter) = ui.allocate_painter(tam_celda, egui::Sense::click());
                            let rect = response.rect;
                            let color = egui::Color32::from_gray(128);
                            if response.hovered() {painter.rect_filled(rect, egui::Rounding::none(), egui::Color32::BLUE);}
                            else {painter.rect_filled(rect, egui::Rounding::none(), color);}
                            if response.clicked() { println!("clicado"); }
                        }
                    }
                });

                ui.add_space(self.tam_fuente * 2.0);

                ui.allocate_ui_with_layout(tam_total,esquema, |ui| {
                    // Selector de complejidad
                    let deslizador = egui::Slider::new( complejidad, 0..=100).prefix("Complejidad: ");
                    ui.add(deslizador).on_hover_text("Arrástrame!");
                });


                ui.add_space(self.tam_fuente * 2.0);

                // Botón de empezar
                let boton = ui.button("EMPEZAR");
                if boton.clicked(){
                    *replicar = true;
                }
                if *replicar { ui.label(egui::RichText::new(texto_introducido.clone()).color(egui::Color32::WHITE));
                } else { ui.label(""); }


                // Avisa si no es una release
                egui::warn_if_debug_build(ui);
            });
            
    });
    });
    }


}
