
// class CSSParser:
//     def __init__(self, s):
//         self.s = s
//         self.i = 0

//     def whitespace(self):
//         while self.i < len(self.s) and self.s[self.i].isspace():
//             self.i += 1

//     def word(self):
//         start = self.i
//         while self.i < len(self.s):
//             if self.s[self.i].isalnum() or self.s[self.i] in "#-.%":
//                 self.i += 1
//             else:
//                 break
//         assert self.i > start
//         return self.s[start:self.i]

//     def literal(self, literal):
//         assert self.i < len(self.s) and self.s[self.i] == literal
//         self.i += 1

//     def pair(self):
//         prop = self.word()
//         self.whitespace()
//         self.literal(":")
//         self.whitespace()
//         val = self.word()
//         return prop.lower(), val

//     def body(self):
//         pairs = {}
//         while self.i < len(self.s):
//             try:
//                 prop, val = self.pair()
//                 pairs[prop.lower()] = val
//                 self.whitespace()
//                 self.literal(";")
//                 self.whitespace()
//             except AssertionError:
//                 why = self.ignore_until([";"])
//                 if why == ";":
//                     self.literal(";")
//                     self.whitespace()
//                 else:
//                     break
//         return pairs

//     def ignore_until(self, chars):
//         while self.i < len(self.s):
//             if self.s[self.i] in chars:
//                 return self.s[self.i]
//             else:
//                 self.i += 1

