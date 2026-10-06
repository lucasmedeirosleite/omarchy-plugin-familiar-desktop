.pragma library

// Smooth trackpads emit many tiny deltas; accumulate a gesture before cycling.
function step(previous, pixelX, pixelY, angleX, angleY) {
    var pixels = pixelX !== 0 || pixelY !== 0
    var x = pixels ? pixelX : angleX
    var y = pixels ? pixelY : angleY
    var delta = Math.abs(x) > Math.abs(y) ? x : -y
    var threshold = pixels ? 40 : 120
    if (!delta) return {pending: previous, direction: 0}
    if (previous * delta < 0) previous = 0
    var total = previous + delta
    if (Math.abs(total) < threshold) return {pending: total, direction: 0}
    return {pending: 0, direction: total > 0 ? 1 : -1}
}
