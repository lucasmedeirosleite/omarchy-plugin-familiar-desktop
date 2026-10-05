#include "../scripts/hyprbars/FamiliarIcons.hpp"
#include <cassert>
#include <cstdint>
#include <string>

int main() {
    for (int size : {18, 27, 36, 64}) {
        for (const auto* style : {"mac", "windows"}) {
            for (const auto* action : {"close", "minimize", "maximize"}) {
                auto* surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, size, size);
                auto* cr = cairo_create(surface);
                cairo_set_source_rgba(cr, 0, 0, 0, 1);
                assert(drawFamiliarIcon(cr, std::string("familiar-") + style + "-" + action, size));
                cairo_surface_flush(surface);
                auto* bytes = cairo_image_surface_get_data(surface);
                int stride = cairo_image_surface_get_stride(surface);
                int painted = 0;
                for (int y = 0; y < size; ++y) for (int x = 0; x < size; ++x) {
                    auto pixel = reinterpret_cast<uint32_t*>(bytes + y * stride)[x];
                    if (pixel >> 24) {
                        ++painted;
                        assert(x > 0 && y > 0 && x < size-1 && y < size-1);
                    }
                }
                assert(painted > size / 3 && painted < size * size / 2);
                cairo_destroy(cr);
                cairo_surface_destroy(surface);
            }
        }
    }
    auto* surface = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 18, 18);
    auto* cr = cairo_create(surface);
    assert(!drawFamiliarIcon(cr, "unknown", 18));
    cairo_destroy(cr);
    cairo_surface_destroy(surface);
}
