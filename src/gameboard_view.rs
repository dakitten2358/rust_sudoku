//! Gameboard view
use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use GameboardController;

/// settings for the view
pub struct GameboardViewSettings {
    /// Offset from the top-left corner
    pub offset: [f64; 2],
    /// Size of teh gameboard
    pub size: f64,
    /// Background Color
    pub background_color: Color,
    /// Border color
    pub border_color: Color,
    /// Edge color (around the whole board)
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections
    pub section_edge_color: Color,
    /// Edge color between the cells
    pub cell_edge_color: Color,
    /// Edge radius around the whole board
    pub board_edge_radius: f64,
    /// Edge radius around teh sections
    pub section_edge_radius: f64,
    /// Edge radius around the cells
    pub cell_edge_radius: f64,
    /// Select cell background color
    pub selected_cell_background_color: Color,
    /// Color of the text
    pub text_color: Color,
}

impl GameboardViewSettings {
    /// Creates a  new gameboard view settings
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            offset: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}

/// Store visual information about a gameboard
pub struct GameboardView {
    /// Stores gameboard view settings
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings: settings }
    }

    /// Draw the gameboard
    pub fn draw<G: Graphics, GlyphType>(&self,
                                        c: &Context,
                                        controller: &GameboardController,
                                        g: &mut G,
                                        glyphs: &mut GlyphType)
        where GlyphType: CharacterCache<Texture = G::Texture>
    {
        use graphics::{Line, Rectangle, Image, Transformed};

        let ref settings = self.settings;
        let board_rect = [settings.offset[0], settings.offset[1], settings.size, settings.size];

        // draw the board background
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // draw the cell borders
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 0..9 {
            // skip lines that are covered by collections
            if (i % 3) == 0 {
                continue;
            }

            let x = settings.offset[0] + (((i as f64) / 9.0) * settings.size);
            let y = settings.offset[1] + (((i as f64) / 9.0) * settings.size);
            let x2 = settings.offset[0] + settings.size;
            let y2 = settings.offset[1] + settings.size;

            let vline = [x, settings.offset[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.offset[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // draw the section borders
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 0..3 {
            let x = settings.offset[0] + (((i as f64) / 3.0) * settings.size);
            let y = settings.offset[1] + (((i as f64) / 3.0) * settings.size);
            let x2 = settings.offset[0] + settings.size;
            let y2 = settings.offset[1] + settings.size;

            let vline = [x, settings.offset[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.offset[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // draw the board edge
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // draw the selected background, if we have a one
        if let Some(ind) = controller.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect =
                [settings.offset[0] + pos[0], settings.offset[1] + pos[1], cell_size, cell_size];
            Rectangle::new(settings.selected_cell_background_color)
                .draw(cell_rect, &c.draw_state, c.transform, g);
        }

        // draw characters
        let text_iamge = Image::new_color(settings.text_color);
        let cell_size = settings.size / 9.0;
        for row in 0..9 {
            for col in 0..9 {
                if let Some(ch) = controller.gameboard.display_character_at([col, row]) {
                    let pos = [settings.offset[0] + (col as f64 * cell_size) + 15.0,
                               settings.offset[1] + (row as f64 * cell_size) + 34.0];
                    let character = glyphs.character(34, ch);
                    let ch_x = pos[0] + character.left();
                    let ch_y = pos[1] - character.top();
                    text_iamge.draw(character.texture,
                                    &c.draw_state,
                                    c.transform.trans(ch_x, ch_y),
                                    g);
                }
            }
        }
    }
}
