import 'dart:convert';
import 'package:http/http.dart' as http;
import '../config.dart';

/// Thin wrapper around [http.Client] that:
///   - prefixes all paths with [AppConfig.baseUrl]
///   - encodes/decodes JSON
///   - converts non-2xx responses into [ApiException]
class ApiClient {
  ApiClient({http.Client? client}) : _client = client ?? http.Client();

  final http.Client _client;
  final String _base = AppConfig.baseUrl;

  // ── GET ───────────────────────────────────────────────────────────────────

  Future<dynamic> get(String path, {Map<String, String>? query}) async {
    final uri = _uri(path, query);
    final res = await _client.get(uri, headers: _headers());
    return _decode(res);
  }

  // ── POST ──────────────────────────────────────────────────────────────────

  Future<dynamic> post(String path, [Object? body]) async {
    final res = await _client.post(
      _uri(path),
      headers: _headers(),
      body: body != null ? jsonEncode(body) : null,
    );
    return _decode(res);
  }

  // ── PATCH ─────────────────────────────────────────────────────────────────

  Future<dynamic> patch(String path, [Object? body]) async {
    final res = await _client.patch(
      _uri(path),
      headers: _headers(),
      body: body != null ? jsonEncode(body) : null,
    );
    return _decode(res);
  }

  // ── DELETE ────────────────────────────────────────────────────────────────

  Future<void> delete(String path) async {
    final res = await _client.delete(_uri(path), headers: _headers());
    _decode(res);
  }

  // ── Helpers ───────────────────────────────────────────────────────────────

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

  @override
  String toString() => 'ApiException($statusCode): $message';
}
