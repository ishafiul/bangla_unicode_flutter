// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// import 'package:flutter/material.dart';
// import 'package:bangla_unicode_flutter/bangla_unicode_flutter.dart';
//
// Future<void> main() async {
//   await RustLib.init();
//   runApp(const MyApp());
// }
//
// class MyApp extends StatelessWidget {
//   const MyApp({super.key});
//
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       home: Scaffold(
//         appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
//         body: Center(
//           child: Text(
//             'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
//           ),
//         ),
//       ),
//     );
//   }
// }
//

import 'package:flutter/material.dart';
import 'package:bangla_unicode_flutter/src/rust/api/bangla/parser.dart';
import 'package:bangla_unicode_flutter/src/rust/frb_generated.dart';
import 'dart:async';
import 'package:flutter/services.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bangla Unicode Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const BanglaUnicodeDemo(),
    );
  }
}

class BanglaUnicodeDemo extends StatefulWidget {
  const BanglaUnicodeDemo({super.key});

  @override
  State<BanglaUnicodeDemo> createState() => _BanglaUnicodeDemoState();
}

class _BanglaUnicodeDemoState extends State<BanglaUnicodeDemo> {
  final TextEditingController _inputController = TextEditingController();
  String _outputText = '';
  List<String> _suggestions = [];
  bool _showSuggestions = false;
  Timer? _debounce;

  @override
  void initState() {
    super.initState();
    _inputController.addListener(_onInputChanged);
  }

  @override
  void dispose() {
    _debounce?.cancel();
    _inputController.removeListener(_onInputChanged);
    _inputController.dispose();
    super.dispose();
  }

  void _onInputChanged() {
    if (_debounce?.isActive ?? false) _debounce!.cancel();
    _debounce = Timer(const Duration(milliseconds: 100), () {
      final input = _inputController.text;
      
      // Update the output text immediately
      if (input.isNotEmpty) {
        setState(() {
          _outputText = parseBangla(text: input, bijoy: false);
        });
      } else {
        setState(() {
          _outputText = '';
        });
      }
      
      // Get suggestions
      if (input.isNotEmpty) {
        _getSuggestions();
      } else {
        setState(() {
          _suggestions = [];
          _showSuggestions = false;
        });
      }
    });
  }

  void _getSuggestions() async {
    final input = _inputController.text;
    if (input.isEmpty) return;

    final suggestions = getAutocompleteSuggestions(
      partialText: input,
      maxSuggestions: 5,
    );

    setState(() {
      _suggestions = suggestions;
      _showSuggestions = suggestions.isNotEmpty;
    });
  }

  void _applySuggestion(String suggestion) {
    setState(() {
      _inputController.text = suggestion;
      _outputText = parseBangla(text: suggestion, bijoy: false);
      _showSuggestions = false;
    });
  }

  void _clearText() {
    setState(() {
      _inputController.clear();
      _outputText = '';
      _showSuggestions = false;
    });
  }
  
  void _copyToClipboard() {
    if (_outputText.isNotEmpty) {
      Clipboard.setData(ClipboardData(text: _outputText));
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Bangla text copied to clipboard'),
          duration: Duration(seconds: 2),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Bangla Unicode Demo'),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                TextField(
                  controller: _inputController,
                  decoration: InputDecoration(
                    labelText: 'Enter English text (e.g., "ami banglay gan gai")',
                    border: const OutlineInputBorder(),
                    hintText: 'Type for instant Bangla conversion',
                    suffixIcon: IconButton(
                      icon: const Icon(Icons.clear),
                      onPressed: _clearText,
                    ),
                  ),
                  maxLines: 3,
                ),
                if (_showSuggestions)
                  Container(
                    decoration: BoxDecoration(
                      border: Border.all(color: Colors.grey.shade300),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: _suggestions.map((suggestion) {
                        return InkWell(
                          onTap: () => _applySuggestion(suggestion),
                          child: Padding(
                            padding: const EdgeInsets.symmetric(
                              vertical: 8.0,
                              horizontal: 16.0,
                            ),
                            child: Text(
                              suggestion,
                              style: const TextStyle(fontSize: 16),
                            ),
                          ),
                        );
                      }).toList(),
                    ),
                  ),
              ],
            ),
            const SizedBox(height: 24),
            Row(
              children: [
                const Text(
                  'Bangla Output:',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const Spacer(),
                IconButton(
                  icon: const Icon(Icons.copy),
                  onPressed: _outputText.isEmpty ? null : _copyToClipboard,
                  tooltip: 'Copy to clipboard',
                ),
              ],
            ),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                border: Border.all(color: Colors.grey),
                borderRadius: BorderRadius.circular(8),
              ),
              constraints: const BoxConstraints(minHeight: 100),
              child: Text(
                _outputText,
                style: const TextStyle(fontSize: 20),
              ),
            ),
            const SizedBox(height: 16),
            const Text(
              'Popular Examples:',
              style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                children: [
                  _buildExampleChip('ami', 'আমি'),
                  _buildExampleChip('bangla', 'বাংলা'),
                  _buildExampleChip('bangladesh', 'বাংলাদেশ'),
                  _buildExampleChip('dhaka', 'ঢাকা'),
                  _buildExampleChip('aami', 'আমি'),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildExampleChip(String english, String bangla) {
    return Padding(
      padding: const EdgeInsets.only(right: 8.0),
      child: ActionChip(
        label: Text(english),
        tooltip: bangla,
        onPressed: () {
          setState(() {
            _inputController.text = english;
            _outputText = bangla;
            _showSuggestions = false;
          });
        },
      ),
    );
  }
}
