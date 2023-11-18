pub(crate) trait Renderable<T> {
    fn render(&self, target: &mut T) -> Result<(), String>;
}
