import 'package:flutter/material.dart';
import 'package:flutter_preferences/flutter_preferences.dart' as flutter_preferences;

Future<void> main() async {
  await flutter_preferences.RustLib.init();
  await flutter_preferences.init(appId: "my amazing app", author: "arthur the great");

  await flutter_preferences.setString(key: "foo", value: "bar");

  final val = (await flutter_preferences.getString(key: "foo"))!;

  runApp(MyApp(val));
}


class MyApp extends StatelessWidget {

  final String val;
  const MyApp(this.val, {super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'res: `$val`'),
        ),
      ),

    );
  }
}
