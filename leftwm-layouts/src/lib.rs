use std::str::FromStr;

use layouts::main_and_vert_stack::MainAndVertStack;

use crate::geometry::Tile;
use crate::layouts::monocle::Monocle;

pub mod geometry;
pub mod layouts;

pub enum Layouts {
    Monocle,
    MainAndVertStack,
}

pub struct LayoutParseError;
impl FromStr for Layouts {
    type Err = LayoutParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "Monocle" => Ok(Layouts::Monocle),
            "MainAndVertStack" => Ok(Layouts::MainAndVertStack),
            _ => Err(LayoutParseError),
        }
    }
}

pub trait Layout {
    // QUESTION: instead of returning Options, this could just return the "Some" values
    // because a layout will probably never leave out a window in the middle?
    // it may return a list smaller than the window_count (monocle, main_and_deck, ...) 
    // but in this case the returned list should be applied to the first windows in order 
    // and the reset should be hidden
    fn apply(&self, window_count: usize, modifiers: &LayoutModifiers) -> Vec<Option<Tile>>;
    
    // QUESTION: might be helpful if the layout_manager can find out if the layout even supports 
    // multiple_master_windows, some might not (monocle?, main_and_deck?)
    //fn supports_multiple_master_windows() -> bool;

    // helper method
    fn master_window_count(&self, window_count: usize, modifiers: &LayoutModifiers) -> usize {
        if window_count < modifiers.master_window_count {
            window_count
        } else {
            modifiers.master_window_count as usize
        }
    }

    // helper methodo
    fn stack_window_count(&self, window_count: usize, modifiers: &LayoutModifiers) -> usize {
        window_count - self.master_window_count(window_count, modifiers)
    }
}

pub struct LayoutModifiers {
    pub container_size: Tile,
    pub master_width_percentage: f32,
    pub master_window_count: usize,
    pub max_column_width: Option<u32>,
    pub flipped_horizontal: bool,
    pub flipped_vertical: bool,
}

impl Default for LayoutModifiers {
    fn default() -> Self {
        Self { 
            container_size: Tile::default(), 
            master_width_percentage: 60.0, 
            master_window_count: 1,
            max_column_width: None,
            flipped_horizontal: false,
            flipped_vertical: false
        }
    }
}

#[derive(Debug)]
pub struct LayoutNotFoundError;
impl Layouts {
    pub fn get(&self) -> Box<dyn Layout> {
        match self {
            Layouts::Monocle => Box::new(Monocle),
            Layouts::MainAndVertStack => Box::new(MainAndVertStack),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{LayoutModifiers, Layouts};

    const ALL_LAYOUTS: &[Layouts] = &[
        Layouts::Monocle,
        Layouts::MainAndVertStack,
    ];

    // todo
    //fn no_overlap_of_rects() {
    //    todo!()
    //}

    // QUESTION: is that a fair assumption?
    // -> follow-up: only works if remaining space is accounted for instead
    //               of rounding off
    //               eg. 3-column layout on 100px width results in 3x 33px leaving a 1px remainder
    //              this remainder should be attributed to one of the columns to fill up the entire width
    #[test]
    fn container_must_always_be_filled() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let container_area = modifiers.container_size.surface_area();
        for window_count in 1..10 {
            for layout in ALL_LAYOUTS {
                let layout = layout.get();
                let filled_area = layout.apply(window_count, &modifiers)
                .into_iter()
                .filter_map(|e| e)
                .fold(0i32, |a,b| a + b.surface_area());
                assert_eq!(container_area, filled_area);
            }
        }
    }

    #[test]
    fn test_monocle_layout() {
        let modifiers: LayoutModifiers = LayoutModifiers::default();
        let monocle = Layouts::Monocle.get();
        let monocle_positions = monocle.apply(1, &modifiers);
        assert_eq!(monocle_positions.len(), 1);
    }
}
