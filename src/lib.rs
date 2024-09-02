use zed_extension_api as zed;

struct NickelExtension {
    // ... state
}

impl zed::Extension for NickelExtension {
    fn new() -> Self {
        NickelExtension {
            // ... state
        }
    }
}

zed::register_extension!(NickelExtension);
