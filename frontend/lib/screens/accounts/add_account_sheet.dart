import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/providers.dart';
import '../../shared/widgets/amount_field.dart';
import '../../shared/widgets/form_sheet.dart';

// ── Entry point ───────────────────────────────────────────────────────────────

/// Shows account type picker, then the appropriate form.
Future<void> showAddAccountSheet(BuildContext context, WidgetRef ref) async {
  final type = await showDialog<String>(
    context: context,
    barrierColor: Colors.black.withOpacity(0.5),
    builder: (_) => const _AccountTypePicker(),
  );
  if (type == null || !context.mounted) return;

  final added = await showFormSheet<bool>(
    context,
    _formForType(type, ref),
  );
  if (added == true) refreshDashboard(ref);
}

Widget _formForType(String type, WidgetRef ref) {
  switch (type) {
    case 'cash':
      return _CashForm(ref: ref);
    case 'wallet':
      return _WalletForm(ref: ref);
    case 'digital_wallet':
      return _DigitalWalletForm(ref: ref);
    case 'investment':
      return _InvestmentForm(ref: ref);
    case 'credit_card':
      return _CreditCardForm(ref: ref);
    case 'loan':
      return _LoanForm(ref: ref);
    default:
      return const SizedBox.shrink();
  }
}

// ── Type picker ───────────────────────────────────────────────────────────────

class _AccountTypePicker extends StatelessWidget {
  const _AccountTypePicker();

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final types = [
      ('cash',          Icons.account_balance_rounded,    'Bank Account',      'Savings, checking, salary'),
      ('wallet',        Icons.wallet_rounded,             'Cash Wallet',       'Physical cash on hand'),
      ('digital_wallet',Icons.contactless_rounded,        'Digital Wallet',    'LINE Pay, Apple Pay, etc.'),
      ('investment',    Icons.show_chart_rounded,         'Investment Account','Stocks, ETFs, crypto'),
      ('credit_card',   Icons.credit_card_rounded,        'Credit Card',       'Revolving credit'),
      ('loan',          Icons.request_quote_rounded,      'Loan',              'Mortgage, personal loan'),
    ];

    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 420),
        child: Material(
          color: Colors.transparent,
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 24, vertical: 32),
            decoration: BoxDecoration(
              color: cs.surface,
              borderRadius: BorderRadius.circular(20),
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.25),
                  blurRadius: 40,
                  offset: const Offset(0, 8),
                ),
              ],
            ),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Padding(
                  padding: const EdgeInsets.fromLTRB(20, 20, 16, 12),
                  child: Row(
                    children: [
                      Text('Add Account',
                          style: Theme.of(context).textTheme.titleLarge
                              ?.copyWith(fontWeight: FontWeight.w700)),
                      const Spacer(),
                      IconButton(
                        onPressed: () => Navigator.pop(context),
                        icon: const Icon(Icons.close_rounded),
                        style: IconButton.styleFrom(
                          backgroundColor: cs.surfaceVariant,
                          foregroundColor: cs.onSurfaceVariant,
                          minimumSize: const Size(36, 36),
                          maximumSize: const Size(36, 36),
                          padding: EdgeInsets.zero,
                        ),
                      ),
                    ],
                  ),
                ),
                const Divider(height: 1),
                ...types.map((t) => ListTile(
                      leading: Container(
                        width: 40, height: 40,
                        decoration: BoxDecoration(
                          color: cs.secondaryContainer,
                          borderRadius: BorderRadius.circular(10),
                        ),
                        child: Icon(t.$2, size: 20, color: cs.onSecondaryContainer),
                      ),
                      title: Text(t.$3,
                          style: const TextStyle(fontWeight: FontWeight.w600)),
                      subtitle: Text(t.$4,
                          style: TextStyle(color: cs.outline, fontSize: 12)),
                      trailing: Icon(Icons.chevron_right_rounded, color: cs.outline),
                      onTap: () => Navigator.pop(context, t.$1),
                    )),
                const SizedBox(height: 16),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

// ── Cash Account form ─────────────────────────────────────────────────────────

class _CashForm extends ConsumerStatefulWidget {
  const _CashForm({required this.ref});
  final WidgetRef ref;

  @override
  ConsumerState<_CashForm> createState() => _CashFormState();
}

class _CashFormState extends ConsumerState<_CashForm> {
  final _formKey   = GlobalKey<FormState>();
  final _name      = TextEditingController();
  final _number    = TextEditingController();
  final _bank      = TextEditingController();
  final _balance   = TextEditingController(text: '0');
  String _currency = 'TWD';
  bool _loading    = false;
  String? _error;

  @override
  void dispose() {
    _name.dispose(); _number.dispose(); _bank.dispose(); _balance.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).openCashAccount(
        name: _name.text.trim(),
        accountNumber: _number.text.trim(),
        bank: _bank.text.trim(),
        currency: _currency,
        initialBalance: _balance.text.trim(),
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
        title: 'Bank Account',
        submitLabel: 'Add Account',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(controller: _name, label: 'Account Name', hint: 'e.g. CTBC Savings'),
          const SizedBox(height: 12),
          AppTextField(controller: _bank, label: 'Bank', hint: 'e.g. CTBC'),
          const SizedBox(height: 12),
          AppTextField(controller: _number, label: 'Account Number'),
          const SizedBox(height: 12),
          CurrencyDropdown(value: _currency, onChanged: (v) => setState(() => _currency = v!)),
          const SizedBox(height: 12),
          AmountField(controller: _balance, label: 'Initial Balance'),
        ],
      ),
    );
  }
}

// ── Physical Wallet form ──────────────────────────────────────────────────────

class _WalletForm extends ConsumerStatefulWidget {
  const _WalletForm({required this.ref});
  final WidgetRef ref;

  @override
  ConsumerState<_WalletForm> createState() => _WalletFormState();
}

class _WalletFormState extends ConsumerState<_WalletForm> {
  final _formKey = GlobalKey<FormState>();
  final _name    = TextEditingController();
  final _balance = TextEditingController(text: '0');
  String _currency = 'TWD';
  bool _loading = false;
  String? _error;

  @override
  void dispose() { _name.dispose(); _balance.dispose(); super.dispose(); }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).addPhysicalWallet(
        name: _name.text.trim(),
        currency: _currency,
        initialBalance: _balance.text.trim(),
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
        title: 'Cash Wallet',
        submitLabel: 'Add Wallet',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(controller: _name, label: 'Wallet Name', hint: 'e.g. Everyday Cash'),
          const SizedBox(height: 12),
          CurrencyDropdown(value: _currency, onChanged: (v) => setState(() => _currency = v!)),
          const SizedBox(height: 12),
          AmountField(controller: _balance, label: 'Current Balance'),
        ],
      ),
    );
  }
}

// ── Digital Wallet form ───────────────────────────────────────────────────────

class _DigitalWalletForm extends ConsumerStatefulWidget {
  const _DigitalWalletForm({required this.ref});
  final WidgetRef ref;

  @override
  ConsumerState<_DigitalWalletForm> createState() => _DigitalWalletFormState();
}

class _DigitalWalletFormState extends ConsumerState<_DigitalWalletForm> {
  final _formKey    = GlobalKey<FormState>();
  final _name       = TextEditingController();
  final _providerId = TextEditingController();
  final _balance    = TextEditingController(text: '0');
  String _currency  = 'TWD';
  String _provider  = 'line-pay';
  bool _loading     = false;
  String? _error;

  static const _providers = [
    ('line-pay',   'LINE Pay'),
    ('apple-pay',  'Apple Pay'),
    ('google-pay', 'Google Pay'),
    ('jko-pay',    'JKO Pay'),
    ('pi-wallet',  'Pi Wallet'),
    ('taiwan-pay', 'Taiwan Pay'),
    ('other',      'Other'),
  ];

  @override
  void dispose() {
    _name.dispose(); _providerId.dispose(); _balance.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).addDigitalWallet(
        name: _name.text.trim(),
        provider: _provider,
        providerAccountId: _providerId.text.trim(),
        currency: _currency,
        initialBalance: _balance.text.trim(),
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
        title: 'Digital Wallet',
        submitLabel: 'Add Wallet',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(controller: _name, label: 'Wallet Name'),
          const SizedBox(height: 12),
          DropdownButtonFormField<String>(
            value: _provider,
            decoration: const InputDecoration(
              labelText: 'Provider',
              border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
              filled: true,
            ),
            items: _providers.map((p) =>
                DropdownMenuItem(value: p.$1, child: Text(p.$2))).toList(),
            onChanged: (v) => setState(() => _provider = v!),
          ),
          const SizedBox(height: 12),
          AppTextField(controller: _providerId, label: 'Phone / Account ID',
              hint: 'e.g. 0912345678'),
          const SizedBox(height: 12),
          CurrencyDropdown(value: _currency, onChanged: (v) => setState(() => _currency = v!)),
          const SizedBox(height: 12),
          AmountField(controller: _balance, label: 'Current Balance'),
        ],
      ),
    );
  }
}

// ── Investment Account form ───────────────────────────────────────────────────

class _InvestmentForm extends ConsumerStatefulWidget {
  const _InvestmentForm({required this.ref});
  final WidgetRef ref;

  @override
  ConsumerState<_InvestmentForm> createState() => _InvestmentFormState();
}

class _InvestmentFormState extends ConsumerState<_InvestmentForm> {
  final _formKey   = GlobalKey<FormState>();
  final _name      = TextEditingController();
  final _number    = TextEditingController();
  final _bank      = TextEditingController();
  final _cash      = TextEditingController(text: '0');
  String _currency = 'TWD';
  bool _loading    = false;
  String? _error;

  @override
  void dispose() {
    _name.dispose(); _number.dispose(); _bank.dispose(); _cash.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).openInvestmentAccount(
        name: _name.text.trim(),
        accountNumber: _number.text.trim(),
        bank: _bank.text.trim(),
        currency: _currency,
        cashBalance: _cash.text.trim(),
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
        title: 'Investment Account',
        submitLabel: 'Open Account',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(controller: _name, label: 'Account Name'),
          const SizedBox(height: 12),
          AppTextField(controller: _bank, label: 'Brokerage / Bank'),
          const SizedBox(height: 12),
          AppTextField(controller: _number, label: 'Account Number'),
          const SizedBox(height: 12),
          CurrencyDropdown(value: _currency, onChanged: (v) => setState(() => _currency = v!)),
          const SizedBox(height: 12),
          AmountField(controller: _cash, label: 'Cash Balance'),
        ],
      ),
    );
  }
}

// ── Credit Card form ──────────────────────────────────────────────────────────

class _CreditCardForm extends ConsumerStatefulWidget {
  const _CreditCardForm({required this.ref});
  final WidgetRef ref;

  @override
  ConsumerState<_CreditCardForm> createState() => _CreditCardFormState();
}

class _CreditCardFormState extends ConsumerState<_CreditCardForm> {
  final _formKey      = GlobalKey<FormState>();
  final _name         = TextEditingController();
  final _lastFour     = TextEditingController();
  final _limit        = TextEditingController();
  final _outstanding  = TextEditingController(text: '0');
  final _rate         = TextEditingController();
  final _stmtDay      = TextEditingController(text: '15');
  final _dueDay       = TextEditingController(text: '10');
  final _expiryMonth  = TextEditingController();
  final _expiryYear   = TextEditingController();
  String _currency    = 'TWD';
  String _network     = 'visa';
  bool _loading       = false;
  String? _error;

  static const _networks = [
    ('visa', 'Visa'),
    ('mastercard', 'Mastercard'),
    ('amex', 'American Express'),
    ('unionpay', 'UnionPay'),
    ('discover', 'Discover'),
    ('other', 'Other'),
  ];

  @override
  void dispose() {
    _name.dispose(); _lastFour.dispose(); _limit.dispose();
    _outstanding.dispose(); _rate.dispose(); _stmtDay.dispose();
    _dueDay.dispose(); _expiryMonth.dispose(); _expiryYear.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).addCreditCard(
        name: _name.text.trim(),
        lastFour: _lastFour.text.trim(),
        network: _network,
        expiryMonth: int.parse(_expiryMonth.text.trim()),
        expiryYear: int.parse(_expiryYear.text.trim()),
        creditLimit: _limit.text.trim(),
        currency: _currency,
        outstanding: _outstanding.text.trim(),
        statementDay: int.parse(_stmtDay.text.trim()),
        dueDay: int.parse(_dueDay.text.trim()),
        interestRate: _rate.text.trim().isNotEmpty ? _rate.text.trim() : null,
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
        title: 'Credit Card',
        submitLabel: 'Add Card',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(controller: _name, label: 'Card Name', hint: 'e.g. CTBC Visa Infinite'),
          const SizedBox(height: 12),
          DropdownButtonFormField<String>(
            value: _network,
            decoration: const InputDecoration(
              labelText: 'Network',
              border: OutlineInputBorder(borderRadius: BorderRadius.all(Radius.circular(12))),
              filled: true,
            ),
            items: _networks.map((n) =>
                DropdownMenuItem(value: n.$1, child: Text(n.$2))).toList(),
            onChanged: (v) => setState(() => _network = v!),
          ),
          const SizedBox(height: 12),
          AppTextField(
            controller: _lastFour,
            label: 'Last 4 Digits',
            hint: '1234',
            keyboardType: TextInputType.number,
            inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(4)],
          ),
          const SizedBox(height: 12),
          Row(children: [
            Expanded(child: AppTextField(
              controller: _expiryMonth, label: 'Exp Month', hint: 'MM',
              keyboardType: TextInputType.number,
              inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(2)],
            )),
            const SizedBox(width: 12),
            Expanded(child: AppTextField(
              controller: _expiryYear, label: 'Exp Year', hint: 'YYYY',
              keyboardType: TextInputType.number,
              inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(4)],
            )),
          ]),
          const SizedBox(height: 12),
          CurrencyDropdown(value: _currency, onChanged: (v) => setState(() => _currency = v!)),
          const SizedBox(height: 12),
          AmountField(controller: _limit, label: 'Credit Limit'),
          const SizedBox(height: 12),
          AmountField(controller: _outstanding, label: 'Current Outstanding'),
          const SizedBox(height: 12),
          Row(children: [
            Expanded(child: AppTextField(
              controller: _stmtDay, label: 'Statement Day',
              keyboardType: TextInputType.number,
              inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(2)],
            )),
            const SizedBox(width: 12),
            Expanded(child: AppTextField(
              controller: _dueDay, label: 'Due Day',
              keyboardType: TextInputType.number,
              inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(2)],
            )),
          ]),
          const SizedBox(height: 12),
          AppTextField(
            controller: _rate, label: 'Annual Interest Rate %',
            hint: 'e.g. 15.99', optional: true,
            keyboardType: const TextInputType.numberWithOptions(decimal: true),
          ),
        ],
      ),
    );
  }
}

// ── Loan form ─────────────────────────────────────────────────────────────────

class _LoanForm extends ConsumerStatefulWidget {
  const _LoanForm({required this.ref});
  final WidgetRef ref;

  @override
  ConsumerState<_LoanForm> createState() => _LoanFormState();
}

class _LoanFormState extends ConsumerState<_LoanForm> {
  final _formKey    = GlobalKey<FormState>();
  final _name       = TextEditingController();
  final _bank       = TextEditingController();
  final _creditor   = TextEditingController();
  final _principal  = TextEditingController();
  final _number     = TextEditingController();
  final _rate       = TextEditingController();
  final _dueDay     = TextEditingController();
  final _maturity   = TextEditingController();
  final _minPayment = TextEditingController();
  String _currency  = 'TWD';
  bool _loading     = false;
  String? _error;

  @override
  void dispose() {
    _name.dispose(); _bank.dispose(); _creditor.dispose();
    _principal.dispose(); _number.dispose(); _rate.dispose();
    _dueDay.dispose(); _maturity.dispose(); _minPayment.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() { _loading = true; _error = null; });
    try {
      await ref.read(ledgerRepoProvider).openLoan(
        name: _name.text.trim(),
        bank: _bank.text.trim(),
        creditor: _creditor.text.trim(),
        currency: _currency,
        principal: _principal.text.trim(),
        accountNumber: _number.text.trim().isNotEmpty ? _number.text.trim() : null,
        interestRate: _rate.text.trim().isNotEmpty ? _rate.text.trim() : null,
        dueDay: _dueDay.text.trim().isNotEmpty ? int.tryParse(_dueDay.text.trim()) : null,
        maturityDate: _maturity.text.trim().isNotEmpty ? _maturity.text.trim() : null,
        minimumPayment: _minPayment.text.trim().isNotEmpty ? _minPayment.text.trim() : null,
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
        title: 'Loan',
        submitLabel: 'Add Loan',
        onSubmit: _submit,
        isLoading: _loading,
        errorMessage: _error,
        children: [
          AppTextField(controller: _name, label: 'Loan Name', hint: 'e.g. Home Mortgage'),
          const SizedBox(height: 12),
          AppTextField(controller: _creditor, label: 'Creditor', hint: 'e.g. Cathay Bank'),
          const SizedBox(height: 12),
          AppTextField(controller: _bank, label: 'Bank'),
          const SizedBox(height: 12),
          CurrencyDropdown(value: _currency, onChanged: (v) => setState(() => _currency = v!)),
          const SizedBox(height: 12),
          AmountField(controller: _principal, label: 'Principal Amount'),
          const SizedBox(height: 12),
          AppTextField(controller: _number, label: 'Account Number', optional: true),
          const SizedBox(height: 12),
          AppTextField(
            controller: _rate, label: 'Annual Interest Rate %',
            hint: 'e.g. 3.5', optional: true,
            keyboardType: const TextInputType.numberWithOptions(decimal: true),
          ),
          const SizedBox(height: 12),
          AppTextField(
            controller: _dueDay, label: 'Monthly Due Day (1-31)',
            optional: true, keyboardType: TextInputType.number,
            inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(2)],
          ),
          const SizedBox(height: 12),
          AppTextField(controller: _maturity, label: 'Maturity Date', hint: 'YYYY-MM-DD', optional: true),
          const SizedBox(height: 12),
          AmountField(controller: _minPayment, label: 'Minimum Monthly Payment'),
        ],
      ),
    );
  }
}
