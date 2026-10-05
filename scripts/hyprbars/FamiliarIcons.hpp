#pragma once
#include <cairo.h>
#include <string_view>

// Paths, not font glyphs: consistent stroke weights at every display scale.
// The caller supplies the foreground colour and a transparent square surface.
inline bool drawFamiliarIcon(cairo_t* cr, std::string_view icon, double size) {
    const bool mac = icon.starts_with("familiar-mac-");
    if (!mac && !icon.starts_with("familiar-windows-")) return false;
    const auto action = icon.substr(mac ? 13 : 17);
    cairo_save(cr);
    cairo_scale(cr, size, size);
    cairo_set_line_width(cr, 0.075);
    cairo_set_line_cap(cr, CAIRO_LINE_CAP_SQUARE);
    cairo_set_line_join(cr, CAIRO_LINE_JOIN_MITER);
    if (action == "close") {
        cairo_move_to(cr, 0.32, 0.32); cairo_line_to(cr, 0.68, 0.68);
        cairo_move_to(cr, 0.68, 0.32); cairo_line_to(cr, 0.32, 0.68);
    } else if (action == "minimize") {
        cairo_move_to(cr, 0.30, mac ? 0.50 : 0.67);
        cairo_line_to(cr, 0.70, mac ? 0.50 : 0.67);
    } else if (action == "maximize" && mac) {
        // Two outward arrows, matching the expand action rather than a plus.
        cairo_move_to(cr, 0.34, 0.55); cairo_line_to(cr, 0.34, 0.66); cairo_line_to(cr, 0.45, 0.66);
        cairo_move_to(cr, 0.34, 0.66); cairo_line_to(cr, 0.46, 0.54);
        cairo_move_to(cr, 0.55, 0.34); cairo_line_to(cr, 0.66, 0.34); cairo_line_to(cr, 0.66, 0.45);
        cairo_move_to(cr, 0.66, 0.34); cairo_line_to(cr, 0.54, 0.46);
    } else if (action == "maximize") {
        cairo_rectangle(cr, 0.31, 0.31, 0.38, 0.38);
    } else {
        cairo_restore(cr);
        return false;
    }
    cairo_stroke(cr);
    cairo_restore(cr);
    return true;
}
