use crate::ui::colors::app_colors::AppColors;

pub trait ColorScheme {
    fn colors(&self) -> AppColors;
}
