use crate::presentation::Present;

use super::context::GodotPresentationContext;

/// Fachada fina: inicia a aplicação serial da saída no Godot.
#[derive(Default)]
pub(crate) struct GodotBridge {
    context: GodotPresentationContext,
}

impl GodotBridge {
    pub(crate) fn context_mut(&mut self) -> &mut GodotPresentationContext {
        &mut self.context
    }

    pub(crate) fn apply<P>(&mut self, output: &mut P)
    where
        P: Present<GodotPresentationContext> + ?Sized,
    {
        output.present(&mut self.context);
    }
}
