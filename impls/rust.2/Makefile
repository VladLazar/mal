STEPS = step0_repl step1_read_print step2_eval step3_env step4_if_fn_do step5_tco step6_file step7_quote step8_macros step9_try stepA_mal

all: $(STEPS)

dist: mal

mal: stepA_mal
	cp $< $@

$(STEPS):
	cargo build --manifest-path lisp/Cargo.toml
	cp lisp/target/debug/lisp $@

.PHONY: clean

clean:
	cargo clean --manifest-path lisp/Cargo.toml
	rm -f $(STEPS)
	rm -f mal
