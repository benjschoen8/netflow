import '../../theme/app_theme.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../models/account_summary.dart';
import '../../shared/widgets/amount_field.dart';
import '../../shared/widgets/form_sheet.dart';

// ── Add Holding ───────────────────────────────────────────────────────────────

class AddHoldingSheet extends ConsumerStatefulWidget {
  const AddHoldingSheet({super.key, required this.account});
  final AccountSummary account;

  @override
  ConsumerState<AddHoldingSheet> createState() => _AddHoldingSheetState();
}

class _AddHoldingSheetState extends ConsumerState<AddHoldingSheet> {
  final _formKey   = GlobalKey<FormState>();
  final _ticker    = TextEditingController();
  final _qty       = TextEditingController();
  final _price     = TextEditingController();
  String _type     = 'stock';
  bool _loading    = false;
  String? _error;

  static const _types = [
    ('stock',       'Stock'),
    ('etf',         'ETF'),
    ('mutual-fund', 'Mutual Fund'),
    ('bond',        'Bond'),
    ('crypto',      'Crypto'),
    ('other',       'Other'),
  ];

  @override
  void dispose() { _ticker.dispose(); _qty.dispose(); _price.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).addHolding(
        accountId: widget.account.accountId,
        ticker: _ticker.text.trim().toUpperCase(),
        investmentType: _type,
        quantity: _qty.text.trim(),
        unitPrice: _price.text.trim(),
        currency: widget.account.currency,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: FormSheet(
        title: 'Add Holding',
        submitLabel: 'Add',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(
            controller: _ticker, label: 'Ticker Symbol',
            hint: 'e.g. AAPL, BTC, 2330',
            keyboardType: TextInputType.text,
          ),
          const SizedBox(height: 12),
          DropdownButtonFormField<String>(
            value: _type,
            decoration: const InputDecoration(
              labelText: 'Type',
              border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
              filled: true,
            ),
            items: _types.map((t) =>
                DropdownMenuItem(value: t.$1, child: Text(t.$2))).toList(),
            onChanged: (v) => setState(() => _type = v!),
          ),
          const SizedBox(height: 12),
          AmountField(controller: _qty, label: 'Quantity', hint: 'e.g. 10'),
          const SizedBox(height: 12),
          AmountField(
            controller: _price, label: 'Unit Price',
            prefixText: widget.account.currency == 'TWD' ? 'NT\$ ' : '\$ ',
          ),
        ],
      ),
    );
  }
}

// ── Update Price ──────────────────────────────────────────────────────────────

class UpdatePriceSheet extends ConsumerStatefulWidget {
  const UpdatePriceSheet({
    super.key,
    required this.account,
    required this.ticker,
  });
  final AccountSummary account;
  final String ticker;

  @override
  ConsumerState<UpdatePriceSheet> createState() => _UpdatePriceSheetState();
}

class _UpdatePriceSheetState extends ConsumerState<UpdatePriceSheet> {
  final _formKey = GlobalKey<FormState>();
  final _price   = TextEditingController();
  bool _loading  = false;
  String? _error;

  @override
  void dispose() { _price.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).updateHoldingPrice(
        accountId: widget.account.accountId,
        ticker: widget.ticker,
        newPrice: _price.text.trim(),
        currency: widget.account.currency,
      );
      if (mounted) Navigator.pop(context, true);
    } catch (e) {
      setState(() { _error = e.toString(); _loading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: FormSheet(
        title: 'Update Price — ${widget.ticker}',
        submitLabel: 'Update',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AmountField(
            controller: _price,
            label: 'New Unit Price',
            prefixText: widget.account.currency == 'TWD' ? 'NT\$ ' : '\$ ',
            autofocus: true,
          ),
        ],
      ),
    );
  }
}
