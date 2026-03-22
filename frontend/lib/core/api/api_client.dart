import 'dart:convert';
import 'package:http/http.dart' as http;
import '../config.dart';

class ApiClient {
  ApiClient({http.Client? client}) : _client = client ?? http.Client();

  final http.Client _client;
  final String _base = AppConfig.baseUrl;

  Future<dynamic> get(String path, {Map<String, String>? query}) async {
    final res = await _client.get(_uri(path, query), headers: _headers());
    return _decode(res);
  }

  Future<dynamic> post(String path, [Object? body]) async {
    final res = await _client.post(
      _uri(path),
      headers: _headers(),
      body: body != null ? jsonEncode(body) : null,
    );
    return _decode(res);
  }

  Future<dynamic> patch(String path, [Object? body]) async {
    final res = await _client.patch(
      _uri(path),
      headers: _headers(),
      body: body != null ? jsonEncode(body) : null,
    );
    return _decode(res);
  }

  Future<void> delete(String path) async {
    final res = await _client.delete(_uri(path), headers: _headers());
    _decode(res);
  }

  Uri _uri(String path, [Map<String, String>? query]) {
    final base = Uri.parse(_base);
    return base.replace(
      path: '${base.path}$path',
      queryParameters: query,
    );
  }

  Map<String, String> _headers() => {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      };

  dynamic _decode(http.Response res) {
    if (res.statusCode >= 200 && res.statusCode < 300) {
      if (res.body.isEmpty) return null;
      return jsonDecode(res.body);
    }
    final msg = _tryErrorMessage(res.body) ?? 'HTTP ${res.statusCode}';
    throw ApiException(res.statusCode, msg);
  }

  String? _tryErrorMessage(String body) {
    try {
      final j = jsonDecode(body);
      return j is Map ? j['error'] as String? : null;
    } catch (_) {
      return null;
    }
  }
}

class ApiException implements Exception {
  final int statusCode;
  final String message;
  const ApiException(this.statusCode, this.message);

  bool get isNotFound => statusCode == 404;

  @override
  String toString() => 'ApiException($statusCode): $message';
}
