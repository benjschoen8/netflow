import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/amount_field.dart';
import '../../shared/widgets/form_sheet.dart';
import '../../shared/widgets/label_picker_field.dart';
import '../../theme/app_theme.dart';

// ── Deposit ───────────────────────────────────────────────────────────────────

class DepositSheet extends ConsumerStatefulWidget {
  const DepositSheet({super.key, required this.account});
  final AccountSummary account;
  @override ConsumerState<DepositSheet> createState() => _DepositSheetState();
}
class _DepositSheetState extends ConsumerState<DepositSheet> {
  final _formKey = GlobalKey<FormState>();
  final _amount  = TextEditingController();
  final _label   = TextEditingController();
  final _desc    = TextEditingController();
  bool _loading  = false; String? _error;

  @override void dispose() { _amount.dispose(); _label.dispose(); _desc.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).depositWithAnnotation(
        widget.account.accountId, _amount.text.trim(), widget.account.currency,
        label: _label.text.trim().isNotEmpty ? _label.text.trim() : null,
        description: _desc.text.trim().isNotEmpty ? _desc.text.trim() : null,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) { setState(() { _error = e.toString(); _loading = false; }); }
  }

  @override
  Widget build(BuildContext context) => Form(
    key: _formKey,
    child: FormSheet(
      title: 'Deposit', submitLabel: 'Deposit',
      onSubmit: _submit, isLoading: _loading, errorMessage: _error,
      children: [
        _AccountChip(account: widget.account),
        const SizedBox(height: 16),
        AmountField(controller: _amount, label: 'Amount',
            prefixText: widget.account.currency == 'TWD' ? 'NT\$ ' : '\$ ',
            autofocus: true),
        const SizedBox(height: 12),
        LabelPickerField(controller: _label),
        const SizedBox(height: 12),
        _DescriptionField(controller: _desc),
      ],
    ),
  );
}

// ── Withdraw ──────────────────────────────────────────────────────────────────

class WithdrawSheet extends ConsumerStatefulWidget {
  const WithdrawSheet({super.key, required this.account});
  final AccountSummary account;
  @override ConsumerState<WithdrawSheet> createState() => _WithdrawSheetState();
}
class _WithdrawSheetState extends ConsumerState<WithdrawSheet> {
  final _formKey = GlobalKey<FormState>();
  final _amount  = TextEditingController();
  final _label   = TextEditingController();
  final _desc    = TextEditingController();
  bool _loading  = false; String? _error;

  @override void dispose() { _amount.dispose(); _label.dispose(); _desc.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).withdrawWithAnnotation(
        widget.account.accountId, _amount.text.trim(), widget.account.currency,
        label: _label.text.trim().isNotEmpty ? _label.text.trim() : null,
        description: _desc.text.trim().isNotEmpty ? _desc.text.trim() : null,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) { setState(() { _error = e.toString(); _loading = false; }); }
  }

  @override
  Widget build(BuildContext context) => Form(
    key: _formKey,
    child: FormSheet(
      title: 'Withdraw', submitLabel: 'Withdraw',
      onSubmit: _submit, isLoading: _loading, errorMessage: _error,
      children: [
        _AccountChip(account: widget.account),
        const SizedBox(height: 4),
        _BalanceHint(balance: widget.account.balance, currency: widget.account.currency),
        const SizedBox(height: 16),
        AmountField(controller: _amount, label: 'Amount',
            prefixText: widget.account.currency == 'TWD' ? 'NT\$ ' : '\$ ',
            autofocus: true),
        const SizedBox(height: 12),
        LabelPickerField(controller: _label),
        const SizedBox(height: 12),
        _DescriptionField(controller: _desc),
      ],
    ),
  );
}

// ── Charge (credit card) ──────────────────────────────────────────────────────

class ChargeSheet extends ConsumerStatefulWidget {
  const ChargeSheet({super.key, required this.account});
  final AccountSummary account;
  @override ConsumerState<ChargeSheet> createState() => _ChargeSheetState();
}
class _ChargeSheetState extends ConsumerState<ChargeSheet> {
  final _formKey = GlobalKey<FormState>();
  final _amount  = TextEditingController();
  final _label   = TextEditingController();
  final _desc    = TextEditingController();
  bool _loading  = false; String? _error;

  @override void dispose() { _amount.dispose(); _label.dispose(); _desc.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).chargeWithAnnotation(
        widget.account.accountId, _amount.text.trim(), widget.account.currency,
        label: _label.text.trim().isNotEmpty ? _label.text.trim() : null,
        description: _desc.text.trim().isNotEmpty ? _desc.text.trim() : null,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) { setState(() { _error = e.toString(); _loading = false; }); }
  }

  @override
  Widget build(BuildContext context) => Form(
    key: _formKey,
    child: FormSheet(
      title: 'Charge', submitLabel: 'Record Charge',
      onSubmit: _submit, isLoading: _loading, errorMessage: _error,
      children: [
        _AccountChip(account: widget.account),
        const SizedBox(height: 16),
        AmountField(controller: _amount, label: 'Charge Amount',
            prefixText: widget.account.currency == 'TWD' ? 'NT\$ ' : '\$ ',
            autofocus: true),
        const SizedBox(height: 12),
        LabelPickerField(controller: _label, hint: 'e.g. Lunch, Groceries, Transport'),
        const SizedBox(height: 12),
        _DescriptionField(controller: _desc,
            hint: "e.g. McDonald's double cheeseburger + fries"),
      ],
    ),
  );
}

// ── Payment (from asset → debt) ───────────────────────────────────────────────

class PaymentSheet extends ConsumerStatefulWidget {
  const PaymentSheet({super.key, required this.debtAccount, required this.allAccounts});
  final AccountSummary debtAccount;
  final List<AccountSummary> allAccounts;
  @override ConsumerState<PaymentSheet> createState() => _PaymentSheetState();
}
class _PaymentSheetState extends ConsumerState<PaymentSheet> {
  final _formKey = GlobalKey<FormState>();
  final _amount  = TextEditingController();
  final _label   = TextEditingController();
  final _desc    = TextEditingController();
  String? _fromId;
  bool _loading  = false; String? _error;

  List<AccountSummary> get _assetAccounts =>
      widget.allAccounts.where((a) => !a.isDebt).toList();

  @override
  void initState() {
    super.initState();
    if (_assetAccounts.isNotEmpty) _fromId = _assetAccounts.first.accountId;
  }
  @override void dispose() { _amount.dispose(); _label.dispose(); _desc.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    if (_fromId == null) { setState(() { _error = 'Select a source account'; }); return; }
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).payWithAnnotation(
        debtAccountId: widget.debtAccount.accountId,
        fromAccountId: _fromId!,
        amount: _amount.text.trim(),
        currency: widget.debtAccount.currency,
        label: _label.text.trim().isNotEmpty ? _label.text.trim() : null,
        description: _desc.text.trim().isNotEmpty ? _desc.text.trim() : null,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) { setState(() { _error = e.toString(); _loading = false; }); }
  }

  @override
  Widget build(BuildContext context) => Form(
    key: _formKey,
    child: FormSheet(
      title: 'Make Payment', submitLabel: 'Pay',
      onSubmit: _submit, isLoading: _loading, errorMessage: _error,
      children: [
        _AccountChip(account: widget.debtAccount, label: 'Paying off'),
        const SizedBox(height: 16),
        DropdownButtonFormField<String>(
          value: _fromId,
          decoration: const InputDecoration(
            labelText: 'Pay from account',
            border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
            filled: true,
          ),
          items: _assetAccounts.map((a) => DropdownMenuItem(
            value: a.accountId,
            child: Text('${a.accountName} (${a.currency} ${a.balance})'),
          )).toList(),
          onChanged: (v) => setState(() => _fromId = v),
          validator: (v) => v == null ? 'Required' : null,
        ),
        const SizedBox(height: 12),
        AmountField(controller: _amount, label: 'Payment Amount',
            prefixText: widget.debtAccount.currency == 'TWD' ? 'NT\$ ' : '\$ ',
            autofocus: true),
        const SizedBox(height: 4),
        _BalanceHint(label: 'Outstanding',
            balance: widget.debtAccount.balance, currency: widget.debtAccount.currency),
        const SizedBox(height: 12),
        LabelPickerField(controller: _label),
        const SizedBox(height: 12),
        _DescriptionField(controller: _desc),
      ],
    ),
  );
}

// ── Close Statement ───────────────────────────────────────────────────────────

class CloseStatementSheet extends ConsumerStatefulWidget {
  const CloseStatementSheet({super.key, required this.account});
  final AccountSummary account;
  @override ConsumerState<CloseStatementSheet> createState() => _CloseStatementSheetState();
}
class _CloseStatementSheetState extends ConsumerState<CloseStatementSheet> {
  final _formKey    = GlobalKey<FormState>();
  final _minPayment = TextEditingController();
  bool _loading     = false; String? _error;

  @override void dispose() { _minPayment.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).closeStatement(
        accountId: widget.account.accountId,
        minimumPayment: _minPayment.text.trim().isNotEmpty ? _minPayment.text.trim() : null,
        currency: widget.account.currency,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) { setState(() { _error = e.toString(); _loading = false; }); }
  }

  @override
  Widget build(BuildContext context) => Form(
    key: _formKey,
    child: FormSheet(
      title: 'Close Statement', submitLabel: 'Close Statement',
      onSubmit: _submit, isLoading: _loading, errorMessage: _error,
      children: [
        _AccountChip(account: widget.account),
        const SizedBox(height: 8),
        Text('Records the current outstanding balance as the statement balance.',
            style: Theme.of(context).textTheme.bodySmall
                ?.copyWith(color: Theme.of(context).colorScheme.outline)),
        const SizedBox(height: 16),
        AmountField(controller: _minPayment, label: 'Minimum Payment (optional)'),
      ],
    ),
  );
}

// ── Grant Temporary Limit ─────────────────────────────────────────────────────

class GrantLimitSheet extends ConsumerStatefulWidget {
  const GrantLimitSheet({super.key, required this.account});
  final AccountSummary account;
  @override ConsumerState<GrantLimitSheet> createState() => _GrantLimitSheetState();
}
class _GrantLimitSheetState extends ConsumerState<GrantLimitSheet> {
  final _formKey = GlobalKey<FormState>();
  final _limit   = TextEditingController();
  final _expires = TextEditingController();
  bool _loading  = false; String? _error;

  @override void dispose() { _limit.dispose(); _expires.dispose(); super.dispose(); }

  Future<void> _pickDate() async {
    final now    = DateTime.now();
    final picked = await showDatePicker(
      context: context,
      initialDate: now.add(const Duration(days: 30)),
      firstDate: now,
      lastDate: DateTime(now.year + 5),
    );
    if (picked != null) {
      _expires.text = '${picked.year}-${picked.month.toString().padLeft(2,'0')}-${picked.day.toString().padLeft(2,'0')}';
    }
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).grantTemporaryLimit(
        accountId: widget.account.accountId,
        newLimit: _limit.text.trim(),
        currency: widget.account.currency,
        expiresOn: _expires.text.trim(),
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) { setState(() { _error = e.toString(); _loading = false; }); }
  }

  @override
  Widget build(BuildContext context) => Form(
    key: _formKey,
    child: FormSheet(
      title: 'Grant Temporary Limit', submitLabel: 'Grant Limit',
      onSubmit: _submit, isLoading: _loading, errorMessage: _error,
      children: [
        _AccountChip(account: widget.account),
        const SizedBox(height: 16),
        AmountField(controller: _limit, label: 'New Temporary Limit'),
        const SizedBox(height: 12),
        TextFormField(
          controller: _expires, readOnly: true, onTap: _pickDate,
          decoration: const InputDecoration(
            labelText: 'Expires On', hintText: 'Tap to pick date',
            suffixIcon: Icon(Icons.calendar_today_rounded),
            border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
            filled: true,
          ),
          validator: (v) => (v == null || v.isEmpty) ? 'Required' : null,
        ),
      ],
    ),
  );
}

// ── Transfer (between asset accounts) ────────────────────────────────────────

class TransferSheet extends ConsumerStatefulWidget {
  const TransferSheet({
    super.key,
    required this.account,
    required this.allAccounts,
  });
  final AccountSummary       account;
  final List<AccountSummary> allAccounts;
  @override
  ConsumerState<TransferSheet> createState() => _TransferSheetState();
}

class _TransferSheetState extends ConsumerState<TransferSheet> {
  final _formKey = GlobalKey<FormState>();
  final _amount  = TextEditingController();
  final _label   = TextEditingController();
  final _desc    = TextEditingController();
  String? _toId;
  bool    _loading = false;
  String? _error;

  List<AccountSummary> get _destinations => widget.allAccounts
      .where((a) => !a.isDebt && a.accountId != widget.account.accountId)
      .toList();

  @override
  void initState() {
    super.initState();
    if (_destinations.isNotEmpty) _toId = _destinations.first.accountId;
  }

  @override
  void dispose() {
    _amount.dispose();
    _label.dispose();
    _desc.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    if (_toId == null) {
      setState(() => _error = 'Select a destination account');
      return;
    }
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).transferFunds(
        fromAccountId: widget.account.accountId,
        toAccountId:   _toId!,
        amount:        _amount.text.trim(),
        currency:      widget.account.currency,
        label:         _label.text.trim().isNotEmpty ? _label.text.trim() : null,
        description:   _desc.text.trim().isNotEmpty  ? _desc.text.trim()  : null,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    final destinations = _destinations;
    return Form(
      key: _formKey,
      child: FormSheet(
        title: 'Transfer',
        submitLabel: 'Transfer',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          _AccountChip(account: widget.account, label: 'From'),
          const SizedBox(height: 16),
          if (destinations.isEmpty)
            Container(
              padding: const EdgeInsets.all(14),
              decoration: BoxDecoration(
                color: AppTheme.amber.withOpacity(0.1),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: AppTheme.amber.withOpacity(0.3)),
              ),
              child: const Row(
                children: [
                  Icon(Icons.info_outline_rounded,
                      size: 16, color: AppTheme.amber),
                  SizedBox(width: 10),
                  Expanded(
                    child: Text(
                      'You need at least one other asset account to transfer to.',
                      style: TextStyle(
                          color: AppTheme.amber, fontSize: 13),
                    ),
                  ),
                ],
              ),
            )
          else ...[
            DropdownButtonFormField<String>(
              value: _toId,
              decoration: const InputDecoration(
                labelText: 'To account',
                prefixIcon: Icon(Icons.arrow_forward_rounded),
                border: OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(12))),
                filled: true,
              ),
              dropdownColor: AppTheme.surface,
              items: destinations.map((a) {
                final sym = a.currency == 'TWD' ? 'NT\$' : '\$';
                return DropdownMenuItem(
                  value: a.accountId,
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Flexible(
                        child: Text(a.accountName,
                            style: const TextStyle(
                                color: AppTheme.textPrimary),
                            overflow: TextOverflow.ellipsis),
                      ),
                      const SizedBox(width: 8),
                      Text('$sym ${a.balance}',
                          style: const TextStyle(
                              color: AppTheme.textSecondary,
                              fontSize: 12)),
                    ],
                  ),
                );
              }).toList(),
              onChanged: (v) => setState(() => _toId = v),
              validator: (v) => v == null ? 'Required' : null,
            ),
            const SizedBox(height: 12),
            AmountField(
              controller: _amount,
              label: 'Amount',
              prefixText:
                  widget.account.currency == 'TWD' ? 'NT\$ ' : '\$ ',
              autofocus: true,
            ),
            const SizedBox(height: 4),
            _BalanceHint(
              balance:  widget.account.balance,
              currency: widget.account.currency,
            ),
            const SizedBox(height: 12),
            LabelPickerField(controller: _label, hint: 'e.g. Savings transfer'),
            const SizedBox(height: 12),
            _DescriptionField(controller: _desc),
          ],
        ],
      ),
    );
  }
}

// ── Shared helper widgets ─────────────────────────────────────────────────────

class _AccountChip extends StatelessWidget {
  const _AccountChip({required this.account, this.label});
  final AccountSummary account; final String? label;
  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Row(children: [
      if (label != null) ...[
        Text(label!, style: Theme.of(context).textTheme.labelMedium?.copyWith(color: cs.outline)),
        const SizedBox(width: 8),
      ],
      Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
        decoration: BoxDecoration(color: cs.secondaryContainer, borderRadius: BorderRadius.circular(20)),
        child: Text(account.accountName,
            style: Theme.of(context).textTheme.labelLarge?.copyWith(color: cs.onSecondaryContainer)),
      ),
    ]);
  }
}

class _BalanceHint extends StatelessWidget {
  const _BalanceHint({required this.balance, required this.currency, this.label = 'Balance'});
  final String balance, currency, label;
  @override
  Widget build(BuildContext context) => Text('$label: $currency $balance',
      style: Theme.of(context).textTheme.bodySmall
          ?.copyWith(color: Theme.of(context).colorScheme.outline));
}

class _LabelField extends StatelessWidget {
  const _LabelField({required this.controller, this.hint});
  final TextEditingController controller;
  final String? hint;
  @override
  Widget build(BuildContext context) => TextField(
    controller: controller,
    textCapitalization: TextCapitalization.words,
    decoration: InputDecoration(
      labelText: 'Label (optional)',
      hintText: hint ?? 'e.g. Lunch, Salary, Rent',
      prefixIcon: const Icon(Icons.label_outline_rounded),
      border: const OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
      filled: true,
    ),
  );
}

class _DescriptionField extends StatelessWidget {
  const _DescriptionField({required this.controller, this.hint});
  final TextEditingController controller;
  final String? hint;
  @override
  Widget build(BuildContext context) => TextField(
    controller: controller,
    maxLines: 2,
    textCapitalization: TextCapitalization.sentences,
    decoration: InputDecoration(
      labelText: 'Description (optional)',
      hintText: hint ?? 'More details about this transaction',
      prefixIcon: const Padding(
        padding: EdgeInsets.only(bottom: 24),
        child: Icon(Icons.notes_rounded),
      ),
      border: const OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
      filled: true,
    ),
  );
}
