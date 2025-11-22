#!/bin/bash
# Render all examples and save to docs/images

set -e

mkdir -p docs/images

echo "════════════════════════════════════════════════════════════════"
echo "  Rendering Diomanim Examples to docs/images"
echo "════════════════════════════════════════════════════════════════"
echo ""

# Build all examples
echo "Building examples..."
cargo build --release --examples 2>&1 | grep -E "(Compiling|Finished)" | tail -5

echo ""
echo "Rendering examples..."
echo ""

# Example 1: All Shapes Demo
echo "1. all_shapes_demo → docs/images/all_shapes.png"
timeout 30 ./target/release/examples/all_shapes_demo 2>/dev/null || true
if [ -f "output/all_shapes.png" ]; then
    cp output/all_shapes.png docs/images/all_shapes.png
    echo "   ✓ Saved ($(du -h docs/images/all_shapes.png | cut -f1))"
fi

# Example 2: Simple Circle
echo "2. simple_circle → docs/images/simple_circle.png"
timeout 30 ./target/release/examples/simple_circle 2>/dev/null || true
if [ -f "output/simple_circle.png" ]; then
    cp output/simple_circle.png docs/images/simple_circle.png
    echo "   ✓ Saved ($(du -h docs/images/simple_circle.png | cut -f1))"
fi

# Example 3: Builder API Demo
echo "3. builder_api_demo → docs/images/builder_demo.png"
timeout 30 ./target/release/examples/builder_api_demo 2>/dev/null || true
if [ -f "output/builder_demo.png" ]; then
    cp output/builder_demo.png docs/images/builder_demo.png
    echo "   ✓ Saved ($(du -h docs/images/builder_demo.png | cut -f1))"
fi

# Example 4: Math Demo
echo "4. math_demo → docs/images/math_demo.png"
timeout 30 ./target/release/examples/math_demo 2>/dev/null || true
if [ -f "output/math_demo.png" ]; then
    cp output/math_demo.png docs/images/math_demo.png
    echo "   ✓ Saved ($(du -h docs/images/math_demo.png | cut -f1))"
fi

# Example 5: Text Demo
echo "5. text_demo → docs/images/text_demo.png"
timeout 30 ./target/release/examples/text_demo 2>/dev/null || true
if [ -f "output/text_demo.png" ]; then
    cp output/text_demo.png docs/images/text_demo.png
    echo "   ✓ Saved ($(du -h docs/images/text_demo.png | cut -f1))"
fi

# Example 6: Phase 2 Demo
echo "6. phase2_demo → docs/images/phase2_demo.png"
timeout 30 ./target/release/examples/phase2_demo 2>/dev/null || true
if [ -f "output/phase2_demo.png" ]; then
    cp output/phase2_demo.png docs/images/phase2_demo.png
    echo "   ✓ Saved ($(du -h docs/images/phase2_demo.png | cut -f1))"
fi

# Example 7: Showcase
echo "7. showcase → docs/images/showcase.png"
timeout 30 ./target/release/examples/showcase 2>/dev/null || true
if [ -f "output/showcase.png" ]; then
    cp output/showcase.png docs/images/showcase.png
    echo "   ✓ Saved ($(du -h docs/images/showcase.png | cut -f1))"
fi

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "  Rendering Complete!"
echo "════════════════════════════════════════════════════════════════"
echo ""
ls -lh docs/images/*.png 2>/dev/null | awk '{print $9, "("$5")"}'
