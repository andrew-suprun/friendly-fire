import 'dart:io';
import 'dart:convert';

main() async {
  var process = await Process.start('dart', ['bin/friendly-fire.dart']);
  await for (final data in process.stdout.transform(UTF8.decoder)) {
    print(data);
  }
  final exitCode = await process.exitCode;
  print('exit code: $exitCode');
}
