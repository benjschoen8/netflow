import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../shell/app_shell.dart';

class OnboardingScreen extends ConsumerStatefulWidget {
  const OnboardingScreen({super.key});

  @override
  ConsumerState<OnboardingScreen> createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends ConsumerState<OnboardingScreen> {
  bool _loading = false;
  String? _error;

  Future<void> _init() async {
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).init();

      // Refresh bootstrap so the router re-evaluates
      ref.invalidate(bootstrapProvider);

      if (!mounted) return;
      Navigator.of(context).pushReplacement(
        MaterialPageRoute(builder: (_) => const AppShell()),
      );
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final tt = Theme.of(context).textTheme;

    return Scaffold(
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 32),
          child: Column(
            children: [
              const Spacer(flex: 2),

              // ── Logo / icon ───────────────────────────────────────────────
              Container(
                width: 88,
                height: 88,
                decoration: BoxDecoration(
                  color: cs.primaryContainer,
                  borderRadius: BorderRadius.circular(24),
                ),
                child: Icon(
                  Icons.account_balance_wallet_rounded,
                  size: 48,
                  color: cs.onPrimaryContainer,
                ),
              ),
              const SizedBox(height: 24),

              // ── Headline ──────────────────────────────────────────────────
              Text(
                'Welcome to netflow',
                style: tt.headlineMedium?.copyWith(
                  fontWeight: FontWeight.w700,
                ),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 12),
              Text(
                'Your personal finance ledger.\nTrack accounts, debts, investments\nand your net worth in one place.',
                style: tt.bodyMedium?.copyWith(color: cs.outline),
                textAlign: TextAlign.center,
              ),

              const Spacer(flex: 2),

              // ── Feature list ──────────────────────────────────────────────
              _FeatureRow(
                icon: Icons.account_balance_rounded,
                label: 'Bank accounts, wallets & credit cards',
              ),
              const SizedBox(height: 12),
              _FeatureRow(
                icon: Icons.show_chart_rounded,
                label: 'Investments & holdings',
              ),
              const SizedBox(height: 12),
              _FeatureRow(
                icon: Icons.request_quote_rounded,
                label: 'Loans & debt tracking',
              ),
              const SizedBox(height: 12),
              _FeatureRow(
                icon: Icons.pie_chart_rounded,
                label: 'Net worth across all currencies',
              ),

              const Spacer(flex: 3),

              // ── Error message ─────────────────────────────────────────────
              if (_error != null) ...[
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: cs.errorContainer,
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Row(
                    children: [
                      Icon(Icons.error_outline,
                          color: cs.onErrorContainer, size: 18),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          _error!,
                          style: tt.bodySmall
                              ?.copyWith(color: cs.onErrorContainer),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 16),
              ],

              // ── CTA button ────────────────────────────────────────────────
              SizedBox(
                width: double.infinity,
                height: 52,
                child: FilledButton(
                  onPressed: _loading ? null : _init,
                  child: _loading
                      ? const SizedBox(
                          width: 22,
                          height: 22,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Text(
                          'Get Started',
                          style: TextStyle(
                              fontSize: 16, fontWeight: FontWeight.w600),
                        ),
                ),
              ),
              const SizedBox(height: 32),
            ],
          ),
        ),
      ),
    );
  }
}

class _FeatureRow extends StatelessWidget {
  const _FeatureRow({required this.icon, required this.label});
  final IconData icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Row(
      children: [
        Container(
          width: 36,
          height: 36,
          decoration: BoxDecoration(
            color: cs.secondaryContainer,
            borderRadius: BorderRadius.circular(10),
          ),
          child: Icon(icon, size: 18, color: cs.onSecondaryContainer),
        ),
        const SizedBox(width: 12),
        Text(label, style: Theme.of(context).textTheme.bodyMedium),
      ],
    );
  }
}
