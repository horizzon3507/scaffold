main.pdf: main.tex
	pdflatex -interaction=nonstopmode main.tex

watch:
	while inotifywait -e modify main.tex; do $(MAKE) main.pdf; done

clean:
	rm -f main.aux main.log main.out main.toc main.pdf

.PHONY: watch clean
