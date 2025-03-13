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
    _debounce = Timer(const Duration(milliseconds: 300), () {
      if (_inputController.text.isNotEmpty) {
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

  void _convertText() {
    final input = _inputController.text;
    if (input.isEmpty) return;

    setState(() {
      _outputText = parseBangla(text: input, bijoy: false);
      _showSuggestions = false;
    });
  }

  void _reverseText() {
    if (_outputText.isEmpty) return;

    setState(() {
      _inputController.text = reverseBangla(text: _outputText);
      _outputText = '';
      _showSuggestions = false;
    });
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
                  decoration: const InputDecoration(
                    labelText: 'Enter English text (e.g., "ami banglay gan gai")',
                    border: OutlineInputBorder(),
                    hintText: 'Type for autocomplete suggestions',
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
            const SizedBox(height: 16),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 4.0),
                    child: ElevatedButton(
                      onPressed: _convertText,
                      child: const Text('Convert to Bangla'),
                    ),
                  ),
                ),
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 4.0),
                    child: ElevatedButton(
                      onPressed: _reverseText,
                      child: const Text('Reverse to English'),
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 24),
            const Text(
              'Output:',
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                border: Border.all(color: Colors.grey),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Text(
                _outputText,
                style: const TextStyle(fontSize: 20),
              ),
            ),
          ],
        ),
      ),
    );
  }

}
