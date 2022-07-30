use beryllium::{ContextFlag, GlProfile, InitFlags, SDL, SdlGlAttr};

fn main() {
    let sdl = SDL::init(InitFlags::Everything);
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
    {
        sdl
            .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
            .unwrap();
    }
}

// Language: rust
// Path: DNAmaster10 | main.rs