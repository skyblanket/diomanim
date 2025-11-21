//! Glyph Rasterization Test
//!
//! Tests the glyph rasterization system before full integration.

use diomanim::text::rasterizer::GlyphAtlas;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Glyph Rasterization Test                                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Create glyph atlas
    println!("Creating glyph atlas...");
    let mut atlas = GlyphAtlas::from_system_font(48.0)?;
    println!("✓ Atlas created (1024x1024)");

    // Test text
    let test_text = "Hello, Diomanim!";
    println!("\nRasterizing text: \"{}\"", test_text);

    // Rasterize all characters
    atlas.rasterize_string(test_text)?;
    println!("✓ Rasterized {} characters", test_text.len());

    // Measure text width
    let width = atlas.measure_text(test_text)?;
    println!("✓ Text width: {:.2} pixels", width);

    // Check individual glyphs
    println!("\nGlyph details:");
    for c in test_text.chars().take(10) {
        if let Some(glyph) = atlas.get_glyph(c) {
            println!(
                "  '{}': {}x{} pixels, advance: {:.2}, UV: ({:.3}, {:.3}, {:.3}, {:.3})",
                c,
                glyph.width,
                glyph.height,
                glyph.advance,
                glyph.uv.0,
                glyph.uv.1,
                glyph.uv.2,
                glyph.uv.3
            );
        }
    }

    // Atlas statistics
    let (width, height) = atlas.atlas_dimensions();
    println!("\n✓ Atlas dimensions: {}x{}", width, height);
    println!("✓ Atlas data size: {} bytes", atlas.atlas_data().len());

    println!("\n✅ Glyph rasterization system working!");
    println!("Ready for GPU integration.\n");

    Ok(())
}
