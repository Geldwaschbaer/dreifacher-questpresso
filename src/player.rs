pub struct Player {
    map_position: usize,
    dialog_position: usize,
}

impl Player {
    pub fn new() -> Player {
        Player {
            map_position: 0,
            dialog_position: 0,
        }
    }

    pub fn enter_room(&mut self, room: usize) {
        self.map_position = room;
    }

    pub fn get_map_position(&self) -> usize {
        self.map_position
    }

    pub fn get_dialog_position(&self) -> usize {
        self.dialog_position
    }

    pub fn set_dialog_position(&mut self, position: usize) {
        self.dialog_position = position;
    }
}
