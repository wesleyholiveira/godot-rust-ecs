use godot::prelude::*;

struct DemoExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DemoExtension {}
