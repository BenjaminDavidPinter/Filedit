# Png Parser

The goal of this project is to read, parse, and make sense of a png file's bytes via the spec, and only the spec.

Current Chunk Completion;
- Png Signature
- IHDR
- iCCP (Partial)

Example output;

```
===="IHDR"====
Width: 1446
Height: 986
Bit Depth: 8
Color Type: TruecolorWithAlpha
Compression Method: DeflateInflate
Filter Method: Method0
Interface Method: Method0
===="iCCP"====
Profile Name: "ICC Profile"
Compression Method: Method0
Profile Data:
	Profile Size: 4032
	Preferred CMM: "appl"
	Profile Version: 2.16.0.0
	Profile Class: "mntr"
	Color Space: "RGB "
	PCS Encoding: "XYZ "
	Profile Created On: "3/25/2023 12:40:00"
	Profile Signature: "acsp"
	Primary Platform: "APPL"
===="eXIf"====
===="pHYs"====
===="iTXt"====
===="iDOT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IDAT"====
===="IEND"====
```