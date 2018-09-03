//! The accurate, low resource renderer.
//! 
//! # Motivation
//! API's like Vulkan, Metal and OpenGL have trouble with certain things like anti-aliasing (which  gets "solved" with Multisampling), and how to make a perfect sphere (It's not really possible).
//! 
//! This renderer will have a lot of CPU-based function calls, so a GPU isn't necessary - but if you have vulkan installed, compute shaders will be used to speed things up.  In order to get *even faster* SIMD will be used!  Hopefully, SIMD + Vulkan Compute for CPU rendering will be just as fast as normal GPU rendering (or at least close).
//! 
//! Having support for 3D curves, will not only make certain things (particulary living things, like plants and people) look better, but also reduce the amount of vertices that need to be stored in memory to draw models with curves.
//! 
//! Another factor that will be improved is color blending and worrying how to do that and have it look accurate.  HSV relative to linear sRGB will be interpolated for blending effects.
//! 
//! # Naming
//! The name is a combination of "bar" as in "foo bar baz qux" and "g" as in "graphics".  It is also fun to yell because "BARG!!!" sounds like "ARGH!!!!".
//! 
//! # More ideas
//! Rendering is done face-by-face.  So a cube will be 6 draw calls (1 for each face), but of course at least 3 faces will be culled.  A sphere will need 4 draw calls / 4 faces, these will be 3D faces unlike the cube.
//! 
//! Alpha blending will be done backwards.  No matter what faces that are closest to the camera will always be drawn first.  The alpha value will be stored on the surface we're rendering to.  If it's 255 then pixels will be culled.  If it's less faces will be blended behind the face currently in the render buffer.

extern crate afi;
extern crate fonterator;
extern crate ami;
extern crate arrayvec;
extern crate ordered_float;
#[macro_use] extern crate approx;

mod draw;

/// The default font.
pub const FONT: &'static [u8] = include_bytes!("../font/LiberationMono-Regular.ttf");
pub use fonterator::Font;

use afi::{ColorChannels::*, VFrame};
pub use afi::{PathOp};
pub use PathOp::{Move, Line, Quad};

/// Size of an image (width, height).
#[derive(Copy, Clone)] pub struct Size(pub u16, pub u16);

/// Texture Coordinates (Mapped to a `PathOp`).
#[derive(Copy, Clone)]
pub struct TexCoord(pub f32, pub f32);

/// An HSV Surface.
pub struct Surface {
	#[allow(unused)] // TODO
	size: Size,
	// Linear HSV (TODO: stop color precision loss by u8 -> u16).
	pixels: VFrame, // Vec<[u16; 4]>,
}

impl Surface {
	/// Create a new HSV Surface.
	pub fn new(size: Size) -> Surface {
		Surface {
			size, pixels: VFrame(vec![
				0; size.0 as usize * size.1 as usize * 4
			]),
		}
	}

	/// Clear the Surface one color (sRGBA).
	pub fn clear(&mut self, color: [u8; 4]) {
		let mut index = 0;
		let size = self.size.0 as usize * self.size.1 as usize;
		let color = Lhsva.from(Srgba, color);
		for _ in 0..size {
			self.pixels.0[index + 0] = color[0];
			self.pixels.0[index + 1] = color[1];
			self.pixels.0[index + 2] = color[2];
			self.pixels.0[index + 3] = color[3];

			index += 4;
		}
	}

	/// Draw a path a solid color (sRGBA).
	#[allow(unused)] // TODO
	pub fn draw<T>(&mut self, color: [u8; 4], path: T)
		where T: IntoIterator<Item=PathOp>
	{
		let iter = path.into_iter();

		draw::draw(&mut self.pixels, self.size, iter, color);
	}

	/// Read surface pixels (sRGBA).
	pub fn srgba(&self, pixels: &mut [u8]) {
		let mut index = 0;
		let size = self.size.0 as usize * self.size.1 as usize;
		for _ in 0..size {
			let h = self.pixels.0[index + 0];
			let s = self.pixels.0[index + 1];
			let v = self.pixels.0[index + 2];
			let a = self.pixels.0[index + 3];
			let [r, g, b, a] = Srgba.from(Lhsva, [h,s,v,a]);

			pixels[index + 0] = r;
			pixels[index + 1] = g;
			pixels[index + 2] = b;
			pixels[index + 3] = a;

			index += 4;
		}
	}
}

/// A Graphical User Interface.
pub struct Gui {
	
}

impl Gui {
	pub fn new() -> Gui {
		Gui {
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
