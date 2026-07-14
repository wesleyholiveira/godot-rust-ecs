/// Contrato mínimo implementado por cada domínio de apresentação.
///
/// `Context` é genérico: o derive procedural não conhece Godot. Neste projeto,
/// as implementações concretas usam `GodotPresentationContext`.
pub(crate) trait Present<Context> {
    fn present(self, context: &mut Context);
}
