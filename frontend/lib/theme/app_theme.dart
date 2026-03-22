import 'package:flutter/material.dart';

class AppTheme {
  AppTheme._();

  // ── Exact tokens from the source components ──────────────────────────────
  static const pageBg   = Color(0xFF0A0A0E); // page/scaffold
  static const cardBg   = Color(0xFF0E0E14); // card surface
  static const surface  = Color(0xFF13131F); // sidebar / elevated
  static const surface2 = Color(0xFF1A1A2E); // inputs

  static const purple      = Color(0xFF8B5CF6); // primary accent (button bg)
  static const purpleDark  = Color(0xFF7C3AED); // hover
  static const purpleLight = Color(0xFFA78BFA); // secondary text/outline buttons
  static const purpleValue = Color(0xFFC7B8FF); // big value numbers

  static const border      = Color(0xFF2A2A40);
  // Purple glow border (rgba(168,85,247,0.25)):
  static const purpleBorder = Color(0x40A855F7);

  static const textPrimary   = Color(0xFFE7E7F0);
  static const textSecondary = Color(0xFF9CA3AF);
  static const textTertiary  = Color(0xFF6B7280);

  static const green  = Color(0xFF2AF07A); // positive delta
  static const red    = Color(0xFFFF5C8A); // negative delta
  static const cyan   = Color(0xFF22D3EE); // secondary chart line
  static const amber  = Color(0xFFFBBF24);

  // ── Chart colours ────────────────────────────────────────────────────────
  static const chartPurple = Color(0xFF8B5CF6);
  static const chartCyan   = Color(0xFF22D3EE);
  static const chartGrid   = Color(0x1AA78BFA); // rgba(167,139,250,0.10)

  // ── Purple glow card decoration (matches component box-shadow) ───────────
  static BoxDecoration glowCard({
    Color? color,
    double radius = 16,
  }) =>
      BoxDecoration(
        color: color ?? cardBg,
        borderRadius: BorderRadius.circular(radius),
        border: Border.all(color: purpleBorder, width: 1),
        boxShadow: const [
          BoxShadow(
            color: Color(0x1AA855F7), // rgba(168,85,247,0.10)
            blurRadius: 28,
            spreadRadius: 0,
          ),
        ],
      );

  static ThemeData get dark => ThemeData(
        useMaterial3: true,
        brightness: Brightness.dark,
        scaffoldBackgroundColor: pageBg,
        colorScheme: const ColorScheme.dark(
          primary:               purple,
          onPrimary:             Colors.white,
          secondary:             purpleLight,
          onSecondary:           Colors.white,
          surface:               cardBg,
          onSurface:             textPrimary,
          surfaceContainerLow:         cardBg,
          surfaceContainerHighest:     surface2,
          outline:               textSecondary,
          error:                 red,
          onError:               Colors.white,
          errorContainer:        Color(0xFF3B1A1A),
          onErrorContainer:      red,
          primaryContainer:      Color(0x338B5CF6),
          onPrimaryContainer:    purpleLight,
          secondaryContainer:    Color(0x1A8B5CF6),
          onSecondaryContainer:  purpleLight,
          tertiaryContainer:     Color(0x1A22D3EE),
          onTertiaryContainer:   cyan,
        ),
        cardTheme: CardThemeData(
          elevation: 0,
          color: cardBg,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.all(Radius.circular(16)),
            side: BorderSide(color: purpleBorder, width: 1),
          ),
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: pageBg,
          surfaceTintColor: Colors.transparent,
          foregroundColor: textPrimary,
          centerTitle: false,
          elevation: 0,
          scrolledUnderElevation: 0,
        ),
        drawerTheme: const DrawerThemeData(backgroundColor: surface),
        tabBarTheme: const TabBarThemeData(
          labelColor: purple,
          unselectedLabelColor: textSecondary,
          indicatorColor: purple,
          dividerColor: border,
        ),
        inputDecorationTheme: InputDecorationTheme(
          filled: true,
          fillColor: surface2,
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: const BorderSide(color: border),
          ),
          enabledBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: const BorderSide(color: border),
          ),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: const BorderSide(color: purple, width: 1.5),
          ),
          labelStyle: const TextStyle(color: textSecondary),
          hintStyle: const TextStyle(color: textTertiary),
        ),
        filledButtonTheme: FilledButtonThemeData(
          style: FilledButton.styleFrom(
            backgroundColor: purple,
            foregroundColor: Colors.white,
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            textStyle: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14),
          ).copyWith(
            overlayColor: WidgetStateProperty.all(purpleDark.withOpacity(0.3)),
          ),
        ),
        outlinedButtonTheme: OutlinedButtonThemeData(
          style: OutlinedButton.styleFrom(
            foregroundColor: purpleLight,
            side: const BorderSide(color: purpleBorder),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            textStyle: const TextStyle(fontWeight: FontWeight.w500, fontSize: 14),
          ),
        ),
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(
            foregroundColor: purpleLight,
            textStyle: const TextStyle(fontWeight: FontWeight.w500),
          ),
        ),
        dividerTheme: const DividerThemeData(color: border, space: 1),
        listTileTheme: const ListTileThemeData(
          tileColor: Colors.transparent,
          textColor: textPrimary,
          iconColor: textSecondary,
        ),
        snackBarTheme: SnackBarThemeData(
          backgroundColor: surface,
          contentTextStyle: const TextStyle(color: textPrimary),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
          behavior: SnackBarBehavior.floating,
        ),
        textTheme: const TextTheme(
          displayLarge:   TextStyle(color: textPrimary,   fontWeight: FontWeight.w700),
          displayMedium:  TextStyle(color: textPrimary,   fontWeight: FontWeight.w700),
          displaySmall:   TextStyle(color: textPrimary,   fontWeight: FontWeight.w700),
          headlineLarge:  TextStyle(color: textPrimary,   fontWeight: FontWeight.w700),
          headlineMedium: TextStyle(color: textPrimary,   fontWeight: FontWeight.w600),
          headlineSmall:  TextStyle(color: textPrimary,   fontWeight: FontWeight.w600),
          titleLarge:     TextStyle(color: textPrimary,   fontWeight: FontWeight.w600),
          titleMedium:    TextStyle(color: textPrimary,   fontWeight: FontWeight.w600),
          titleSmall:     TextStyle(color: textPrimary,   fontWeight: FontWeight.w500),
          bodyLarge:      TextStyle(color: textPrimary),
          bodyMedium:     TextStyle(color: textPrimary),
          bodySmall:      TextStyle(color: textSecondary),
          labelLarge:     TextStyle(color: textPrimary,   fontWeight: FontWeight.w600),
          labelMedium:    TextStyle(color: textSecondary, fontWeight: FontWeight.w500),
          labelSmall:     TextStyle(color: textSecondary),
        ),
      );

  static ThemeData get light => dark;

  static Color assetColor(BuildContext context) => green;
  static Color debtColor(BuildContext context)  => red;
  static Color neutralColor(BuildContext context) => textSecondary;
}
