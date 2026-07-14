/// Um domínio de apresentação sabe aplicar seus dados em um contexto.
pub(crate) trait Present<Context> {
    fn present(&mut self, context: &mut Context);
}
