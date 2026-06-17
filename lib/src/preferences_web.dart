import 'package:web/web.dart' as web;

/// No-op stub so callers can use RustLib.init() on all platforms.
class RustLib {
  static Future<void> init() async {}
}

String _prefix = '';

Future<void> init({required String appId, required String author}) async {
  _prefix = '${appId}__${author}__';
}

Future<void> setString({required String key, required String value}) async {
  web.window.localStorage.setItem('$_prefix$key', value);
}

Future<String?> getString({required String key}) async {
  return web.window.localStorage.getItem('$_prefix$key');
}
