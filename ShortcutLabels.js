.pragma library

function normalize(value) {
    return value === "mac" ? "mac" : "standard"
}

// Only modifier tokens before the final key are renamed. Never alter key names,
// descriptions, configuration snippets or executable bindings.
function format(keys, style) {
    var parts = String(keys || "").split(" + ")
    var names = normalize(style) === "mac"
        ? {Super: "Command", Ctrl: "Control", Alt: "Option"} : {}
    for (var i = 0; i < parts.length - 1; i++)
        parts[i] = names[parts[i]] || parts[i]
    return parts.join(" + ")
}

function matches(keys, description, query, style) {
    var text = keys + " " + format(keys, style) + " " + description
    return text.toLowerCase().indexOf(String(query || "").toLowerCase()) >= 0
}
