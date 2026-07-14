use crate::presentation::{Present, PresentationOutput};

use super::context::GodotPresentationContext;

/// Fachada fina da camada de apresentação Godot.
///
/// Não contém loops nem `match` por tipo de comando. O derive procedural gera
/// a composição e cada domínio implementa seu próprio `Present<Context>`.
#[derive(Default)]
pub(crate) struct GodotBridge {
    context: GodotPresentationContext,
}

impl GodotBridge {
    pub(crate) fn context_mut(&mut self) -> &mut GodotPresentationContext {
        &mut self.context
    }

    pub(crate) fn apply(&mut self, output: PresentationOutput) {
        output.present(&mut self.context);
    }
}
