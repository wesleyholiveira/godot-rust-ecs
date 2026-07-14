/// Contrato mínimo implementado por cada domínio de apresentação.
///
/// O valor é recebido por referência mutável para que os presenters possam
/// drenar seus buffers sem descartar a capacidade alocada. Assim, o mesmo
/// `PresentationOutput` é reutilizado em todos os ticks.
///
/// `Context` continua genérico: o derive procedural não conhece Godot. Neste
/// projeto, as implementações concretas usam `GodotPresentationContext`.
pub(crate) trait Present<Context> {
    fn present(&mut self, context: &mut Context);
}
