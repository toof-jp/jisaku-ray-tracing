use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(
            writer,
            "{} {} {}\n",
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32
        )
    }
}
