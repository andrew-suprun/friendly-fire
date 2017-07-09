import 'dart:io';
import 'dart:convert';

main() async {
  var process = await Process.start('dart', ['bin/friendly-fire.dart']);
  process.stdout
      .transform(UTF8.decoder)
      .listen((data) { print(data); });
  process.exitCode.then((exitCode) {
    print('exit code: $exitCode');
  });
}
