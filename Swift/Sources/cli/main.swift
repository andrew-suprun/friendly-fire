import Foundation

var pipe = Pipe()
var p = Process()
p.launchPath = "friendly-fire"
p.standardOutput = pipe
p.launch()
p.waitUntilExit()
let data = pipe.fileHandleForReading.readDataToEndOfFile()
let got = String(bytes: data, encoding: .utf8) ?? "<nil>"
print("Got: `\(got)`")

