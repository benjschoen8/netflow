import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/config.dart';
import '../../core/providers.dart';
import '../../theme/app_theme.dart';

class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      backgroundColor: AppTheme.pageBg,
      body: CustomScrollView(
        slivers: [
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(24, 24, 24, 0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Settings',
                      style: Theme.of(context)
                          .textTheme
                          .headlineMedium
                          ?.copyWith(fontWeight: FontWeight.w700)),
                  const SizedBox(height: 4),
                  const Text('Manage your app configuration',
                      style: TextStyle(
                          color: AppTheme.textSecondary, fontSize: 14)),
                ],
              ),
            ),
          ),
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _AppearanceSection(),
                  const SizedBox(height: 20),
                  _SettingsSection(title: 'Connection', children: [
                    _InfoRow(
                        icon: Icons.dns_rounded,
                        label: 'Backend URL',
                        value: AppConfig.baseUrl),
                    const _InfoRow(
                        icon: Icons.code_rounded,
                        label: 'Change in',
                        value: 'lib/core/config.dart'),
                  ]),
                  const SizedBox(height: 20),
                  _SettingsSection(title: 'About', children: [
                    const _InfoRow(
                        icon: Icons.info_outline_rounded,
                        label: 'App',
                        value: 'netflow'),
                    const _InfoRow(
                        icon: Icons.layers_rounded,
                        label: 'Stack',
                        value: 'Rust · Flutter · SQLite'),
                    const _InfoRow(
                        icon: Icons.palette_outlined,
                        label: 'Design',
                        value: 'netflow dark theme'),
                  ]),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _SettingsSection extends StatelessWidget {
  const _SettingsSection(
      {required this.title, required this.children});
  final String        title;
  final List<Widget>  children;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(title.toUpperCase(),
            style: const TextStyle(
                color: AppTheme.purple,
                fontSize: 10,
                fontWeight: FontWeight.w700,
                letterSpacing: 1.2)),
        const SizedBox(height: 10),
        Container(
          decoration: BoxDecoration(
            color: AppTheme.surface,
            borderRadius: BorderRadius.circular(14),
            border: Border.all(color: AppTheme.border),
          ),
          child: Column(
            children: children
                .asMap()
                .entries
                .map((e) => Column(children: [
                      e.value,
                      if (e.key < children.length - 1)
                        const Divider(
                            height: 1, indent: 48, color: AppTheme.border),
                    ]))
                .toList(),
          ),
        ),
      ],
    );
  }
}

class _InfoRow extends StatelessWidget {
  const _InfoRow({
    required this.icon,
    required this.label,
    required this.value,
  });
  final IconData icon;
  final String   label;
  final String   value;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding:
          const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      child: Row(
        children: [
          Icon(icon, size: 18, color: AppTheme.textSecondary),
          const SizedBox(width: 12),
          Text(label,
              style: const TextStyle(
                  color: AppTheme.textSecondary, fontSize: 13)),
          const Spacer(),
          Flexible(
            child: Text(value,
                textAlign: TextAlign.right,
                style: const TextStyle(
                    color: AppTheme.textPrimary,
                    fontSize: 13,
                    fontWeight: FontWeight.w500)),
          ),
        ],
      ),
    );
  }
}

// ── Appearance section ────────────────────────────────────────────────────────

class _AppearanceSection extends ConsumerWidget {
  const _AppearanceSection();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final current = ref.watch(fontSizeProvider);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('APPEARANCE',
            style: TextStyle(
                color: AppTheme.purple,
                fontSize: 10,
                fontWeight: FontWeight.w700,
                letterSpacing: 1.2)),
        const SizedBox(height: 10),
        Container(
          decoration: BoxDecoration(
            color: AppTheme.surface,
            borderRadius: BorderRadius.circular(14),
            border: Border.all(color: AppTheme.border),
          ),
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
            child: Row(
              children: [
                const Icon(Icons.text_fields_rounded,
                    size: 18, color: AppTheme.textSecondary),
                const SizedBox(width: 12),
                const Text('Font Size',
                    style: TextStyle(
                        color: AppTheme.textSecondary, fontSize: 13)),
                const Spacer(),
                _FontSizePicker(current: current),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _FontSizePicker extends ConsumerWidget {
  const _FontSizePicker({required this.current});
  final AppFontSize current;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Container(
      decoration: BoxDecoration(
        color: AppTheme.surface2,
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: AppTheme.border),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          _SizeOption(
            label: 'S',
            fontSize: 11,
            selected: current == AppFontSize.small,
            onTap: () => ref.read(fontSizeProvider.notifier).set(AppFontSize.small),
            isFirst: true,
          ),
          Container(width: 1, height: 28, color: AppTheme.border),
          _SizeOption(
            label: 'M',
            fontSize: 13,
            selected: current == AppFontSize.medium,
            onTap: () => ref.read(fontSizeProvider.notifier).set(AppFontSize.medium),
          ),
          Container(width: 1, height: 28, color: AppTheme.border),
          _SizeOption(
            label: 'L',
            fontSize: 15,
            selected: current == AppFontSize.large,
            onTap: () => ref.read(fontSizeProvider.notifier).set(AppFontSize.large),
            isLast: true,
          ),
        ],
      ),
    );
  }
}

class _SizeOption extends StatelessWidget {
  const _SizeOption({
    required this.label,
    required this.fontSize,
    required this.selected,
    required this.onTap,
    this.isFirst = false,
    this.isLast  = false,
  });
  final String label;
  final double fontSize;
  final bool   selected;
  final VoidCallback onTap;
  final bool   isFirst;
  final bool   isLast;

  @override
  Widget build(BuildContext context) {
    final radius = BorderRadius.horizontal(
      left:  isFirst ? const Radius.circular(9) : Radius.zero,
      right: isLast  ? const Radius.circular(9) : Radius.zero,
    );
    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        width: 44,
        height: 34,
        decoration: BoxDecoration(
          color: selected ? AppTheme.purple.withOpacity(0.2) : Colors.transparent,
          borderRadius: radius,
        ),
        child: Center(
          child: Text(
            label,
            style: TextStyle(
              fontSize: fontSize,
              fontWeight: selected ? FontWeight.w700 : FontWeight.w400,
              color: selected ? AppTheme.purpleLight : AppTheme.textSecondary,
            ),
          ),
        ),
      ),
    );
  }
}
