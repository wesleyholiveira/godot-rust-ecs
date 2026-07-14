use godot::prelude::*;

/// Etiqueta usada pelo godot-rust para gerar o entrypoint da GDExtension.
struct DemoExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DemoExtension {}
