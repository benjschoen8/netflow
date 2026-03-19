/// Central configuration. Override via environment or build flavours.
class AppConfig {
  /// Base URL of the Rust API server.
  /// Change to your machine's IP when running on a physical device.
  static const String baseUrl =
      String.fromEnvironment('API_BASE_URL', defaultValue: 'http://127.0.0.1:3000');
}
