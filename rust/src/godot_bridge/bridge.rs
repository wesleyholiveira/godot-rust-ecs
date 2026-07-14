use crate::presentation::Present;

use super::context::GodotPresentationContext;

/// Fachada fina da camada de apresentação Godot.
///
/// O bridge aceita qualquer saída que implemente `Present` para o contexto do
/// Godot. O derive procedural compõe os domínios; cada presenter drena seu
/// próprio buffer e preserva as alocações para os ticks seguintes.
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
