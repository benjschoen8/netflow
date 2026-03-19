import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'screens/dashboard/dashboard_screen.dart';
import 'theme/app_theme.dart';

void main() {
  runApp(
    const ProviderScope(
      child: NetflowApp(),
    ),
  );
}

class NetflowApp extends StatelessWidget {
  const NetflowApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'netflow',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
      themeMode: ThemeMode.system,
      home: const DashboardScreen(),
    );
  }
}
