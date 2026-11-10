import AppKit
import Foundation
let root = CommandLine.arguments[1]
let data = try Data(contentsOf: URL(fileURLWithPath: root + "/summary.json"))
let rows = try JSONSerialization.jsonObject(with: data) as! [[String: Any]]
let image = NSImage(size: NSSize(width: 1200, height: 690))
image.lockFocus()
NSColor.white.setFill()
NSRect(x: 0, y: 0, width: 1200, height: 690).fill()
func text(_ value: String, _ x: Double, _ y: Double, _ size: Double) {
    (value as NSString).draw(at: NSPoint(x: x, y: y), withAttributes: [.font: NSFont.systemFont(ofSize: size), .foregroundColor: NSColor.black])
}
text("Extractor operation rate: change vs baseline", 30, 640, 27)
text("Eight alternating pairs; bars show geometric mean; whiskers show 95% paired bootstrap intervals", 30, 605, 17)
let zero = 530.0
let scale = 6.0
NSColor.lightGray.setFill()
NSRect(x: zero, y: 135, width: 1, height: 445).fill()
for (index, row) in rows.enumerated() {
    let y = 550.0 - Double(index) * 48
    let gain = row["speed_gain_percent"] as! Double
    let low = row["low"] as! Double
    let high = row["high"] as! Double
    text(row["name"] as! String, 30, y, 19)
    (gain > 0 ? NSColor.systemTeal : NSColor.systemOrange).setFill()
    NSRect(x: zero + min(gain, 0) * scale, y: y, width: abs(gain) * scale, height: 22).fill()
    NSColor.black.setStroke()
    let line = NSBezierPath()
    line.move(to: NSPoint(x: zero + low * scale, y: y + 11))
    line.line(to: NSPoint(x: zero + high * scale, y: y + 11))
    for bound in [low, high] {
        line.move(to: NSPoint(x: zero + bound * scale, y: y + 6))
        line.line(to: NSPoint(x: zero + bound * scale, y: y + 16))
    }
    line.lineWidth = 2
    line.stroke()
    text(String(format: "%+.1f%%", gain), 1060, y, 19)
}
text("-40%", 275, 105, 16)
text("0", zero - 4, 105, 16)
text("+40%", 750, 105, 16)
text("+80%", 990, 105, 16)
text("Higher is better. Failure rates fall because deserialization runs twice.", 30, 65, 18)
text("Query: direct parsing. JSON/form: request construction + extraction. No sockets or CodSpeed.", 30, 30, 17)
image.unlockFocus()
let bitmap = NSBitmapImageRep(data: image.tiffRepresentation!)!
try bitmap.representation(using: .png, properties: [:])!.write(to: URL(fileURLWithPath: root + "/results.png"))
