pub(crate) trait Renderable<T> {
    fn render(&self, target: &mut T) -> Result<(), String>;
}

pub(crate) trait Renderer<T> {
    fn render(&self, target: &mut T, renderables: &[&dyn Renderable<T>]) -> Result<(), String>;
}
