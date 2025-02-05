PDF_ENGINE=typst

pdf :
	pandoc --pdf-engine=$(PDF_ENGINE) -o output.pdf README.md

clean:
	rm -rf output.*
