; Unicode installer example
Unicode true

Name "Unicode Test"
OutFile "unicode-test.exe"
RequestExecutionLevel user
ShowInstDetails show

Section "Unicode in UI"

  DetailPrint "Hello, World!"
  DetailPrint "שלום, עולם!"
  DetailPrint "مرحبا بالعالم!"
  DetailPrint "こんにちは、世界！"
  DetailPrint "你好，世界！"
  DetailPrint "привет, мир!"
  DetailPrint "안녕하세요!"
  DetailPrint "สวัสดีชาวโลก!"
  DetailPrint "Γεια σου, Κόσμε!"
  DetailPrint "Hej, Pair Stóry!"

  DetailPrint "${U+00A9}" ; arbitrary unicode char

SectionEnd
