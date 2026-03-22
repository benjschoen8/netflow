import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/form_sheet.dart';

/// Bottom sheet that lets the user patch an account's display-info fields.
///
/// All fields are optional — only non-empty changed values are sent.
/// The server uses patch semantics: omitted fields remain unchanged.
class EditAccountSheet extends ConsumerStatefulWidget {
  const EditAccountSheet({super.key, required this.account});
  final AccountSummary account;

  @override
  ConsumerState<EditAccountSheet> createState() => _EditAccountSheetState();
}

class _EditAccountSheetState extends ConsumerState<EditAccountSheet> {
  final _formKey = GlobalKey<FormState>();
  late final TextEditingController _name;
  late final TextEditingController _bank;
  late final TextEditingController _accountNumber;
  bool _loading = false;
  String? _error;

  // Account types that carry a bank + account-number.
  bool get _hasBankFields =>
      const {'cash', 'investment', 'loan'}
          .contains(widget.account.accountType);

  @override
  void initState() {
    super.initState();
    _name          = TextEditingController(text: widget.account.accountName);
    _bank          = TextEditingController();
    _accountNumber = TextEditingController();
  }

  @override
  void dispose() {
    _name.dispose();
    _bank.dispose();
    _accountNumber.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;

    final newName   = _name.text.trim();
    final newBank   = _bank.text.trim();
    final newNumber = _accountNumber.text.trim();

    // Nothing changed — close silently.
    final nameChanged   = newName.isNotEmpty && newName != widget.account.accountName;
    final bankChanged   = _hasBankFields && newBank.isNotEmpty;
    final numberChanged = _hasBankFields && newNumber.isNotEmpty;

    if (!nameChanged && !bankChanged && !numberChanged) {
      Navigator.pop(context, false);
      return;
    }

    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).updateAccountInfo(
        widget.account.accountId,
        name:          nameChanged   ? newName   : null,
        bank:          bankChanged   ? newBank   : null,
        accountNumber: numberChanged ? newNumber : null,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Form(
      key: _formKey,
      child: FormSheet(
        title: 'Edit Account',
        submitLabel: 'Save changes',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          // ── Account badge ──────────────────────────────────────────────
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
            decoration: BoxDecoration(
              color: cs.primaryContainer,
              borderRadius: BorderRadius.circular(10),
            ),
            child: Row(
              children: [
                Icon(_typeIcon, size: 18, color: cs.onPrimaryContainer),
                const SizedBox(width: 10),
                Expanded(
                  child: Text(
                    widget.account.accountName,
                    style: Theme.of(context).textTheme.titleSmall?.copyWith(
                          color: cs.onPrimaryContainer,
                          fontWeight: FontWeight.w600,
                        ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                Text(
                  widget.account.typeLabel,
                  style: Theme.of(context).textTheme.labelSmall?.copyWith(
                        color: cs.onPrimaryContainer.withOpacity(0.7),
                      ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 20),

          // ── Name ──────────────────────────────────────────────────────
          TextFormField(
            controller: _name,
            autofocus: true,
            textCapitalization: TextCapitalization.words,
            decoration: const InputDecoration(
              labelText: 'Account name',
              hintText: 'e.g. Main Savings',
              prefixIcon: Icon(Icons.label_outline_rounded),
            ),
            validator: (v) {
              if (v != null && v.trim().isEmpty) {
                return 'Name cannot be empty';
              }
              return null;
            },
          ),

          // ── Bank + account number (bank-backed types only) ─────────────
          if (_hasBankFields) ...[
            const SizedBox(height: 12),
            TextFormField(
              controller: _bank,
              textCapitalization: TextCapitalization.words,
              decoration: const InputDecoration(
                labelText: 'Bank (leave blank to keep current)',
                hintText: 'e.g. CTBC Bank',
                prefixIcon: Icon(Icons.account_balance_outlined),
              ),
            ),
            const SizedBox(height: 12),
            TextFormField(
              controller: _accountNumber,
              decoration: const InputDecoration(
                labelText: 'Account number (leave blank to keep current)',
                hintText: 'e.g. 012-345-678901',
                prefixIcon: Icon(Icons.tag_rounded),
              ),
            ),
          ],

          const SizedBox(height: 8),
          Text(
            'Leave a field blank to keep its current value.',
            style: Theme.of(context).textTheme.bodySmall,
          ),
        ],
      ),
    );
  }

  IconData get _typeIcon => switch (widget.account.accountType) {
        'cash'            => Icons.account_balance_rounded,
        'physical_wallet' => Icons.wallet_rounded,
        'digital_wallet'  => Icons.contactless_rounded,
        'investment'      => Icons.show_chart_rounded,
        'credit_card'     => Icons.credit_card_rounded,
        'loan'            => Icons.request_quote_rounded,
        _                 => Icons.account_balance_wallet,
      };
}
