import Foundation
var p = Process()
p.launchPath = "friendly-fire"
var pipe = Pipe()
p.standardOutput = pipe
p.launch()
p.waitUntilExit()
let data = pipe.fileHandleForReading.readDataToEndOfFile()
let got = String(bytes: data, encoding: .utf8) ?? "<nil>"
print("Read: `\(got)`")
//let task = Process.launchedProcess(launchPath: "friendly-fire", arguments: ["-la"])
//if let out = task.standardOutput {
//  out.
//}
//task.waitUntilExit() // waits for call to complete

