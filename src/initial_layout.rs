
pub struct InitialLayout {
    pub paddle_y: f32,
    pub bottom_wall_y: f32,
}

pub fn create_initial_layout() -> InitialLayout {
  const BOTTOM_WALL: f32 = -300.0;
  const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
  
  let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
  let bottom_wall_y = BOTTOM_WALL;

  let layout = InitialLayout{
      paddle_y,
      bottom_wall_y
  };
  return layout;
}

fn get_init_paddle_y() -> f32 {
    return create_initial_layout().paddle_y;
}

fn get_init_bottom_wall_y() -> f32 {
    return create_initial_layout().bottom_wall_y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_wall_y() {
        
        assert_eq!(get_init_bottom_wall_y(), -300.0);
    }

    #[test]
    fn test_initial_layout() {
        assert_eq!(
          get_init_paddle_y(),
            create_initial_layout().paddle_y
        );
    }

    
}

