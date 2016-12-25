use sdl2::render::TextureValueError;
use sdl2::ttf::FontError;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SDL2Error {
    Texture(TextureValueError),
    Font(FontError),
    // rust-sdl2 uses string to represent the errors
    // in the drawing methods
    Draw(String),
}

impl fmt::Display for SDL2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            SDL2Error::Texture(ref err) => write!(f, "Texture error: {}", err),
            SDL2Error::Font(ref err) => write!(f, "Font error: {}", err),
            SDL2Error::Draw(ref err) => write!(f, "Draw error: {}", err),
        }
    }
}

impl Error for SDL2Error {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            SDL2Error::Texture(ref err) => err.description(),
            SDL2Error::Font(ref err) => err.description(),
            SDL2Error::Draw(ref err) => &err,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            SDL2Error::Texture(ref err) => Some(err),
            SDL2Error::Font(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<String> for SDL2Error {
    fn from(err: String) -> SDL2Error {
        SDL2Error::Draw(err)
    }
}

impl From<TextureValueError> for SDL2Error {
    fn from(err: TextureValueError) -> SDL2Error {
        SDL2Error::Texture(err)
    }
}

impl From<FontError> for SDL2Error {
    fn from(err: FontError) -> SDL2Error {
        SDL2Error::Font(err)
    }
}